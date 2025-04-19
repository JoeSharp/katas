use crate::arr2d::Arr2d;
use crate::arr2d::Cell;
use crate::arr2d::ParseError;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum GoPlayer {
    White,
    Black,
}

impl fmt::Display for GoPlayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                GoPlayer::White => GoBoard::WHITE,
                GoPlayer::Black => GoBoard::BLACK,
            }
        )
    }
}

impl TryFrom<char> for GoPlayer {
    type Error = ParseError;

    fn try_from(c: char) -> Result<GoPlayer, ParseError> {
        match c {
            GoBoard::WHITE => Ok(GoPlayer::White),
            GoBoard::BLACK => Ok(GoPlayer::Black),
            _ => Err(ParseError::InvalidCharacter),
        }
    }
}

impl Into<char> for GoPlayer {
    fn into(self) -> char {
        match self {
            GoPlayer::White => GoBoard::WHITE,
            GoPlayer::Black => GoBoard::BLACK,
        }
    }
}

impl GoPlayer {
    pub fn other(&self) -> GoPlayer {
        match *self {
            GoPlayer::White => GoPlayer::Black,
            GoPlayer::Black => GoPlayer::White,
        }
    }
}

#[derive(Debug, PartialEq)]
enum LastMove {
    Ok,
    IllegalKo,
    IllegalSuicidal,
}
impl fmt::Display for LastMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LastMove::Ok => write!(f, "ok"),
            LastMove::IllegalKo => write!(f, "illegal_ko"),
            LastMove::IllegalSuicidal => write!(f, "illegal_suicidal"),
        }
    }
}

impl FromStr for LastMove {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        match s {
            "ok" => Result::Ok(LastMove::Ok),
            "illegal_ko" => Result::Ok(LastMove::IllegalKo),
            "illegal_suicidal" => Result::Ok(LastMove::IllegalSuicidal),
            _ => Err(ParseError::InvalidValue),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum GoCell {
    White,
    WhitePending,
    Black,
    BlackPending,
    Empty,
}

impl GoCell {
    fn player(&self) -> Option<GoPlayer> {
        match self {
            GoCell::White => Some(GoPlayer::White),
            GoCell::WhitePending => Some(GoPlayer::White),
            GoCell::Black => Some(GoPlayer::Black),
            GoCell::BlackPending => Some(GoPlayer::Black),
            _ => None,
        }
    }
}

impl fmt::Display for GoCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                GoCell::White => GoBoard::WHITE,
                GoCell::WhitePending => GoBoard::WHITE_PENDING,
                GoCell::Black => GoBoard::BLACK,
                GoCell::BlackPending => GoBoard::BLACK_PENDING,
                GoCell::Empty => GoBoard::EMPTY,
            }
        )
    }
}

impl TryFrom<char> for GoCell {
    type Error = ParseError;

    fn try_from(c: char) -> Result<GoCell, ParseError> {
        match c {
            GoBoard::WHITE => Ok(GoCell::White),
            GoBoard::WHITE_PENDING => Ok(GoCell::WhitePending),
            GoBoard::BLACK => Ok(GoCell::Black),
            GoBoard::BLACK_PENDING => Ok(GoCell::BlackPending),
            GoBoard::EMPTY => Ok(GoCell::Empty),
            _ => Err(ParseError::InvalidCharacter),
        }
    }
}

impl Into<char> for GoCell {
    fn into(self) -> char {
        match self {
            GoCell::White => GoBoard::WHITE,
            GoCell::WhitePending => GoBoard::WHITE_PENDING,
            GoCell::Black => GoBoard::BLACK,
            GoCell::BlackPending => GoBoard::BLACK_PENDING,
            GoCell::Empty => GoBoard::EMPTY,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GoBoard {
    whos_turn: GoPlayer,
    last_move: LastMove,
    white_captures: u16,
    black_captures: u16,
    board: Arr2d<GoCell>,
}

impl GoBoard {
    pub const WHITE: char = 'W';
    pub const WHITE_PENDING: char = 'w';
    pub const BLACK: char = 'B';
    pub const BLACK_PENDING: char = 'b';
    pub const EMPTY: char = '-';

    pub fn iterate(&mut self) {}

    /**
     * It would probably be nicer if it allowed reading of K/V pairs in whatever order, put them in
     * a map and then we would look for specific keys
     */
    fn read_kv<'a, 'b>(input: &'a str, name: &'b str) -> Result<&'a str, ParseError> {
        let parts: Vec<&str> = input.split("=").collect();
        match parts.len() {
            2 => {
                if parts[0] == name {
                    Ok(parts[1])
                } else {
                    Err(ParseError::InvalidValue)
                }
            }
            _ => Err(ParseError::InvalidValue),
        }
    }

    pub fn from_str(as_str: &str) -> Result<GoBoard, ParseError> {
        let lines: Vec<&str> = as_str
            .split("\n")
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>();

        if lines.len() <= 4 {
            return Err(ParseError::NotEnoughLines);
        }

        let whos_turn = Self::read_kv(&lines[0], "turn")?;
        let whos_turn: GoPlayer = match whos_turn.chars().nth(0) {
            Some(c) => match GoPlayer::try_from(c) {
                Ok(gp) => gp,
                Err(e) => return Err(e),
            },
            None => return Err(ParseError::NotEnoughChars),
        };

        let last_move = Self::read_kv(lines[1], "last_move")?;
        let last_move: LastMove = match last_move.parse() {
            Ok(i) => i,
            Err(e) => return Err(e),
        };

        let white_captures = Self::read_kv(lines[2], "capturesW")?;
        let white_captures: u16 = match white_captures.parse() {
            Ok(i) => i,
            Err(_) => return Err(ParseError::InvalidValue),
        };

        let black_captures = Self::read_kv(lines[3], "capturesB")?;
        let black_captures: u16 = match black_captures.parse() {
            Ok(i) => i,
            Err(_) => return Err(ParseError::InvalidValue),
        };

        let slice = &lines[4..];
        let board: Arr2d<GoCell> = match Arr2d::from_lines(slice.iter().copied()) {
            Ok(i) => i,
            Err(e) => return Err(e),
        };

        Ok(GoBoard {
            whos_turn,
            last_move,
            white_captures,
            black_captures,
            board,
        })
    }

    fn get_liberties(
        &self,
        row: usize,
        column: usize,
    ) -> Result<impl Iterator<Item = &Cell<GoCell>>, &str> {
        match self.board.get_perimeter(row, column) {
            Ok(p) => Ok(p.filter(|c| c.value == GoCell::Empty)),
            _ => Err("Could not retrieve perimeter of {row}, {column}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use test_case::test_case;

    #[test_case(GoPlayer::White, GoPlayer::Black)]
    fn test_other_player(player: GoPlayer, expected: GoPlayer) {
        let result = player.other();

        assert_eq!(result, expected)
    }

    fn create_go_from_test_file(name: &str) -> Result<GoBoard, ParseError> {
        let filename = format!("resources/tests/go/{}", name);

        let file_contents = &fs::read_to_string(&filename).expect(&format!(
            "Expected to find hardcoded test resource at {}",
            filename
        ));
        GoBoard::from_str(file_contents)
    }

    #[test]
    fn test_parse() {
        let state = create_go_from_test_file("parse/1.txt").unwrap();

        use GoCell::*;

        assert_eq!(
            state,
            GoBoard {
                whos_turn: GoPlayer::White,
                last_move: LastMove::Ok,
                white_captures: 16,
                black_captures: 23,
                board: Arr2d::from_contents(vec![
                    vec![Empty, White, Empty, Empty, Empty,],
                    vec![Empty, Empty, White, Empty, Empty],
                    vec![Empty, Black, Empty, Empty, Empty],
                    vec![Empty, Black, Empty, BlackPending, Empty],
                    vec![Empty, Empty, Empty, Empty, Empty],
                ])
            }
        );
    }

    #[test]
    fn test_parse_str() {
        let as_str = r#"
    turn=W
last_move=ok
capturesW=16
capturesB=23
-W---
--W--
-B---
-B-b-
-----
        "#;
        let state = GoBoard::from_str(as_str).unwrap();

        use GoCell::*;

        assert_eq!(
            state,
            GoBoard {
                whos_turn: GoPlayer::White,
                last_move: LastMove::Ok,
                white_captures: 16,
                black_captures: 23,
                board: Arr2d::from_contents(vec![
                    vec![Empty, White, Empty, Empty, Empty,],
                    vec![Empty, Empty, White, Empty, Empty],
                    vec![Empty, Black, Empty, Empty, Empty],
                    vec![Empty, Black, Empty, BlackPending, Empty],
                    vec![Empty, Empty, Empty, Empty, Empty],
                ])
            }
        );
    }

    #[test]
    fn test_get_liberties() {
        // Given
        let as_str = r#"
    turn=W
last_move=ok
capturesW=16
capturesB=23
-W-W-
W-W--
WBBW-
WB-bW
WWWW-
        "#;
        let state = GoBoard::from_str(as_str).unwrap();

        // When
        let result: Vec<&Cell<GoCell>> = match state.get_liberties(2, 1) {
            Ok(p) => p.collect(),
            _ => panic!("Could not get liberties"),
        };

        // Then
        for (exp_row, exp_column) in [(1, 1), (3, 2)] {
            let exp_cell: Cell<GoCell> = Cell::from(exp_row, exp_column, GoCell::Empty);

            assert!(
                result.contains(&&exp_cell),
                "Result {result:?} does not contain {exp_cell:?}"
            );
        }
    }

    #[test_case("captures/simple_1")]
    #[test_case("captures/simple_2")]
    #[test_case("captures/corner_1")]
    #[test_case("captures/corner_2")]
    fn test_captures(name: &str) {
        let file_before = format!("{}/1_before.txt", name);
        let file_execute = format!("{}/1_execute.txt", name);
        let mut state_before = create_go_from_test_file(&file_before).unwrap();
        state_before.iterate();
        let state_execute = create_go_from_test_file(&file_execute).unwrap();
        assert_eq!(state_before, state_execute);
    }
}
