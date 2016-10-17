#![feature(test)]

extern crate test;
extern crate accountant;

use accountant::core::point::Point;
use test::{Bencher, black_box};

#[bench]
fn bench_distance(b: &mut Bencher) {
	let point1 = Point::new(0, 0);
	b.iter(|| {
		point1.distance(&black_box(Point::new(100, 100)))
	});
}
