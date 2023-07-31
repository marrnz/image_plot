use crate::{GridUnit, Point, Pixel};

use super::spatial_heatmap::HeatmapConfig;

pub struct Grid {
    pub grid_width: GridUnit,
    pub grid_height: GridUnit,
    pub cells: Vec<usize>,
    pub config: HeatmapConfig,
    pub max_value: usize,
}

impl Grid {
    pub fn new(config: &HeatmapConfig) -> Self {
        let grid_width = (conbfig. / config.cell_size.0) as GridUnit;
        let grid_height: usize = (height / config.cell_size.1) as GridUnit;
        let cells = vec![0; grid_width * grid_height];

        Self {
            grid_width,
            grid_height,
            cells,
            config,
            max_value: 0,
        }
    }

    pub fn increment_count(&mut self, point: &Point) {
        let mut cell_x = f64::ceil(point.x / self.config.cell_size.0 as f64) as i64;
        let mut cell_y = f64::ceil(point.y / self.config.cell_size.1 as f64) as i64;
        if self.config.contains_negatives {
            let (norm_vector_x, norm_vector_y) = (self.grid_width / 2, self.grid_height / 2);
            cell_x = cell_x + norm_vector_x as i64;
            cell_y = cell_y + norm_vector_y as i64;
        }
        let index = self.idx_from_point(cell_x as GridUnit, cell_y as GridUnit);
        self.cells[index] = self.cells[index] + 1;
    }

    pub fn evaluate_max_value(&self) -> Option<&usize> {
        self.cells.iter().max()
    }

    pub fn idx_from_point(&self, x: GridUnit, y: GridUnit) -> usize {
        y * self.grid_width + x
    }

    pub fn point_from_idx(&self, index: usize) -> (GridUnit, GridUnit) {
        let x = index % self.grid_width;
        let y = index / self.grid_width;
        (x as GridUnit, y as GridUnit)
    }
}
