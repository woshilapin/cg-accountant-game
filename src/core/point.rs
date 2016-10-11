extern crate euclid;

pub type Point = euclid::point::Point2D<isize>;

pub fn distance(p1: &Point, p2: &Point) -> f64 {
	let dir = *p2 - *p1;
	let distance = f64::sqrt((dir.x * dir.x + dir.y * dir.y) as f64);
	distance
}

pub fn move_towards(src_pos: &Point, dst_pos: &Point, speed: isize) -> Point {
	let dir = *dst_pos - *src_pos;
	let distance = distance(src_pos, dst_pos);
	if distance <= (speed as f64) {
		(*dst_pos).clone()
	} else {
		let inc_x = (dir.x as f64) * (speed as f64) / distance;
		let inc_y = (dir.y as f64) * (speed as f64) / distance;
		let new_x = src_pos.x + (inc_x.floor() as isize);
		let new_y = src_pos.y + (inc_y.floor() as isize);
		Point::new(new_x, new_y)
	}
}
