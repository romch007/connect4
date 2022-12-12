use crate::{ai, game};

pub struct MinimaxAI {}

impl MinimaxAI {
    pub fn new() -> Self {
        Self {}
    }

    fn heuristic(&self, board: &game::Board) -> i64 {
        match board.winner() {
            Some(winner) => {
                if winner == current_player {
                    1000
                } else {
                    -1000
                }
            }
            None => 0,
        }
    }

    fn minimax(&self, board: &game::Board, depth: u32) -> (i64, usize) {}
}

impl ai::AI for MinimaxAI {
    fn choose_column(&self, board: &game::Board) -> usize {
        0
    }
}
