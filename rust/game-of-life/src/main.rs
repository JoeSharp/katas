type Arr2d = [[bool; 10]; 10];
struct Board {
    index: usize,
    contents: [Arr2d; 2],
}

fn cell_to_str(cell: bool) -> char {
    if cell { 'X' } else { '-' }
}

type CellState = (bool, u8);

fn next_state(state: CellState) -> bool {
    match state {
        (true, 0..=1) => false, // Underpopulation
        (true, 2 | 3) => true,  // Lives on
        (true, _) => false,     // Overpopulation
        (false, 3) => true,     // Reproduction
        (false, _) => false,
    }
}

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

impl Board {
    fn new() -> Board {
        Board {
            index: 0,
            contents: [[[false; 10]; 10]; 2],
        }
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
        let curr: Arr2d = self.contents[self.index];
        let mut next: Arr2d = self.contents[next_index];
        for (r, row) in curr.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                let n = count_neighbours(&curr, r, c);
                next[r][c] = next_state((*cell, n));
            }
        }
        self.index = next_index;
    }

    fn print(&self) {
        println!("Board");
        for row in self.contents[self.index] {
            for cell in row {
                print!("{}", cell_to_str(cell));
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
    }

    println!("Done");
}
