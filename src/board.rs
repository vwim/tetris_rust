use block::*;
use tetromino::*;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

#[derive(Copy, Clone, Debug)]
pub struct Board {
    cells: [[Option<Color>; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    pub fn new() -> Board {
        Board { cells: [[None; BOARD_WIDTH]; BOARD_HEIGHT] }
    }

    pub fn cells(&self) -> &[[Option<Color>; BOARD_WIDTH]; BOARD_HEIGHT] {
        &self.cells
    }

    pub fn overlap(&self, block: &Block) -> bool {
        let (px, py) = block.position();
        block.rotation().into_iter().any(|&(bx, by)| {
            let (x, y) = (px as isize + bx, py as isize + by);
            if x >= 0 && x < BOARD_WIDTH as isize && y >= 0 && y < BOARD_HEIGHT as isize {
                self.cells[y as usize][x as usize].is_some()
            } else {
                true
            }
        })
    }

    pub fn merge(&mut self, block: &Block) {
        let (px, py) = block.position();
        for &(bx, by) in block.rotation().into_iter() {
            let (x, y) = (px as isize + bx, py as isize + by);
            self.cells[y as usize][x as usize] = Some(block.color());
        }
    }

    pub fn remove_lines(&mut self) -> usize {
        let mut new_cells = [[None; BOARD_WIDTH]; BOARD_HEIGHT];
        let mut count = 0;
        for (line, new_line) in self.cells
            .iter()
            .rev()
            .filter(|&line| {
                if line.iter().all(|&xx| xx.is_some()) {
                    count += 1;
                    false
                } else {
                    true
                }
            })
            .zip(new_cells.iter_mut().rev()) {
            *new_line = *line;
        }
        self.cells = new_cells;
        count
    }

}
