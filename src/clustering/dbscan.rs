use std::{cell::RefCell, collections::HashMap};

use crate::GridUnit;

pub struct DBSCANPoint {
    pub x: GridUnit,
    pub y: GridUnit,
    kind: PointKind,
    visited: bool,
}

impl From<(GridUnit, GridUnit)> for DBSCANPoint {
    fn from((x, y): (GridUnit, GridUnit)) -> Self {
        Self {
            x,
            y,
            visited: false,
            kind: PointKind::Noise,
        }
    }
}

enum PointKind {
    Core,
    Border,
    Noise,
}

type Cluster = Vec<usize>;

// TODO: Revisit algorithm, see https://github.com/s3rvac/blog/blob/master/en-2017-01-01-implementing-dbscan-from-distance-matrix-in-rust/src/dbscan.rs
pub fn cluster(
    data_set: Vec<(GridUnit, GridUnit)>,
    eps: f64,
    min_pts: usize,
) -> HashMap<usize, Cluster> {
    let data_set: Vec<RefCell<DBSCANPoint>> = data_set
        .into_iter()
        .map(|p| RefCell::new(DBSCANPoint::from(p)))
        .collect();
    let mut clusters: HashMap<usize, Cluster> = HashMap::new();
    for i in 0..data_set.len() {
        if let Ok(mut point) = data_set[i].try_borrow_mut() {
            if point.visited {
                continue;
            }
            point.visited = true;
        }
        let neighbours = region_query(&data_set, eps, &data_set[i]);
        if neighbours.len() >= min_pts {
            if let Ok(mut point) = data_set[i].try_borrow_mut() {
                point.kind = PointKind::Core;
            }
            clusters.insert(i, Vec::new());
            expand_cluster(i, &data_set, neighbours, &mut clusters, eps, min_pts)
        }
    }
    clusters
}

fn expand_cluster<'a>(
    center_idx: usize,
    data_set: &'a Vec<RefCell<DBSCANPoint>>,
    mut neighbours: Vec<&'a RefCell<DBSCANPoint>>,
    clusters: &mut HashMap<usize, Cluster>,
    eps: f64,
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
            neighbours[i].borrow_mut().kind = PointKind::Border;
            clusters.get_mut(&center_idx).unwrap().push(idx);
        }
    }
}

fn region_query<'a>(
    data_set: &'a [RefCell<DBSCANPoint>],
    eps: f64,
    center: &RefCell<DBSCANPoint>,
) -> Vec<&'a RefCell<DBSCANPoint>> {
    data_set
        .iter()
        .filter(|p| calculate_distance(center, p) <= eps)
        .collect()
}

fn calculate_distance(center: &RefCell<DBSCANPoint>, p2: &RefCell<DBSCANPoint>) -> f64 {
    let exponent: u32 = 2;
    let x = p2.borrow().x as isize - center.borrow().x as isize;
    let y = p2.borrow().y as isize - center.borrow().y as isize;
    let distance = x.pow(exponent) + y.pow(exponent);
    let distance = distance as f64;
    distance.sqrt().abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;

    #[test]
    fn should_cluster_data() {
        let data_set = vec![
            (1, 1),
            (2, 2),
            (3, 3),
            (8, 8),
            (12, 12),
            (24, 24),
            (26, 26),
            (27, 27),
        ];

        let result = cluster(data_set, 3., 3);

        let mut expected: HashMap<GridUnit, Vec<GridUnit>> = HashMap::new();
        expected.insert(0, vec![0, 1, 2]);
        expected.insert(6, vec![5, 6, 7]);
        for (_, cluster) in result {
            assert!(expected
                .iter()
                .map(|(_, expected_cluster)| expected_cluster)
                .any(|expected_cluster| *expected_cluster == cluster));
        }
    }
}
