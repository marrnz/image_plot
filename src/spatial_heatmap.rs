use draw::{Canvas, Drawing, Shape, Style, Color, SvgRenderer, render};

use crate::Point;

struct Grid {
    width: usize,
    cell_size: (f64, f64),
    cells: Vec<usize>,
}

impl Grid {
    fn new(width: f64, height: f64, cell_size: (f64, f64)) -> Self {
        let grid_width = (width / cell_size.0) as usize;
        let grid_height: usize = (height / cell_size.1) as usize;
        let cells = vec![0; grid_width * grid_height];

        Self {
            width: grid_width,
            cell_size,
            cells,
        }
    }

    fn increment_count(&mut self, point: &Point) {
        let cell_x = f64::ceil(point.x / self.cell_size.0) as usize;
        let cell_y = f64::ceil(point.y / self.cell_size.1) as usize;

        let index = self.idx_from_point(cell_x, cell_y);
        self.cells[index] = self.cells[index] + 1;
    }

    fn idx_from_point(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn point_from_idx(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.width;
        (x, y)
    }


}

pub fn calculate(
    data_set: &Vec<Point>,
    width: f64,
    height: f64,
    cell_size: (f64, f64),
) -> Vec<usize> {
    let mut grid = Grid::new(width, height, cell_size);
    data_set.iter().for_each(|p| grid.increment_count(p));
    draw(width, height, &grid);
    grid.cells
}

fn draw(width: f64, height: f64, grid: &Grid) {
    let mut canvas = Canvas::new(width as u32, height as u32);

    for (idx, counter) in grid.cells.iter().enumerate() {
        let (x, y) = grid.point_from_idx(idx);
        let color = if *counter > 0 { Color::gray(50) } else {Color::black()}; 
        let rect = Drawing::new()
        // give it a shape
        .with_shape(Shape::Rectangle { width: grid.cell_size.0 as u32, height: grid.cell_size.1 as u32})
        // move it around
        .with_xy(x as f32, y as f32)
        // give it a cool style
        .with_style(Style::stroked(1, color));
    canvas.display_list.add(rect);
    }
 
    // save the canvas as an svg
    render::save(
        &canvas,
        "test_data/svg/test_result.svg",
        SvgRenderer::new(),
    )
    .expect("Failed to save")
}
