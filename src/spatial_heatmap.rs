use crate::{
    draw::draw_image,
    grid::Grid,
    outlier::suppression::{OutlierSuppression, SuppressionStrategy},
    GridUnit, ImageUnit, Point,
};

pub struct CoordinateSystem {
    pub min_x_axis: isize,
    pub max_x_axis: usize,
    pub min_y_axis: isize,
    pub max_y_axis: usize,
}

//TODO: Hide fields for users
pub struct Config {
    pub coordinate_system: CoordinateSystem,
    pub cell_size: (GridUnit, GridUnit),
    pub has_negative_values: bool,
    pub image_width: ImageUnit,
    pub image_height: ImageUnit,
    pub suppression_strategy: Option<SuppressionStrategy>,
}

// TODO:  Impl. Builder ??
impl Config {
    pub fn new(
        coordinate_system: CoordinateSystem,
        cell_size: (GridUnit, GridUnit),
        image_width: ImageUnit,
        image_height: ImageUnit,
        suppression_strategy: Option<SuppressionStrategy>,
    ) -> Self {
        let has_negative_values =
            coordinate_system.min_x_axis < 0 || coordinate_system.min_y_axis < 0;
        Self {
            coordinate_system,
            has_negative_values,
            cell_size,
            image_width,
            image_height,
            suppression_strategy,
        }
    }
}

fn get_intensity_grid<'a>(
    data_set: &Vec<Point>,
    config: &'a Config,
) -> Grid<'a> {
    let mut grid = Grid::new(&config);
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
    config: Config,
) -> Result<(), String> {
    let mut grid = get_intensity_grid(data_set, &config);
    suppress_outliers(&mut grid);
    let mut grid_points: Vec<Point> = Vec::with_capacity(grid.cells.len());
    for idx in 0..grid.cells.len() {
        let (x, y) = grid.point_from_idx(idx);
        grid_points.push(Point::new(x as f64, y as f64));
    }
    //let clustered_result = cluster(grid_points, 5., 5);
    draw_image(&config, &grid)
}
