use crate::{
    draw::draw_image,
    grid::Grid,
    outlier::suppression::{OutlierSuppression, SuppressionStrategy},
    GridUnit, Pixel, Point,
};

pub struct CoordinateSystem {
    min_x_axis: isize,
    max_x_axis: usize,
    min_y_axis: isize,
    max_y_axis: usize,
}

pub struct HeatmapConfig {
    grid_width: GridUnit,
    grid_height: GridUnit,
    cell_size: (GridUnit, GridUnit),
    has_negative_values: bool,
    image_width: Pixel,
    image_height: Pixel,
    suppression_strategy: Option<SuppressionStrategy>,
}

// TODO:  Impl. Builder ??
impl HeatmapConfig {
    pub fn new(
        coordinate_system: CoordinateSystem,
        cell_size: (GridUnit, GridUnit),
        image_width: Pixel,
        image_height: Pixel,
        suppression_strategy: Option<SuppressionStrategy>,
    ) -> Self {
        let has_negative_values =
            coordinate_system.min_x_axis < 0 || coordinate_system.min_y_axis < 0;
        let grid_width =
            isize::abs(coordinate_system.min_x_axis) as usize + coordinate_system.max_x_axis;
        let grid_height =
            isize::abs(coordinate_system.min_y_axis) as usize + coordinate_system.max_y_axis;
        Self {
            grid_width,
            grid_height,
            has_negative_values,
            cell_size,
            image_width,
            image_height,
            suppression_strategy,
        }
    }
}

fn get_intensity_grid(
    data_set: &Vec<Point>,
    width: Pixel,
    height: Pixel,
    config: HeatmapConfig,
) -> Grid {
    let mut grid = Grid::new(config);
    data_set.iter().for_each(|p| grid.increment_count(p));
    grid
}

fn suppress_outliers(grid: &mut Grid) {
    if let Some(suppression_strategy) = &grid.config.suppression_strategy {
        match suppression_strategy {
            SuppressionStrategy::Removing(config) => config.suppress(&mut grid.cells),
            _ => {}
        }
    }
}

pub fn create_heatmap(
    data_set: &Vec<Point>,
    width: Pixel,
    height: Pixel,
    config: HeatmapConfig,
) -> Result<(), String> {
    let mut grid = get_intensity_grid(data_set, width, height, config);
    suppress_outliers(&mut grid);
    let mut grid_points: Vec<Point> = Vec::with_capacity(grid.cells.len());
    for idx in 0..grid.cells.len() {
        let (x, y) = grid.point_from_idx(idx);
        grid_points.push(Point::new(x as f64, y as f64));
    }
    //let clustered_result = cluster(grid_points, 5., 5);
    draw_image(width, height, &grid)
}
