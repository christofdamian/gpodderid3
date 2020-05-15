extern crate rusqlite;

use clap::{App, Arg};
use id3::{ErrorKind, Tag, Version};
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};
use std::path::Path;

#[derive(Debug, Clone)]
struct Episode {
    title: String,
    description: String,
    mime_type: String,
    download_filename: String,
    podcast_title: String,
    download_folder: String,
}

fn main() -> Result<()> {
    let matches = App::new("gpooderid3")
        .version("0.1")
        .author("Christof Damian <christof@damian.net>")
        .about("Add missing tags to gpodder downloads")
        .arg(
            Arg::with_name("database")
                .short("d")
                .long("database")
                .help("Sets gpodder database")
                .takes_value(true)
                .default_value("gpodder.db"),
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .help("Sets download path")
                .takes_value(true)
                .default_value("."),
        )
        .get_matches();

    let database = matches.value_of("database").unwrap();
    let path = matches.value_of("path").unwrap();

    gpodderid3(database, path)
}

fn gpodderid3(database: &str, path: &str) -> Result<()> {
    let conn = Connection::open(database)?;

    let mut stmt = conn.prepare(
        "
	select
          episode.title,
          episode.description,
          episode.mime_type,
          episode.download_filename,
          podcast.title AS podcast_title,
          podcast.download_folder
        from episode,podcast
	where podcast.id=podcast_id AND download_filename IS NOT NULL
	",
    )?;

    let episode_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok(Episode {
            title: row.get("title")?,
            description: row.get("description")?,
            mime_type: row.get("mime_type")?,
            download_filename: row.get("download_filename")?,
            podcast_title: row.get("podcast_title")?,
            download_folder: row.get("download_folder")?,
        })
    })?;

    for episode in episode_iter {
        episode_tag(path, &episode.unwrap())?;
    }

    Ok(())
}

fn episode_tag(base_path: &str, episode: &Episode) -> Result<()> {
    let path = episode_path(base_path, episode);

    if Path::new(&path).exists() {
        println!("processing :{}", path);

        let mut tag = read_or_new_tag(&path);
        let mut modified = false;

        if tag.title().unwrap_or("")=="" {
            tag.set_title(
                tag.title().unwrap_or_else(|| episode.title.as_str()).to_owned()
            );
            modified = true;
        }
        if tag.album().unwrap_or("")=="" {
            tag.set_album(
                tag.album().unwrap_or_else(|| episode.podcast_title.as_str()).to_owned()
            );
            modified = true;
        }

        if modified {
            println!("updating tags");
            if let Err(_err) = tag.write_to_path(&path, Version::Id3v24) {
                println!("Failed to write tag");
            }
        }
    }

    Ok(())
}

fn episode_path(base_path: &str, episode: &Episode) -> String {
    format!(
        "{}/{}/{}",
        base_path,
        episode.download_folder,
        episode.download_filename
    )
}

fn read_or_new_tag(path: &str) -> Tag {
    let tag = Tag::read_from_path(&path);

    match tag {
        Ok(tag) => tag,
        Err(error) => match error.kind {
            ErrorKind::NoTag => Tag::new(),
            other_error => panic!("Problem reading tag: {:?}", other_error),
        },
    }
}
