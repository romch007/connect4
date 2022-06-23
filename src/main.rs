use ai::AI;
use std::io::{self, Write};

mod ai;
mod game;
mod utils;

fn main() {
    let mut board = game::Board::new();
    let best_ai = ai::minimax::MinimaxAI::new(&board);
    let mut player_turn = true;
    loop {
        let column: Option<usize> = match player_turn {
            true => {
                print!("Column: ");

                let mut input = String::new();

                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                player_turn = false;

                match input.trim().parse::<usize>() {
                    Ok(num) => Some(num),
                    Err(_) => None,
                }
            }
            false => {
                player_turn = true;
                Some(best_ai.choose_column())
            }
        };

        let column = match column {
            Some(value) => value,
            None => continue,
        };

        board.play(column).unwrap();
        println!("{}", board);

        match board.winner() {
            Some(winner) => {
                println!("Winner is {}", winner);
                break;
            }
            None => (),
        };
        board.switch_player();
    }
}
