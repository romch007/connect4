use crate::{ai, game};
use rand::Rng;

pub struct RandomAI {}

impl RandomAI {
    pub fn new() -> Self {
        Self {}
    }
}

impl ai::AI for RandomAI {
    fn choose_column(&self, board: &game::Board) -> usize {
        loop {
            let nb: usize = rand::thread_rng().gen_range(0..6);
            if board.is_column_full(nb).unwrap() {
                continue;
            }
            break nb;
        }
    }
}
