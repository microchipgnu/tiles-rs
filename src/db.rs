use crate::tiles::{Map, Biome};
use rusqlite::{params, Connection, Result};

pub fn store_map_in_db(map: &Map) -> Result<()> {
    let mut conn = Connection::open("map.db")?;

    conn.execute("DROP TABLE IF EXISTS tile", [])?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tile (
            id INTEGER PRIMARY KEY,
            x INTEGER NOT NULL,
            y INTEGER NOT NULL,
            biome TEXT NOT NULL
        )",
        [],
    )?;

    // Start a transaction
    let tx = conn.transaction()?;

    {
        // Prepare the SQL statement within its own scope
        let mut stmt = tx.prepare("INSERT INTO tile (x, y, biome) VALUES (?1, ?2, ?3)")?;

        for (y, row) in map.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                stmt.execute(params![x as i32, y as i32, format!("{:?}", tile.biome)])?;
            }
        }
    } // Statement `stmt` goes out of scope here, so it's dropped before committing.

    // Commit the transaction
    tx.commit()?;

    Ok(())
}
