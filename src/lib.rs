mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//
// Represents a cell state in the Universe
//
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

//
// Represents a Universe
//
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

//
// Public methods, exported to JavaScript
//
#[wasm_bindgen]
impl Universe {
    //
    // Create a new Universe with an interesting start
    //
    pub fn new(width: u32, height: u32) -> Universe {
        let cells = (0..width*height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Universe {
            width,
            height,
            cells
        }
    }

    //
    // Advance the Universe
    //
    pub fn tick(&mut self) {
        let mut cells_next = self.cells.clone();
        for i in 0..self.width {
            for j in 0..self.height {
                let index = self.get_index(i, j);
                let cell_state = self.cells[index];
                let live_neighbour_count = self.get_live_neighbor_count(i, j);
                let cell_next_state;
                match (cell_state, live_neighbour_count) {
                    (Cell::Alive, count) if count < 2 => {
                        cell_next_state = Cell::Dead
                    },
                    (Cell::Alive, 2) | (Cell::Alive, 3) => {
                        cell_next_state = Cell::Alive
                    },
                    (Cell::Alive, count) if count > 3 => {
                        cell_next_state = Cell::Dead
                    },
                    (Cell::Dead, 3) => {
                        cell_next_state = Cell::Alive
                    },
                    (state, _count) => {
                        cell_next_state = state
                    },
                };
                cells_next[index] = cell_next_state;
            }
        }
        self.cells = cells_next;
    }

    //
    // Returns a String representation of the Universe as a 2D grid
    //
    pub fn render(&self) -> String {
        self.to_string()
    }
}

//
// Private methods
//
impl Universe {
    //
    // Returns a 1D index given a 2D index
    //
    // # Arguments
    //
    // * `row` - A u32 to indicate the row in the Universe matrix
    //
    // * `column` - A u32 to indicate the column in the Universe matrix
    //
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    //
    // Given a 2D index for a cell, returns the number of alive cells around it
    //
    // # Arguments
    //
    // * `row` - A u32 to indicate the row in the Universe matrix
    //
    // * `column` - A u32 to indicate the column in the Universe matrix
    //
    fn get_live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for i in [-1, 0, 1].iter().cloned() {
            for j in [-1, 0, 1].iter().cloned() {
                if i == 0 && j == 0 {
                    continue;
                } else {
                    // NTD
                }
                let x = (row + i as u32 + self.width) % self.width;
                let y = (column + j as u32 + self.height) % self.height;
                let index = self.get_index(x, y);
                if self.cells[index] == Cell::Alive {
                    count += 1;
                } else {
                    // NTD
                }
            }
        }
        return count;
    }
}

use std::fmt;

//
// Implements the Display trait for Universe
//
impl fmt::Display for Universe {
    //
    // Format the Universe as a String
    //
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { "░░" } else { "██" };
                if write!(f, "{}", symbol).is_ok() {
                    // NTD
                } else {
                    // Error
                }
            }
            if write!(f, "\n").is_ok() {
                // NTD
            } else {
                // Error
            }
        }
        Ok(())
    }
}
