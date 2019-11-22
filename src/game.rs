use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use ncurses::{getch, mvaddstr};

use crate::level::{Block, Direction, Level};
use crate::render::Render;

pub struct Game<'a> {
    render: &'a mut Render,
    level: &'a mut Level,
    points: i32,
}

impl<'a> Game<'_> {
    pub fn new(render: &'a mut Render, level: &'a mut Level) -> Game<'a> {
        Game {
            render,
            level,
            points: 0,
        }
    }

    pub fn start(&mut self) {
        let (tx, rx) = mpsc::sync_channel(1);

        thread::spawn(move || loop {
            let mut last_c = 0;
            let c = getch();
            if c != last_c {
                tx.send(c).unwrap();
            }
            last_c = c;
        });

        self.render.start();

        self.render.draw_map(self.level);
        loop {
            let pac = self.level.get_pacman();
            self.render.draw_pacman(pac);
            self.render.draw_ghosts(self.level.get_ghosts());

            let &mut pac = self.level.get_pacman();

            if pac.next_direction != pac.direction
                && self.level.is_walkable(&pac.position, &pac.next_direction)
            {
                self.level.get_pacman().set_direction(pac.next_direction);
                self.level.get_pacman().walk();
            } else if self.level.is_walkable(&pac.position, &pac.direction) {
                self.level.get_pacman().walk();
            }

            match self.level.get_block_at_position(pac.position) {
                Block::DOT => {
                    self.level.clear_position(pac.position);
                    self.points += 1
                }
                Block::POWERUP => {}
                Block::GATE | Block::WALL | Block::OTHER => {}
            }

            self.render.draw_points(self.points);

            match rx.try_recv() {
                Ok(key) => {
                    let next_direction: Option<Direction> = match key {
                        ncurses::KEY_RIGHT => Option::from(Direction::RIGHT),
                        ncurses::KEY_UP => Option::from(Direction::UP),
                        ncurses::KEY_DOWN => Option::from(Direction::DOWN),
                        ncurses::KEY_LEFT => Option::from(Direction::LEFT),
                        _ => Option::None
                    };

                    match next_direction {
                        Some(direction) => self.level.get_pacman().set_next_direction(direction),
                        None => {}
                    }
                }
                Err(_) => {}
            }

            let sleep_millis = match pac.direction {
                Direction::UP | Direction::DOWN => 100,
                Direction::LEFT | Direction::RIGHT => 50,
            };

            sleep(Duration::from_millis(sleep_millis));
        }
    }

    pub fn stop(&self) {
        self.render.stop();
    }
}
