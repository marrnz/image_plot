mod dbscan;

use std::cell::RefCell;
use dbscan::{calculate, Point};

pub fn do_it() {
    let data_set = vec![
        RefCell::new(Point::new(1., 1.)),
        RefCell::new(Point::new(2., 2.)),
        RefCell::new(Point::new(3., 3.)),
        RefCell::new(Point::new(7., 7.)),
        RefCell::new(Point::new(8., 8.)),
        RefCell::new(Point::new(9., 9.)),
        RefCell::new(Point::new(7., 7.)),
        RefCell::new(Point::new(8., 8.)),
        RefCell::new(Point::new(9., 9.)),
        RefCell::new(Point::new(12., 12.)),
        RefCell::new(Point::new(13., 13.)),
        RefCell::new(Point::new(14., 14.)),
    ];
    let result = calculate(data_set, 3., 2);
    println!("My Result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        do_it();
    }
}
