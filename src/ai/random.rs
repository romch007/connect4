use crate::ai;
use rand::Rng;

pub struct RandomAI {}

impl RandomAI {
    pub fn new() -> Self {
        Self {}
    }
}

impl ai::AI for RandomAI {
    fn choose_column(&self) -> usize {
        rand::thread_rng().gen_range(0..6)
    }
}
