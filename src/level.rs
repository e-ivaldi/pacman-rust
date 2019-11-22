use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::level::Block::{DOT, GATE, OTHER, POWERUP, TELEPORT, WALL};

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub y: usize,
    pub x: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

const DEFAULT_MOBILE: Mobile = Mobile {
    position: Position { y: 0, x: 0 },
    previous_position: Position { y: 0, x: 0 },
    direction: Direction::LEFT,
    next_direction: Direction::LEFT,
};

#[derive(Debug, Copy, Clone)]
pub struct Mobile {
    pub position: Position,
    pub previous_position: Position,
    pub direction: Direction,
    pub next_direction: Direction,
}

#[derive(Debug, PartialEq)]
pub enum Block {
    WALL,
    GATE,
    DOT,
    POWERUP,
    TELEPORT,
    OTHER,
}

pub struct Level {
    grid: Vec<Vec<Block>>,
    pacman: Mobile,
    ghosts: [Mobile; 4],
    teleports: HashSet<Position>,
}

impl Eq for Position{
//    fn eq(&self, other: &Self) -> bool {
//        self.x == other.x && self.y == other.y
//    }
}

impl Mobile {
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn set_next_direction(&mut self, direction: Direction) {
        self.next_direction = direction;
    }

    pub fn walk(&mut self) {
        self.previous_position = self.position;
        match self.direction {
            Direction::LEFT => self.position.x -= 1,
            Direction::RIGHT => self.position.x += 1,
            Direction::UP => self.position.y -= 1,
            Direction::DOWN => self.position.y += 1,
        };
    }
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
        let mut teleports: HashSet<Position> = HashSet::new();

        reader.lines().for_each(|line| {
            for c in line.unwrap().chars() {
                let current_position = Position{y, x};
                let piece = match c {
                    'W' => WALL,
                    'G' => GATE,
                    'd' => DOT,
                    'X' => POWERUP,
                    'T' => {
                        teleports.insert(current_position);
                        TELEPORT
                    }
                    'P' => {
                        pacman.position = current_position;
                        OTHER
                    }
                    '1' | '2' | '3' | '4' => {
                        ghosts[c.to_digit(10).unwrap() as usize - 1] = Mobile {
                            position: current_position,
                            previous_position: current_position,
                            direction: Direction::RIGHT,
                            next_direction: Direction::RIGHT,
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
            teleports,
        }
    }

    pub fn teleport(&self, from_position: Position) -> &Option<&Position> {
        &self.teleports.iter().filter(|p| p!= from_position).last()
    }

    pub fn get_pacman(&mut self) -> &mut Mobile {
        &mut self.pacman
    }

    pub fn get_ghosts(&self) -> &[Mobile; 4] {
        &self.ghosts
    }

    pub fn get_block_at_position(&self, position: Position) -> &Block {
        self.get_block_at_point(position.y, position.x)
    }

    pub fn clear_position(&mut self, position: Position) {
        self.grid[position.y][position.x] = Block::OTHER;
    }

    pub fn get_block_at_point(&self, y: usize, x: usize) -> &Block {
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

    pub fn is_walkable(&self, position: &Position, direction: &Direction) -> bool {
        match direction {
            Direction::LEFT => self.is_walkable_block(&self.grid[position.y][position.x - 1]),
            Direction::RIGHT => self.is_walkable_block(&self.grid[position.y][position.x + 2]),
            Direction::UP => {
                self.is_walkable_block(&self.grid[position.y - 1][position.x])
                    && self.is_walkable_block(&self.grid[position.y - 1][position.x + 1])
            }
            Direction::DOWN => {
                self.is_walkable_block(&self.grid[position.y + 1][position.x])
                    && self.is_walkable_block(&self.grid[position.y + 1][position.x + 1])
            }
        }
    }

    fn is_walkable_block(&self, block: &Block) -> bool {
        match block {
            Block::WALL | Block::GATE => false,
            Block::POWERUP | Block::DOT | Block::OTHER => true,
        }
    }
}
