use crate::arr2d::Arr2d;
use crate::arr2d::AsChar;
use crate::arr2d::ParseError;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum GoPlayer {
    White,
    Black,
}
impl AsChar for GoPlayer {
    fn from_char(c: &char) -> Result<Self, ParseError> {
        match *c {
            GoBoard::WHITE => Ok(GoPlayer::White),
            GoBoard::BLACK => Ok(GoPlayer::Black),
            _ => Err(ParseError::InvalidCharacter),
        }
    }

    fn to_char(&self) -> char {
        match *self {
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

#[derive(Debug)]
struct GoError;

// Implement std::fmt::Display for GoError
impl fmt::Display for GoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

impl FromStr for LastMove {
    type Err = GoError;

    fn from_str(s: &str) -> Result<Self, GoError> {
        match s {
            "ok" => Result::Ok(LastMove::Ok),
            "illegal_ko" => Result::Ok(LastMove::IllegalKo),
            "illegal_suicidal" => Result::Ok(LastMove::IllegalSuicidal),
            _ => Err(GoError),
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

impl AsChar for GoCell {
    fn from_char(c: &char) -> Result<Self, ParseError> {
        match *c {
            GoBoard::WHITE => Ok(GoCell::White),
            GoBoard::WHITE_PENDING => Ok(GoCell::WhitePending),
            GoBoard::BLACK => Ok(GoCell::Black),
            GoBoard::BLACK_PENDING => Ok(GoCell::BlackPending),
            GoBoard::EMPTY => Ok(GoCell::Empty),
            _ => Err(ParseError::InvalidCharacter),
        }
    }

    fn to_char(&self) -> char {
        match *self {
            GoCell::White => GoBoard::WHITE,
            GoCell::WhitePending => GoBoard::WHITE_PENDING,
            GoCell::Black => GoBoard::BLACK,
            GoCell::BlackPending => GoBoard::BLACK_PENDING,
            _ => GoBoard::EMPTY,
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

    pub fn from_str(as_str: &str) -> Result<GoBoard, ParseError> {
        let lines = as_str.split("\n").collect::<Vec<&str>>();

        if lines.len() <= 4 {
            return Err(ParseError::NotEnoughLines);
        }
        let whos_turn: GoPlayer = match &lines[0].chars().nth(0) {
            Some(c) => match GoPlayer::from_char(c) {
                Ok(gp) => gp,
                Err(e) => return Err(e),
            },
            None => return Err(ParseError::NotEnoughChars),
        };
        let last_move: LastMove = lines[1].parse().unwrap();
        let white_captures: u16 = lines[2].parse().unwrap();
        let black_captures: u16 = lines[3].parse().unwrap();
        //let board = Arr2d::from_lines(lines[4..]);
        let board = Arr2d::new();

        Ok(GoBoard {
            whos_turn,
            last_move,
            white_captures,
            black_captures,
            board,
        })
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
}
