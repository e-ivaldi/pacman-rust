use crate::level::Level;
use crate::render::Render;

pub struct Game<'a> {
    render: &'a Render,
    level: &'a Level,
}

impl<'a> Game<'_> {
    pub fn new(render: &'a Render, level: &'a Level) -> Game<'a> {
        Game { render, level }
    }

    pub fn start(&self) {
        self.render.start();

        self.render.draw_map(self.level);
        self.render.draw_pacman(self.level.get_pacman());
        self.render.draw_ghosts(self.level.get_ghosts());
        self.render.wait();
    }

    pub fn stop(&self) {
        self.render.stop();
    }
}
