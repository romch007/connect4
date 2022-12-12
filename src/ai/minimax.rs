use std::f32::NEG_INFINITY;

use crate::{ai, game};

pub struct MinimaxAI {
    pub max_depth: u32,
}

impl MinimaxAI {
    pub fn new(max_depth: u32) -> Self {
        Self { max_depth }
    }

    fn minimax(
        &self,
        board: Box<game::Board>,
        depth: u32,
        mut alpha: f64,
        mut beta: f64,
        maximizing_player: bool,
    ) -> (usize, f64) {
        let possible_moves = board.possible_moves();
        let is_terminal = board.winner().is_some();

        if depth == 0 || is_terminal {
            (0, board.evaluate_board())
        } else if maximizing_player {
            let mut best_value: f64 = f64::NEG_INFINITY;
            let mut best_col: usize = 0; // FIXME: random value ?
            for col in possible_moves {
                let mut new_board = Box::new(*board.clone());
                new_board.play(col).unwrap();
                let new_score = self.minimax(new_board, depth - 1, alpha, beta, false).1;
                if new_score > best_value {
                    best_value = new_score;
                    best_col = col;
                }
                alpha = alpha.max(best_value);
                if alpha >= beta {
                    break;
                }
            }

            (best_col, best_value)
        } else {
            let mut best_value: f64 = f64::INFINITY;
            let mut best_col: usize = 0; // FIXME: random value ?
            for col in possible_moves {
                let mut new_board = Box::new(*board.clone());
                new_board.play(col).unwrap();
                let new_score = self.minimax(new_board, depth - 1, alpha, beta, true).1;
                if new_score < best_value {
                    best_value = new_score;
                    best_col = col;
                }
                beta = beta.max(best_value);
                if alpha >= beta {
                    break;
                }
            }

            (best_col, best_value)
        }
    }
}

impl ai::AI for MinimaxAI {
    fn choose_column(&self, board: &game::Board) -> usize {
        let (col, minimax_score) = self.minimax(
            Box::new(board.clone()),
            self.max_depth,
            f64::NEG_INFINITY,
            f64::INFINITY,
            true,
        );

        col
    }
}
