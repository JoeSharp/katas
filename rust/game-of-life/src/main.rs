type Arr2d = Vec<Vec<bool>>;

struct Board {
    index: usize,
    contents: [Arr2d; 2],
}

mod game_of_life {
    pub fn next_state(state: (bool, u8)) -> bool {
        match state {
            (true, 0..=1) => false, // Underpopulation
            (true, 2 | 3) => true,  // Lives on
            (true, _) => false,     // Overpopulation
            (false, 3) => true,     // Reproduction
            (false, _) => false,
        }
    }
    pub fn cell_to_str(cell: bool) -> char {
        if cell { 'X' } else { '-' }
    }
}

impl Board {
    fn count_neighbours(arr2d: &Arr2d, r: usize, c: usize) -> u8 {
        let mut n = 0;

        let top = r > 0;
        let left = c > 0;
        let bottom = r < arr2d.len() - 1;
        let right = c < arr2d[r].len() - 1;

        if top && left && arr2d[r - 1][c - 1] {
            n += 1;
        }
        if top && arr2d[r - 1][c] {
            n += 1;
        }
        if top && right && arr2d[r - 1][c + 1] {
            n += 1;
        }
        if left && arr2d[r][c - 1] {
            n += 1;
        }
        if right && arr2d[r][c + 1] {
            n += 1;
        }
        if bottom && left && arr2d[r + 1][c - 1] {
            n += 1;
        }
        if bottom && arr2d[r + 1][c] {
            n += 1;
        }
        if bottom && right && arr2d[r + 1][c + 1] {
            n += 1;
        }

        n
    }

    fn new_2d(size: usize) -> Arr2d {
        let mut rows: Arr2d = Vec::new();

        for _ in 0..size {
            let mut cells: Vec<bool> = Vec::new();
            for _ in 0..size {
                cells.push(false);
            }
            rows.push(cells);
        }

        rows
    }

    fn new() -> Board {
        let contents: [Arr2d; 2] = [Self::new_2d(10), Self::new_2d(10)];
        Board { index: 0, contents }
    }

    fn populate(&mut self) {
        self.contents[self.index][2][2] = true;
        self.contents[self.index][2][3] = true;
        self.contents[self.index][2][4] = true;
        self.contents[self.index][1][3] = true;
        self.contents[self.index][1][4] = true;
        self.contents[self.index][1][5] = true;
    }

    fn iterate(&mut self) {
        let next_index = if self.index == 0 { 1 } else { 0 };
        for r in 0..self.contents[self.index].len() {
            for c in 0..self.contents[self.index][r].len() {
                let n = Self::count_neighbours(&self.contents[self.index], r, c);
                self.contents[next_index][r][c] =
                    game_of_life::next_state((self.contents[self.index][r][c], n));
            }
        }
        self.index = next_index;
    }

    fn print(&self) {
        println!("Board");
        for row in &self.contents[self.index] {
            for cell in row {
                print!("{}", game_of_life::cell_to_str(*cell));
            }
            print!("\n");
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut board: Board = Board::new();

    board.populate();

    for _ in 0..=10 {
        board.print();
        board.iterate();

        println!("Done");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_state() {
        assert!(game_of_life::next_state((true, 3)));
    }
}
