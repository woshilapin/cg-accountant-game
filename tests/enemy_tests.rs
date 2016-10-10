extern crate accountant;

use accountant::core::point::Point;
use accountant::core::enemy::Enemy;

#[test]
fn test_walk() {
    let data1_pos = Point::new(0, 0);

    let enemy1 = Enemy::new(1, 0, 0, 10);
    assert_eq!(enemy1.pos, Point::new(0, 0));

    let mut enemy2 = Enemy::new(2, 500, 0, 10);
    assert_eq!(enemy2.pos, Point::new(500, 0));
    enemy2.walk(&data1_pos);
    assert_eq!(enemy2.pos, Point::new(0, 0));
    enemy2.walk(&data1_pos);
    assert_eq!(enemy2.pos, Point::new(0, 0));

    let mut enemy3 = Enemy::new(3, 750, 0, 10);
    assert_eq!(enemy3.pos, Point::new(750, 0));
    enemy3.walk(&data1_pos);
    assert_eq!(enemy3.pos, Point::new(250, 0));
    enemy3.walk(&data1_pos);
    assert_eq!(enemy3.pos, Point::new(0, 0));

    let data2_pos = Point::new(500, 500);
    let mut enemy4 = Enemy::new(4, 0, 0, 10);
    assert_eq!(enemy4.pos, Point::new(0, 0));
    enemy4.walk(&data2_pos);
    assert_eq!(enemy4.pos, Point::new(353, 353));
    enemy4.walk(&data2_pos);
    assert_eq!(enemy4.pos, Point::new(500, 500));

    let data3_pos = Point::new(950, 7000);
    let mut enemy5 = Enemy::new(5, 3100, 8000, 10);
    assert_eq!(enemy5.pos, Point::new(3100, 8000));
    enemy5.walk(&data3_pos);
    assert_eq!(enemy5.pos, Point::new(2646, 7789));
    enemy5.walk(&data3_pos);
    assert_eq!(enemy5.pos, Point::new(2192, 7578));
    enemy5.walk(&data3_pos);
    assert_eq!(enemy5.pos, Point::new(1738, 7367));
    enemy5.life = 0;
    enemy5.walk(&data3_pos);
    assert_eq!(enemy5.pos, Point::new(1738, 7367));
}

#[test]
fn test_shot() {
    let sniper_pos = Point::new(0, 0);

    let mut enemy1 = Enemy::new(1, 0, 5000, 10);
    assert_eq!(enemy1.life, 10);
    assert_eq!(enemy1.shot(&sniper_pos), false);
    assert_eq!(enemy1.life, 5);
    assert_eq!(enemy1.shot(&sniper_pos), true);
    assert_eq!(enemy1.life, 0);
}

