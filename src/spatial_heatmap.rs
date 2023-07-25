use hsl::HSL;
use image::ImageBuffer;

use crate::Point;

pub struct HeatmapConfig {
    // TODO: Remove as config and evaluate by itself
    pub contains_negatives: bool,
    pub suppression_strategy: Option<SuppressionStrategy>,
}

struct Grid {
    width: usize,
    height: usize,
    cell_size: (u32, u32),
    cells: Vec<usize>,
    config: HeatmapConfig,
    max_value: usize,
}

pub enum SuppressionStrategy {
    Thresholding,
    Clipping,
    LogarithmicScaling,
    Removing(RemoveConfig),
}

trait OutlierSuppression {
    fn suppress(&self, cells: &mut Vec<usize>);
}

struct ThresholdConfig;
struct ClipConfig;
struct LogarithmicScalingConfig;
pub struct RemoveConfig {
    pub threshold: usize,
}

impl OutlierSuppression for ThresholdConfig {
    fn suppress(&self, cells: &mut Vec<usize>) {
        todo!()
    }
}

impl OutlierSuppression for ClipConfig {
    fn suppress(&self, cells: &mut Vec<usize>) {
        todo!()
    }
}

impl OutlierSuppression for LogarithmicScalingConfig {
    fn suppress(&self, cells: &mut Vec<usize>) {
        todo!()
    }
}

impl OutlierSuppression for RemoveConfig {
    fn suppress(&self, cells: &mut Vec<usize>) {
        // elements can't be removed because it skews the grid
        // instead removing means zeroing the entry
        cells
            .iter_mut()
            .filter(|counter| **counter > self.threshold)
            .for_each(|counter| *counter = 0)
    }
}

impl Grid {
    fn new(width: u32, height: u32, cell_size: (u32, u32), config: HeatmapConfig) -> Self {
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

    fn increment_count(&mut self, point: &Point) {
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

    fn evaluate_max_value(&mut self) {
        if let Some(max) = self.cells.iter().max() {
            self.max_value = *max;
        }
    }

    fn idx_from_point(&self, x: usize, y: usize) -> usize {
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

pub fn calculate(
    data_set: &Vec<Point>,
    width: u32,
    height: u32,
    cell_size: (u32, u32),
    config: HeatmapConfig,
) -> Vec<usize> {
    let mut grid = Grid::new(width, height, cell_size, config);
    data_set.iter().for_each(|p| grid.increment_count(p));
    if let Some(suppression_strategy) = &grid.config.suppression_strategy {
        match suppression_strategy {
            SuppressionStrategy::Removing(config) => config.suppress(&mut grid.cells),
            _ => {}
        }
    }
    grid.evaluate_max_value();
    draw(width, height, &grid);
    grid.cells
}

fn draw(width: u32, height: u32, grid: &Grid) {
    let mut img = ImageBuffer::from_fn(width, height, |_, _| image::Rgba::<u8>([0, 0, 0, 255]));
    for (idx, counter) in (&grid.cells).iter().enumerate() {
        if *counter > 0 {
            grid.get_pixels_from_cell(idx)
                .iter()
                .for_each(|&(x, y)| img.put_pixel(x, y, counter_to_rgb(grid, *counter)));
        }
    }
    img.save("test_data/png/test.png").unwrap();
}

fn counter_to_rgb(grid: &Grid, counter: usize) -> image::Rgba<u8> {
    if grid.max_value != 0 {
        let h = (1. - (counter as f64 / grid.max_value as f64)) * 240.;
        let hsl = HSL { h, s: 1., l: 0.5 };
        let rgb = hsl.to_rgb();
        return image::Rgba([rgb.0, rgb.1, rgb.2, 125]);
    }
    image::Rgba([255, 255, 255, 0])
}
