use crate::{
    clustering::dbscan::cluster,
    draw::draw_image,
    grid::Grid,
    outlier::suppression::{OutlierSuppression, SuppressionStrategy},
    Point,
};

pub struct HeatmapConfig {
    // TODO: Remove as config and evaluate by itself
    pub contains_negatives: bool,
    pub suppression_strategy: Option<SuppressionStrategy>,
}

fn get_intensity_grid(
    data_set: &Vec<Point>,
    width: u32,
    height: u32,
    cell_size: (u32, u32),
    config: HeatmapConfig,
) -> Grid {
    let mut grid = Grid::new(width, height, cell_size, config);
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
    width: u32,
    height: u32,
    cell_size: (u32, u32),
    config: HeatmapConfig,
) -> Result<(), String> {
    let mut grid = get_intensity_grid(data_set, width, height, cell_size, config);
    suppress_outliers(&mut grid);
    let mut grid_points: Vec<Point> = Vec::with_capacity(grid.cells.len());
    for idx in 0..grid.cells.len() {
        let (x, y) = grid.point_from_idx(idx);
        grid_points.push(Point::new(x as f64, y as f64));
    }
    let clustered_result = cluster(grid_points, 5., 5);
    draw_image(width, height, &grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_pixels_when_providing_index() {
        let grid = Grid::new(
            21,
            21,
            (3, 3),
            HeatmapConfig {
                contains_negatives: false,
                suppression_strategy: None,
            },
        );

        // index = coordinate (2,1)
        let pixels = grid.get_pixels_from_cell(9);

        // pixels should be (6, 3) to (8, 5) with y = 0 at top
        assert_eq!(
            pixels,
            vec![
                (6, 18),
                (6, 17),
                (6, 16),
                (7, 18),
                (7, 17),
                (7, 16),
                (8, 18),
                (8, 17),
                (8, 16)
            ]
        )
    }
}
