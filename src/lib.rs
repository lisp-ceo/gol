mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Generate wasm
#[wasm_bindgen]
// Restrict each enum value to a byte for linear webassembly data allocation
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}


#[wasm_bindgen]
impl Universe {

    // Construct a new Universe.
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe{
            height,
            width,
            cells,
        }
    }

    // Serialize universe for presentation.
    pub fn render(&self) -> String {
        self.to_string()
    }

    //  Width of universe.
    pub fn width(&self) -> u32 {
        self.width
    }

    // Height of universe.
    pub fn height(&self) -> u32 {
        self.height
    }

    // Pointer to cells
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }


    // Map linear array vector indices to 2D array.
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    // Count live neighbours of cell at given row and column.
    fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for curr_row in [self.height - 1, 0, 1].iter().cloned() {
            for curr_col in [self.width - 1, 0, 1].iter().cloned() {
                if row == 0 && column == 0 {
                    continue;
                }

                let neighbour_row = (row + curr_row) % self.height;
                let neighbour_col = (column + curr_col) % self.width;
                let idx = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    // Computes each tick of the game of life.
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);

                let next_cell = match (cell, live_neighbours) {
                    // Any live cell with fewer than 2 live neighours dies from underpopulation
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Any live cell with two or three live neighbours lives on to the next generation
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Any live cell with more than three live neighbours dies from overpopulation
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Any dead cell with exactly 3 live neighbours becomes a live cell from reproduction
                    (Cell::Dead, 3) => Cell::Alive,
                    // All others retain previous state
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        if self.cells == next {
          let width = 64;
          let height = 64;

          self.cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        } else {
            self.cells = next;
        }

    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize ) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
