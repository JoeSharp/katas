use std::env;
use std::fs;

mod arr2d;
use arr2d::Arr2d;

struct Board {
    index: usize,
    contents: [Arr2d<bool>; 2],
}

mod game_of_life {
    const ALIVE: char = 'x';
    const DEAD: char = '-';

    pub fn next_state(state: (bool, u8)) -> bool {
        match state {
            (true, 0..=1) => false, // Underpopulation
            (true, 2 | 3) => true,  // Lives on
            (true, _) => false,     // Overpopulation
            (false, 3) => true,     // Reproduction
            (false, _) => false,
        }
    }

    pub fn cell_from_char(value: char) -> bool {
        if let ALIVE = value { true } else { false }
    }

    pub fn cell_to_str(cell: &bool) -> char {
        if *cell { ALIVE } else { DEAD }
    }
}

impl Board {
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

    fn new_2d_from_str(asstr: &str) -> Arr2d<bool> {
        let mut rows: Arr2d<bool> = Arr2d::new();

        for row in asstr.split("\n") {
            let mut cells: Vec<bool> = Vec::new();
            for cell in row.trim().chars() {
                cells.push(game_of_life::cell_from_char(cell));
            }
            rows.add_row(cells);
        }

        rows
    }

    fn from_str(asstr: &str) -> Board {
        let contents: [Arr2d<bool>; 2] =
            [Self::new_2d_from_str(asstr), Self::new_2d_from_str(asstr)];
        Board { index: 0, contents }
    }

    fn iterate(&mut self) {
        let next_index = if self.index == 0 { 1 } else { 0 };
        for r in 0..self.contents[self.index].rows() {
            for c in 0..self.contents[self.index].columns(r) {
                let n = Self::count_neighbours(&self.contents[self.index], r, c);
                self.contents[next_index].set(
                    r,
                    c,
                    game_of_life::next_state((*self.contents[self.index].get(r, c), n)),
                );
            }
        }
        self.index = next_index;
    }

    fn print(&self) {
        println!("Board");
        self.contents[self.index].print(&game_of_life::cell_to_str);
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

    let mut board: Board = Board::from_str(&contents);

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
        assert!(game_of_life::next_state((true, 3)));
    }
}
