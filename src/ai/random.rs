use crate::ai;
use rand::Rng;

pub struct RandomAI {}

impl ai::AI for RandomAI {
    fn choose_column(&self) -> usize {
        rand::thread_rng().gen_range(0..6)
    }
}
