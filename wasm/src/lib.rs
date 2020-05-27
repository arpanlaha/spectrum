// use wasm_bindgen::prelude::*;
// extern crate web_sys;
// use web_sys::console;
use rand::Rng;
use std::iter;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RGB(u8, u8, u8);

// #[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Hue(u16);

impl Hue {
    fn new(hue: u16) -> Result<Hue, String> {
        if hue < 360 {
            Ok(Hue(hue))
        } else {
            Err(String::from("Hue cannot be 360 or above"))
        }
    }

    fn get(&self) -> u16 {
        self.0
    }

    fn to_rgb(&self) -> RGB {
        let hue = self.0;
        let primary = 255;
        let secondary = ((1f32 - ((hue as f32 / 60f32) % 2f32 - 1f32).abs()) * 255f32) as u8;
        match hue / 60 {
            0 => RGB(primary, secondary, 0),
            1 => RGB(secondary, primary, 0),
            2 => RGB(0, primary, secondary),
            3 => RGB(0, secondary, primary),
            4 => RGB(secondary, 0, primary),
            _ => RGB(primary, 0, secondary),
        }
    }
}

struct Source {
    x: usize,
    y: usize,
    hue: Hue,
}

impl Source {
    pub fn new(width: usize, height: usize, hue: Hue) -> Source {
        Source {
            x: rand::thread_rng().gen_range(0, width),
            y: rand::thread_rng().gen_range(0, height),
            hue,
        }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn hue_vectors(&self) -> (f32, f32) {
        let hue_val = self.hue.get() as f32;
        (hue_val.cos(), hue_val.sin())
    }
}

pub fn draw_spectrum(width: usize, height: usize, num_sources: usize) -> Vec<Vec<RGB>> {
    let sources: Vec<Source> = iter::repeat(())
        .map(|()| {
            Source::new(
                width,
                height,
                Hue::new(rand::thread_rng().gen_range(0, 360)).unwrap(),
            )
        })
        .take(num_sources)
        .collect();

    let mut spectrum = (0..width)
        .map(|x| {
            (0..height)
                .map(|y| RGB(0, (x % 255) as u8, (y % 255) as u8))
                .collect()
        })
        .collect();

    spectrum
}

// #[wasm_bindgen]
// #[repr(u8)]
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum Cell {
//     Dead = 0,
//     Alive = 1,
// }

// impl Cell {
//     fn toggle(&mut self) {
//         *self = match *self {
//             Cell::Dead => Cell::Alive,
//             Cell::Alive => Cell::Dead,
//         };
//     }
// }

// #[wasm_bindgen]
// pub struct Universe {
//     width: u32,
//     height: u32,
//     cells: Vec<Cell>,
// }

// impl Universe {
//     fn get_index(&self, row: u32, column: u32) -> usize {
//         (row * self.width + column) as usize
//     }

//     fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
//         let mut count = 0;

//         let north = if row == 0 { self.height - 1 } else { row - 1 };

//         let south = if row == self.height - 1 { 0 } else { row + 1 };

//         let west = if column == 0 {
//             self.width - 1
//         } else {
//             column - 1
//         };

//         let east = if column == self.width - 1 {
//             0
//         } else {
//             column + 1
//         };

//         let nw = self.get_index(north, west);
//         count += self.cells[nw] as u8;

//         let n = self.get_index(north, column);
//         count += self.cells[n] as u8;

//         let ne = self.get_index(north, east);
//         count += self.cells[ne] as u8;

//         let w = self.get_index(row, west);
//         count += self.cells[w] as u8;

//         let e = self.get_index(row, east);
//         count += self.cells[e] as u8;

//         let sw = self.get_index(south, west);
//         count += self.cells[sw] as u8;

//         let s = self.get_index(south, column);
//         count += self.cells[s] as u8;

//         let se = self.get_index(south, east);
//         count += self.cells[se] as u8;

//         count
//     }
// }

// /// Public methods, exported to JavaScript.
// #[wasm_bindgen]
// impl Universe {
//     pub fn tick(&mut self) {
//         // let _timer = Timer::new("Universe::tick");

//         let mut next = self.cells.clone();

//         for row in 0..self.height {
//             for col in 0..self.width {
//                 let idx = self.get_index(row, col);
//                 let cell = self.cells[idx];
//                 let live_neighbors = self.live_neighbor_count(row, col);

//                 let next_cell = match (cell, live_neighbors) {
//                     (Cell::Alive, x) if x < 2 => Cell::Dead,
//                     (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
//                     (Cell::Alive, x) if x > 3 => Cell::Dead,
//                     (Cell::Dead, 3) => Cell::Alive,
//                     (otherwise, _) => otherwise,
//                 };

//                 next[idx] = next_cell;
//             }
//         }

//         self.cells = next;
//     }

//     // ...
//     pub fn new() -> Universe {
//         let width = 128;
//         let height = 128;

//         let cells = (0..width * height)
//             .map(|i| {
//                 if i % 2 == 0 || i % 7 == 0 {
//                     Cell::Alive
//                 } else {
//                     Cell::Dead
//                 }
//             })
//             .collect();

//         Universe {
//             width,
//             height,
//             cells,
//         }
//     }

//     pub fn width(&self) -> u32 {
//         self.width
//     }

//     pub fn height(&self) -> u32 {
//         self.height
//     }

//     pub fn cells(&self) -> *const Cell {
//         self.cells.as_ptr()
//     }

//     /// Set the width of the universe.
//     ///
//     /// Resets all cells to the dead state.
//     pub fn set_width(&mut self, width: u32) {
//         self.width = width;
//         self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
//     }

//     /// Set the height of the universe.
//     ///
//     /// Resets all cells to the dead state.
//     pub fn set_height(&mut self, height: u32) {
//         self.height = height;
//         self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
//     }

//     pub fn toggle_cell(&mut self, row: u32, column: u32) {
//         let idx = self.get_index(row, column);
//         self.cells[idx].toggle();
//     }
// }

// impl Universe {
//     /// Get the dead and alive values of the entire universe.
//     pub fn get_cells(&self) -> &[Cell] {
//         &self.cells
//     }

//     /// Set cells to be alive in a universe by passing the row and column
//     /// of each cell as an array.
//     pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
//         for (row, col) in cells.iter().cloned() {
//             let idx = self.get_index(row, col);
//             self.cells[idx] = Cell::Alive;
//         }
//     }
// }

// pub struct Timer<'a> {
//     name: &'a str,
// }

// impl<'a> Timer<'a> {
//     pub fn new(name: &'a str) -> Timer<'a> {
//         console::time_with_label(name);
//         Timer { name }
//     }
// }

// impl<'a> Drop for Timer<'a> {
//     fn drop(&mut self) {
//         console::time_end_with_label(self.name);
//     }
// }
