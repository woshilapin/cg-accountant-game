#![feature(test)]

extern crate test;
extern crate accountant;

use accountant::core::game::Game;
use test::{Bencher, black_box};

#[bench]
fn bench_game_loop_move(b: &mut Bencher) {
	let mut g = black_box(Game::new(16000, 9000));
	g.add_sniper(5000, 1000);
	g.add_enemy(0, 3100, 8000, 10);
	g.add_enemy(1, 14500, 8100, 10);
	g.add_data_point(0, 950, 7000);
	g.add_data_point(1, 8000, 7100);
	b.iter(|| {
		black_box(g.clone()).game_loop(black_box("MOVE"), black_box(0), black_box(0));
	});
}

#[bench]
fn bench_game_loop_shoot(b: &mut Bencher) {
	let mut g = black_box(Game::new(16000, 9000));
	g.add_sniper(5000, 1000);
	g.add_enemy(0, 3100, 8000, 10);
	g.add_enemy(1, 14500, 8100, 10);
	g.add_data_point(0, 950, 7000);
	g.add_data_point(1, 8000, 7100);
	b.iter(|| {
		black_box(g.clone()).game_loop(black_box("SHOOT"), black_box(0), black_box(0));
	});
}
