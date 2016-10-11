use core::point::{Point, move_towards};

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
		self.pos = move_towards(&self.pos, dst_pos, SPEED);
	}
}
