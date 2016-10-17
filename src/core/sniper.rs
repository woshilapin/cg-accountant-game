use core::point::Point;

#[derive(Debug, Clone)]
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
		self.pos.move_towards(dst_pos, SPEED);
	}
}
