extern crate ncurses;

mod game;
mod level;
mod render;

fn main() {
    let mut level = level::Level::new("resources/level");
    let mut render = render::Render::new();
    let mut game = game::Game::new(&mut render, &mut level);

    game.start();
    game.stop();
}
