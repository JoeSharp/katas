use std::env;
use std::fs;

mod arr2d;
use arr2d::Arr2d;
use arr2d::AsChar;

struct GameOfLife {
    index: usize,
    contents: [Arr2d<bool>; 2],
}

impl AsChar for bool {
    fn from_char(c: &char) -> Self {
        GameOfLife::ALIVE == *c
    }

    fn to_char(&self) -> char {
        if *self {
            GameOfLife::ALIVE
        } else {
            GameOfLife::DEAD
        }
    }
}

impl GameOfLife {
    pub const ALIVE: char = 'x';
    pub const DEAD: char = '-';

    pub fn next_state(state: (bool, u8)) -> bool {
        match state {
            (true, 0..=1) => false, // Underpopulation
            (true, 2 | 3) => true,  // Lives on
            (true, _) => false,     // Overpopulation
            (false, 3) => true,     // Reproduction
            (false, _) => false,
        }
    }

    fn count_neighbours(arr2d: &Arr2d<bool>, r: usize, c: usize) -> u8 {
        let mut n = 0;

        let top = r > 0;
        let left = c > 0;
        let bottom = r < arr2d.rows() - 1;
        let right = c < arr2d.columns(r) - 1;

        if top && left && *arr2d.get(r - 1, c - 1) {
            n += 1;
        }
        if top && *arr2d.get(r - 1, c) {
            n += 1;
        }
        if top && right && *arr2d.get(r - 1, c + 1) {
            n += 1;
        }
        if left && *arr2d.get(r, c - 1) {
            n += 1;
        }
        if right && *arr2d.get(r, c + 1) {
            n += 1;
        }
        if bottom && left && *arr2d.get(r + 1, c - 1) {
            n += 1;
        }
        if bottom && *arr2d.get(r + 1, c) {
            n += 1;
        }
        if bottom && right && *arr2d.get(r + 1, c + 1) {
            n += 1;
        }

        n
    }

    fn from_str(as_str: &str) -> GameOfLife {
        let contents: [Arr2d<bool>; 2] = [Arr2d::from_str(as_str), Arr2d::from_str(as_str)];
        GameOfLife { index: 0, contents }
    }

    fn iterate(&mut self) {
        let next_index = if self.index == 0 { 1 } else { 0 };
        for r in 0..self.contents[self.index].rows() {
            for c in 0..self.contents[self.index].columns(r) {
                let n = Self::count_neighbours(&self.contents[self.index], r, c);
                self.contents[next_index].set(
                    r,
                    c,
                    Self::next_state((*self.contents[self.index].get(r, c), n)),
                );
            }
        }
        self.index = next_index;
    }

    fn print(&self) {
        println!("Game of Life");
        self.contents[self.index].print();
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_state() {
        assert!(GameOfLife::next_state((true, 3)));
    }
}
