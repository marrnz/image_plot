use hsl::HSL;
use image::ImageBuffer;

use crate::Pixel;

use super::grid::Grid;

pub fn draw_image(width: u32, height: u32, grid: &Grid) -> Result<(), String> {
    let max_value = *grid
        .evaluate_max_value()
        .ok_or("Error finding max intensity")?;
    let mut img = ImageBuffer::from_fn(width, height, |_, _| image::Rgba::<u8>([0, 0, 0, 255]));
    for (idx, counter) in (&grid.cells).iter().enumerate() {
        if *counter > 0 {
            get_pixels_from_cell(grid, idx)
                .iter()
                .for_each(|&(x, y)| img.put_pixel(x, y, counter_to_rgb(max_value, *counter)));
        }
    }
    //TODO: return byte array, impl. debug mode for saving result
    img.save("test_data/png/test.png").unwrap();
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

fn get_pixels_from_cell(grid: &Grid, index: usize) -> Vec<(u32, u32)> {
    let mut pixels: Vec<(u32, u32)> = Vec::new();
    let (x, y) = grid.point_from_idx(index);
    let start_pixel_x = x as Pixel * grid.config.cell_size.0;
    // graph library starts y at the top left so we have to adjust (grid has origin in the bottom left)
    let start_pixel_y =
        (grid.grid_height as Pixel * grid.config.cell_size.1) - (y as Pixel * grid.config.cell_size.1);

    for x_increment in 0..grid.config.cell_size.0 {
        for y_increment in 0..grid.config.cell_size.1 {
            pixels.push((start_pixel_x + x_increment, start_pixel_y - y_increment));
        }
    }

    pixels
}

#[cfg(test)]
mod tests {
    use crate::spatial_heatmap::HeatmapConfig;

    use super::*;

    #[test]
    fn should_get_pixels_when_providing_index() {
        let grid = Grid::new(
            21,
            21,
            HeatmapConfig {
                cell_size: (3, 3),
                contains_negatives: false,
                suppression_strategy: None,
            },
        );

        // index = coordinate (2,1)
        let pixels = get_pixels_from_cell(&grid, 9);

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
