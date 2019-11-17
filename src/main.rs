extern crate ncurses;

mod render;
mod level;
mod game;

fn main() {
    let level = level::Level::new("/home/manu/wahanda/pacman/src/level");
    let render = render::Render{};
    let game = game::Game::new(&render, &level);

    game.start();
    game.stop();

}