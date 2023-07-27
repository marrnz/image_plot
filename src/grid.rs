use crate::Point;

use super::spatial_heatmap::HeatmapConfig;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cell_size: (u32, u32),
    pub cells: Vec<usize>,
    pub config: HeatmapConfig,
    pub max_value: usize,
}

impl Grid {
    pub fn new(width: u32, height: u32, cell_size: (u32, u32), config: HeatmapConfig) -> Self {
        let grid_width = (width / cell_size.0) as usize;
        let grid_height: usize = (height / cell_size.1) as usize;
        let cells = vec![0; grid_width * grid_height];

        Self {
            width: grid_width,
            height: grid_height,
            cell_size,
            cells,
            config,
            max_value: 0,
        }
    }

    pub fn increment_count(&mut self, point: &Point) {
        let mut cell_x = f64::ceil(point.x / self.cell_size.0 as f64) as i64;
        let mut cell_y = f64::ceil(point.y / self.cell_size.1 as f64) as i64;
        if self.config.contains_negatives {
            let (norm_vector_x, norm_vector_y) = (self.width / 2, self.height / 2);
            cell_x = cell_x + norm_vector_x as i64;
            cell_y = cell_y + norm_vector_y as i64;
        }
        let index = self.idx_from_point(cell_x as usize, cell_y as usize);
        self.cells[index] = self.cells[index] + 1;
    }

    pub fn evaluate_max_value(&self) -> Option<&usize> {
        self.cells.iter().max()
    }

    pub fn idx_from_point(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn point_from_idx(&self, index: usize) -> (u32, u32) {
        let x = index % self.width;
        let y = index / self.width;
        (x as u32, y as u32)
    }

    pub fn get_pixels_from_cell(&self, index: usize) -> Vec<(u32, u32)> {
        let mut pixels: Vec<(u32, u32)> = Vec::new();
        let (x, y) = self.point_from_idx(index);
        let start_pixel_x = x * self.cell_size.0;
        // graph library starts y at the top left so we have to adjust (grid has origin in the bottom left)
        let start_pixel_y = (self.height as u32 * self.cell_size.1) - (y * self.cell_size.1);

        for x_increment in 0..self.cell_size.0 {
            for y_increment in 0..self.cell_size.1 {
                pixels.push((start_pixel_x + x_increment, start_pixel_y - y_increment));
            }
        }

        pixels
    }
}
