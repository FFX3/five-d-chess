mod bit_board;
mod definitions;

use std::str::FromStr;

use bit_board::{BitBoardPosition, calculations::precalculations::PreComputedAttackSets};
use bit_board::calculations::precalculations;
use definitions::{
    INITIAL_POSITION,
    SimplePosition,
    Square,
    Player,
    Piece,
    PieceType,
    Occupant,
};
use std::io::{stdin, stdout, Write};

fn main() {
    use std::env;
    env::set_var("RUST_BACKTRACE", "1");

    start_game();
}

fn start_game() {
    let attack_sets = precalculations::build_piece_attack_set();
    let mut position = BitBoardPosition::from_position(&INITIAL_POSITION);

    loop {
        if position.promotion_square == Square::Invalid {
            position = handle_move(position, &attack_sets);
        } else {
            position = handle_promotion(position);
        }
    }
}

fn handle_promotion(position: BitBoardPosition) -> BitBoardPosition {
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

    let piece_type = match input.as_str() {
        "rook" => Some(PieceType::Rook),
        "queen" => Some(PieceType::Queen),
        "knight" => Some(PieceType::Knight),
        "bishop" => Some(PieceType::Bishop),
        _ => None
    };

    if piece_type.is_none() {
        return position
    }

     position.promote(piece_type.unwrap()).unwrap()
}

fn handle_move(position: BitBoardPosition, attack_sets: &PreComputedAttackSets) -> BitBoardPosition {
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
        return position;
    }

    match position.play_move(move_instruction, &attack_sets) {
        Ok(_position) => _position,
        Err(_position) => { 
            println!("Illegal Move");
            _position
        }
    }
}
