use image::ImageBuffer;

use crate::Point;

struct Grid {
    width: usize,
    height: usize,
    cell_size: (u32, u32),
    cells: Vec<usize>,
    normalize: bool,
}

impl Grid {
    fn new(width: u32, height: u32, cell_size: (u32, u32), normalize_data: bool) -> Self {
        let grid_width = (width / cell_size.0) as usize;
        let grid_height: usize = (height / cell_size.1) as usize;
        let cells = vec![0; grid_width * grid_height];

        Self {
            width: grid_width,
            height: grid_height,
            cell_size,
            cells,
            normalize: normalize_data,
        }
    }

    fn increment_count(&mut self, point: &Point) {
        let mut cell_x = f64::ceil(point.x / self.cell_size.0 as f64) as i64;
        let mut cell_y = f64::ceil(point.y / self.cell_size.1 as f64) as i64;
        if self.normalize {
            let (norm_vector_x, norm_vector_y) = (self.width / 2, self.height / 2);
            cell_x = cell_x + norm_vector_x as i64;
            cell_y = cell_y + norm_vector_y as i64;
        }
        let index = self.idx_from_point(cell_x as usize, cell_y as usize);
        self.cells[index] = self.cells[index] + 1;
    }

    fn idx_from_point(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn point_from_idx(&self, index: usize) -> (u32, u32) {
        let x = index % self.width;
        let y = index / self.width;
        (x as u32, y as u32)
    }
}

pub fn calculate(
    data_set: &Vec<Point>,
    width: u32,
    height: u32,
    cell_size: (u32, u32),
    normalize_data: bool,
) -> Vec<usize> {
    let mut grid = Grid::new(width, height, cell_size, normalize_data);
    data_set.iter().for_each(|p| grid.increment_count(p));
    draw(width, height, &grid);
    grid.cells
}

fn draw(width: u32, height: u32, grid: &Grid) {
    let mut img = ImageBuffer::from_fn(width, height, |_, _| image::Rgb::<u8>([0, 0, 0]));
    for (idx, counter) in (&grid.cells).iter().enumerate() {
        if *counter > 0 {
            let (x, mut y) = grid.point_from_idx(idx);
            // graph library starts y at the top left so we have to adjust (grid has origin in the bottom left)
            y = height - y;
            img.put_pixel(x, y, image::Rgb([255, 0, 0]));
        }
    }
    img.save("test_data/png/test.png").unwrap();
}
