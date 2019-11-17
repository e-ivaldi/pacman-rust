extern crate ncurses;

mod game;
mod level;
mod render;

fn main() {
    let level = level::Level::new("resources/level");
    let render = render::Render {};
    let game = game::Game::new(&render, &level);

    game.start();
    game.stop();
}
