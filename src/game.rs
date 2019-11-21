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

        let p = tx.clone();

        thread::spawn(move || loop {
            p.send(getch()).unwrap();
        });

        self.render.start();

        loop {
            self.render.draw_map(self.level);
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
                Ok(key) => match key {
                    ncurses::KEY_RIGHT => {
                        mvaddstr(3, 80, &format!("going right\t"));
                        self.level.get_pacman().set_next_direction(Direction::RIGHT);
                    }
                    ncurses::KEY_UP => {
                        mvaddstr(3, 80, &format!("going up\t"));
                        self.level.get_pacman().set_next_direction(Direction::UP);
                    }
                    ncurses::KEY_DOWN => {
                        mvaddstr(3, 80, &format!("going down\t"));
                        self.level.get_pacman().set_next_direction(Direction::DOWN);
                    }
                    ncurses::KEY_LEFT => {
                        mvaddstr(3, 80, &format!("going left\t"));
                        self.level.get_pacman().set_next_direction(Direction::LEFT);
                    }
                    _ => {}
                },
                Err(_) => {}
            }

            sleep(Duration::from_millis(100));
        }
    }

    pub fn stop(&self) {
        self.render.stop();
    }
}
