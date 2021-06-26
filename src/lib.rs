mod utils;

extern crate js_sys;
extern crate web_sys;

// Macro to provide println!(..)-style syntax for logging to the javascript console.
macro_rules! log {
    ( $( $t:tt  )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

extern crate derive_more;
use derive_more::{DerefMut,Display};

use wasm_bindgen::prelude::*;
use std::fmt;
use std::ops::Deref;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Generate wasm
#[wasm_bindgen]
// Restrict each enum value to a byte for linear webassembly data allocation
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[derive(DerefMut)]
struct Cells (Vec<Cell>);

impl Cells {

    // Constructs a new vector of dead ( uWu ) cells.
    fn new(width: u32, height: u32) -> Cells {
        let v = (0..width * height)
            .map(|_| { Cell::Dead })
            .collect::<Vec<Cell>>();
        Cells(v)
    }

    // Constructs a new vector of cells that might be alive or dead.
    fn new_random(width: u32, height: u32) -> Cells {
        let v = (0..width * height)
            .map(|_| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect::<Vec<Cell>>();
        Cells(v)
    }
}

// Implementing Deref to expose methods of alias
// https://doc.rust-lang.org/book/ch15-02-deref.html#treating-smart-pointers-like-regular-references-with-the-deref-trait
impl Deref for Cells {

    type Target = Vec<Cell>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Cells,
}

// Methods generating wasm functions
#[wasm_bindgen]
impl Universe {

    // Construct a new Universe.
    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;

        let cells = Cells::new(width, height);

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

    // Pointer to cells.
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    // Sets the width of the universe.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = Cells::new(width, self.height);
    }

    // Sets the height of the universe.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = Cells::new(self.width, height);
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
                if curr_row == 0 && curr_col == 0 {
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

                log!(
                    "cell[{}, {}] is initially {:?} and has {} live neighbours",
                    row,
                    col,
                    cell,
                    live_neighbours
                );

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

                log!(
                    "cell is now {}",
                    next_cell,
                );

                next[idx] = next_cell;
            }
        }

        // Fork required to prevent use after borrow
        if (*self.cells.deref()) == next {
            self.cells = Cells::new_random(self.width, self.height);
        } else {
            self.cells.0 = next;
        }
    }
}

// Methods *not* generating wasm methods. Used to return borrowed references.
impl Universe {
   // Get the dead and alive values of the entire universe
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    // Set cells to be alive in a universe by passing the row and column of each cell as an array
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
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

