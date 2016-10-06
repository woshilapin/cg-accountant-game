use point::{Point, distance};

pub struct Sniper {
    pub pos: Point,
}

impl Sniper {
    pub fn new(x: isize, y: isize) -> Sniper {
        Sniper {
            pos: Point::new(x, y),
        }
    }
    pub fn walk(&mut self, dst_pos: &Point) {
        const SPEED: isize = 1000;
        let dir = *dst_pos - self.pos;
        let distance = distance(&self.pos, dst_pos);
        if distance <= (SPEED as f64) {
            self.pos = (*dst_pos).clone();
        } else {
            let new_pos_f64 = dir.cast::<f64>().unwrap() * (SPEED as f64) / distance;
            self.pos = self.pos + new_pos_f64.cast::<isize>().unwrap();
        }
    }
    pub fn is_dead(&self, enemy_pos: &Point) -> bool {
        const SECURE_DISTANCE: f64 = 2000.0;
        if distance(&self.pos, enemy_pos) <= SECURE_DISTANCE {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use point::Point;

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

    #[test]
    fn test_is_dead() {
        let sniper = Sniper::new(0, 0);

        let enemy1_pos = Point::new(2000, 0);
        assert_eq!(sniper.is_dead(&enemy1_pos), true);

        let enemy2_pos = Point::new(2001, 0);
        assert_eq!(sniper.is_dead(&enemy2_pos), false);
    }
}
