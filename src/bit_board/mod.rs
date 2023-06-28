use num_enum::TryFromPrimitive;
use self::calculations::intercect_with_player_pieces;

use super::{
    Square, 
    Piece, 
    PieceType, 
    SimplePosition, 
    Player, 
    Occupant, 
    definitions:: {
        Move, 
        File,
        Rank,
    }
};

impl Square {
    fn to_u64(&self) -> u64 {
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
                let start = tentative_move.start.to_u64();
                let end = tentative_move.end.to_u64();
                let owner = tentative_move.piece.owner;

                if (calculations::pawn_attacks(start, owner) & end) != 0 {
                    if intercect_with_player_pieces(tentative_move.end.to_u64(), self, self.to_play.opponent()) {
                        return Ok(());
                    }
                    return Err("Diagonal pawn moves need to be a capture".to_string())
                }
                
                if (calculations::pawn_moves(start, owner) & end) != 0 {
                    if !intercect_with_player_pieces(tentative_move.end.to_u64(), self, self.to_play.opponent()) 
                        && !intercect_with_player_pieces(tentative_move.end.to_u64(), self, self.to_play) {
                        return Ok(());
                    }
                    return Err("Pawn cannot attack forward".to_string())
                }

                Err("Illegal pawn move".to_string())
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
    to_play: Player,
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
impl Rank {
    fn to_u64(&self) -> u64 {
        let descriminant = self.to_owned() as u32;
        (0..8).into_iter().fold(0, |acc, colum_number| -> u64 { acc + 2u64.pow((colum_number) + descriminant) }) << (descriminant * 7) //TODO: why 7? it works?
    }
}


mod calculations {
    use crate::definitions::Rank;

    use super::{ File, Player, BitBoardPosition };

    pub fn intercect_with_player_pieces(map: u64, position: &BitBoardPosition, player: Player) -> bool {
        for piece_type_determinant in 0..6 {
            if (position.board[piece_type_determinant + (6 * player as usize)] & map) != 0 {
                return true;
            }
        }            
        false
    }

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

    pub fn pawn_moves(positions: u64, player: Player) -> u64 {
        let unmoved_pawns: u64;
        let no_last_rank: u64;

        if player == Player::White {
            no_last_rank = positions & ( u64::MAX ^ Rank::Eight as u64 );
            unmoved_pawns = no_last_rank & Rank::Second.to_u64();
            return (unmoved_pawns << 16 ) | (no_last_rank << 8);
        }
        no_last_rank = positions & ( u64::MAX ^ Rank::First as u64 );
        print!("{}\n",super::BitBoard(Rank::First.to_u64()).to_string());
        print!("{}\n",super::BitBoard(Rank::Second.to_u64()).to_string());
        print!("{}\n",super::BitBoard(Rank::Seventh.to_u64()).to_string());
        unmoved_pawns = no_last_rank & Rank::Seventh.to_u64();
        (unmoved_pawns >> 16 ) | (no_last_rank >> 8)
    }
}
