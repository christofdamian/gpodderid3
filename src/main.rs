extern crate rusqlite;

use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};
// use std::collections::HashMap;

#[derive(Debug)]
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
        println!("Found episode {:?}", episode);
    }

    Ok(())
}
