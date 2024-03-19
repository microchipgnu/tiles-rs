use ggez::{ conf, event, ContextBuilder, GameResult };
mod ui;
mod tiles;
mod map_generator;
mod db;

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("xxx", "yyy")
        .window_setup(conf::WindowSetup::default().title("Map"))
        // Make window resizable
        .window_mode(conf::WindowMode::default().dimensions(1000.0, 1000.0).resizable(true))
        .build()?;

    let state = ui::GameState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
