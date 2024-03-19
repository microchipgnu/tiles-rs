#[derive(Clone, Copy, Debug)]
pub enum Biome {
    Forest,
    Desert,
    Water,
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub biome: Biome,
    pub altitude: f64,
}


pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn print(&self) {
        for row in &self.tiles {
            for tile in row {
                let symbol = match tile.biome {
                    Biome::Forest => "F",
                    Biome::Desert => "D",
                    Biome::Water => "W",
                };
                print!("{}", symbol);
            }
            println!();
        }
    }
}
