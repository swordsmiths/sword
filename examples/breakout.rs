extern crate sword;

use sword::{GameBuilder, WindowBuilder};

struct GameState {

}

impl GameState {
    pub fn new() -> GameState {
        GameState {}
    }
}

fn main() {
    let game = GameBuilder::new()
        .with_window(WindowBuilder::new()
            .with_title("Breakout")
            .with_width(800)
            .with_height(600))
        .build();

    game.run(GameState::new(), |input, state| {
        println!("{}", input.dt);
    }, |state| {

    });
}