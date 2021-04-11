mod utils;
mod wasm;

use fixedbitset::FixedBitSet;
use js_sys::Math;
use wasm::Universe;

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
}
