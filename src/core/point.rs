#[derive(Debug, Clone)]
pub struct Point {
	pub x: isize,
	pub y: isize,
}

impl Point {
	pub fn new(x: isize, y:isize) -> Point {
		Point {
			x: x,
			y: y,
		}
	}
	pub fn distance(&self, p: &Point) -> f64 {
		let dirx = p.x - self.x;
		let diry = p.y - self.y;
		let distance = f64::sqrt((dirx * dirx + diry * diry) as f64);
		distance
	}
	pub fn distance_square(&self, p: &Point) -> isize {
		let dirx = p.x - self.x;
		let diry = p.y - self.y;
		let distance = dirx * dirx + diry * diry;
		distance
	}
	pub fn move_towards(&mut self, p: &Point, speed: isize) {
		let dirx = p.x - self.x;
		let diry = p.y - self.y;
		let distance = self.distance(p);
		if distance <= (speed as f64) {
			self.x = p.x;
			self.y = p.y;
		} else {
			let inc_x = (dirx as f64) * (speed as f64) / distance;
			let inc_y = (diry as f64) * (speed as f64) / distance;
			self.x = self.x + (inc_x.floor() as isize);
			self.y = self.y + (inc_y.floor() as isize);
		}
	}
}

impl PartialEq for Point {
	fn eq(&self, p: &Point) -> bool {
		if self.x == p.x && self.y == p.y {
			true
		} else {
			false
		}
	}
}
