use ggez::{conf, event, Context, GameResult};
use std::path;

// This struct will hold all our game state
// For now there is nothing to be held
struct Game {}

impl event::EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        println!("In the update function");
        Ok(())
    }

    fn draw(&mut self, _context: &mut Context) -> GameResult {
        println!("In the draw function");
        Ok(())
    }
}

fn main() -> GameResult {
    println!("Hello, world!");

    // Create a game context and event loop
    let ctx_builder = ggez::ContextBuilder::new("Fireball island", "me")
        .window_setup(conf::WindowSetup::default().title("Rust Fireball Island"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (ctx, ev_loop) = &mut ctx_builder.build()?;
    // Create the game state
    let game = &mut Game{};
    event::run(ctx, ev_loop, game)
}
