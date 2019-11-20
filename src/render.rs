use std::time::SystemTime;

use ncurses::*;

use crate::level::{Block, Direction, Level, Mobile};
use crate::render::colours::*;

mod colours;

pub struct Render {
    time: SystemTime,
}

impl Render {
    pub fn new() -> Render {
        Render {
            time: SystemTime::now(),
        }
    }

    pub fn start(&self) {
        setlocale(LcCategory::all, "");
        let window = initscr();
        keypad(window, true);
        start_color();
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        nodelay(window, false);
        init_color(COLOR_BLACK, 0, 0, 0);
        cbreak();
        //timeout(5000);
        self.initialize_colors();
    }

    pub fn stop(&self) {
        endwin();
    }

    pub fn draw_map(&self, level: &Level) {
        for y in 0..level.height() {
            for x in 0..level.width() {
                let block = level.get_block_at_point(x, y);

                let (color, token) = match block {
                    Block::WALL => (BLUE_ON_BLUE, ' '),
                    Block::GATE => (BLUE_ON_BLUE, ' '),
                    Block::DOT => (WHITE_ON_BLACK, 'o'),
                    Block::POWERUP => (YELLOW_ON_BLACK, 'x'),
                    Block::OTHER => (BLACK_ON_BLACK, ' '),
                };

                self.use_color(color);
                self.draw_cell(y, x, token);
            }
        }
        refresh();
    }

    pub fn draw_pacman(&self, pacman: &Mobile) {
        self.use_color(BLACK_ON_YELLOW);
        let token = self.pacman_mouth_from_direction(&pacman.direction);
        self.draw_mobile(pacman, BLACK_ON_YELLOW, token, ' ');
        refresh();
    }

    pub fn draw_ghosts(&self, ghosts: &[Mobile; 4]) {
        let available_colours = [
            WHITE_ON_CYAN,
            BLACK_ON_GREEN,
            WHITE_ON_RED,
            BLACK_ON_MAGENTA,
        ];

        let mut counter = 0;
        for ghost in ghosts {
            let colours_index = counter % available_colours.len();
            let color = *available_colours.get(colours_index).unwrap();
            self.draw_mobile(
                ghost,
                color,
                self.random_ghost_eye(),
                self.random_ghost_eye(),
            );
            refresh();
            counter += 1;
        }
    }

    fn clean_mobile(&self, mobile: &Mobile) {
        self.use_color(BLACK_ON_BLACK);
        self.draw_cell(mobile.previous_position.y, mobile.previous_position.x, ' ');
        self.draw_cell(
            mobile.previous_position.y,
            mobile.previous_position.x + 1,
            ' ',
        );
    }

    fn draw_mobile(&self, mobile: &Mobile, color: i16, token_left: char, token_right: char) {
        self.clean_mobile(mobile);
        self.use_color(color);
        self.draw_cell(mobile.position.y, mobile.position.x, token_left);
        self.draw_cell(mobile.position.y, mobile.position.x + 1, token_right);
    }

    fn draw_cell(&self, y: usize, x: usize, token: char) {
        ncurses::mvaddch(y as i32, x as i32, token as u64);
    }

    fn use_color(&self, color: i16) {
        ncurses::attron(ncurses::COLOR_PAIR(color));
    }

    fn random_ghost_eye(&self) -> char {
        if self.random_time_has_passed() {
            return 'O';
        }
        'o'
    }

    fn pacman_mouth_from_direction(&self, direction: &Direction) -> char {
        match direction {
            Direction::LEFT => {
                if self.time_has_passed(500) {
                    return '>';
                }
                '-'
            }
            Direction::RIGHT => '<',
            _ => '>',
        }
    }

    fn random_time_has_passed(&self) -> bool {
        self.time.elapsed().unwrap().as_millis() % 1000 > 500 && rand::random::<u8>() % 10 == 0
    }

    fn time_has_passed(&self, time: u128) -> bool {
        self.time.elapsed().unwrap().as_millis() % 1000 > time
    }

    fn initialize_colors(&self) {
        init_pair(BLACK_ON_BLACK, COLOR_BLACK, COLOR_BLACK);
        init_pair(WHITE_ON_WHITE, COLOR_WHITE, COLOR_WHITE);
        init_pair(YELLOW_ON_YELLOW, COLOR_YELLOW, COLOR_YELLOW);
        init_pair(WHITE_ON_BLACK, COLOR_WHITE, COLOR_BLACK);
        init_pair(BLUE_ON_BLUE, COLOR_BLUE, COLOR_BLUE);
        init_pair(BLACK_ON_RED, COLOR_BLACK, COLOR_RED);
        init_pair(WHITE_ON_RED, COLOR_WHITE, COLOR_RED);
        init_pair(BLACK_ON_YELLOW, COLOR_BLACK, COLOR_YELLOW);
        init_pair(BLACK_ON_CYAN, COLOR_BLACK, COLOR_CYAN);
        init_pair(BLACK_ON_MAGENTA, COLOR_BLACK, COLOR_MAGENTA);
        init_pair(BLACK_ON_GREEN, COLOR_BLACK, COLOR_GREEN);
        init_pair(YELLOW_ON_BLACK, COLOR_YELLOW, COLOR_BLACK);
        init_pair(WHITE_ON_CYAN, COLOR_WHITE, COLOR_CYAN);
        init_pair(RED_ON_RED, COLOR_RED, COLOR_RED);
        init_pair(RED_ON_BLACK, COLOR_RED, COLOR_BLACK);
        init_pair(BLUE_ON_WHITE, COLOR_BLUE, COLOR_WHITE);
        init_pair(WHITE_ON_BLUE, COLOR_WHITE, COLOR_BLUE);
    }
}
