mod bit_board;
mod definitions;

use std::str::FromStr;

use bit_board::BitBoardPosition;
use definitions::{
    INITIAL_POSITION,
    SimplePosition,
    Square,
    Player,
    Piece,
    PieceType,
    Occupant,
};

fn main() {
    use std::env;
    env::set_var("RUST_BACKTRACE", "1");

    let mut position = BitBoardPosition::from_position(&INITIAL_POSITION);
    use std::io::{stdin, stdout, Write};
    loop {
        println!("\n\n{}", position.to_position().to_string());


        let mut input = String::new();
        let _=stdout().flush();
        stdin().read_line(&mut input).expect("Error on move entry");
        if let Some('\n')=input.chars().next_back() {
            input.pop();
        }
        if let Some('\r')=input.chars().next_back() {
            input.pop();
        }

        let mut move_instruction: (Square, Square) = (Square::Invalid, Square::Invalid);

        let mut square_iter = input.split(","); 

        if let Some(square_string) = square_iter.next() {
            if let Ok(square) = Square::from_str(square_string) {
                move_instruction.0 = square;
            }
        } else {
            println!("invalid input");
        }

        if let Some(square_string) = square_iter.next() {
            if let Ok(square) = Square::from_str(square_string) {
                move_instruction.1 = square;
            }
        } else {
            println!("invalid input");
        }
    

        if move_instruction.0 == Square::Invalid || move_instruction.1 == Square::Invalid {
            println!("Invalid square");
            continue;
        }

        position = match position.play_move(move_instruction) {
            Ok(_position) => _position,
            Err(_position) => { 
                println!("Illegal Move");
                _position
            }
        }
    }
}
