use point::{Point, distance};
use sniper::Sniper;
use enemy::Enemy;
use data_point::DataPoint;

pub struct Game {
    pub size: Point,
    pub sniper: Sniper,
    pub enemies: Vec<Enemy>,
    pub data_points: Vec<DataPoint>,
    pub shots_fired: usize,
    pub total_life: usize,
}

impl Game {
    pub fn new(w: isize, h: isize) -> Game {
        Game {
            size: Point::new(w, h),
            sniper: Sniper::new(0, 0),
            enemies: Vec::new(),
            data_points: Vec::new(),
            shots_fired: 0,
            total_life: 0,
        }
    }
    pub fn add_sniper(&mut self, x: isize, y: isize) -> &mut Game {
        self.sniper = Sniper::new(x, y);
        self
    }
    pub fn add_enemy(&mut self, id: usize, x: isize, y: isize, life: usize) -> &mut Game {
        self.enemies.push(Enemy::new(id, x, y, life));
        self.total_life = self.total_life + life;
        self
    }
    pub fn add_data_point(&mut self, id: usize, x: isize, y: isize) -> &mut Game {
        self.data_points.push(DataPoint::new(id, x, y));
        self
    }
    pub fn enemies_walk(&mut self) {
        if self.data_points.is_empty() {
            return;
        }
        for enemy in &mut self.enemies {
            if enemy.life == 0 {
                continue;
            }
            let mut closest_index = self.data_points.len();
            let mut closest_distance = distance(&Point::new(0, 0), &self.size);
            for (index, data_point) in self.data_points.iter().enumerate() {
                if data_point.is_taken {
                    continue;
                }
                let distance = distance(&enemy.pos, &data_point.pos);
                if distance < closest_distance {
                    closest_index = index;
                    closest_distance = distance;
                } else if distance == closest_distance && data_point.id < self.data_points[closest_index].id {
                    closest_index = index;
                }
            }
            if closest_index != self.data_points.len() {
                enemy.walk(&self.data_points[closest_index].pos);
            }
        }
    }
    pub fn sniper_walk(&mut self, x: isize, y: isize) {
        self.sniper.walk(&Point::new(x, y));
    }
    pub fn sniper_is_dead(&self) -> bool {
        for enemy in &self.enemies {
            if self.sniper.is_dead(&enemy.pos) && enemy.life > 0 {
                return true;
            }
        }
        return false;
    }
    pub fn sniper_fire(&mut self, id: usize) {
        for enemy in &mut self.enemies {
            if id == enemy.id {
                enemy.hit(&self.sniper.pos);
                return;
            }
        }
    }
    pub fn data_points_collect(&mut self) -> bool {
        let mut all_data_points_taken = true;
        for data_point in &mut self.data_points {
            let mut data_point_taken = data_point.is_taken;
            if data_point_taken {
                continue;
            }
            for enemy in &self.enemies {
                if enemy.pos == data_point.pos {
                    data_point.take();
                    data_point_taken = true;
                }
            }
            if ! data_point_taken {
                all_data_points_taken = false;
            }
        }
        return all_data_points_taken;
    }
    pub fn game_loop(&mut self, cmd: &str, param1: isize, param2: isize) -> bool {
        // Step 1: Enemies walk
        self.enemies_walk();
        // Step 2: Sniper walkes
        if cmd == "MOVE" {
            self.sniper_walk(param1, param2);
        }
        // Step 3: Check sniper is not dead
        let sniper_is_dead = self.sniper_is_dead();
        // Step 4: Sniper fires
        if cmd == "SHOOT" {
            self.sniper_fire(param1 as usize);
            self.shots_fired = self.shots_fired + 1;
        }
        // Step 5: Collect data points
        let all_data_points_taken = self.data_points_collect();
        // Return true if game is finished
        let mut all_enemies_dead = true;
        for enemy in &self.enemies {
            if enemy.life != 0 {
                all_enemies_dead = false;
                break;
            }
        }
        if sniper_is_dead || all_data_points_taken || all_enemies_dead {
            true
        } else {
            false
        }
    }
    pub fn score(&self) -> usize {
        let mut dp = 0;
        for data_point in &self.data_points {
            if ! data_point.is_taken {
                dp = dp + 1;
            }
        }
        let l = self.total_life;
        let s = self.shots_fired;
        let mut k = 0;
        let subtotal = if l > 3*s { l - 3*s } else { 0 };
        for enemy in &self.enemies {
            if enemy.life == 0 {
                k = k + 1;
            }
        }
        let score = dp * ( 3 * subtotal + 100 ) + 10 * k;
        score
    }
    pub fn status(&self) {
        println!("{} {}", self.sniper.pos.x, self.sniper.pos.y);
        println!("{}", self.data_points.len());
        for data_point in &self.data_points {
            println!("{} {} {}", data_point.id, data_point.pos.x, data_point.pos.y);
        }
        let mut enemies_alive = 0;
        for enemy in &self.enemies {
            if enemy.life != 0 {
                enemies_alive = enemies_alive + 1;
            }
        }
        println!("{}", enemies_alive);
        for enemy in &self.enemies {
            if enemy.life != 0 {
                println!("{} {} {} {}", enemy.id, enemy.pos.x, enemy.pos.y, enemy.life);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use point::Point;

    #[test]
    fn test_new() {
        let mut game = Game::new(16000, 9000);

        assert_eq!(game.size, Point::new(16000, 9000));
        assert_eq!(game.enemies.len(), 0);
        assert_eq!(game.data_points.len(), 0);

        game.add_sniper(100, 200);
        assert_eq!(game.sniper.pos, Point::new(100, 200));

        game.add_enemy(42, 300, 400, 32);
        assert_eq!(game.enemies.len(), 1);
        assert_eq!(game.enemies[0].id, 42);
        assert_eq!(game.enemies[0].pos, Point::new(300, 400));
        assert_eq!(game.enemies[0].life, 32);

        game.add_data_point(84, 500, 600);
        assert_eq!(game.data_points.len(), 1);
        assert_eq!(game.data_points[0].id, 84);
        assert_eq!(game.data_points[0].pos, Point::new(500, 600));
    }

    #[test]
    fn test_enemies_walk() {
        let mut game = Game::new(16000, 9000);
        game.add_enemy(0, 400, 0, 10);
        game.add_enemy(1, 1100, 0, 10);
        game.add_data_point(0, 0, 0);
        game.add_data_point(1, 2000, 0);

        assert_eq!(game.enemies[0].pos, Point::new(400, 0));
        assert_eq!(game.enemies[1].pos, Point::new(1100, 0));

        game.enemies_walk();
        assert_eq!(game.enemies[0].pos, Point::new(0, 0));
        assert_eq!(game.enemies[1].pos, Point::new(1600, 0));

        game.data_points[0].is_taken = true;
        game.enemies_walk();
        assert_eq!(game.enemies[0].pos, Point::new(500, 0));
        assert_eq!(game.enemies[1].pos, Point::new(2000, 0));
    }

    #[test]
    fn test_sniper_move() {
        let mut game = Game::new(16000, 9000);

        game.add_sniper(1500, 0);
        assert_eq!(game.sniper.pos, Point::new(1500, 0));

        game.sniper_walk(0, 0);
        assert_eq!(game.sniper.pos, Point::new(500, 0));

        game.sniper_walk(0, 0);
        assert_eq!(game.sniper.pos, Point::new(0, 0));
    }

    #[test]
    fn test_sniper_is_dead() {
        let mut game = Game::new(16000, 9000);

        game.add_sniper(0, 0);
        assert_eq!(game.sniper_is_dead(), false);

        game.add_enemy(0, 2001, 0, 10);
        assert_eq!(game.sniper_is_dead(), false);

        game.add_enemy(1, 2000, 0, 0);
        assert_eq!(game.sniper_is_dead(), false);

        game.add_enemy(2, 2000, 0, 10);
        assert_eq!(game.sniper_is_dead(), true);
    }

    #[test]
    fn test_sniper_fire() {
        let mut game = Game::new(16000, 9000);
        game.add_sniper(0, 0);
        game.add_enemy(0, 2000, 0, 1);
        game.add_enemy(1, 0, 2000, 100);

        assert_eq!(game.enemies[0].life, 1);
        assert_eq!(game.enemies[1].life, 100);

        game.sniper_fire(0);
        assert_eq!(game.enemies[0].life, 0);
        assert_eq!(game.enemies[1].life, 100);

        game.sniper_fire(1);
        assert_eq!(game.enemies[0].life, 0);
        assert!(game.enemies[1].life < 100);
    }

    #[test]
    fn test_data_points_collect() {
        let mut game = Game::new(16000, 9000);
        assert_eq!(game.data_points_collect(), true);

        game.add_data_point(0, 0, 0);
        assert_eq!(game.data_points_collect(), false);

        game.add_data_point(1, 100, 0);
        assert_eq!(game.data_points_collect(), false);

        game.add_enemy(0, 0, 0, 10);
        assert_eq!(game.data_points_collect(), false);

        game.add_enemy(1, 100, 0, 10);
        assert_eq!(game.data_points_collect(), true);
    }

    #[test]
    fn test_game_loop() {
        let mut game = Game::new(16000, 9000);
        game.add_sniper(1100, 1200);
        game.add_enemy(0, 8250, 8999, 10);
        game.add_data_point(0, 8250, 4500);

        assert_eq!(game.sniper.pos, Point::new(1100, 1200));
        assert_eq!(game.enemies[0].pos, Point::new(8250, 8999));
        assert_eq!(game.enemies[0].life, 10);
        assert_eq!(game.data_points[0].is_taken, false);
        assert_eq!(game.game_loop("SHOOT", 0, 0), false);

        assert_eq!(game.sniper.pos, Point::new(1100, 1200));
        assert_eq!(game.enemies[0].pos, Point::new(8250, 8499));
        assert_eq!(game.enemies[0].life, 8);
        assert_eq!(game.data_points[0].is_taken, false);
        assert_eq!(game.game_loop("SHOOT", 0, 0), false);

        assert_eq!(game.sniper.pos, Point::new(1100, 1200));
        assert_eq!(game.enemies[0].pos, Point::new(8250, 7999));
        assert_eq!(game.enemies[0].life, 6);
        assert_eq!(game.data_points[0].is_taken, false);
        assert_eq!(game.game_loop("SHOOT", 0, 0), false);

        assert_eq!(game.sniper.pos, Point::new(1100, 1200));
        assert_eq!(game.enemies[0].pos, Point::new(8250, 7499));
        assert_eq!(game.enemies[0].life, 4);
        assert_eq!(game.data_points[0].is_taken, false);
        assert_eq!(game.game_loop("SHOOT", 0, 0), false);

        assert_eq!(game.sniper.pos, Point::new(1100, 1200));
        assert_eq!(game.enemies[0].pos, Point::new(8250, 6999));
        assert_eq!(game.enemies[0].life, 2);
        assert_eq!(game.data_points[0].is_taken, false);
        assert_eq!(game.game_loop("SHOOT", 0, 0), true);

        assert_eq!(game.sniper.pos, Point::new(1100, 1200));
        assert_eq!(game.enemies[0].life, 0);
        assert_eq!(game.data_points[0].is_taken, false);
        assert_eq!(game.shots_fired, 5);
        assert_eq!(game.game_loop("MOVE", 0, 1200), true);
        assert_eq!(game.sniper.pos, Point::new(100, 1200));
    }
}
