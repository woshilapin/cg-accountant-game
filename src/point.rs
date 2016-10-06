extern crate euclid;

pub type Point = euclid::point::Point2D<isize>;

pub fn distance(p1: &Point, p2: &Point) -> f64 {
    let dir = *p2 - *p1;
    let distance = f64::sqrt((dir.x * dir.x + dir.y * dir.y) as f64);
    distance
}
