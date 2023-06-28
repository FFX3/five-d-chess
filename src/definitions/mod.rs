use std::str::FromStr;
use std::result::Result;

use num_enum::TryFromPrimitive;

pub static INITIAL_POSITION: SimplePosition = SimplePosition {
    to_play: Player::White,
    board: [
        Occupant::Piece(Piece { piece_type: PieceType::Rook, owner: Player::White }),
        Occupant::Piece(Piece { piece_type: PieceType::Knight, owner: Player::White }),
        Occupant::Piece(Piece { piece_type: PieceType::Bishop, owner: Player::White }),
        Occupant::Piece(Piece { piece_type: PieceType::Queen, owner: Player::White }),
        Occupant::Piece(Piece { piece_type: PieceType::King, owner: Player::White }),
        Occupant::Piece(Piece { piece_type: PieceType::Bishop, owner: Player::White }),
        Occupant::Piece(Piece { piece_type: PieceType::Knight, owner: Player::White }),
        Occupant::Piece(Piece { piece_type: PieceType::Rook, owner: Player::White }),

        Occupant::Piece(Piece { piece_type: PieceType::Pawn, owner: Player::White }),
        Occupant::Piece(Piece { piece_type: PieceType::Pawn, owner: Player::White }),
        Occupant::Piece(Piece { piece_type: PieceType::Pawn, owner: Player::White }),
        Occupant::Piece(Piece { piece_type: PieceType::Pawn, owner: Player::White }),
        Occupant::Piece(Piece { piece_type: PieceType::Pawn, owner: Player::White }),
        Occupant::Piece(Piece { piece_type: PieceType::Pawn, owner: Player::White }),
        Occupant::Piece(Piece { piece_type: PieceType::Pawn, owner: Player::White }),
        Occupant::Piece(Piece { piece_type: PieceType::Pawn, owner: Player::White }),

        Occupant::None,Occupant::None,Occupant::None,Occupant::None,
        Occupant::None,Occupant::None,Occupant::None,Occupant::None,
        Occupant::None,Occupant::None,Occupant::None,Occupant::None,
        Occupant::None,Occupant::None,Occupant::None,Occupant::None,
        Occupant::None,Occupant::None,Occupant::None,Occupant::None,
        Occupant::None,Occupant::None,Occupant::None,Occupant::None,
        Occupant::None,Occupant::None,Occupant::None,Occupant::None,
        Occupant::None,Occupant::None,Occupant::None,Occupant::None,

        Occupant::Piece(Piece { piece_type: PieceType::Pawn, owner: Player::Black }),
        Occupant::Piece(Piece { piece_type: PieceType::Pawn, owner: Player::Black }),
        Occupant::Piece(Piece { piece_type: PieceType::Pawn, owner: Player::Black }),
        Occupant::Piece(Piece { piece_type: PieceType::Pawn, owner: Player::Black }),
        Occupant::Piece(Piece { piece_type: PieceType::Pawn, owner: Player::Black }),
        Occupant::Piece(Piece { piece_type: PieceType::Pawn, owner: Player::Black }),
        Occupant::Piece(Piece { piece_type: PieceType::Pawn, owner: Player::Black }),
        Occupant::Piece(Piece { piece_type: PieceType::Pawn, owner: Player::Black }),

        Occupant::Piece(Piece { piece_type: PieceType::Rook, owner: Player::Black }),
        Occupant::Piece(Piece { piece_type: PieceType::Knight, owner: Player::Black }),
        Occupant::Piece(Piece { piece_type: PieceType::Bishop, owner: Player::Black }),
        Occupant::Piece(Piece { piece_type: PieceType::Queen, owner: Player::Black }),
        Occupant::Piece(Piece { piece_type: PieceType::King, owner: Player::Black }),
        Occupant::Piece(Piece { piece_type: PieceType::Bishop, owner: Player::Black }),
        Occupant::Piece(Piece { piece_type: PieceType::Knight, owner: Player::Black }),
        Occupant::Piece(Piece { piece_type: PieceType::Rook, owner: Player::Black }),
        ]
};

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
   Invalid,
}


#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum File {
    A,B,C,D,E,F,G,H
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eight,
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
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}


#[derive(Clone, Debug, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub owner: Player,
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
pub enum Occupant {
    None,
    Piece(Piece),
}

impl Occupant {
    pub fn piece(self) -> Option<Piece> {
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
    pub fn opponent(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(Debug)]
pub struct SimplePosition {
    pub board: [Occupant; 64],
    pub to_play: Player,
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

pub struct Move {
    pub start: Square,
    pub end: Square,
    pub piece: Piece,
}
