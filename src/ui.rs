use ggez::{event, graphics, Context, GameResult};

use crate::tiles::{Map, Biome};
use crate::db::store_map_in_db;
use crate::map_generator::generate_map;


pub struct GameState {
    pub map: Map,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let map = generate_map(100, 100);
        
        // Attempt to store the generated map in the database
        match store_map_in_db(&map) {
            Ok(_) => println!("Map generated and stored successfully."),
            Err(e) => println!("Failed to store map in database: {}", e),
        }

        Ok(GameState { map })
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let mut mesh_builder = graphics::MeshBuilder::new();

        for (y, row) in self.map.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let brightness_factor = 0.5 + (tile.altitude + 1.0) / 2.0 * 0.5;
                let brightness_factor = brightness_factor as f32;

                let color = match tile.biome {
                    Biome::Forest => [0.0 * brightness_factor, 0.5 * brightness_factor, 0.0 * brightness_factor, 1.0],
                    Biome::Desert => [0.8 * brightness_factor, 0.8 * brightness_factor, 0.0 * brightness_factor, 1.0],
                    Biome::Water => [0.0 * brightness_factor, 0.0 * brightness_factor, 1.0 * brightness_factor, 1.0],
                };
                let color: [f32; 4] = color.map(|c| c as f32);
                mesh_builder.rectangle(
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32(x as i32 * 32, y as i32 * 32, 32, 32),
                    color.into(),
                );
            }
        }

        let mesh = mesh_builder.build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool
    ) {
        match keycode {
            event::KeyCode::R => {
                self.map = generate_map(100, 100);
                match store_map_in_db(&self.map) {
                    Ok(_) => println!("Map regenerated and stored successfully."),
                    Err(e) => println!("Failed to store map in database: {}", e),
                }
            },
            event::KeyCode::Escape => ggez::event::quit(ctx),
            _ => (),
        }
    }
}
