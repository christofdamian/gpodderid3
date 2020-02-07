extern crate rusqlite;

use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};
use std::path::Path;
use id3::{Tag, Version, ErrorKind};

#[derive(Debug)]
#[derive(Clone)]
struct Episode {
    title: String,
    description: String,
    mime_type: String,
    download_filename: String,
    podcast_title: String,
    download_folder: String,
}

fn main() -> Result<()> {
    let conn = Connection::open("gpodder.db")?;

    let mut stmt = conn.prepare(
	"select episode.title,episode.description,episode.mime_type,episode.download_filename,podcast.title,podcast.download_folder from episode,podcast where podcast.id=podcast_id"
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

	let path = format!("{}/{}", e.download_folder, e.download_filename);

	if Path::new(&path).exists() {
	    println!("path:{}", path);
	    println!("path exists!");

	    let mut tag = read_or_new_tag(&path);
	    let tag2 = tag.clone();

	    tag.set_title(tag2.title().unwrap_or(e.title.as_str()));
	    tag.set_album(tag2.album().unwrap_or(e.podcast_title.as_str()));

	    tag.write_to_path(&path, Version::Id3v24).unwrap();
	}
    }

    Ok(())
}

fn read_or_new_tag(path: &String) -> Tag {
    println!("path:{}", path);

    let tag = Tag::read_from_path(&path);

    let tag = match tag {
	Ok(tag) => tag,
	Err(error) => match error.kind {
	    ErrorKind::NoTag => Tag::new(),
	    other_error => panic!("Problem reading tag: {:?}", other_error),
	},
    };
    tag
}
