use spatial_heatmap::{HeatmapConfig, RemoveConfig, SuppressionStrategy};

mod dbscan;
mod spatial_heatmap;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

pub fn do_it() {
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

    /*
        McDonalds
        let data_set = include_str!("../test_data/mcdonalds_stripped.csv");
        let data_set = data_set
            .lines()
            .enumerate()
            .map(|(idx, line)| {
                line.split(",")
                    .take(2)
                    .map(|value| value.parse::<f64>().expect(format!("WTF in line {}", idx).as_str()))
                    .collect::<Vec<f64>>()
            })
            // file has latitude first, but longitude represents x and latitude y so we have to swap it
            .map(|split_line| Point::new(split_line[1], split_line[0]))
            .collect();
    */
    //DBSCAN Test

    /*
    let result = calculate(data_set, 3., 2);
    println!("My Result: {:?}", result);
    */
    let config = HeatmapConfig {
        contains_negatives: true,
        suppression_strategy: Some(SuppressionStrategy::Removing(RemoveConfig {
            threshold: 200,
        })),
    };
    let result = spatial_heatmap::calculate(&data_set, 360, 180, (1, 1), config);
    //println!("My Result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        do_it();
    }
}
