extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::event::*;
use ggez::ContextBuilder;

mod game_object;
mod assets;
mod main_state;
mod utils;

fn main() {
    let (width, height) = (800, 450);
    let cb = ContextBuilder::new("gift", "ggez")
        .window_setup(conf::WindowSetup::default().title("WINDOW_TITLE"))
        .window_mode(conf::WindowMode::default().dimensions(width, height));

    let ctx = &mut cb.build().unwrap();
    match main_state::MainState::new(ctx, width, height) {
        Err(e) => {
            println!("Could not load game!");
            println!("Error: {}", e);
        }
        Ok(ref mut game) => {
            let result = run(ctx, game);
            if let Err(e) = result {
                println!("Error encountered running game: {}", e);
            } else {
                println!("Game exited cleanly.");
            }
        }
    }
}
