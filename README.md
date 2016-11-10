[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg?style=flat)](https://spdx.org/licenses/GPL-3.0.html)
[![Build Status](https://img.shields.io/travis/woshilapin/cg-accountant-game.svg)](https://travis-ci.org/woshilapin/cg-accountant-game)
[![Code Coverage](https://img.shields.io/codecov/c/github/woshilapin/cg-accountant-game.svg)](https://codecov.io/gh/woshilapin/cg-accountant-game)

# Introduction
This is a program that simulate the behaviour of _The Accountant_ game from
Codingame website.

# Compile
The project is instrumented with Cargo.  To build it, you need
[Cargo](http://doc.crates.io/index.html) and
[Rust](https://www.rust-lang.org/index.html) installed.  Then you can launch the
following command.

```
cargo build
```

# Run
To run the program, launch the following command.

```
cargo run
```

Note that it will a specific configuration.  You may change it in
[src/main.rs](https://github.com/woshilapin/cg-accountant-game/blob/master/src/main.rs)
near the beginning of the `main` function.

Once launched, the game is expecting you to type one of the 2 following
commands:
* `SHOOT <enemy_id>`
* `MOVE <x> <y>`

# Library
This project has implemented all of the logic of the game as a library (see
[src/lib.rs](https://github.com/woshilapin/cg-accountant-game/blob/master/src/lib.rs)).

Most of the code has been tested.  You may run the test with the following
command.

```
cargo test
```

Benchmarks are also available through the following command.

```
cargo bench
```
