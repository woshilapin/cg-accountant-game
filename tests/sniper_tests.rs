extern crate accountant;

use accountant::core::sniper::Sniper;
use accountant::core::point::Point;

#[test]
fn test_walk() {
    let dst1_pos = Point::new(0, 0);

    let sniper1 = Sniper::new(0, 0);
    assert_eq!(sniper1.pos, Point::new(0, 0));

    let mut sniper2 = Sniper::new(1000, 0);
    assert_eq!(sniper2.pos, Point::new(1000, 0));
    sniper2.walk(&dst1_pos);
    assert_eq!(sniper2.pos, Point::new(0, 0));

    let mut sniper3 = Sniper::new(1500, 0);
    assert_eq!(sniper3.pos, Point::new(1500, 0));
    sniper3.walk(&dst1_pos);
    assert_eq!(sniper3.pos, Point::new(500, 0));
    sniper3.walk(&dst1_pos);
    assert_eq!(sniper3.pos, Point::new(0, 0));

    let dst2_pos = Point::new(1000, 1000);
    let mut sniper4 = Sniper::new(0, 0);
    assert_eq!(sniper4.pos, Point::new(0, 0));
    sniper4.walk(&dst2_pos);
    assert_eq!(sniper4.pos, Point::new(707, 707));
    sniper4.walk(&dst2_pos);
    assert_eq!(sniper4.pos, Point::new(1000, 1000));
}
