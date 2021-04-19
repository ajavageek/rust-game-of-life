extern crate web_sys;

use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;
use web_sys::console;

use crate::utils;

#[wasm_bindgen]
pub struct Universe {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) cells: FixedBitSet,
}

#[wasm_bindgen]
pub enum StartState {
    Empty,
    Base,
    Spaceship,
    Random,
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new(state: StartState) -> Universe {
        utils::set_panic_hook();

        let width = 64;
        let height = 64;

        let cells = match state {
            StartState::Base => Self::base_cells(width, height),
            StartState::Spaceship => Self::spaceship(width, height),
            StartState::Random => Self::random(width, height),
            StartState::Empty => FixedBitSet::with_capacity((width * height) as usize),
        };

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                log!(
                    "cell[{}, {}] is initially {:?} and has {} live neighbors",
                    row,
                    col,
                    cell,
                    live_neighbors
                );

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (true, x) if x < 2 => false,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (true, 2) | (true, 3) => true,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (true, x) if x > 3 => false,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (false, 3) => true,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                log!("    it becomes {:?}", next_cell);

                next.set(idx, next_cell);
            }
        }
        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = Universe::generate(width, self.height, |_i| false);
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = Universe::generate(self.width, height, |_i| false);
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        let value = self.cells.contains(idx);
        self.cells.set(idx, !value);
    }

    pub fn reset(&mut self, state: StartState) {
        if matches!(state, StartState::Random) {
            self.cells = Self::random(self.width, self.height);
        } else if matches!(state, StartState::Empty) {
            self.cells = FixedBitSet::with_capacity((self.width * self.height) as usize)
        }
    }

    pub fn insert_glider(&mut self, row: u32, col: u32) {
        let values = [[false, true, false], [false, false, true], [true, true, true]];
        for i in 0..values.len() {
            for j in 0..values.len() {
                self.cells.set(self.get_index(row + i as u32, col + j as u32), values[j][i]);
            }
        }
    }

    pub fn insert_pulsar(&mut self, row: u32, col: u32) {
        let values = [
            [false, false, true, true, true, false, false, false, true, true, true, false, false],
            [false, false, false, false, false, false, false, false, false, false, false, false, false],
            [true, false, false, false, false, true, false, true, false, false, false, false, true],
            [true, false, false, false, false, true, false, true, false, false, false, false, true],
            [true, false, false, false, false, true, false, true, false, false, false, false, true],
            [false, false, true, true, true, false, false, false, true, true, true, false, false],
            [false, false, false, false, false, false, false, false, false, false, false, false, false],
            [false, false, true, true, true, false, false, false, true, true, true, false, false],
            [true, false, false, false, false, true, false, true, false, false, false, false, true],
            [true, false, false, false, false, true, false, true, false, false, false, false, true],
            [true, false, false, false, false, true, false, true, false, false, false, false, true],
            [false, false, false, false, false, false, false, false, false, false, false, false, false],
            [false, false, true, true, true, false, false, false, true, true, true, false, false]
        ];
        for i in 0..values.len() {
            for j in 0..values.len() {
                self.cells.set(self.get_index(row + i as u32, col + j as u32), values[j][i]);
            }
        }
    }
}
