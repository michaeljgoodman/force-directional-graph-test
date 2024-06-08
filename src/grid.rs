use crate::graph::Circle;
use std::collections::HashMap;

pub struct SpatialGrid {
    cell_size: f64,
    grid: HashMap<(i32, i32), Vec<usize>>,
}

impl SpatialGrid {
    pub fn new(cell_size: f64) -> SpatialGrid {
        SpatialGrid {
            cell_size,
            grid: HashMap::new(),
        }
    }

    fn get_cell(&self, x: f64, y: f64) -> (i32, i32) {
        ((x / self.cell_size) as i32, (y / self.cell_size) as i32)
    }

    pub fn clear(&mut self) {
        self.grid.clear();
    }

    pub fn insert(&mut self, id: usize, x: f64, y: f64) {
        let cell = self.get_cell(x, y);
        self.grid.entry(cell).or_insert_with(Vec::new).push(id);
    }

    pub fn query(&self, x: f64, y: f64) -> Vec<usize> {
        let cell = self.get_cell(x, y);
        let mut neighbors = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if let Some(cell_neighbors) = self.grid.get(&(cell.0 + dx, cell.1 + dy)) {
                    neighbors.extend(cell_neighbors);
                }
            }
        }
        neighbors
    }
}
