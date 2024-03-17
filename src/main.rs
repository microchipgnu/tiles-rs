use ggez::{ conf, event, graphics, Context, ContextBuilder, GameResult };
use map_generator::generate_map;
use tiles::{ Map, Biome };

mod map_generator;
mod tiles;
mod db;

struct GameState {
    map: Map,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let map = generate_map(20, 20); // Large map for demonstration
        Ok(GameState { map })
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let mut mesh_builder = graphics::MeshBuilder::new();

        for (y, row) in self.map.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let color = match tile.biome {
                    Biome::Forest => [0.0, 0.5, 0.0, 1.0],
                    Biome::Desert => [0.8, 0.8, 0.0, 1.0],
                    Biome::Water => [0.0, 0.0, 1.0, 1.0],
                };
                mesh_builder.rectangle(
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32((x as i32) * 32, (y as i32) * 32, 32, 32),
                    color.into()
                );
            }
        }

        let mesh = mesh_builder.build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

        graphics::present(ctx)
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
                let color = match tile.biome {
                    Biome::Forest => [0.0, 0.5, 0.0, 1.0],
                    Biome::Desert => [0.8, 0.8, 0.0, 1.0],
                    Biome::Water => [0.0, 0.0, 1.0, 1.0],
                };
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
        if keycode == event::KeyCode::R {
            // Regenerate the map when 'R' key is pressed
            self.map = map_generator::generate_map(20, 20);
            // Optionally, you can also clear and redraw the canvas here if necessary
            match db::store_map_in_db(&self.map) {
                Ok(_) => println!("Map regenerated and stored successfully."),
                Err(e) => println!("Failed to store map in database: {}", e),
            }
        } else if keycode == event::KeyCode::Escape {
            // Close the game when 'Escape' key is pressed
            ggez::event::quit(ctx);
        }
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("tiles-rs", "")
        .window_setup(conf::WindowSetup::default().title("Tile Map"))
        .window_mode(conf::WindowMode::default().dimensions(500.0, 500.0))
        .build()?;

    let state = GameState::new(&mut ctx)?;

    // Store the map in the database
    db::store_map_in_db(&state.map).expect("Failed to store map in database");

    println!("Map generated and stored successfully.");
    event::run(ctx, event_loop, state)
}
