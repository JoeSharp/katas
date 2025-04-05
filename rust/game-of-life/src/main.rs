use std::env;
use std::fs;

mod arr2d;
mod game_of_life;

use game_of_life::GameOfLife;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments, you need to pass a filename");
    }

    let basefile = &args[1];
    println!("Game of Life - Example {}", basefile);

    let contents = fs::read_to_string(basefile).expect("Should have been able to read the file");

    let mut board: GameOfLife = GameOfLife::from_str(&contents);

    for _ in 0..=3 {
        board.print();
        board.iterate();
    }

    println!("Done");
}
