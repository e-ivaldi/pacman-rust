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
                    '1' | '2' | '3' | '4' => {
                        ghosts[c.to_digit(10).unwrap() as usize - 1] = Mobile {
                            position: Position { y, x },
                            direction: Direction::RIGHT,
                        };
                        OTHER
                    }
                    _ => OTHER,
                };

                if !grid.get(y).is_some() {
                    grid.push(Vec::new());
                }
                grid[y].push(piece);

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
