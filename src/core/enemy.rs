use core::point::Point;

#[derive(Debug, Clone)]
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
		if self.life != 0 {
			self.pos.move_towards(data_pos, SPEED);
		}
	}
	pub fn shot(&mut self, sniper_pos: &Point) -> bool {
		const DAMAGE_PARAM: f64 = 125000.0;
		const DAMAGE_EXP: f64 = 1.2;
		let distance = self.pos.distance(sniper_pos);
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
