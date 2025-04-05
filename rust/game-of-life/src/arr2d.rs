pub trait AsChar {
    fn to_char(&self) -> char;
    fn from_char(c: &char) -> Self;
}

#[derive(Debug)]
pub struct Arr2d<T: AsChar + PartialEq + Copy> {
    contents: Vec<Vec<T>>,
}

impl<T: AsChar + PartialEq + Copy> Arr2d<T> {
    pub fn new() -> Arr2d<T> {
        Arr2d {
            contents: Vec::new(),
        }
    }

    pub fn expand(&mut self, width: usize, height: usize, filler: T) {
        for row in self.contents.iter_mut() {
            while row.len() < width {
                row.push(filler);
            }
            while row.len() > width {
                row.pop();
            }
        }

        while self.contents.len() < height {
            self.contents.push(vec![filler; width]);
        }
        while self.contents.len() > height {
            self.contents.pop();
        }
    }

    pub fn from_str(as_str: &str) -> Arr2d<T> {
        let mut rows: Arr2d<T> = Arr2d::new();

        for row in as_str.split("\n") {
            let mut cells: Vec<T> = Vec::new();
            for cell in row.trim().chars() {
                cells.push(<T>::from_char(&cell));
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

    pub fn to_str(&self) -> String {
        let mut as_str = String::new();
        for row in &self.contents {
            for cell in row {
                as_str.push(cell.to_char());
            }
            as_str.push_str("\n");
        }

        as_str
    }
}

impl<T: AsChar + PartialEq + Copy> PartialEq for Arr2d<T> {
    fn eq(&self, other: &Self) -> bool {
        self.contents == other.contents
    }
}

#[cfg(test)]
mod tests {
    use super::Arr2d;

    #[test]
    fn test_expand() {
        let mut a: Arr2d<bool> = Arr2d::new();
        a.expand(3, 5, false);

        let mut b: Arr2d<bool> = Arr2d::new();
        b.contents = vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false],
        ];

        assert_eq!(a, b);
    }

    #[test]
    fn test_eq() {
        let mut a: Arr2d<bool> = Arr2d::new();
        let mut b: Arr2d<bool> = Arr2d::new();

        for x in [&mut a, &mut b] {
            x.add_row(vec![true, false]);
        }

        assert_eq!(a, b);
    }

    #[test]
    fn test_neq() {
        let mut a: Arr2d<bool> = Arr2d::new();
        let b: Arr2d<bool> = Arr2d::new();
        a.add_row(vec![true, false]);

        assert_ne!(a, b);
    }
}
