pub struct Arr2d<T> {
    contents: Vec<Vec<T>>,
}

pub type ToChar<T> = fn(&T) -> char;
pub type FromChar<T> = fn(&char) -> T;

impl<T> Arr2d<T> {
    pub fn new() -> Arr2d<T> {
        Arr2d {
            contents: Vec::new(),
        }
    }

    pub fn from_str(as_str: &str, from_char: FromChar<T>) -> Arr2d<T> {
        let mut rows: Arr2d<T> = Arr2d::new();

        for row in as_str.split("\n") {
            let mut cells: Vec<T> = Vec::new();
            for cell in row.trim().chars() {
                cells.push(from_char(&cell));
            }
            rows.add_row(cells);
        }

        rows
    }

    pub fn rows(&self) -> usize {
        self.contents.len()
    }

    pub fn columns(&self, row: usize) -> usize {
        self.contents[row].len()
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.contents[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.contents[row][col] = value;
    }

    pub fn add_row(&mut self, row: Vec<T>) {
        self.contents.push(row);
    }

    pub fn print(&self, to_char: ToChar<T>) {
        for row in &self.contents {
            for cell in row {
                print!("{}", to_char(cell));
            }
            print!("\n");
        }
    }
}
