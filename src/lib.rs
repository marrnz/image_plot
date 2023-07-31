mod clustering;
mod draw;
mod grid;
mod outlier;
mod smoothing;
mod spatial_heatmap;

use outlier::suppression::{RemoveConfig, SuppressionStrategy};
use spatial_heatmap::{create_heatmap, CoordinateSystem, Config};

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

type ImageUnit = u32;
type GridUnit = usize;

pub fn do_it() -> Result<(), String> {
    let data_set = include_str!("../test_data/global_terrorism_stripped.csv");
    let data_set = data_set
        .lines()
        .enumerate()
        .filter(|(_, line)| line.split(",").count() > 1)
        .map(|(idx, line)| {
            line.split(",")
                .take(2)
                .map(|value| {
                    value
                        .parse::<f64>()
                        .expect(format!("WTF in line {}", idx).as_str())
                })
                .collect::<Vec<f64>>()
        })
        // file has latitude first, but longitude represents x and latitude y so we have to swap it
        .map(|split_line| Point::new(split_line[1], split_line[0]))
        .collect();

    let coordinate_system = CoordinateSystem {
        min_x_axis: -180,
        max_x_axis: 180,
        min_y_axis: -90,
        max_y_axis: 90,
    };
    let config = Config::new(
        coordinate_system,
        (2, 2),
        720,
        360,
        Some(SuppressionStrategy::Removing(RemoveConfig {
            threshold: 200,
        })),
    );
    create_heatmap(&data_set, config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _ = do_it();
    }
}
