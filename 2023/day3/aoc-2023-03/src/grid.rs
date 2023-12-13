use core::panic;
use std::io::stdin;

pub struct LinearGrid {
    pub grid: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

pub fn construct_grid_from_input(width: usize) -> LinearGrid {
    let mut grid = LinearGrid::new(width);
    for line in stdin().lines() {
        let line = line.unwrap();
        // Remove any newline chars, also lets assume ascii for simpler indexing
        let trimmed = line.trim().as_bytes();
        grid.add_row(trimmed);
    }
    grid
}

impl LinearGrid {
    pub fn new(width: usize) -> Self {
        Self {
            grid: Vec::new(),
            width,
            height: 0,
        }
    }

    pub fn add_row(&mut self, row: &[u8]) {
        if row.len() != self.width {
            panic!("OH NO BAD INPUT!");
        }
        self.grid.extend_from_slice(row);
        self.height += 1;
    }

    pub fn value_at_coord(&self, x: usize, y: usize) -> Option<u8> {
        // check if these coords are valid, if not return None, else return the usize
        if x < self.width && y < self.height {
            Some(self.grid[x + (self.width * y)])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::LinearGrid;
    use pretty_assertions::assert_eq;

    #[test]
    fn grid_construction() {
        let mut g = LinearGrid::new(1);
        let row: [u8; 1] = [1];
        g.add_row(&row);
        // add a new row to a grid
        assert_eq!(g.height, 1);
    }
}
