use crate::tiles::{Map, Tile};
use rusqlite::{params, Connection, Result};

pub fn store_map_in_db(map: &Map) -> Result<()> {
    let conn = Connection::open("map.db")?;

    conn.execute(
        "DROP TABLE IF EXISTS tile",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tile (
             id INTEGER PRIMARY KEY,
             x INTEGER NOT NULL,
             y INTEGER NOT NULL,
             biome TEXT NOT NULL
         )",
        [],
    )?;

    for (y, row) in map.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            conn.execute(
                "INSERT INTO tile (x, y, biome) VALUES (?1, ?2, ?3)",
                params![x as i32, y as i32, format!("{:?}", tile.biome)],
            )?;
        }
    }

    Ok(())
}
