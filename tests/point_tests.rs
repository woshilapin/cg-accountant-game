extern crate accountant;

use accountant::core::point::Point;

#[test]
fn test_distance() {
    let point1 = Point::new(0, 0);
    let point2 = Point::new(100, 0);
    let point3 = Point::new(100, 100);

    assert_eq!(point1.distance(&point1), 0.0);
    assert_eq!(point1.distance(&point2), 100.0);
    let d = point1.distance(&point3);
    assert!(d > 141.0 && d < 142.0);
}

#[test]
fn test_move_towards() {
    let mut src = Point::new(0, 0);
    let dst = Point::new(500, 500);
    let speed = 500;
    src.move_towards(&dst, speed);
    assert_eq!(src, Point::new(353, 353));
}
