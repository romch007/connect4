use ai::AI;
use std::io::{self, Write};

mod ai;
mod game;
mod utils;

fn ask_player() -> Option<usize> {
    print!("Column: ");

    let mut input = String::new();

    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<usize>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}

fn main() {
    let mut board = game::Board::new();
    let best_ai = ai::random::RandomAI::new();
    let mut player_turn = true;
    loop {
        let column: Option<usize> = match player_turn {
            true => {
                player_turn = false;
                ask_player()
            }
            false => {
                player_turn = true;
                Some(best_ai.choose_column(&board))
            }
        };

        let column = match column {
            Some(value) => value,
            None => continue,
        };

        match board.play(column) {
            Ok(_) => (),
            Err(_) => continue,
        }
        println!("{}", board);

        match board.winner() {
            Some(winner) => {
                println!("Winner is {}", winner);
                break;
            }
            None => (),
        };

        if board.is_full() {
            println!("It's a draw!");
            break;
        }

        board.switch_player();
    }
}
