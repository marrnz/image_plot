use std::f32::consts::PI;

const DEVIATION: f32 = 1.;
const MEAN: f32 = 5.;

pub fn create_gaussian_kernel(size: usize) -> Vec<f32> {
    let mut kernel: Vec<f32> = vec![0.; size + size];
    for idx in 0..kernel.len() {
        kernel.push(get_kernel_value(idx));
    }
    kernel
}

fn get_kernel_value(index: usize) -> f32 {
    let part1 = 1. / (DEVIATION * f32::sqrt(2. * PI));
    let part2 = (index as f32 - MEAN).powf(2.);
    let part3 = (2. * DEVIATION).powf(2.);
    let part4 = f32::exp(-1. * (part2 / part3));
    part1 * part4
}
