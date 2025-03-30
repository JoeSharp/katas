pub struct Arr2d<T> {
    contents: Vec<Vec<T>>,
}

impl<T> Arr2d<T> {
    pub fn new() -> Arr2d<T> {
        Arr2d {
            contents: Vec::new(),
        }
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

    pub fn print(&self, to_str: &dyn Fn(&T) -> char) {
        for row in &self.contents {
            for cell in row {
                print!("{}", to_str(cell));
            }
            print!("\n");
        }
    }
}
