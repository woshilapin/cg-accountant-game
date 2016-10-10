extern crate accountant;

use accountant::core::point::Point;
use accountant::core::game::Game;
use accountant::core::game::Status;

#[test]
fn test_new() {
    let mut game = Game::new(16000, 9000);

    assert_eq!(game.size, Point::new(16000, 9000));
    assert_eq!(game.enemies.len(), 0);
    assert_eq!(game.data_points.len(), 0);

    game.add_sniper(100, 200);
    assert_eq!(game.sniper.pos, Point::new(100, 200));

    game.add_enemy(42, 300, 400, 32);
    assert_eq!(game.enemies.len(), 1);
    assert_eq!(game.enemies[&42].id, 42);
    assert_eq!(game.enemies[&42].pos, Point::new(300, 400));
    assert_eq!(game.enemies[&42].life, 32);

    game.add_data_point(84, 500, 600);
    assert_eq!(game.data_points.len(), 1);
    assert_eq!(game.data_points[&84].id, 84);
    assert_eq!(game.data_points[&84].pos, Point::new(500, 600));
}

#[test]
fn test_enemies_walk() {
    let mut game = Game::new(16000, 9000);
    game.add_enemy(0, 400, 0, 10);
    game.add_enemy(1, 1100, 0, 10);
    game.add_data_point(0, 0, 0);
    game.add_data_point(1, 2000, 0);

    assert_eq!(game.enemies[&0].pos, Point::new(400, 0));
    assert_eq!(game.enemies[&1].pos, Point::new(1100, 0));

    game.enemies_walk();
    assert_eq!(game.enemies[&0].pos, Point::new(0, 0));
    assert_eq!(game.enemies[&1].pos, Point::new(1600, 0));

    game.data_points.remove(&0);
    game.enemies_walk();
    assert_eq!(game.enemies[&0].pos, Point::new(500, 0));
    assert_eq!(game.enemies[&1].pos, Point::new(2000, 0));
}

#[test]
fn test_sniper_move() {
    let mut game = Game::new(16000, 9000);

    game.add_sniper(1500, 0);
    assert_eq!(game.sniper.pos, Point::new(1500, 0));

    game.sniper_walk(0, 0);
    assert_eq!(game.sniper.pos, Point::new(500, 0));

    game.sniper_walk(0, 0);
    assert_eq!(game.sniper.pos, Point::new(0, 0));
}

#[test]
fn test_sniper_is_dead() {
    let mut game = Game::new(16000, 9000);

    game.add_sniper(0, 0);
    assert_eq!(game.sniper_is_dead(), false);

    game.add_enemy(0, 2001, 0, 10);
    assert_eq!(game.sniper_is_dead(), false);

    game.add_enemy(1, 2000, 0, 10);
    assert_eq!(game.sniper_is_dead(), true);
}

#[test]
fn test_sniper_fire() {
    let mut game = Game::new(16000, 9000);
    game.add_sniper(0, 0);
    game.add_enemy(0, 2000, 0, 1);
    game.add_enemy(1, 0, 2000, 100);

    assert_eq!(game.enemies[&0].life, 1);
    assert_eq!(game.enemies[&1].life, 100);

    game.sniper_fire(0);
    assert_eq!(game.enemies[&0].life, 0);
    assert_eq!(game.enemies[&1].life, 100);

    game.sniper_fire(1);
    assert_eq!(game.enemies[&0].life, 0);
    assert_eq!(game.enemies[&1].life, 86);
}

#[test]
fn test_data_points_collect() {
    let mut game = Game::new(16000, 9000);
    assert_eq!(game.data_points_collect(), true);

    game.add_data_point(0, 0, 0);
    assert_eq!(game.data_points_collect(), false);

    game.add_data_point(1, 100, 0);
    assert_eq!(game.data_points_collect(), false);

    game.add_enemy(0, 0, 0, 10);
    assert_eq!(game.data_points_collect(), false);

    game.add_enemy(1, 100, 0, 10);
    assert_eq!(game.data_points_collect(), true);
}

#[test]
fn test_game_loop() {
    let mut game = Game::new(16000, 9000);
    game.add_sniper(1100, 1200);
    game.add_enemy(0, 8250, 8999, 10);
    game.add_data_point(0, 8250, 4500);

    assert_eq!(game.sniper.pos, Point::new(1100, 1200));
    assert_eq!(game.enemies[&0].pos, Point::new(8250, 8999));
    assert_eq!(game.enemies[&0].life, 10);
    assert_eq!(game.data_points.len(), 1);
    assert_eq!(game.status, Status::Running);

    game.game_loop("SHOOT", 0, 0);
    assert_eq!(game.sniper.pos, Point::new(1100, 1200));
    assert_eq!(game.enemies[&0].pos, Point::new(8250, 8499));
    assert_eq!(game.enemies[&0].life, 8);
    assert_eq!(game.data_points.len(), 1);
    assert_eq!(game.status, Status::Running);

    game.game_loop("SHOOT", 0, 0);
    assert_eq!(game.sniper.pos, Point::new(1100, 1200));
    assert_eq!(game.enemies[&0].pos, Point::new(8250, 7999));
    assert_eq!(game.enemies[&0].life, 6);
    assert_eq!(game.data_points.len(), 1);
    assert_eq!(game.status, Status::Running);

    game.game_loop("SHOOT", 0, 0);
    assert_eq!(game.sniper.pos, Point::new(1100, 1200));
    assert_eq!(game.enemies[&0].pos, Point::new(8250, 7499));
    assert_eq!(game.enemies[&0].life, 4);
    assert_eq!(game.data_points.len(), 1);
    assert_eq!(game.status, Status::Running);

    game.game_loop("SHOOT", 0, 0);
    assert_eq!(game.sniper.pos, Point::new(1100, 1200));
    assert_eq!(game.enemies[&0].pos, Point::new(8250, 6999));
    assert_eq!(game.enemies[&0].life, 2);
    assert_eq!(game.data_points.len(), 1);
    assert_eq!(game.status, Status::Running);

    game.game_loop("SHOOT", 0, 0);
    assert_eq!(game.sniper.pos, Point::new(1100, 1200));
    assert_eq!(game.enemies.len(), 0);
    assert_eq!(game.data_points.len(), 1);
    assert_eq!(game.shots_fired, 5);
    assert_eq!(game.status, Status::Ended);
}
