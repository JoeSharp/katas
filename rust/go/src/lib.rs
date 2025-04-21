use arr2d::Arr2d;
use arr2d::Cell;
use arr2d::ParseError;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Hash, Eq, Debug, PartialEq, Copy, Clone)]
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

impl GoPlayer {
    fn into_pending(self) -> GoCell {
        match self {
            GoPlayer::White => GoCell::WhitePending,
            GoPlayer::Black => GoCell::BlackPending,
        }
    }
}

impl Into<GoCell> for GoPlayer {
    fn into(self) -> GoCell {
        match self {
            GoPlayer::White => GoCell::White,
            GoPlayer::Black => GoCell::Black,
        }
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

#[derive(Hash, Eq, Debug, PartialEq)]
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

#[derive(Debug, Clone, PartialEq, Copy, Hash, Eq)]
enum GoCell {
    White,
    WhitePending,
    Black,
    BlackPending,
    Empty,
}

#[derive(Debug)]
pub enum GoBoardError {
    IllegalMove,
    InvalidPlayer,
    NoPendingFound,
    WrongPlayerTurn,
}

impl TryInto<GoPlayer> for GoCell {
    type Error = GoBoardError;

    fn try_into(self) -> Result<GoPlayer, GoBoardError> {
        match self {
            GoCell::White => Ok(GoPlayer::White),
            GoCell::WhitePending => Ok(GoPlayer::White),
            GoCell::Black => Ok(GoPlayer::Black),
            GoCell::BlackPending => Ok(GoPlayer::Black),
            _ => Err(GoBoardError::InvalidPlayer),
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

#[derive(Debug, PartialEq, Eq)]
pub struct GoBoard {
    whos_turn: GoPlayer,
    last_move: LastMove,
    captures: HashMap<GoPlayer, u16>,
    last_captures: VecDeque<String>,
    board: Arr2d<GoCell>,
}

impl fmt::Display for GoBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "turn={}\nlast_move={}\ncapturesW={}\ncapturesB={}\n{}",
            self.whos_turn,
            self.last_move,
            self.captures.get(&GoPlayer::White).unwrap(),
            self.captures.get(&GoPlayer::Black).unwrap(),
            self.board
        )
    }
}

impl GoBoard {
    pub const WHITE: char = 'W';
    pub const WHITE_PENDING: char = 'w';
    pub const BLACK: char = 'B';
    pub const BLACK_PENDING: char = 'b';
    pub const EMPTY: char = '-';

    fn locate_pending(&self) -> Option<&Cell<GoCell>> {
        self.board
            .all_cells()
            .filter(|c| match c.value() {
                GoCell::WhitePending | GoCell::BlackPending => true,
                _ => false,
            })
            .next()
    }

    fn calculate_captures(
        &self,
        from: &Cell<GoCell>,
        opponent: GoPlayer,
    ) -> impl Iterator<Item = &Cell<GoCell>> {
        let opponent_cell: GoCell = opponent.into();
        let mut seen_ids = HashSet::new();
        self.board
            .get_neighbours(from.row(), from.column())
            .filter(move |neighbour| neighbour.value() == opponent_cell)
            .filter(|neighbour| {
                !self
                    .has_liberties(neighbour.row(), neighbour.column())
                    .unwrap()
            })
            .flat_map(|captured_neighbour| {
                self.board
                    .flood_fill(captured_neighbour.row(), captured_neighbour.column())
                    .unwrap()
            })
            .filter(move |c| seen_ids.insert(c.id()))
    }

    pub fn make_move(&mut self, row: usize, column: usize) -> Result<(), GoBoardError> {
        self.board.set(row, column, self.whos_turn.into_pending());
        self.iterate()
    }

    fn check_suicidal(
        &self,
        row: usize,
        column: usize,
        captures: &Vec<(usize, usize)>,
    ) -> Result<(), GoBoardError> {
        // Zero captures, and zero empty neighbours, indicates suicidal move
        match captures.len() {
            0 => match self
                .board
                .get_neighbours(row, column)
                .filter(|c| c.value() == GoCell::Empty)
                .count()
            {
                0 => Err(GoBoardError::IllegalMove),
                _ => Ok(()),
            },
            _ => Ok(()),
        }
    }

    fn check_ko(&mut self, captures: &Vec<(usize, usize)>) -> Result<(), GoBoardError> {
        if captures.len() == 0 {
            return Ok(());
        }

        let this_str = captures
            .iter()
            .map(|(a, b)| format!("{},{}", a, b))
            .collect::<Vec<String>>()
            .join("-");

        if self.last_captures.contains(&this_str) {
            return Err(GoBoardError::IllegalMove);
        }
        self.last_captures.push_back(this_str);

        while self.last_captures.len() > 2 {
            self.last_captures.pop_front();
        }

        Ok(())
    }

    pub fn iterate(&mut self) -> Result<(), GoBoardError> {
        let cell = match self.locate_pending() {
            Some(c) => c,
            None => return Err(GoBoardError::NoPendingFound),
        };
        let who: GoPlayer = match cell.value().try_into() {
            Ok(w) => w,
            Err(e) => return Err(e),
        };

        if who != self.whos_turn {
            return Err(GoBoardError::WrongPlayerTurn);
        }

        let row = cell.row();
        let column = cell.column();

        let opponent = who.other();
        let captures = self
            .calculate_captures(cell, opponent)
            .map(|c| (c.row(), c.column()))
            .collect::<Vec<_>>();

        if let Err(e) = self.check_ko(&captures) {
            self.last_move = LastMove::IllegalKo;
            self.board.set(row, column, GoCell::Empty);
            return Err(e);
        }

        if let Err(e) = self.check_suicidal(row, column, &captures) {
            self.last_move = LastMove::IllegalSuicidal;
            self.board.set(row, column, GoCell::Empty);
            return Err(e);
        }

        for (row, column) in captures {
            self.board.set(row, column, GoCell::Empty);
            self.captures.entry(who).and_modify(|e| *e += 1);
        }

        let played_cell: GoCell = who.into();
        self.board.set(row, column, played_cell);

        self.whos_turn = opponent;

        Ok(())
    }

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

        let mut captures: HashMap<_, _> = HashMap::new();
        captures.insert(GoPlayer::White, white_captures);
        captures.insert(GoPlayer::Black, black_captures);

        Ok(GoBoard {
            whos_turn,
            last_move,
            captures,
            last_captures: VecDeque::new(),
            board,
        })
    }

    fn has_liberties(&self, row: usize, column: usize) -> Result<bool, &str> {
        match self.get_liberties(row, column) {
            Ok(c) => Ok(c.count() > 0),
            Err(e) => Err(e),
        }
    }

    fn get_liberties(
        &self,
        row: usize,
        column: usize,
    ) -> Result<impl Iterator<Item = &Cell<GoCell>>, &str> {
        match self.board.get_perimeter(row, column) {
            Ok(p) => Ok(p.filter(|c| c.value() == GoCell::Empty)),
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

    fn test_file_raw_contents(basefile: &str) -> String {
        let filename = format!("resources/tests/go/{}", basefile);
        fs::read_to_string(&filename).expect(&format!(
            "Expected to find hardcoded test resource at {}",
            filename
        ))
    }

    fn create_go_from_test_file(basefile: &str) -> Result<GoBoard, ParseError> {
        let file_contents = test_file_raw_contents(basefile);
        GoBoard::from_str(&file_contents)
    }

    fn extract_coord<'a>(mut parts: impl Iterator<Item = &'a str>) -> Result<usize, ParseError> {
        match parts.next() {
            Some(e) => match e.trim().parse() {
                Ok(r) => Ok(r),
                _ => return Err(ParseError::InvalidValue),
            },
            None => return Err(ParseError::InvalidValue),
        }
    }

    fn create_move_from_test_file(basefile: &str) -> Result<(usize, usize), ParseError> {
        let file_contents = test_file_raw_contents(basefile);

        let mut parts = file_contents.split(",");

        let row = match extract_coord(&mut parts) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        let column = match extract_coord(&mut parts) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        Ok((row, column))
    }

    #[test]
    fn test_ko_rule() {
        let mut state = create_go_from_test_file("ko/simple_1/1_before.txt").unwrap();
        let state_1_execute = create_go_from_test_file("ko/simple_1/1_execute.txt").unwrap();
        let move_2 = create_move_from_test_file("ko/simple_1/2_move.txt").unwrap();
        let state_2_execute = create_go_from_test_file("ko/simple_1/2_execute.txt").unwrap();
        let move_3 = create_move_from_test_file("ko/simple_1/3_move.txt").unwrap();
        let state_3_execute = create_go_from_test_file("ko/simple_1/3_execute.txt").unwrap();
        let move_4 = create_move_from_test_file("ko/simple_1/4_move.txt").unwrap();
        let state_4_execute = create_go_from_test_file("ko/simple_1/4_execute.txt").unwrap();

        let _ = state.iterate().unwrap();
        assert_board_equal(&state_1_execute, &state);

        let _ = state.make_move(move_2.0, move_2.1).unwrap();
        assert_board_equal(&state_2_execute, &state);

        let _ = state.make_move(move_3.0, move_3.1).unwrap();
        assert_board_equal(&state_3_execute, &state);

        let result4 = state.make_move(move_4.0, move_4.1);

        assert!(result4.is_err(), "Expected move 4 to generate an error");
        assert_board_equal(&state_4_execute, &state);
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
                last_captures: VecDeque::new(),
                captures: [(GoPlayer::White, 16), (GoPlayer::Black, 23)]
                    .iter()
                    .cloned()
                    .collect(),
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
                last_captures: VecDeque::new(),
                captures: [(GoPlayer::White, 16), (GoPlayer::Black, 23)]
                    .iter()
                    .cloned()
                    .collect(),
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
    fn test_has_liberties_true() {
        // Given
        let as_str = r#"
    turn=W
last_move=ok
capturesW=16
capturesB=23
-W-W-
WBW--
WBBW-
WBBbW
WWWW-
        "#;
        let state = GoBoard::from_str(as_str).unwrap();

        // When
        let result = state.has_liberties(2, 1).unwrap();

        // Then
        assert!(!result);
    }

    #[test]
    fn test_has_liberties_false() {
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
        let result = state.has_liberties(2, 1).unwrap();

        // Then
        assert!(result);
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
    #[test_case("suicidal_move/simple_1")]
    #[test_case("suicidal_move/simple_2")]
    fn test_iterate(name: &str) {
        let file_before = format!("{}/1_before.txt", name);
        let file_execute = format!("{}/1_execute.txt", name);
        let mut state_before = create_go_from_test_file(&file_before).unwrap();
        let _ = state_before.iterate();
        let state_execute = create_go_from_test_file(&file_execute).unwrap();
        assert_board_equal(&state_execute, &state_before);
    }

    fn assert_board_equal(expected: &GoBoard, result: &GoBoard) {
        assert_eq!(
            expected.whos_turn, result.whos_turn,
            "Incorrect turn \n{expected}\n\n{result}"
        );
        assert_eq!(
            expected.captures, result.captures,
            "Incorrect captures \n{expected}\n\n{result}"
        );
        assert_eq!(
            expected.last_move, result.last_move,
            "Incorrect Last Move \n{expected}\n\n{result}"
        );
        assert_eq!(
            expected.board, result.board,
            "Board states do not match \n{expected}\n\n{result}"
        );
    }
}
