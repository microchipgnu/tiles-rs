use crate::tiles::{Map, Tile, Biome};
use noise::{NoiseFn, Perlin};
use rand::Rng;
use rayon::prelude::*;

pub fn generate_map(width: usize, height: usize) -> Map {
    let perlin = Perlin::new();
    let mut rng = rand::thread_rng();
    let noise_offset_x = rng.gen_range(0.0..10000.0);
    let noise_offset_y = rng.gen_range(0.0..10000.0);
    let noise_scale = 0.1;

    let tiles: Vec<Vec<Tile>> = (0..height).into_par_iter().map(|y| {
        (0..width).map(|x| {
            let noise_value = perlin.get([x as f64 * noise_scale + noise_offset_x, y as f64 * noise_scale + noise_offset_y]);
            Tile {
                biome: if noise_value < -0.1 { Biome::Water } else if noise_value < 0.2 { Biome::Forest } else { Biome::Desert },
            }
        }).collect()
    }).collect();

    Map { tiles, width, height }
}
