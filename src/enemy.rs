use point::{Point, distance};

pub struct Enemy {
    pub id: usize,
    pub pos: Point,
    pub life: usize,
}

impl Enemy {
    pub fn new(id: usize, x: isize, y: isize, life: usize) -> Enemy {
        Enemy {
            id: id,
            pos: Point::new(x, y),
            life: life,
        }
    }
    pub fn walk(&mut self, data_pos: &Point) {
        const SPEED: isize = 500;
        if self.life == 0 {
            return;
        }
        let dir = *data_pos - self.pos;
        let distance = distance(&self.pos, data_pos);
        if distance <= (SPEED as f64) {
            self.pos = (*data_pos).clone();
        } else {
            let new_x = (dir.x as f64) * (SPEED as f64) / distance;
            let new_y = (dir.y as f64) * (SPEED as f64) / distance;
            self.pos = self.pos + Point::new(new_x.floor() as isize, new_y.floor() as isize);
        }
    }
    pub fn hit(&mut self, sniper_pos: &Point) -> bool {
        const DAMAGE_PARAM: f64 = 125000.0;
        const DAMAGE_EXP: f64 = 1.2;
        let distance = distance(sniper_pos, &self.pos);
        let damage_f64 = DAMAGE_PARAM / distance.powf(DAMAGE_EXP);
        let damage = damage_f64.round() as usize;
        if damage < self.life {
            self.life -= damage;
            false
        } else {
            self.life = 0;
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use point::Point;

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

        let mut enemy3 = Enemy::new(3, 750, 0, 10);
        assert_eq!(enemy3.pos, Point::new(750, 0));
        enemy3.walk(&data1_pos);
        assert_eq!(enemy3.pos, Point::new(250, 0));
        enemy3.walk(&data1_pos);
        assert_eq!(enemy3.pos, Point::new(0, 0));
        enemy3.walk(&data1_pos);

        let data2_pos = Point::new(500, 500);
        let mut enemy4 = Enemy::new(4, 0, 0, 10);
        assert_eq!(enemy4.pos, Point::new(0, 0));
        enemy4.walk(&data2_pos);
        assert_eq!(enemy4.pos, Point::new(353, 353));
        enemy4.walk(&data2_pos);
        assert_eq!(enemy4.pos, Point::new(500, 500));
        enemy4.walk(&data2_pos);

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
    fn test_hit() {
        let sniper_pos = Point::new(0, 0);

        let mut enemy1 = Enemy::new(1, 0, 5000, 10);
        assert_eq!(enemy1.life, 10);
        assert_eq!(enemy1.hit(&sniper_pos), false);
        assert_eq!(enemy1.life, 5);
        assert_eq!(enemy1.hit(&sniper_pos), true);
        assert_eq!(enemy1.life, 0);
    }
}
