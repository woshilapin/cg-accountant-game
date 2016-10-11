use core::point::Point;

pub struct DataPoint {
	pub id: usize,
	pub pos: Point,
}

impl DataPoint {
	pub fn new(id: usize, x: isize, y: isize) -> DataPoint {
		DataPoint {
			id: id,
			pos: Point::new(x, y),
		}
	}
}
