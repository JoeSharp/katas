use std::env;
use std::error::Error;
use std::fs;
use std::process;

use grid_games::game_of_life::GameOfLife;

use std::{io, thread, time::Duration};

const FRAME_TIME: u64 = 100;
const FRAME_COUNT: u64 = 10;

struct Config<'a> {
    basefile: &'a str,
}

impl<'a> Config<'a> {
    fn new(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments, you need to pass a filename");
        }

        let basefile = &args[1];
        Ok(Config { basefile })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Game of Life - Example {}", config.basefile);

    let contents =
        fs::read_to_string(config.basefile).expect("Should have been able to read the file");

    let mut board: GameOfLife = GameOfLife::from_str(&contents).unwrap();

    for _ in 0..=FRAME_COUNT {
        let as_str = board.to_str();
        print!("{}", as_str);

        board.iterate();
        thread::sleep(Duration::from_millis(FRAME_TIME));
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }

    Ok(())
}
