use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use ncurses::{chtype, getch, mvprintw};

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
            mvprintw(1, 80, &format!("Asking for key.. {:?}     ", SystemTime::now()));
            let m: i32 = getch();
            mvprintw(2, 80, &format!("sending key {}     ", m));

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
            match rx.try_recv() {
                Ok(key) => match key {
                    260 => self.level.get_pacman().set_direction(Direction::LEFT),
                    261 => self.level.get_pacman().set_direction(Direction::RIGHT),
                    259 => self.level.get_pacman().set_direction(Direction::UP),
                    258 => self.level.get_pacman().set_direction(Direction::DOWN),
                    _ => {},
                }
                Err(x) => { mvprintw(4, 80, &format!("err: {:?}", x)); }
            }

                sleep(Duration::from_millis(200));
            }
        }

        pub fn stop(&self) {
            self.render.stop();
        }
    }
