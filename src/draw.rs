use std::collections::HashMap;

use hsl::HSL;
use image::{imageops::colorops::contrast_in_place, ImageBuffer};

use crate::{spatial_heatmap::Config, GridUnit, ImageUnit};

use super::grid::Grid;

pub fn draw_image(config: &Config, grid: &Grid) -> Result<(), String> {
    let max_value = *grid
        .evaluate_max_value()
        .ok_or("Error finding max intensity")?;

    let mut img = ImageBuffer::from_fn(config.image_width, config.image_height, |_, _| {
        image::Rgba::<u8>([0, 0, 0, 0])
    });
    for (idx, counter) in (&grid.cells).iter().enumerate() {
        if *counter > 0 {
            get_pixels_from_cell(grid, config, idx)
                .iter()
                .for_each(|&(x, y)| img.put_pixel(x, y, counter_to_rgb(max_value, *counter)));
        }
    }
    //TODO: return byte array, impl. debug mode for saving result
    img.save("test_data/png/test.png").unwrap();
    Ok(())
}

pub fn draw_cluster(
    config: &Config,
    grid: &Grid,
    cluster_data: HashMap<usize, Vec<usize>>,
) -> Result<(), String> {
    let max_value = cluster_data.iter().map(|entry| entry.0).max().unwrap();
    let mut img = ImageBuffer::from_fn(config.image_width, config.image_height, |_, _| {
        image::Rgba::<u8>([0, 0, 0, 255])
    });
    for (&idx, cluster) in &cluster_data {
        let color = counter_to_rgb(*max_value, idx);
        get_pixels_from_cell(grid, config, idx)
            .iter()
            .for_each(|&(x, y)| img.put_pixel(x, y, color));
        for point in cluster {
            get_pixels_from_cell(grid, config, *point)
                .iter()
                .for_each(|&(x, y)| img.put_pixel(x, y, color));
        }
    }
    img.save("test_data/png/test_cluster.png").unwrap();
    Ok(())
}

fn counter_to_rgb(max_value: usize, counter: usize) -> image::Rgba<u8> {
    if max_value != 0 {
        let h = (1. - (counter as f64 / max_value as f64)) * 240.;
        let hsl = HSL { h, s: 1., l: 0.5 };
        let rgb = hsl.to_rgb();
        return image::Rgba([rgb.0, rgb.1, rgb.2, 125]);
    }
    image::Rgba([255, 255, 255, 0])
}

fn get_pixels_from_cell(grid: &Grid, config: &Config, index: usize) -> Vec<(ImageUnit, ImageUnit)> {
    let x_pixels_per_cell = config.image_width / grid.grid_width as u32;
    let y_pixels_per_cell = config.image_height / grid.grid_height as u32;
    let mut pixels: Vec<(ImageUnit, ImageUnit)> = Vec::new();
    let (x, y) = grid.point_from_idx(index);
    let start_pixel_x = x as ImageUnit * x_pixels_per_cell;
    // graph library starts y at the top left so we have to adjust (grid has origin in the bottom left)
    let start_pixel_y = grid.config.image_height - y as ImageUnit * y_pixels_per_cell;

    for x_increment in 0..x_pixels_per_cell {
        for y_increment in 0..y_pixels_per_cell {
            pixels.push((
                start_pixel_x + x_increment as ImageUnit,
                start_pixel_y - y_increment as ImageUnit,
            ));
        }
    }

    pixels
}

#[cfg(test)]
mod tests {
    use crate::spatial_heatmap::{Config, CoordinateSystem};

    use super::*;

    #[test]
    fn should_get_pixels_when_providing_index() {
        let coordinate_system = CoordinateSystem {
            min_x_axis: 0,
            max_x_axis: 30,
            min_y_axis: 0,
            max_y_axis: 30,
        };
        let config = Config::new(coordinate_system, (3, 3), 30, 30, None);
        let grid = Grid::new(&config);

        // index = coordinate (2,1)
        let pixels = get_pixels_from_cell(&grid, &config, 12);

        // pixels should be (6, 3) to (8, 5) with y = 0 at top
        assert_eq!(
            pixels,
            vec![
                (6, 27),
                (6, 26),
                (6, 25),
                (7, 27),
                (7, 26),
                (7, 25),
                (8, 27),
                (8, 26),
                (8, 25)
            ]
        )
    }
}
