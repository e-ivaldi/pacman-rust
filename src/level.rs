use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::level::Block::{DOT, GATE, OTHER, POWERUP, WALL};

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub y: usize,
    pub x: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

const DEFAULT_MOBILE: Mobile = Mobile {
    position: Position { y: 0, x: 0 },
    direction: Direction::LEFT,
};

#[derive(Debug, Copy, Clone)]
pub struct Mobile {
    pub position: Position,
    pub direction: Direction,
}

#[derive(Debug)]
pub enum Block {
    WALL,
    GATE,
    DOT,
    POWERUP,
    OTHER,
}

pub struct Level {
    grid: Vec<Vec<Block>>,
    pacman: Mobile,
    ghosts: [Mobile; 4],
}

impl Level {
    pub fn new(filename: &str) -> Level {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut y: usize = 0;
        let mut x: usize = 0;

        let mut grid: Vec<Vec<Block>> = Vec::new();

        let mut pacman = DEFAULT_MOBILE.clone();
        let mut ghosts = [DEFAULT_MOBILE.clone(); 4];

        reader.lines().for_each(|line| {
            for c in line.unwrap().chars() {
                let piece = match c {
                    'W' => WALL,
                    'T' => GATE,
                    'd' => DOT,
                    'X' => POWERUP,
                    'P' => {
                        pacman.position = Position { y, x };
                        OTHER
                    }
                    '1' => {
                        ghosts[0] = Mobile {
                            position: Position { y, x },
                            direction: Direction::RIGHT,
                        };
                        OTHER
                    }
                    '2' => {
                        ghosts[1] = Mobile {
                            position: Position { y, x },
                            direction: Direction::UP,
                        };
                        OTHER
                    }
                    '3' => {
                        ghosts[2] = Mobile {
                            position: Position { y, x },
                            direction: Direction::DOWN,
                        };
                        OTHER
                    }
                    '4' => {
                        ghosts[3] = Mobile {
                            position: Position { y, x },
                            direction: Direction::LEFT,
                        };
                        OTHER
                    }
                    _ => OTHER,
                };

                if x == 0 {
                    let mut line: Vec<Block> = Vec::new();
                    line.push(piece);
                    grid.push(line);
                } else {
                    grid[y].push(piece);
                }
                x += 1
            }
            x = 0;
            y += 1;
        });

        Level {
            grid,
            pacman,
            ghosts,
        }
    }

    pub fn get_pacman(&self) -> &Mobile {
        &self.pacman
    }

    pub fn get_ghosts(&self) -> &[Mobile; 4] {
        &self.ghosts
    }

    pub fn get_block_at_point(&self, x: usize, y: usize) -> &Block {
        let row = self.grid.get(y).unwrap();
        let block = row.get(x).unwrap();
        block
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn width(&self) -> usize {
        self.grid.get(0).unwrap().len()
    }
}
