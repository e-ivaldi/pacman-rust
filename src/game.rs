use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use ncurses::getch;

use crate::level::{Direction, Level};
use crate::render::Render;

pub struct Game<'a> {
    render: &'a Render,
    level: &'a mut Level,
}

impl<'a> Game<'_> {
    pub fn new(render: &'a Render, level: &'a mut Level) -> Game<'a> {
        Game { render, level }
    }

    pub fn start(&mut self) {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || loop {
            let m = getch();
            tx.send(m).unwrap()
        });

        self.render.start();
        self.render.draw_map(self.level);

        loop {
            let pac = self.level.get_pacman();
            self.render.draw_pacman(pac);
            self.render.draw_ghosts(self.level.get_ghosts());

            let &mut pac = self.level.get_pacman();
            if self.level.is_walkable(&pac.position, &pac.direction) {
                self.level.get_pacman().walk();
            }

            let pac = self.level.get_pacman();
            match rx.recv().unwrap() {
                ncurses::KEY_UP => pac.set_direction(Direction::UP),
                ncurses::KEY_DOWN => pac.set_direction(Direction::DOWN),
                ncurses::KEY_LEFT => pac.set_direction(Direction::LEFT),
                ncurses::KEY_RIGHT => pac.set_direction(Direction::RIGHT),
                _ => {}
            }

            sleep(Duration::from_millis(200));
        }
    }

    pub fn stop(&self) {
        self.render.stop();
    }
}
