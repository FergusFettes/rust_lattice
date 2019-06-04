mod utils;
extern crate js_sys;
extern crate fixedbitset;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use fixedbitset::FixedBitSet;

// // A macro to provide `println!(..)`-style syntax for `console.log` logging.
// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    // Each of the last 9 bits corresponds to a neighbor count that leads to life
    alive_rules: u16,
    // Each of the last 9 bits corresponds to a neighbor count that leads to death
    dead_rules: u16,
    initial_density: f64,
    cells: FixedBitSet,
}

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

}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                // log! (
                //     "cell[{}, {}] is initially {:?} and has {} live neighbors",
                //     row,
                //     col,
                //     cell,
                //     live_neighbors
                //     );

                next.set(idx, match (cell, live_neighbors) {
                    // This is checking if the number of neighbors is in the ruleset.
                    (true, x) => ((self.alive_rules >> x) & 1) != 0,
                    (false, x) => ((self.dead_rules >> x) & 1) != 0,
                });

                // log! ("     it becomes {:?}", self.cells[idx])
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 128; // default values
        let height = 128;
        let initial_density = 0.5;

        let mut alive_rules = 0;
        let mut dead_rules = 0;
        let a = [2,3];
        let d = [3];
        for i in a.iter() {alive_rules = alive_rules | (1u16 << i)}
        for i in d.iter() {dead_rules = dead_rules | (1u16 << i)}

        let size = (width*height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, js_sys::Math::random() < initial_density)
        }

        Universe {
            width,
            height,
            alive_rules,
            dead_rules,
            initial_density,
            cells,
        }
    }

    /// Set the rules of the universe.
    pub fn set_rules(&mut self, alive_rules: u16, dead_rules: u16) {
        self.alive_rules = alive_rules;
        self.dead_rules = dead_rules;
    }

    /// Set the width of the universe.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        let size = (width*self.height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            cells.set(i, false)
        }
        self.cells = cells
    }

    /// Set the height of the universe.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        let size = (self.width*height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            cells.set(i, false)
        }
        self.cells = cells
    }

    /// Initialise the universe
    pub fn initialise_cells(&mut self, initial_density: f64) {
        let size = (self.width*self.height) as usize;
        for i in 0..size {
            self.cells.set(i, js_sys::Math::random() < initial_density)
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn initial_density(&self) -> f64 {
        self.initial_density
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
}

// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello operator!");
// }
