use hsl::HSL;
use image::ImageBuffer;

use super::grid::Grid;

pub fn draw_image(width: u32, height: u32, grid: &Grid) -> Result<(), String> {
    let max_value = *grid
        .evaluate_max_value()
        .ok_or("Error finding max intensity")?;
    let mut img = ImageBuffer::from_fn(width, height, |_, _| image::Rgba::<u8>([0, 0, 0, 255]));
    for (idx, counter) in (&grid.cells).iter().enumerate() {
        if *counter > 0 {
            grid.get_pixels_from_cell(idx)
                .iter()
                .for_each(|&(x, y)| img.put_pixel(x, y, counter_to_rgb(max_value, *counter)));
        }
    }
    //TODO: return byte array, impl. debug mode for saving result
    img.save("test_data/png/test.png").unwrap();
    Ok(())
}

pub fn counter_to_rgb(max_value: usize, counter: usize) -> image::Rgba<u8> {
    if max_value != 0 {
        let h = (1. - (counter as f64 / max_value as f64)) * 240.;
        let hsl = HSL { h, s: 1., l: 0.5 };
        let rgb = hsl.to_rgb();
        return image::Rgba([rgb.0, rgb.1, rgb.2, 125]);
    }
    image::Rgba([255, 255, 255, 0])
}
