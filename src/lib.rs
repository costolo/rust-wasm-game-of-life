extern crate cfg_if;
extern crate fixedbitset;
extern crate js_sys;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 64;
        let height = 64;
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, js_sys::Math::random() < 0.5);
        }

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
                // log!(
                //     "cell[{}, {}] is initially {:?} and has {} live neighbors",
                //     row,
                //     col,
                //     cell,
                //     live_neighbors
                // );

                next.set(
                    idx,
                    match (cell, live_neighbors) {
                        (true, x) if x < 2 => false,
                        (true, 2) | (true, 3) => true,
                        (true, x) if x > 3 => false,
                        (false, 3) => true,
                        (otherwise, _) => otherwise,
                    },
                );
            }
        }

        self.cells = next;
    }

    pub fn generate_random_cells(&mut self) {
        let mut next = self.cells.clone();
        let size = (self.width * self.height) as usize;
        for i in 0..size {
            next.set(i, js_sys::Math::random() < 0.5);
        }

        self.cells = next;
    }

    pub fn draw_glider(&mut self, row: u32, column: u32, glider_type: String) {
        let mut next = self.cells.clone();
        let pt1;
        let pt2;
        let pt3;
        let pt4;
        let pt5;

        if glider_type == "north-west" {
            pt1 = self.get_index(row, column);
            pt2 = self.get_index(row, column + 1);
            pt3 = self.get_index(row, column + 2);
            pt4 = self.get_index(row + 1, column);
            pt5 = self.get_index(row + 2, column + 1);
        } else if glider_type == "north-east" {
            pt1 = self.get_index(row, column);
            pt2 = self.get_index(row, column - 1);
            pt3 = self.get_index(row, column - 2);
            pt4 = self.get_index(row + 1, column);
            pt5 = self.get_index(row + 2, column - 1);
        } else if glider_type == "south-west" {
            pt1 = self.get_index(row, column);
            pt2 = self.get_index(row, column + 1);
            pt3 = self.get_index(row, column + 2);
            pt4 = self.get_index(row - 1, column);
            pt5 = self.get_index(row - 2, column + 1);
        } else {
            pt1 = self.get_index(row, column);
            pt2 = self.get_index(row, column - 1);
            pt3 = self.get_index(row, column - 2);
            pt4 = self.get_index(row - 1, column);
            pt5 = self.get_index(row - 2, column - 1);
        }

        next.set(pt1, true);
        next.set(pt2, true);
        next.set(pt3, true);
        next.set(pt4, true);
        next.set(pt5, true);

        self.cells = next;
    }

    pub fn kill_all_cells(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                next.set(idx, false);
            }
        }
        self.cells = next;
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        let cell = self.cells[idx];
        self.cells.set(idx, !cell);
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        let size = (width * self.height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, false);
        }
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        let size = (self.width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, false);
        }
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
                count += self.cells[idx] as u8
            }
        }
        count
    }
}
