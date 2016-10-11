use std::collections::HashMap;

use core::point::{Point, distance};
use core::sniper::Sniper;
use core::data_point::DataPoint;
use core::enemy::Enemy;

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
	Running,
	Ended,
	GameOver,
}

#[derive(Debug, Clone)]
pub struct Game {
	pub size: Point,
	pub sniper: Sniper,
	pub enemies: HashMap<usize, Enemy>,
	pub enemies_dead: usize,
	pub data_points: HashMap<usize, DataPoint>,
	pub shots_fired: usize,
	pub total_life: usize,
	pub score: usize,
	pub status: Status,
}

impl Game {
	pub fn new(w: isize, h: isize) -> Game {
		Game {
			size: Point::new(w, h),
			sniper: Sniper::new(0, 0),
			enemies: HashMap::new(),
			enemies_dead: 0,
			data_points: HashMap::new(),
			shots_fired: 0,
			total_life: 0,
			score: 0,
			status: Status::Running,
		}
	}
	pub fn add_sniper(&mut self, x: isize, y: isize) -> &mut Game {
		self.sniper = Sniper::new(x, y);
		self
	}
	pub fn add_data_point(&mut self, id: usize, x: isize, y: isize) -> &mut Game {
		self.data_points.insert(id, DataPoint::new(id, x, y));
		self
	}
	pub fn add_enemy(&mut self, id: usize, x: isize, y: isize, life: usize) -> &mut Game {
		self.enemies.insert(id, Enemy::new(id, x, y, life));
		self.total_life = self.total_life + life;
		self
	}
	pub fn enemies_walk(&mut self) {
		for (_, enemy) in &mut self.enemies {
			let mut closest_index: usize = match self.data_points.values().next() {
				Some(dp) => { dp.id }
				None => { panic!("enemies_walk: There is no data point") }
			};
			let mut closest_distance: f64 = distance(&Point::new(0, 0), &self.size);
			for (_, data_point) in &self.data_points {
				let distance = distance(&enemy.pos, &data_point.pos);
				if distance < closest_distance {
					closest_index = data_point.id;
					closest_distance = distance;
				} else if distance == closest_distance && data_point.id < closest_index {
					closest_index = data_point.id;
				}
			}
			match self.data_points.get(&closest_index) {
				Some(dp) => { enemy.walk(&dp.pos) }
				None => { panic!("enemies_walk: There is no data point with key '{}'", closest_index) }
			};
		}
	}
	pub fn sniper_walk(&mut self, x: isize, y: isize) {
		self.sniper.walk(&Point::new(x, y));
	}
	pub fn sniper_is_dead(&self) -> bool {
		for (_, enemy) in &self.enemies {
			if distance(&self.sniper.pos, &enemy.pos) <= 2000.0 {
				return true;
			}
		}
		false
	}
	pub fn sniper_fire(&mut self, target_id: usize) {
		for (id, enemy) in &mut self.enemies {
			if *id == target_id {
				enemy.shot(&self.sniper.pos);
				return;
			}
		}
	}
	pub fn enemies_remove(&mut self) -> bool {
		let ids_to_remove: Vec<usize> = self.enemies
			.iter()
			.filter(|&(_, ref e)| e.life == 0)
			.map(|(k, _)| k.clone())
			.collect();
		for id_to_remove in ids_to_remove {
			self.enemies.remove(&id_to_remove);
			self.enemies_dead += 1;
		}
		self.enemies.is_empty()
	}
	pub fn data_points_collect(&mut self) -> bool {
		for (_, enemy) in &self.enemies {
			let ids_to_remove: Vec<usize> = self.data_points
				.iter()
				.filter(|&(_, ref dp)| dp.pos == enemy.pos)
				.map(|(k, _)| k.clone())
				.collect();
			for id_to_remove in ids_to_remove {
				self.data_points.remove(&id_to_remove);
			}
		}
		self.data_points.is_empty()
	}
	pub fn game_loop(&mut self, cmd: &str, param1: isize, param2: isize) -> &mut Game {
		// Step 1: Enemies walk
		self.enemies_walk();
		// Step 2: Sniper walkes
		if cmd == "MOVE" {
			self.sniper_walk(param1, param2);
		}
		// Step 3: Check sniper is not dead
		let sniper_is_dead = self.sniper_is_dead();
		if sniper_is_dead {
			self.score = 0;
			self.status = Status::GameOver;
		}
		// Step 4: Sniper fires
		if cmd == "SHOOT" {
			self.sniper_fire(param1 as usize);
			self.shots_fired = self.shots_fired + 1;
		}
		// Step 5
		let all_enemies_dead = self.enemies_remove();
		if all_enemies_dead {
			self.score = self.score();
			self.status = Status::Ended;
		}
		// Step 6: Collect data points
		let all_data_points_taken = self.data_points_collect();
		if all_data_points_taken {
			self.score = 0;
			self.status = Status::GameOver;
		}
		self
	}
	pub fn score(&self) -> usize {
		let dp = self.data_points.len();
		let l = self.total_life;
		let s = self.shots_fired;
		let k = self.enemies_dead;
		let subtotal = if l > 3*s { l - 3*s } else { 0 };
		let score = dp * ( 3 * subtotal + 100 ) + 10 * k;
		score
	}
	pub fn print_status(&self) {
		println!("{} {}", self.sniper.pos.x, self.sniper.pos.y);
		println!("{}", self.data_points.len());
		for (_, data_point) in &self.data_points {
			println!("{} {} {}", data_point.id, data_point.pos.x, data_point.pos.y);
		}
		println!("{}", self.enemies.len());
		for (_, enemy) in &self.enemies {
			if enemy.life != 0 {
				println!("{} {} {} {}", enemy.id, enemy.pos.x, enemy.pos.y, enemy.life);
			}
		}
	}
}
