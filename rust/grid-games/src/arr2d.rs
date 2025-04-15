#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidCharacter,
    NotEnoughLines,
    NotEnoughChars,
    InvalidValue,
}

#[derive(Debug, PartialEq)]
pub struct Cell<T>
where
    T: TryFrom<char> + Into<char> + PartialEq + Copy,
{
    row: usize,
    column: usize,
    value: T,
}

#[derive(Debug)]
pub struct Arr2d<T: TryFrom<char> + Into<char> + PartialEq + Copy> {
    contents: Vec<Vec<Cell<T>>>,
}

impl<T: TryFrom<char, Error = ParseError> + Into<char> + PartialEq + Copy> Arr2d<T> {
    pub fn new() -> Arr2d<T> {
        Arr2d {
            contents: Vec::new(),
        }
    }

    pub fn from_contents(contents: Vec<Vec<T>>) -> Arr2d<T> {
        Arr2d {
            contents: contents
                .iter()
                .enumerate()
                .map(|(row, row_c)| {
                    return row_c
                        .iter()
                        .enumerate()
                        .map(|(column, &value)| Cell { row, column, value })
                        .collect();
                })
                .collect(),
        }
    }

    pub fn from_lines<'a>(lines: impl Iterator<Item = &'a str>) -> Result<Arr2d<T>, ParseError> {
        let mut rows: Vec<Vec<T>> = Vec::new();

        for row in lines {
            let mut cells: Vec<T> = Vec::new();
            for cell in row.trim().chars() {
                match <T>::try_from(cell) {
                    Ok(v) => cells.push(v),
                    Err(e) => return Err(e),
                }
            }
            rows.push(cells);
        }

        Ok(Arr2d::from_contents(rows))
    }

    pub fn expand(&self, width: usize, height: usize, filler: T) -> Arr2d<T> {
        let mut contents: Vec<Vec<T>> = self
            .contents
            .iter()
            .map(|v| v.iter().map(|c| c.value).collect())
            .collect();

        for row in contents.iter_mut() {
            while row.len() < width {
                row.push(filler);
            }
            while row.len() > width {
                row.pop();
            }
        }

        while contents.len() < height {
            contents.push(vec![filler; width]);
        }
        while contents.len() > height {
            contents.pop();
        }

        Arr2d::from_contents(contents)
    }

    pub fn from_str(as_str: &str) -> Result<Arr2d<T>, ParseError> {
        Self::from_lines(
            as_str
                .split("\n")
                .map(|line| line.trim())
                .filter(|line| !line.is_empty()),
        )
    }

    pub fn rows(&self) -> usize {
        self.contents.len()
    }

    pub fn columns(&self, row: usize) -> usize {
        self.contents[row].len()
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.contents[row][col].value
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.contents[row][col].value = value;
    }

    pub fn to_str(&self) -> String {
        let mut as_str = String::new();
        for row in &self.contents {
            for cell in row {
                as_str.push(cell.value.into());
            }
            as_str.push_str("\n");
        }

        as_str
    }
}

impl<T: TryFrom<char> + Into<char> + PartialEq + Copy> PartialEq for Arr2d<T> {
    fn eq(&self, other: &Self) -> bool {
        self.contents == other.contents
    }
}

#[cfg(test)]
mod tests {
    use super::Arr2d;
    use super::ParseError;

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct TestBool(bool);

    impl TryFrom<char> for TestBool {
        type Error = ParseError;

        fn try_from(c: char) -> Result<TestBool, ParseError> {
            Ok(TestBool(c == 'y'))
        }
    }
    impl Into<char> for TestBool {
        fn into(self) -> char {
            if self.0 { 'y' } else { 'n' }
        }
    }

    #[test]
    fn test_expand() {
        let a: Arr2d<TestBool> = Arr2d::new();
        let b = a.expand(3, 5, TestBool(false));

        let c: Arr2d<TestBool> = Arr2d::from_contents(vec![
            vec![TestBool(false), TestBool(false), TestBool(false)],
            vec![TestBool(false), TestBool(false), TestBool(false)],
            vec![TestBool(false), TestBool(false), TestBool(false)],
            vec![TestBool(false), TestBool(false), TestBool(false)],
            vec![TestBool(false), TestBool(false), TestBool(false)],
        ]);

        assert_eq!(b, c);
    }

    #[test]
    fn test_eq() {
        let a: Arr2d<TestBool> = Arr2d::from_contents(vec![vec![TestBool(true), TestBool(false)]]);
        let b: Arr2d<TestBool> = Arr2d::from_contents(vec![vec![TestBool(true), TestBool(false)]]);

        assert_eq!(a, b);
    }

    #[test]
    fn test_neq() {
        let a: Arr2d<TestBool> = Arr2d::from_contents(vec![vec![TestBool(true), TestBool(false)]]);
        let b: Arr2d<TestBool> = Arr2d::new();

        assert_ne!(a, b);
    }
}
