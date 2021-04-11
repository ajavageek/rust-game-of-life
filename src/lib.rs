mod utils;
mod wasm;

use js_sys::Math;
use wasm::{Cell, Universe};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    fn generate<F: Fn(u32) -> Cell>(width: u32, height: u32, f: F) -> Vec<Cell> {
        (0..width * height)
            .map(|i| f(i))
            .collect()
    }

    fn base_cells(width: u32, height: u32) -> Vec<Cell> {
        let f = |i: u32| {
            if i % 2 == 0 || i % 7 == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        };
        Self::generate(width, height, f)
    }

    fn spaceship(width: u32, height: u32) -> Vec<Cell> {
        let spaceship = vec![70, 73, 106, 134, 138, 167, 168, 169, 170];
        let f = |i: u32| {
            if spaceship.contains(&i) {
                Cell::Alive
            } else {
                Cell::Dead
            }
        };
        Self::generate(width, height, f)
    }

    fn random_boolean() -> bool {
        Math::random() < 0.5
    }

    fn random(width: u32, height: u32) -> Vec<Cell> {
        let f = |_i|
            if Universe::random_boolean() {
                Cell::Alive
            } else {
                Cell::Dead
            };
        Self::generate(width, height, f)
    }
}
