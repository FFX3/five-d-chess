use std::str::FromStr;
use std::result::Result;

use num_enum::TryFromPrimitive;


mod bit_board {
    use num_enum::TryFromPrimitive;
    use super::{
        Square, 
        Piece, 
        PieceType, 
        SimplePosition, 
        Player, 
        Occupant, 
        Move, 
        ToPlay, 
        File
    };

    impl Square {
        pub fn to_u64(&self) -> u64 {
            2u64.pow(self.to_owned() as u32)
        }
    }

    impl BitBoardPosition {
        pub fn play_move(mut self, tentative_move: (Square, Square)) -> Result<Self, Self> {
            let tentative_move_result = Move::from_bitboard(&self, tentative_move);

            if let Err(err) = tentative_move_result {
                println!("{}\n\n", err);
                return Err(self);
            }

            let detailed_move = tentative_move_result.unwrap();

            if let Err(err) = self.validate_move(&detailed_move) {
                println!("{}\n\n", err);
                return Err(self);
            }

            self.board[detailed_move.piece.piece_type as usize + (6 * self.to_play as usize)] += detailed_move.end.to_u64();
            self.board[detailed_move.piece.piece_type as usize + (6 * self.to_play as usize)] -= detailed_move.start.to_u64();

            for piece_type_determinant in 0..6 {
                let mut layer = self.board[piece_type_determinant + (6 * self.to_play.opponent() as usize)];

                layer = layer - (layer & detailed_move.end.to_u64());

                if (layer & detailed_move.end.to_u64()) != 0 {
                    println!("Piece captured!");
                }

                self.board[piece_type_determinant + (6 * self.to_play.opponent() as usize)] = layer;
            }

            self.to_play = self.to_play.opponent();

            Ok(self)
        }

        fn validate_move(&self, tentative_move: &Move) -> Result<(), String> {
            if self.to_play != tentative_move.piece.owner { return Err("Player doesn't own this piece".to_string()); }

            for piece_type_determinant in 0..6 {
                if (self.board[piece_type_determinant + (6 * self.to_play as usize)] & tentative_move.end.to_u64()) != 0 {
                    return Err("Player already has a piece occupying the end square".to_string());
                }
            }            

            match tentative_move.piece.piece_type {
                PieceType::Knight => {
                    let intersection = calculations::knight_attacks(tentative_move.start.to_u64()) & tentative_move.end.to_u64();
                    if intersection != 0 { return Ok(()); }
                    Err("Illegal knight move".to_string())
                },
                PieceType::Pawn => {
                    let intersection = calculations::pawn_attacks(tentative_move.start.to_u64(), tentative_move.piece.owner) & tentative_move.end.to_u64();
                    if intersection == 0 { return Err("Illegal pawn move".to_string()); }

                    for piece_type_determinant in 0..6 {
                        if (self.board[piece_type_determinant + (6 * self.to_play.opponent() as usize)] & tentative_move.end.to_u64()) != 0 {
                            return Ok(());
                        }
                    }            
                    Err("Diagonal pawn moves need to be a capture".to_string())
                }
                _ => Err("Not implemented".to_string())
            }
        }

        pub fn from_position(position: &SimplePosition) -> Self {
            const PIECE_TYPE_COUNT: usize = 6;

            let mut board: [u64; PIECE_TYPE_COUNT * 2] = [0; PIECE_TYPE_COUNT * 2];

            for (square_index, occupant) in position.board.iter().enumerate() {
                let piece_result = occupant.piece();
                if piece_result.is_none() { continue; }
                let piece = piece_result.unwrap();
                let bit_board_layer = match piece.owner {
                    Player::White => (piece.piece_type as usize) + (PIECE_TYPE_COUNT * Player::White as usize),
                    Player::Black => (piece.piece_type as usize) + (PIECE_TYPE_COUNT * Player::Black as usize),
                };
                board[bit_board_layer] += (2 as u64).pow(square_index as u32);
            }

            Self {
                to_play: position.to_play, 
                board,
            }
        }

        pub fn to_position(&self) -> SimplePosition {
            let mut board = [Occupant::None; 64];
            const PIECE_TYPE_COUNT: u8 = 6;

            for (i, layer) in self.board.iter().enumerate() {
                for square_index in 0..64 {
                    let intersection_check = (2 as u64).pow(square_index as u32);
                    if (layer & intersection_check) == intersection_check {
                        let player = if i < 6 { Player::White } else { Player::Black };
                        board[square_index] = Occupant::Piece(Piece {
                            owner: player,
                            piece_type: PieceType::try_from(i as u8 - (PIECE_TYPE_COUNT * player as u8)).unwrap()
                        })
                    }
                }
            }

            SimplePosition {
                to_play: self.to_play,
                board,
            }
        }

        #[allow(dead_code)]
        fn to_string(&self) -> String {
            const PIECE_TYPE_COUNT: u8 = 6;
            let mut result_string = String::new();
            for (i, layer) in self.board.iter().enumerate() {
                let player = if i < 6 { Player::White } else { Player::Black };
                result_string.push_str(&format!("{:?} {:?}\n\n", player, PieceType::try_from(i as u8 - (PIECE_TYPE_COUNT * player as u8)).unwrap()));

                result_string.push_str(&BitBoard(layer.to_owned()).to_string());
                result_string.push_str("\n\n");
            }
            result_string
        }
    }

    struct BitBoard(u64);

    impl BitBoard {
        fn to_string(&self) -> String {
            let mut result_string = String::new();
            for square_index in 0..64 {
                let intersection_check = (2 as u64).pow(square_index as u32);
                result_string.push_str(if (self.0 & intersection_check) == intersection_check { "1" } else { "." });
                if (square_index+1) % 8 == 0 {
                    result_string.push_str("\n");
                }
            }
            result_string
        }
    }

    #[derive(Debug)]
    pub struct BitBoardPosition {
        to_play: ToPlay,
        board: [u64; 12], //indexes with piece type enum
    }

    impl Move {
        fn from_bitboard(position: &BitBoardPosition, squares: (Square, Square)) -> Result<Self, &str> {

            let start_square = squares.0;
            let end_square = squares.1;

            for piece_type_determinant in 0..6 {
                if (position.board[piece_type_determinant + (6 * position.to_play as usize)] & start_square.to_u64()) != 0 {
                    return Ok(Move {
                        start: start_square,
                        end: end_square,
                        piece: Piece {
                            owner: position.to_play,
                            piece_type: PieceType::try_from_primitive(piece_type_determinant as u8).unwrap(),
                        }
                    })
                }
            }
            Err("Square is empty")
        }
    }

    impl File {
        fn to_u64(&self) -> u64 {
            (0..8).into_iter().fold(0, |acc, row_numer| -> u64 { acc + 2u64.pow((row_numer * 8) + self.to_owned() as u32) })
        }
    }


    mod calculations {
        use super::{ File, Player };

        pub fn knight_attacks(positions: u64) -> u64 {
            (positions << 17) & (u64::MAX ^ File::A.to_u64())
            | (positions << 10) & (u64::MAX ^ (File::A.to_u64() | File::B.to_u64()))
            | (positions >> 6) & (u64::MAX ^ (File::A.to_u64() | File::B.to_u64()))
            | (positions >> 15) & (u64::MAX ^ File::A.to_u64())
            | (positions << 15) & (u64::MAX ^ File::H.to_u64())
            | (positions << 6) & (u64::MAX ^ (File::H.to_u64() | File::G.to_u64()))
            | (positions >> 10) & (u64::MAX ^ (File::H.to_u64() | File::G.to_u64()))
            | (positions >> 17) & (u64::MAX ^ File::H.to_u64())
        }

        pub fn pawn_attacks(positions: u64, player: Player) -> u64 {
            if player == Player::White {
                return (positions << 7) & (u64::MAX ^ File::H.to_u64())
                | (positions << 9) & (u64::MAX ^ File::A.to_u64())
            } 
            (positions >> 9) & (u64::MAX ^ File::H.to_u64())
            | (positions >> 7) & (u64::MAX ^ File::A.to_u64())
        }
    }
}

fn main() {
    use std::env;
    use bit_board::BitBoardPosition;
    env::set_var("RUST_BACKTRACE", "1");
    let initial_position: SimplePosition = SimplePosition {
        to_play: Player::White,
        board: [
            [
                Occupant::Piece(Piece { piece_type: PieceType::Rook, owner: Player::White }),
                Occupant::Piece(Piece { piece_type: PieceType::Knight, owner: Player::White }),
                Occupant::Piece(Piece { piece_type: PieceType::Bishop, owner: Player::White }),
                Occupant::Piece(Piece { piece_type: PieceType::Queen, owner: Player::White }),
                Occupant::Piece(Piece { piece_type: PieceType::King, owner: Player::White }),
                Occupant::Piece(Piece { piece_type: PieceType::Bishop, owner: Player::White }),
                Occupant::Piece(Piece { piece_type: PieceType::Knight, owner: Player::White }),
                Occupant::Piece(Piece { piece_type: PieceType::Rook, owner: Player::White }),
            ],
            [Occupant::Piece(Piece { piece_type: PieceType::Pawn, owner: Player::White }); 8],
            [Occupant::None; 8],
            [Occupant::None; 8],
            [Occupant::None; 8],
            [Occupant::None; 8],
            [Occupant::Piece(Piece { piece_type: PieceType::Pawn, owner: Player::Black }); 8],
            [
                Occupant::Piece(Piece { piece_type: PieceType::Rook, owner: Player::Black }),
                Occupant::Piece(Piece { piece_type: PieceType::Knight, owner: Player::Black }),
                Occupant::Piece(Piece { piece_type: PieceType::Bishop, owner: Player::Black }),
                Occupant::Piece(Piece { piece_type: PieceType::Queen, owner: Player::Black }),
                Occupant::Piece(Piece { piece_type: PieceType::King, owner: Player::Black }),
                Occupant::Piece(Piece { piece_type: PieceType::Bishop, owner: Player::Black }),
                Occupant::Piece(Piece { piece_type: PieceType::Knight, owner: Player::Black }),
                Occupant::Piece(Piece { piece_type: PieceType::Rook, owner: Player::Black }),
            ]
        ].concat().try_into().expect("Expect Vec length of 64 for board array"),
    };

    let mut position = BitBoardPosition::from_position(&initial_position);
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

        let mut square_iter = input.split(","); 
        let start_square = Square::from_str(square_iter.next().unwrap()).expect("invalid start square");
        let end_square = Square::from_str(square_iter.next().unwrap()).expect("invalid end square");

        let move_instruction = (start_square, end_square);

        position = match position.play_move(move_instruction) {
            Ok(_position) => _position,
            Err(_position) => { 
                println!("Illegal Move");
                _position
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Square {
   A1,B1,C1,D1,E1,F1,G1,H1,
   A2,B2,C2,D2,E2,F2,G2,H2,
   A3,B3,C3,D3,E3,F3,G3,H3,
   A4,B4,C4,D4,E4,F4,G4,H4,
   A5,B5,C5,D5,E5,F5,G5,H5,
   A6,B6,C6,D6,E6,F6,G6,H6,
   A7,B7,C7,D7,E7,F7,G7,H7,
   A8,B8,C8,D8,E8,F8,G8,H8,

}


#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
#[allow(dead_code)]
enum File {
    A,B,C,D,E,F,G,H
}

impl FromStr for Square {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, &'static str> {
            match s {
                "A1" => Ok(Square::A1), "A2" => Ok(Square::A2), "A3" => Ok(Square::A3), "A4" => Ok(Square::A4), "A5" => Ok(Square::A5), "A6" => Ok(Square::A6), "A7" => Ok(Square::A7), "A8" => Ok(Square::A8),
                "B1" => Ok(Square::B1), "B2" => Ok(Square::B2), "B3" => Ok(Square::B3), "B4" => Ok(Square::B4), "B5" => Ok(Square::B5), "B6" => Ok(Square::B6), "B7" => Ok(Square::B7), "B8" => Ok(Square::B8),
                "C1" => Ok(Square::C1), "C2" => Ok(Square::C2), "C3" => Ok(Square::C3), "C4" => Ok(Square::C4), "C5" => Ok(Square::C5), "C6" => Ok(Square::C6), "C7" => Ok(Square::C7), "C8" => Ok(Square::C8),
                "D1" => Ok(Square::D1), "D2" => Ok(Square::D2), "D3" => Ok(Square::D3), "D4" => Ok(Square::D4), "D5" => Ok(Square::D5), "D6" => Ok(Square::D6), "D7" => Ok(Square::D7), "D8" => Ok(Square::D8),
                "E1" => Ok(Square::E1), "E2" => Ok(Square::E2), "E3" => Ok(Square::E3), "E4" => Ok(Square::E4), "E5" => Ok(Square::E5), "E6" => Ok(Square::E6), "E7" => Ok(Square::E7), "E8" => Ok(Square::E8),
                "F1" => Ok(Square::F1), "F2" => Ok(Square::F2), "F3" => Ok(Square::F3), "F4" => Ok(Square::F4), "F5" => Ok(Square::F5), "F6" => Ok(Square::F6), "F7" => Ok(Square::F7), "F8" => Ok(Square::F8),
                "G1" => Ok(Square::G1), "G2" => Ok(Square::G2), "G3" => Ok(Square::G3), "G4" => Ok(Square::G4), "G5" => Ok(Square::G5), "G6" => Ok(Square::G6), "G7" => Ok(Square::G7), "G8" => Ok(Square::G8),
                "H1" => Ok(Square::H1), "H2" => Ok(Square::H2), "H3" => Ok(Square::H3), "H4" => Ok(Square::H4), "H5" => Ok(Square::H5), "H6" => Ok(Square::H6), "H7" => Ok(Square::H7), "H8" => Ok(Square::H8),
                _ => Err("Invalid square name")
            }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl PieceType {

}


#[derive(Clone, Debug, Copy)]
struct Piece {
    piece_type: PieceType,
    owner: Player,
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        if self.owner == Player::White {
            return match self.piece_type {
                PieceType::Pawn => "♟",
                PieceType::Rook => "♜",
                PieceType::Knight => "♞",
                PieceType::Bishop => "♝",
                PieceType::Queen => "♛",
                PieceType::King => "♚",
            }.to_string()
        }

        match self.piece_type {
            PieceType::Pawn => "♙",
            PieceType::Rook => "♖",
            PieceType::Knight => "♘",
            PieceType::Bishop => "♗",
            PieceType::Queen => "♕",
            PieceType::King => "♔",
        }.to_string()

    }
}

#[derive(Clone, Debug, Copy)]
enum Occupant {
    None,
    Piece(Piece),
}

impl Occupant {
    fn piece(self) -> Option<Piece> {
        match self {
            Self::Piece(piece) => Some(piece),
            Self::None => None,
        }
    }
}


#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Player {
    White,
    Black,
}

impl Player {
    fn opponent(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

type ToPlay = Player;

#[derive(Debug)]
pub struct SimplePosition {
    board: [Occupant; 64],
    to_play: ToPlay,
}

impl ToString for SimplePosition {
    fn to_string(&self) -> String {
        let mut result = String::new();
        let mut i = 0;
        let squares = self.board;
        while i < squares.len() {
            result.push_str(
                &match squares[i] {
                    Occupant::None => "-".to_owned(),
                    Occupant::Piece(piece) => piece.to_string(),
                }
            );
            i = i + 1;
            if i % 8 == 0 {
                result.push_str("\n");
            }
        }
        result.push_str("\n\n");
        result.push_str(match self.to_play { Player::White => "White", Player::Black => "Black" });
        result.push_str(" to play");
        result.push_str("\n\n");
        result
    }
}

struct Move {
    start: Square,
    end: Square,
    piece: Piece,
}
