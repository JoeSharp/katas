use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidCharacter,
    NotEnoughLines,
    NotEnoughChars,
    InvalidValue,
}

pub trait CellValue: TryFrom<char> + Into<char> + PartialEq + Copy + Hash + Display {}

impl<T> CellValue for T where T: TryFrom<char> + Into<char> + PartialEq + Copy + Hash + Display {}

#[derive(Eq, Hash, Debug)]
pub struct Cell<T: CellValue> {
    id: u32,
    row: usize,
    column: usize,
    value: T,
}

impl<T> fmt::Display for Cell<T>
where
    T: CellValue,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> Cell<T>
where
    T: CellValue,
{
    pub fn row(&self) -> usize {
        self.row
    }
    pub fn column(&self) -> usize {
        self.column
    }
    pub fn value(&self) -> T {
        self.value
    }

    pub fn from(row: usize, column: usize, value: T) -> Cell<T> {
        Cell {
            id: 0,
            row,
            column,
            value,
        }
    }
}

impl<T> PartialEq for Cell<T>
where
    T: CellValue,
{
    fn eq(&self, b: &Cell<T>) -> bool {
        self.row == b.row && self.column == b.column && self.value == b.value
    }
}

#[derive(Debug, Hash, Eq)]
pub struct Arr2d<T: CellValue> {
    contents: Vec<Vec<Cell<T>>>,
}

impl<T> fmt::Display for Arr2d<T>
where
    T: CellValue,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.contents {
            for (i, val) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", val)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T> Arr2d<T>
where
    T: CellValue,
{
    pub fn new() -> Arr2d<T> {
        Arr2d {
            contents: Vec::new(),
        }
    }

    pub fn from_contents(contents: Vec<Vec<T>>) -> Arr2d<T> {
        let mut id = 0;

        Arr2d {
            contents: contents
                .iter()
                .enumerate()
                .map(|(row, row_c)| {
                    row_c
                        .iter()
                        .enumerate()
                        .map(|(column, &value)| {
                            id += 1;
                            return Cell {
                                id,
                                row,
                                column,
                                value,
                            };
                        })
                        .collect()
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
                    Err(_) => return Err(ParseError::InvalidValue),
                }
            }
            rows.push(cells);
        }

        Ok(Arr2d::from_contents(rows))
    }

    pub fn from_str(as_str: &str) -> Result<Arr2d<T>, ParseError> {
        Self::from_lines(
            as_str
                .split("\n")
                .map(|line| line.trim())
                .filter(|line| !line.is_empty()),
        )
    }

    pub fn get_cell(&self, row: usize, column: usize) -> Result<&Cell<T>, &str> {
        match &self.contents.get(row) {
            Some(r) => match r.get(column) {
                Some(c) => Ok(c),
                None => return Err("Invalid column index"),
            },
            None => return Err("Invalid row index"),
        }
    }

    pub fn all_cells(&self) -> impl Iterator<Item = &Cell<T>> {
        self.contents.iter().flat_map(|row| row.into_iter())
    }

    pub fn get_neighbours(&self, row: usize, column: usize) -> impl Iterator<Item = &Cell<T>> {
        [
            (Some(row), column.checked_sub(1)),
            (Some(row), column.checked_add(1)),
            (row.checked_sub(1), Some(column)),
            (row.checked_add(1), Some(column)),
        ]
        .into_iter()
        .filter_map(|(r, c)| {
            if let (Some(r), Some(c)) = (r, c) {
                self.get_cell(r, c).ok()
            } else {
                None
            }
        })
    }

    pub fn get_perimeter(
        &self,
        row: usize,
        column: usize,
    ) -> Result<impl Iterator<Item = &Cell<T>>, &str> {
        let mut seen = HashSet::new();
        let start_cell = match self.get_cell(row, column) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };

        match self.flood_fill(row, column) {
            Ok(c) => Ok(c
                .flat_map(|cell| self.get_neighbours(cell.row, cell.column))
                .filter(|cell| cell.value != start_cell.value)
                .filter(move |cell| seen.insert(cell.id))),
            Err(e) => Err(e),
        }
    }

    pub fn flood_fill(
        &self,
        row: usize,
        column: usize,
    ) -> Result<impl Iterator<Item = &Cell<T>>, &str> {
        let mut to_visit: Vec<&Cell<T>> = Vec::new();
        let mut ids_seen: HashSet<u32> = HashSet::new();
        let start_cell = match self.get_cell(row, column) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        to_visit.push(start_cell);
        ids_seen.insert(start_cell.id);

        Ok(std::iter::from_fn(move || match to_visit.pop() {
            Some(cell) => {
                self.get_neighbours(cell.row, cell.column)
                    .filter(|c| ids_seen.insert(c.id) && c.value == start_cell.value)
                    .for_each(|c| to_visit.push(c));

                return Some(cell);
            }
            None => None,
        }))
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

impl<T: CellValue> PartialEq for Arr2d<T> {
    fn eq(&self, other: &Self) -> bool {
        self.contents == other.contents
    }
}

#[cfg(test)]
mod tests {
    use super::Arr2d;
    use super::Cell;
    use super::ParseError;
    use std::fmt;
    use test_case::test_case;

    type Coordinate = (usize, usize);
    type ExpectedCell = (usize, usize, bool);

    fn ex_cells_with_value(coordinates: Vec<Coordinate>, value: bool) -> Vec<ExpectedCell> {
        coordinates
            .into_iter()
            .map(|(row, column)| (row, column, value))
            .collect()
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct TestBool(bool);

    impl fmt::Display for TestBool {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

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
    fn test_from_str() {
        // Given
        let expected: Arr2d<TestBool> = Arr2d::from_contents(vec![
            vec![
                TestBool(true),
                TestBool(true),
                TestBool(true),
                TestBool(false),
                TestBool(false),
            ],
            vec![
                TestBool(false),
                TestBool(true),
                TestBool(false),
                TestBool(false),
                TestBool(true),
            ],
            vec![
                TestBool(true),
                TestBool(false),
                TestBool(false),
                TestBool(true),
                TestBool(false),
            ],
        ]);

        // When
        let result: Arr2d<TestBool> = Arr2d::from_str(
            r#"
            yyynn
            nynny
            ynnyn
"#,
        )
        .expect("Arr2d should have parsed test input");

        // Then
        assert_eq!(result, expected);
    }

    #[test_case((0, 0), vec![(0, 1, true), (1, 0, false)] )]
    #[test_case((1, 1), vec![(0, 1, true), (2, 1, false), (1, 0, false), (1, 2, false)] )]
    #[test_case((2, 2), vec![(2, 1, false), (1, 2, false), (2, 3, true)] )]
    #[test_case((2, 4), vec![(2, 3, true), (1, 4, true)] )]
    #[test_case((0, 4), vec![(0, 3, false), (1, 4, true)] )]
    fn test_get_neighbours((row, column): Coordinate, expected: Vec<ExpectedCell>) {
        // Given
        let input: Arr2d<TestBool> = Arr2d::from_str(
            r#"
            yyynn
            nynny
            ynnyn
"#,
        )
        .expect("Arr2d should have parsed test input");

        // When
        let result: Vec<&Cell<TestBool>> = input.get_neighbours(row, column).collect();

        // Then
        assert_cells(&result, expected);
    }

    #[test_case((1, 1), vec![(1, 0, false), (0, 3, false), (2, 1, false), (1, 2, false)])]
    #[test_case((2, 1), vec![(2, 0, true), (2, 3, true), (1, 1, true), (1, 4, true), (0, 2, true)])]
    fn test_get_perimeter((row, column): Coordinate, expected: Vec<ExpectedCell>) {
        // Given
        let input: Arr2d<TestBool> = Arr2d::from_str(
            r#"
            yyynn
            nynny
            ynnyn
"#,
        )
        .expect("Arr2d should have parsed test input");

        // When
        let result: Vec<&Cell<TestBool>> = match input.get_perimeter(row, column) {
            Ok(i) => i.collect(),
            _ => panic!("Could not flood fill"),
        };

        // Then
        assert_cells(&result, expected);
    }

    #[test_case((1, 1, true), vec![(1, 1), (0, 0), (0, 1), (0, 2)])]
    #[test_case((2, 1, false), vec![(2, 1), (2, 2), (1, 2), (1, 3), (0, 3), (0, 4)])]
    fn test_flood_fill((row, column, value): ExpectedCell, expected: Vec<Coordinate>) {
        // Given
        let input: Arr2d<TestBool> = Arr2d::from_str(
            r#"
            yyynn
            nynny
            ynnyn
"#,
        )
        .expect("Arr2d should have parsed test input");

        // When
        let result: Vec<&Cell<TestBool>> = match input.flood_fill(row, column) {
            Ok(i) => i.collect(),
            _ => panic!("Could not flood fill"),
        };

        // Then
        let expected_cells = ex_cells_with_value(expected, value);
        assert_cells(&result, expected_cells);
    }

    fn assert_cells(result: &Vec<&Cell<TestBool>>, expected: Vec<ExpectedCell>) {
        assert_eq!(
            expected.len(),
            result.len(),
            "Results should only contain expected cells"
        );
        for (ex_row, ex_column, value) in expected {
            let expected_cell = Cell::from(ex_row, ex_column, TestBool(value));
            assert!(
                result.contains(&&expected_cell),
                "result {result:?} does not contain {expected_cell:?}"
            );
        }
    }

    #[test]
    fn test_expand() {
        // Given
        let a: Arr2d<TestBool> = Arr2d::new();
        let expected: Arr2d<TestBool> = Arr2d::from_contents(vec![
            vec![TestBool(false), TestBool(false), TestBool(false)],
            vec![TestBool(false), TestBool(false), TestBool(false)],
            vec![TestBool(false), TestBool(false), TestBool(false)],
            vec![TestBool(false), TestBool(false), TestBool(false)],
            vec![TestBool(false), TestBool(false), TestBool(false)],
        ]);

        // When
        let result = a.expand(3, 5, TestBool(false));

        // Then
        assert_eq!(expected, result);
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
