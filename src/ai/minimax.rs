use crate::{ai, game};

pub struct MinimaxAI<'a> {
    board: &'a game::Board,
}

impl<'a> MinimaxAI<'a> {
    pub fn new(board: &'a game::Board) -> Self {
        Self { board: board }
    }
}

impl<'a> ai::AI for MinimaxAI<'a> {
    fn choose_column(&self) -> usize {
        return 0;
    }
}
