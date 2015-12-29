use rustbox;
use rustbox::{Color, RustBox};

use rand::{Rng, thread_rng};

pub struct Cell {
    pub is_alive: bool,
    pub next_state: bool,
}

impl Cell {
    pub fn new(alive: bool) -> Cell {
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
    cells: Vec<Cell>,
}

impl World {
    pub fn new(width: i32, height: i32) -> World {
        let mut rng = thread_rng();
        let cells = (0..(width * height))
            .map(|_| {
                Cell::new(rng.gen::<bool>())
            })
            .collect();

        World {
            width: width,
            height: height,
            cell_color: Color::Default,
            cells: cells,
        }
    }

    fn _cell_is_alive_at(&self, x: i32, y: i32) -> bool {
        self.cells[(x + y * self.width) as usize].is_alive
    }

    pub fn reset(&mut self) {
        let mut rng = thread_rng();
        for cell in &mut self.cells {
            cell.is_alive = rng.gen::<bool>();
        }
    }

    pub fn cell_color(&mut self, color: Color) {
        self.cell_color = color;
    }

    pub fn update(&mut self) {
        let mut cell_surround_num;

        for y in 0..self.height {
            for x in 0..self.width {
                cell_surround_num = 0;

                if (x + 1 < self.width) && self._cell_is_alive_at(x + 1, y) { cell_surround_num += 1}
                if (x - 1 >= 0) && self._cell_is_alive_at(x - 1, y) { cell_surround_num += 1}
                if (y + 1 < self.height) && self._cell_is_alive_at(x, y + 1) { cell_surround_num += 1}
                if (y - 1 >= 0) && self._cell_is_alive_at(x, y - 1) { cell_surround_num += 1}

                if ((x + 1 < self.width) && (y + 1 < self.height)) && self._cell_is_alive_at(x + 1, y + 1) { cell_surround_num += 1}
                if ((x + 1 < self.width) && (y - 1 >= 0)) && self._cell_is_alive_at(x + 1, y - 1) { cell_surround_num += 1}
                if ((x - 1 >= 0) && (y + 1 < self.height)) && self._cell_is_alive_at(x - 1, y + 1) { cell_surround_num += 1}
                if ((x - 1 >= 0) && (y - 1 >= 0)) && self._cell_is_alive_at(x - 1, y - 1) { cell_surround_num += 1}

                let mut cell = &mut self.cells[(x + self.width * y) as usize];
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
                let cell = &self.cells[(x + y * self.width) as usize];

                if cell.is_alive {
                    rustbox.print(x as usize, y as usize,
                                  rustbox::RB_NORMAL,
                                  self.cell_color, Color::Default,
                                  "*");
                }
            }
        }
    }
}
