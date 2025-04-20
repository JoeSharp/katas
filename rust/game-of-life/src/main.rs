use std::env;
use std::error::Error;
use std::fs;
use std::process;

use game_of_life::GameOfLife;

use std::{io, thread, time::Duration};

const FRAME_TIME: u64 = 100;
const FRAME_COUNT: u64 = 10;

struct Config {
    basefile: String,
}

impl Config {
    fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let _bin_path = args.next().unwrap();

        let basefile = match args.next() {
            Some(x) => x,
            None => return Err("Did not receive a basefile"),
        };

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
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }

    Ok(())
}
