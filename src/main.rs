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

    let database = matches.value_of("database").unwrap().to_string();
    let path = matches.value_of("path").unwrap().to_string();

    gpodderid3(database, path)
}

fn gpodderid3(database: String, path: String) -> Result<()> {
    let conn = Connection::open(database)?;

    let mut stmt = conn.prepare(
        "
	select
          episode.title,
          episode.description,
          episode.mime_type,
          episode.download_filename,
          podcast.title,podcast.download_folder
        from episode,podcast
	where podcast.id=podcast_id AND download_filename IS NOT NULL
	",
    )?;

    let episodes = stmt.query_map(NO_PARAMS, |row| {
        Ok(Episode {
            title: row.get(0)?,
            description: row.get(1)?,
            mime_type: row.get(2)?,
            download_filename: row.get(3)?,
            podcast_title: row.get(4)?,
            download_folder: row.get(5)?,
        })
    })?;

    for episode in episodes {
        let e = episode?.clone();

        let path = format!("{}/{}/{}", path, e.download_folder, e.download_filename);

        println!("path:{}", path);

        if Path::new(&path).exists() {
            println!("path exists!");

            let mut tag = read_or_new_tag(&path);
            let tag2 = tag.clone();

            tag.set_title(tag2.title().unwrap_or_else(|| e.title.as_str()));
            tag.set_album(tag2.album().unwrap_or_else(|| e.podcast_title.as_str()));

            tag.write_to_path(&path, Version::Id3v24).unwrap();
        }
    }

    Ok(())
}

fn read_or_new_tag(path: &str) -> Tag {
    println!("path:{}", path);

    let tag = Tag::read_from_path(&path);

    match tag {
        Ok(tag) => tag,
        Err(error) => match error.kind {
            ErrorKind::NoTag => Tag::new(),
            other_error => panic!("Problem reading tag: {:?}", other_error),
        },
    }
}
