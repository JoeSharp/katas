use crate::arr2d::Arr2d;
use crate::arr2d::AsChar;
use crate::arr2d::ParseError;

impl AsChar for bool {
    fn from_char(c: &char) -> Result<Self, ParseError> {
        match *c {
            GameOfLife::DEAD => Ok(false),
            GameOfLife::ALIVE => Ok(true),
            _ => Err(ParseError::InvalidCharacter),
        }
    }

    fn to_char(&self) -> char {
        if *self {
            GameOfLife::ALIVE
        } else {
            GameOfLife::DEAD
        }
    }
}

#[derive(Debug)]
pub struct GameOfLife {
    index: usize,
    contents: [Arr2d<bool>; 2],
}

impl GameOfLife {
    pub const ALIVE: char = 'x';
    pub const DEAD: char = '-';

    pub fn expand(&mut self, width: usize, height: usize) {
        for c in self.contents.iter_mut() {
            c.expand(width, height, false);
        }
    }

    pub fn next_state(state: (bool, u8)) -> bool {
        match state {
            (true, 0..=1) => false, // Underpopulation
            (true, 2 | 3) => true,  // Lives on
            (true, _) => false,     // Overpopulation
            (false, 3) => true,     // Reproduction
            (false, _) => false,
        }
    }

    pub fn from_str(as_str: &str) -> Result<GameOfLife, ParseError> {
        let board0 = match Arr2d::from_str(as_str) {
            Ok(b) => b,
            Err(e) => return Err(e),
        };
        let board1 = match Arr2d::from_str(as_str) {
            Ok(b) => b,
            Err(e) => return Err(e),
        };
        let contents: [Arr2d<bool>; 2] = [board0, board1];
        Ok(GameOfLife { index: 0, contents })
    }

    pub fn iterate(&mut self) {
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

    pub fn to_str(&self) -> String {
        self.contents[self.index].to_str()
    }

    fn current_state(&self) -> &Arr2d<bool> {
        &self.contents[self.index]
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
}

impl PartialEq for GameOfLife {
    fn eq(&self, other: &GameOfLife) -> bool {
        let mine = self.current_state();
        let other = other.current_state();

        mine == other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use test_case::test_case;

    #[test]
    fn test_next_state() {
        assert!(GameOfLife::next_state((true, 3)));
    }

    #[test]
    fn invalid_char() {
        let result = GameOfLife::from_str("F---");
        assert_eq!(result, Err(ParseError::InvalidCharacter));
    }

    fn create_gol_from_test_file(name: &str, index: u8) -> Result<GameOfLife, ParseError> {
        let filename = format!("resources/tests/gol/{}/{}.txt", name, index);

        let file_contents = &fs::read_to_string(&filename).expect(&format!(
            "Expected to find hardcoded test resource at {}",
            filename
        ));
        GameOfLife::from_str(file_contents)
    }

    #[test_case("blinker")]
    #[test_case("toad")]
    #[test_case("beacon")]
    fn test_oscillators(name: &str) {
        let mut state1 = create_gol_from_test_file(name, 1).unwrap();
        let state2 = create_gol_from_test_file(name, 2).unwrap();
        assert!(state1 != state2);

        // Iterate an odd number of times
        state1.iterate();

        assert_eq!(state1, state2);
    }
}
