use crate::{ai, game};
use rand::Rng;

pub struct RandomAI<'a> {
    board: &'a game::Board,
}

impl<'a> RandomAI<'a> {
    pub fn new(board: &'a game::Board) -> Self {
        Self { board: board }
    }
}

impl<'a> ai::AI for RandomAI<'a> {
    fn choose_column(&self) -> usize {
        loop {
            let nb: usize = rand::thread_rng().gen_range(0..6);
            if self.board.is_column_full(nb).unwrap() {
                continue;
            }
            break nb;
        }
    }
}
