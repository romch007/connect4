use crate::game;

pub mod minimax;
pub mod random;

pub trait AI {
    fn choose_column(&self, board: &game::Board) -> usize;
}
