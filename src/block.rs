use tetromino::*;
use rand;
use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Down,
}

#[derive(Copy, Clone, Debug)]
pub struct Block {
    tetromino: &'static Tetromino,
    rotation: usize,
    position: (isize, isize),
}


impl Block {
    pub fn new() -> Self {
        Block {
            tetromino: rand::thread_rng().choose(&TETROMINOS).unwrap(),
            rotation: 0,
            position: (5, 0),
        }
    }

    pub fn rotate(mut self) -> Self {
        self.rotation = (self.rotation + 1) % 4;
        self
    }

    pub fn displace(mut self, d: Direction) -> Self {
        match d {
            Direction::Left => self.position.0 -= 1,
            Direction::Right => self.position.0 += 1,
            Direction::Down => self.position.1 += 1,
        }
        self
    }

    pub fn position(&self) -> (isize, isize) {
        self.position
    }

    pub fn color(&self) -> Color {
        self.tetromino.1
    }

    pub fn rotation(&self) -> &Rotation {
        &self.tetromino.0[self.rotation]
    }
}
