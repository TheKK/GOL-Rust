use rustbox;
use rustbox::{Color, RustBox};

use rand::{Rng, thread_rng};

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub is_alive: bool,
    pub next_state: bool,
}

impl Cell {
    fn new(alive: bool) -> Cell {
        Cell {
            is_alive: alive,
            next_state: false,
        }
    }
}

pub struct World {
    width: i32,
    height: i32,
    cell_color: Color,
    cell_token: char,
    cells: Vec<Cell>,
}

impl World {
    #[inline]
    fn cell_is_alive_at(&self, x: i32, y: i32) -> bool {
        self.cells[(x + y * self.width) as usize].is_alive
    }

    pub fn reset(&mut self) {
        let mut rng = thread_rng();
        for cell in &mut self.cells {
            cell.is_alive = rng.gen();
        }
    }

    pub fn resize(&mut self, w: usize, h: usize) {
        self.width = w as i32;
        self.height = h as i32;
        self.cells.resize(w * h, Cell::new(true));
    }

    pub fn update(&mut self) {
        let mut cell_surround_num;

        for y in 0..self.height {
            for x in 0..self.width {
                cell_surround_num = 0;

                if (x + 1 < self.width) && self.cell_is_alive_at(x + 1, y) {
                    cell_surround_num += 1
                }
                if (x - 1 >= 0) && self.cell_is_alive_at(x - 1, y) {
                    cell_surround_num += 1
                }
                if (y + 1 < self.height) && self.cell_is_alive_at(x, y + 1) {
                    cell_surround_num += 1
                }
                if (y - 1 >= 0) && self.cell_is_alive_at(x, y - 1) {
                    cell_surround_num += 1
                }

                if ((x + 1 < self.width) && (y + 1 < self.height)) &&
                   self.cell_is_alive_at(x + 1, y + 1) {
                    cell_surround_num += 1
                }
                if ((x + 1 < self.width) && (y - 1 >= 0)) && self.cell_is_alive_at(x + 1, y - 1) {
                    cell_surround_num += 1
                }
                if ((x - 1 >= 0) && (y + 1 < self.height)) && self.cell_is_alive_at(x - 1, y + 1) {
                    cell_surround_num += 1
                }
                if ((x - 1 >= 0) && (y - 1 >= 0)) && self.cell_is_alive_at(x - 1, y - 1) {
                    cell_surround_num += 1
                }

                let mut cell = self.cells.get_mut((x + self.width * y) as usize).unwrap();
                match cell_surround_num {
                    2 => {
                        cell.next_state = cell.is_alive;
                    }
                    3 => {
                        cell.next_state = true;
                    }
                    _ => {
                        cell.next_state = false;
                    }
                }
            }
        }

        for cell in &mut self.cells {
            cell.is_alive = cell.next_state;
        }
    }

    pub fn render(&self, rustbox: &RustBox) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self.cells
                    .get((x + y * self.width) as usize)
                    .expect("Cell out of index");

                if cell.is_alive {
                    rustbox.print_char(x as usize,
                                       y as usize,
                                       rustbox::RB_NORMAL,
                                       self.cell_color,
                                       Color::Default,
                                       self.cell_token);
                }
            }
        }
    }
}

pub struct WorldBuilder {
    width: i32,
    height: i32,
    cell_color: Color,
    cell_token: char,
}

impl WorldBuilder {
    pub fn new() -> WorldBuilder {
        WorldBuilder {
            width: 0,
            height: 0,
            cell_color: Color::Default,
            cell_token: '*',
        }
    }

    pub fn build(&self) -> World {
        let mut rng = thread_rng();
        let cells = (0..(self.width * self.height))
            .map(|_| Cell::new(rng.gen::<bool>()))
            .collect();

        World {
            width: self.width,
            height: self.height,
            cell_color: self.cell_color,
            cell_token: self.cell_token,
            cells: cells,
        }
    }

    pub fn world_size(&mut self, width: i32, height: i32) -> &mut WorldBuilder {
        self.width = width;
        self.height = height;

        self
    }

    pub fn cell_color(&mut self, color: Color) -> &mut WorldBuilder {
        self.cell_color = color;

        self
    }

    pub fn cell_token(&mut self, token: char) -> &mut WorldBuilder {
        self.cell_token = token;

        self
    }
}
