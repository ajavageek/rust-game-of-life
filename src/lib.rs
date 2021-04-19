mod utils;
pub mod wasm;

use fixedbitset::FixedBitSet;
use js_sys::Math;
use wasm::Universe;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };

        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }

    fn generate<F: Fn(u32) -> bool>(width: u32, height: u32, f: F) -> FixedBitSet {
        let size = width * height;
        let mut bitset = FixedBitSet::with_capacity(size as usize);
        (0..size).for_each(|i| bitset.set(i as usize, f(i)));
        bitset
    }

    fn base_cells(width: u32, height: u32) -> FixedBitSet {
        Self::generate(width, height, |i: u32| i % 2 == 0 || i % 7 == 0)
    }

    fn spaceship(width: u32, height: u32) -> FixedBitSet {
        let spaceship = vec![70, 73, 106, 134, 138, 167, 168, 169, 170];
        Self::generate(width, height, |i| spaceship.contains(&i))
    }

    fn random_boolean() -> bool {
        Math::random() < 0.5
    }

    fn random(width: u32, height: u32) -> FixedBitSet {
        Self::generate(width, height, |_i| Universe::random_boolean())
    }

    fn insert_pattern(&mut self, row: u32, col: u32, values: Vec<Vec<bool>>) {
        for i in 0..values.len() {
            for j in 0..values.len() {
                self.cells.set(self.get_index(row + i as u32, col + j as u32), values[j][i]);
            }
        }
    }
}
