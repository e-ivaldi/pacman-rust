extern crate ncurses;

mod game;
mod level;
mod render;

fn main() {
    let mut level = level::Level::new("resources/level");
    let render = render::Render::new();
    let mut game = game::Game::new(&render, &mut level);

    game.start();
    game.stop();
}
