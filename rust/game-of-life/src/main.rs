struct Board {
    index: usize,
    contents: [[[bool; 10]; 10]; 2],
}

fn cell_to_str(cell: bool) -> char {
    if cell { 'X' } else { '-' }
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
        self.contents[self.index][1][1] = true;
        self.contents[self.index][1][2] = true;
        self.contents[self.index][1][3] = true;
    }

    fn iterate(&mut self) {
        let nextIndex = if self.index == 0 { 1 } else { 0 };
        for (r, row) in self.contents[self.index].iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                self.contents[nextIndex][r][c] = true;
            }
        }
        self.index = nextIndex;
    }

    fn print(&self) {
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

    for _ in 0..=3 {
        board.print();
        board.iterate();
    }

    println!("Done");
}
