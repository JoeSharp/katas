use crate::arr2d::Arr2d;
use crate::arr2d::AsChar;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum GoPlayer {
    White,
    Black,
    Unknown,
}
impl AsChar for GoPlayer {
    fn from_char(c: &char) -> Self {
        match *c {
            GoBoard::WHITE => GoPlayer::White,
            GoBoard::BLACK => GoPlayer::Black,
            _ => GoPlayer::Unknown,
        }
    }

    fn to_char(&self) -> char {
        match *self {
            GoPlayer::White => GoBoard::WHITE,
            GoPlayer::Black => GoBoard::BLACK,
            _ => GoBoard::EMPTY,
        }
    }
}

impl GoPlayer {
    pub fn other(&self) -> GoPlayer {
        match *self {
            GoPlayer::White => GoPlayer::Black,
            GoPlayer::Black => GoPlayer::White,
            _ => GoPlayer::Unknown,
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

struct GoError;

// Implement std::fmt::Display for GoError
impl fmt::Display for GoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

// Implement std::fmt::Debug for GoError
impl fmt::Debug for GoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
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
    fn from_char(c: &char) -> Self {
        match *c {
            GoBoard::WHITE => GoCell::White,
            GoBoard::WHITE_PENDING => GoCell::WhitePending,
            GoBoard::BLACK => GoCell::Black,
            GoBoard::BLACK_PENDING => GoCell::BlackPending,
            _ => GoCell::Empty,
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

    pub fn from_str(as_str: &str) -> GoBoard {
        let lines = as_str.split("\n").collect::<Vec<&str>>();

        if lines.len() <= 4 {
            panic!("Not enough lines in file")
        }
        let whos_turn = GoPlayer::from_char(&lines[0].chars().nth(0).unwrap());
        let last_move: LastMove = lines[1].parse().unwrap();
        let white_captures: u16 = lines[2].parse().unwrap();
        let black_captures: u16 = lines[3].parse().unwrap();
        //let board = Arr2d::from_lines(lines[4..]);
        let board = Arr2d::new();

        GoBoard {
            whos_turn,
            last_move,
            white_captures,
            black_captures,
            board,
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
}
