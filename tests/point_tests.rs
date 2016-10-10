extern crate accountant;

use accountant::core::point::{Point, distance, move_towards};

#[test]
fn test_distance() {
    let point1 = Point::new(0, 0);
    let point2 = Point::new(100, 0);
    let point3 = Point::new(100, 100);

    assert_eq!(distance(&point1, &point1), 0.0);
    assert_eq!(distance(&point1, &point2), 100.0);
    let d = distance(&point1, &point3);
    assert!(d > 141.0 && d < 142.0);
}

#[test]
fn test_move_towards() {
    let src = Point::new(0, 0);
    let dst = Point::new(500, 500);
    let speed = 500;
    let point = move_towards(&src, &dst, speed);
    assert_eq!(point, Point::new(353, 353));
}
