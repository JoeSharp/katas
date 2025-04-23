use std::fmt;
use std::io::{self, Write};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Command {
    PlaceStone(usize, usize),
    Resign,
    Pass,
    Quit,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Command, Self::Err> {
        let parts: Vec<_> = s.split(" ").collect();

        if let Some(base_command) = parts.get(0) {
            match *base_command {
                "place" => {
                    if let (Some(Ok(row)), Some(Ok(col))) = (
                        parts.get(1).map(|r| r.parse::<usize>()),
                        parts.get(2).map(|r| r.parse::<usize>()),
                    ) {
                        Ok(Command::PlaceStone(row, col))
                    } else {
                        Err(())
                    }
                }
                "pass" => Ok(Command::Pass),
                "resign" => Ok(Command::Resign),
                "quit" => Ok(Command::Quit),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Command::PlaceStone(r, c) => write!(f, "place {} {}", r, c),
            Command::Resign => write!(f, "resign"),
            Command::Pass => write!(f, "pass"),
            Command::Quit => write!(f, "quit"),
        }
    }
}

fn print_go_help() {
    println!("Enter a command");
    println!("---------------");

    println!(
        "place row column => place a stone, e.g. {}",
        Command::PlaceStone(5, 6)
    );
    println!("{} => resign game, other player wins", Command::Resign);
    println!("{} => pass", Command::Pass);
    println!("{} => Quit the program", Command::Quit);
    println!("---------------");
}

fn main() -> Result<(), io::Error> {
    println!("Let's play go!");
    print_go_help();
    io::stdout().flush().expect("Failed to flush output");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if let Ok(cmd) = Command::from_str(input.trim()) {
            match cmd {
                Command::PlaceStone(r, c) => println!("Placing Stone {} {}", r, c),
                Command::Resign => println!("Resigning"),
                Command::Pass => println!("Passing"),
                Command::Quit => {
                    println!("Quit Game");
                    break;
                }
            }
        } else {
            println!("Failed to understand command");
        }

        print_go_help();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("place 5 6", Command::PlaceStone(5, 6))]
    #[test_case("pass", Command::Pass)]
    #[test_case("resign", Command::Resign)]
    fn test_parse_move(input: &str, expected: Command) {
        let result = Command::from_str(input).expect("Failed to parse input");
        let as_str = format!("{}", result);
        assert_eq!(result, expected);
        assert_eq!(as_str, input);
    }
}
