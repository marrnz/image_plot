use std::{cell::RefCell, collections::HashMap};

type Unit = f32;

pub struct Point {
    pub x: Unit,
    pub y: Unit,
    visited: bool,
    is_noise: bool,
}

impl Point {
    pub fn new(x: Unit, y: Unit) -> Self {
        Self {
            x,
            y,
            visited: false,
            is_noise: false,
        }
    }
}

type Cluster = Vec<usize>;

// TODO: Revisit algorithm, see https://github.com/s3rvac/blog/blob/master/en-2017-01-01-implementing-dbscan-from-distance-matrix-in-rust/src/dbscan.rs
pub fn calculate(data_set: Vec<RefCell<Point>>, eps: Unit, min_pts: usize) -> HashMap<usize, Vec<usize>>{
    let mut clusters: HashMap<usize, Cluster> = HashMap::new();
    for i in 0..data_set.len() {
        if let Ok(mut point) = data_set[i].try_borrow_mut() {
            if point.visited {
                continue;
            }
            point.visited = true;
        }
        let neighbours = region_query(&data_set, eps, &data_set[i]);
        if neighbours.len() < min_pts {
            if let Ok(mut point) = data_set[i].try_borrow_mut() {
                point.is_noise = true;
            }
        } else {
            clusters.insert(i, Vec::new());
            expand_cluster(i, &data_set, neighbours, &mut clusters, eps, min_pts)
        }
    }
    clusters
}

fn expand_cluster<'a>(
    center_idx: usize,
    data_set: &'a Vec<RefCell<Point>>,
    mut neighbours: Vec<&'a RefCell<Point>>,
    clusters: &mut HashMap<usize, Cluster>,
    eps: Unit,
    min_pts: usize,
) {
    for i in 0..neighbours.len() {
        if !neighbours[i].borrow().visited {
            neighbours[i].borrow_mut().visited = true;
            let mut neighbors_neighbors = region_query(data_set, eps, neighbours[i]);
            if neighbors_neighbors.len() >= min_pts {
                neighbours.append(&mut neighbors_neighbors);
            }
        }

        let idx = data_set
            .iter()
            .position(|p| std::ptr::eq(p, neighbours[i]))
            .expect("There has to be an index!");
        let already_in_cluster = clusters
            .values()
            .flat_map(|cluster| cluster.iter())
            .any(|point_indexes| *point_indexes == idx);
        if !already_in_cluster {
            neighbours[i].borrow_mut().is_noise = false;
            clusters.get_mut(&center_idx).unwrap().push(idx);
        }
    }
}

fn region_query<'a>(
    data_set: &'a [RefCell<Point>],
    eps: Unit,
    center: &RefCell<Point>,
) -> Vec<&'a RefCell<Point>> {
    data_set
        .iter()
        .filter(|p| calculate_distance(center, p) <= eps)
        .collect()
}

fn calculate_distance(center: &RefCell<Point>, p2: &RefCell<Point>) -> f32 {
    let base: f32 = 2.;
    let distance =
        base.powf(p2.borrow().x - center.borrow().x) + base.powf(p2.borrow().y - center.borrow().y);
    distance.sqrt().abs()
}
