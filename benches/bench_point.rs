#![feature(test)]

extern crate test;
extern crate accountant;

use accountant::core::point::Point;
use test::{Bencher, black_box};

#[bench]
fn bench_distance(b: &mut Bencher) {
	let p = Point::new(0, 0);
	b.iter(|| {
		p.distance(&black_box(Point::new(100, 100)))
	});
}

#[bench]
fn bench_distance_square(b: &mut Bencher) {
	let p = Point::new(0, 0);
	b.iter(|| {
		p.distance_square(&black_box(Point::new(100, 100)))
	});
}

#[bench]
fn bench_move_towards(b: &mut Bencher) {
	b.iter(|| {
		let mut p1 = black_box(Point::new(0, 0));
		let p2 = black_box(Point::new(100, 100));
		p1.move_towards(&p2, black_box(500))
	});
}
