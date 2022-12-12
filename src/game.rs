use rand::Rng;

use crate::utils;
use std::cmp::Ordering;
use std::fmt;
use std::iter::zip;

/// Different players
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Player {
    Red,
    Yellow,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Player::Red => String::from("R"),
            Player::Yellow => String::from("Y"),
        };
        return write!(f, "{}", repr);
    }
}

const WIDTH: usize = 7;
const HEIGHT: usize = 6;
const WINNER_SCORE_EVALUATION: f64 = 100000.0;

/// Connect4 board
#[derive(Clone)]
pub struct Board {
    pub current_player: Player,
    pub grid: [[Option<Player>; WIDTH]; HEIGHT],
    pub last_position_played: (usize, usize),
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.grid.iter() {
            for cell in row.iter() {
                let repr = match cell {
                    Some(value) => value.to_string(),
                    None => String::from("."),
                };
                write!(f, "{}", repr)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            current_player: Player::Yellow,
            grid: [[None; WIDTH]; HEIGHT],
            last_position_played: (0, 0),
        }
    }

    /// Switch turn and pass to next player
    pub fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        }
    }

    /// Check if a specific column is full
    pub fn is_column_full(&self, column: usize) -> Result<bool, &'static str> {
        let column = match column.cmp(&WIDTH) {
            Ordering::Equal | Ordering::Greater => return Err("Column out of board"),
            Ordering::Less => column,
        };
        Ok(self.grid.iter().all(|row| match row[column] {
            None => false,
            Some(_) => true,
        }))
    }

    /// Check if the whole board is full
    pub fn is_full(&self) -> bool {
        (0..WIDTH).all(|i| self.is_column_full(i).unwrap())
    }

    /// Get possible moves (columns)
    pub fn possible_moves(&self) -> Vec<usize> {
        let mut possibles_moves = Vec::new();

        for col in 0..WIDTH {
            if !self.is_column_full(col).unwrap() {
                possibles_moves.push(col);
            }
        }

        possibles_moves
    }

    pub fn play(&mut self, column: usize) -> Result<(), &'static str> {
        if self.is_column_full(column)? {
            return Err("Column is full");
        }

        let mut current_row = 0;
        let free_row: usize = loop {
            if current_row >= HEIGHT {
                break current_row - 1;
            }
            match self.grid[current_row][column] {
                Some(_) => break current_row - 1,
                None => (),
            };
            current_row += 1;
        };

        self.grid[free_row][column] = Some(self.current_player);
        self.last_position_played = (free_row, column);

        Ok(())
    }

    fn get_winner_from_segment(&self, segment: &[(usize, usize); 4]) -> Option<Player> {
        let cells: Vec<Option<Player>> = segment.iter().map(|(y, x)| self.grid[*y][*x]).collect();
        if utils::is_all_same(&cells) {
            cells[0]
        } else {
            None
        }
    }

    fn start_forward_slash(
        max_row: usize,
        min_col: usize,
        mut row: usize,
        mut col: usize,
    ) -> (usize, usize) {
        while row < max_row && col > min_col {
            row += 1;
            col -= 1;
        }
        (row, col)
    }

    fn start_backward_slash(
        max_row: usize,
        max_col: usize,
        mut row: usize,
        mut col: usize,
    ) -> (usize, usize) {
        while row < max_row && col < max_col {
            row += 1;
            col += 1;
        }
        (row, col)
    }

    pub fn winner(&self) -> Option<Player> {
        let (focal_row, focal_col) = self.last_position_played;
        let min = |num: usize| -> usize {
            let tmp = i8::try_from(num).unwrap();
            let result = std::cmp::max(tmp - 3, 0);
            return usize::try_from(result).unwrap();
        };
        let max = |num: usize, max: usize| std::cmp::min(num + 3, max);

        let min_col = min(focal_col);
        let max_col = max(focal_col, WIDTH - 1);
        let min_row = min(focal_row);
        let max_row = max(focal_row, HEIGHT - 1);

        // Rows
        let row = focal_row;

        for col in min_col..=(max_col - 3) {
            let segment = [(row, col), (row, col + 1), (row, col + 2), (row, col + 3)];
            match self.get_winner_from_segment(&segment) {
                Some(value) => return Some(value),
                None => (),
            }
        }

        // Columns
        let col = focal_col;

        for row in min_row..=(max_row - 3) {
            let segment = [(row, col), (row + 1, col), (row + 2, col), (row + 3, col)];
            match self.get_winner_from_segment(&segment) {
                Some(value) => return Some(value),
                None => (),
            }
        }

        // Forward slash

        let (row, col) = Self::start_forward_slash(max_row, min_col, focal_row, focal_col);
        let row_range = ((min_row + 3)..=row).rev();
        let col_range = col..=(max_col - 3);

        for (row, col) in zip(row_range, col_range) {
            let segment = [
                (row, col),
                (row - 1, col + 1),
                (row - 2, col + 2),
                (row - 3, col + 3),
            ];
            match self.get_winner_from_segment(&segment) {
                Some(value) => return Some(value),
                None => (),
            }
        }

        // Backward slash
        let (row, col) = Self::start_backward_slash(max_row, max_col, focal_row, focal_col);
        let row_range = ((min_row + 3)..=row).rev();
        let col_range = ((min_col + 3)..=col).rev();

        for (row, col) in zip(row_range, col_range) {
            let segment = [
                (row, col),
                (row - 1, col - 1),
                (row - 2, col - 2),
                (row - 3, col - 3),
            ];
            match self.get_winner_from_segment(&segment) {
                Some(value) => return Some(value),
                None => (),
            }
        }

        None
    }

    /// Static evaluation of the board
    pub fn evaluate_board(&self) -> f64 {
        match self.winner() {
            Some(winner) => {
                WINNER_SCORE_EVALUATION
                    * match winner {
                        Player::Red => 1.0,
                        Player::Yellow => -1.0,
                    }
            }
            None => {
                // TODO: static board evaluation
                let mut rng = rand::thread_rng();
                rng.gen()
            }
        }
    }
}
