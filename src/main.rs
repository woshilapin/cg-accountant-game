mod sniper;
mod enemy;
mod point;
mod game;
mod data_point;

use std::io;
use game::Game;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*).ok();
        }
    )
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn game_status(game: &Game) {
    print_err!("sniper: {}x{}", game.sniper.pos.x, game.sniper.pos.y);
    for enemy in &game.enemies {
        print_err!("enemy #{}: {}x{} [{}]", enemy.id, enemy.pos.x, enemy.pos.y, enemy.life);
    }
    for data_point in &game.data_points {
        print_err!("data_point #{}: {}x{} [{}]", data_point.id, data_point.pos.x, data_point.pos.y, data_point.is_taken);
    }
    print_err!("shots_fired: {}", game.shots_fired);
    print_err!("score: {}", game.score());
}

fn main() {
    let mut game = Game::new(16000, 9000);
    // PLAY 01
    // game.add_sniper(1100, 1200);
    // game.add_enemy(0, 8250, 8999, 10);
    // game.add_data_point(0, 8250, 4500);
    // PLAY 02
    game.add_sniper(5000, 1000);
    game.add_enemy(0, 3100, 8000, 10);
    game.add_enemy(1, 14500, 8100, 10);
    game.add_data_point(0, 950, 7000);
    game.add_data_point(1, 8000, 7100);

    let mut is_end = false;
    while ! is_end {
        game.status();
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let cmd =  inputs[0];
        let param1 = parse_input!(inputs[1], isize);
        if cmd == "SHOOT" {
            is_end = game.game_loop(&cmd, param1, 0);
        } else {
            let param2 = parse_input!(inputs[2], isize);
            is_end = game.game_loop(&cmd, param1, param2);
        }
        game_status(&game);
    }
    print_err!("Game finished");
    print_err!("Final score: {}", game.score());
    // [14, 2020]
    // [13, 2154]
    // [12, 2309]
    // [11, 2491]
    // [10, 2708]
    // [9, 2971]
    // [8, 3297]
    // [7, 3715]
    // [6, 4270]
    // [5, 5047]
    // [4, 6223]
    // [3, 8237]
    // [2, 12609]
    // [1, 31498]
}
