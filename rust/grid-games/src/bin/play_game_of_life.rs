use std::env;
use std::fs;

use grid_games::game_of_life::GameOfLife;

use std::{io, thread, time::Duration};

const FRAME_TIME: u64 = 100;
const FRAME_COUNT: u64 = 100;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments, you need to pass a filename");
    }

    let basefile = &args[1];
    println!("Game of Life - Example {}", basefile);

    let contents = fs::read_to_string(basefile).expect("Should have been able to read the file");

    let mut board: GameOfLife = GameOfLife::from_str(&contents).unwrap();

    for _ in 0..=FRAME_COUNT {
        let as_str = board.to_str();
        print!("{}", as_str);

        board.iterate();
        thread::sleep(Duration::from_millis(FRAME_TIME));
    }

    Ok(())
}
