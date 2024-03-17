use crate::tiles::{Map, Tile, Biome};
use noise::{NoiseFn, Perlin};
use rand::Rng;

pub fn generate_map(width: usize, height: usize) -> Map {
    let perlin = Perlin::new();
    let mut tiles = vec![vec![Tile { biome: Biome::Water }; width]; height];

    let mut rng = rand::thread_rng();
    let noise_offset_x = rng.gen_range(0.0..10000.0);
    let noise_offset_y = rng.gen_range(0.0..10000.0);
    let noise_scale = 0.1;

    for y in 0..height {
        for x in 0..width {
            let noise_value = perlin.get([x as f64 * noise_scale + noise_offset_x, y as f64 * noise_scale + noise_offset_y]);
            tiles[y][x] = Tile {
                biome: if noise_value < -0.1 { Biome::Water } else if noise_value < 0.2 { Biome::Forest } else { Biome::Desert },
            };
        }
    }

    Map { tiles, width, height }
}