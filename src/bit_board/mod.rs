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
    pub fn to_u64(&self) -> u64 {
        2u64.pow(self.to_owned() as u32)
    }

    pub fn from_u64(d: u64) -> Square {
        match Square::try_from_primitive((d as f64).log2() as u8) {
            Ok(x) => x,
            _ => Square::Invalid,
        }
    }
}

#[derive(Debug)]
pub struct BitBoardPosition {
    to_play: Player,
    board: [u64; 12], //index with piece type enum
}


impl BitBoardPosition {
    pub fn play_move(mut self, tentative_move: (Square, Square), attack_sets: &calculations::precalculations::PreComputedAttackSets) -> Result<Self, Self> {
        let tentative_move_result = Move::from_bitboard(&self, tentative_move);

        if let Err(err) = tentative_move_result {
            println!("{}\n\n", err);
            return Err(self);
        }

        let detailed_move = tentative_move_result.unwrap();

        if let Err(err) = self.validate_move(&detailed_move, attack_sets) {
            println!("{}\n\n", err);
            return Err(self);
        }

        let mut new_board = self.board.clone();

        new_board[detailed_move.piece.piece_type as usize + (6 * self.to_play as usize)] += detailed_move.end.to_u64();
        new_board[detailed_move.piece.piece_type as usize + (6 * self.to_play as usize)] -= detailed_move.start.to_u64();

        for piece_type_determinant in 0..6 {
            let mut layer = new_board[piece_type_determinant + (6 * self.to_play.opponent() as usize)];

            layer = layer - (layer & detailed_move.end.to_u64());

            if (layer & detailed_move.end.to_u64()) != 0 {
                println!("Piece captured!");
            }

            new_board[piece_type_determinant + (6 * self.to_play.opponent() as usize)] = layer;
        }

        if calculations::is_king_in_check(new_board, self.to_play, attack_sets) {
            println!("King can't be in check");
            return Err(self)
        }

        self.board = new_board;
        self.to_play = self.to_play.opponent();

        Ok(self)
    }

    fn validate_move(&self, tentative_move: &Move, attack_sets: &calculations::precalculations::PreComputedAttackSets) -> Result<(), String> {
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
            },
            PieceType::Rook => {
                let start = tentative_move.start;
                let end = tentative_move.end;

                let mut possible_moves = 0;

                for direction_index in 0..4 {
                    for next_square_index in 0..7 {
                        let next_square = &attack_sets.orthogonals[start as usize][direction_index][next_square_index];
                        if next_square == &Square::Invalid { break; }

                        if intercect_with_player_pieces(next_square.to_u64(), self, self.to_play) { break; }

                        possible_moves = possible_moves | next_square.to_u64();

                        if intercect_with_player_pieces(next_square.to_u64(), self, self.to_play.opponent()) { break; }
                    }
                }

                if possible_moves & end.to_u64() != 0 {
                    Ok(())
                } else {
                    Err("Illegal rook move".to_string())
                }
            },
            PieceType::Bishop => {
                let start = tentative_move.start;
                let end = tentative_move.end;

                let mut possible_moves = 0;

                for direction_index in 0..4 {
                    for next_square_index in 0..7 {
                        let next_square = &attack_sets.diagonals[start as usize][direction_index][next_square_index];
                        if next_square == &Square::Invalid { break; }

                        if intercect_with_player_pieces(next_square.to_u64(), self, self.to_play) { break; }

                        possible_moves = possible_moves | next_square.to_u64();

                        if intercect_with_player_pieces(next_square.to_u64(), self, self.to_play.opponent()) { break; }
                    }
                }

                if possible_moves & end.to_u64() != 0 {
                    return Ok(())
                }  
                Err("Illegal bishop move".to_string())
            },
            PieceType::Queen => {
                let mut rook_move = tentative_move.clone();
                let mut bishop_move = tentative_move.clone();
                rook_move.piece.piece_type = PieceType::Rook;
                bishop_move.piece.piece_type = PieceType::Bishop;
                if self.validate_move(&rook_move, attack_sets).is_ok() 
                    || self.validate_move(&bishop_move, attack_sets).is_ok() {
                    return Ok(())
                }
                Err("Illegal queen move".to_string())
            },
            PieceType::King => {
                let intersection = calculations::king_moves(tentative_move.start.to_u64()) & tentative_move.end.to_u64();
                if intersection != 0 { return Ok(()); }
                Err("Illegal king move".to_string())
            }
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

pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn to_string(&self) -> String {
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
        (0..8).into_iter().fold(0, |acc, colum_number| -> u64 { acc + 2u64.pow((colum_number) + descriminant) }) << (descriminant * 7)
    }
}


pub mod calculations {

    use crate::definitions::{Rank, PieceType, Square};

    use super::{ File, Player, BitBoardPosition };

    pub fn is_king_in_check(board: [u64; 12], player: Player, attack_sets: &precalculations::PreComputedAttackSets) -> bool {
        let king_position = board[PieceType::King as usize + (6 * player as usize)];

        if knight_attacks(king_position) & board[PieceType::Knight as usize + (6 * player.opponent() as usize)] != 0 {
            return true
        } 

        if king_moves(king_position) & board[PieceType::King as usize + (6 * player.opponent() as usize)] != 0 {
            return true
        } 

        if pawn_attacks(king_position, player) & board[PieceType::Pawn as usize + (6 * player.opponent() as usize)] != 0 {
            return true
        } 

        for direction_index in 0..4 {
            for next_square_index in (0..7).rev() {
                let next_square = &attack_sets.orthogonals[Square::from_u64(king_position) as usize][direction_index][next_square_index];

                if next_square == &Square::Invalid { continue; }

                if next_square.to_u64() & (board[PieceType::Rook as usize + (6 * player.opponent() as usize)] 
                | board[PieceType::Queen as usize + (6 * player.opponent() as usize)]) !=0 {
                    return true;
                }
                break;
            }
        }

        for direction_index in 0..4 {
            for next_square_index in (0..7).rev() {
                let next_square = &attack_sets.diagonals[Square::from_u64(king_position) as usize][direction_index][next_square_index];

                if next_square == &Square::Invalid { continue; }

                if next_square.to_u64() & (board[PieceType::Bishop as usize + (6 * player.opponent() as usize)] 
                | board[PieceType::Queen as usize + (6 * player.opponent() as usize)]) !=0 {
                    return true;
                }
                break;
            }
        }

        false
    }

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

    pub fn king_moves(positions: u64) -> u64 {
        (positions << 7)
        | (positions << 8)
        | (positions << 9)
        | (positions << 1)
        | (positions >> 7)
        | (positions >> 8)
        | (positions >> 9)
        | (positions >> 1)
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
        unmoved_pawns = no_last_rank & Rank::Seventh.to_u64();
        (unmoved_pawns >> 16 ) | (no_last_rank >> 8)
    }

    pub mod precalculations {

        use num_enum::TryFromPrimitive;

        use crate::{bit_board::{ PieceType, Rank, File, BitBoard }, definitions::Square};

        pub struct PreComputedAttackSets {
            pub attacks: [[u64; 64]; 3],
            pub blockers: [[u64; 64]; 3],
            pub diagonals: [[[Square; 7]; 4]; 64],
            pub orthogonals: [[[Square; 7]; 4]; 64],
        }

        impl PreComputedAttackSets {
            pub fn attacks(&self, square: Square, piece: PieceType) -> u64 {
                self.attacks[piece.precalculated_index()][square as usize]
            }

            pub fn blockers(&self, square: Square, piece: PieceType) -> u64 {
                self.blockers[piece.precalculated_index()][square as usize]
            }
        }

        impl PieceType {
            fn precalculated_index(&self) -> usize {
                match &self {
                    PieceType::Rook => 0,
                    PieceType::Bishop => 1,
                    PieceType::Queen => 2,
                    _ => panic!("These indexes don't exist")
                }
            }
        }

        pub fn build_piece_attack_set() -> PreComputedAttackSets {
            let mut attacks = [[0 as u64; 64]; 3];
            let mut blockers = [[0 as u64; 64]; 3];
            let mut diagonals = [[[Square::Invalid; 7]; 4]; 64];
            let mut orthogonals = [[[Square::Invalid; 7]; 4]; 64];
            
            {
                for square_index in 0..64 {
                    let current_square = Square::try_from_primitive(square_index as u8).unwrap();
                    let mut attack_map = 0;

                    {
                        let edges = Rank::Eight.to_u64();
                        if current_square.to_u64() & edges == 0 {
                            let mut new_square = current_square.to_u64();
                            let mut count = 0;
                            while new_square & edges == 0 {
                                new_square = new_square << 8;
                                orthogonals[square_index][0][count] = Square::from_u64(new_square);
                                count += 1;
                                attack_map = attack_map | new_square;
                            }
                        }
                    }


                    {
                        let edges = File::H.to_u64();
                        if current_square.to_u64() & edges == 0 {
                            let mut new_square = current_square.to_u64();
                            let mut count = 0;
                            while new_square & edges == 0 {
                                new_square = new_square << 1;
                                orthogonals[square_index][1][count] = Square::from_u64(new_square);
                                count += 1;
                                attack_map = attack_map | new_square;
                            }
                        }
                    }

                    {
                        let edges = Rank::First.to_u64();
                        if current_square.to_u64() & edges == 0 {
                            let mut new_square = current_square.to_u64();
                            let mut count = 0;
                            while new_square & edges == 0 {
                                new_square = new_square >> 8;
                                orthogonals[square_index][2][count] = Square::from_u64(new_square);
                                count += 1;
                                attack_map = attack_map | new_square;
                            }
                        }
                    }


                    {
                        let edges = File::A.to_u64();
                        if current_square.to_u64() & edges == 0 {
                            let mut new_square = current_square.to_u64();
                            let mut count = 0;
                            while new_square & edges == 0 {
                                new_square = new_square >> 1;
                                orthogonals[square_index][3][count] = Square::from_u64(new_square);
                                count += 1;
                                attack_map = attack_map | new_square;
                            }
                        }
                    }

                    attacks[PieceType::Rook.precalculated_index()][square_index as usize] = attack_map;
                }
            }

            {
                for square_index in 0..64 {
                    let current_square = Square::try_from_primitive(square_index as u8).unwrap();
                    let mut attack_map = 0;

                    {
                        let edges = File::H.to_u64() | Rank::Eight.to_u64();
                        if current_square.to_u64() & edges == 0 {
                            let mut new_square = current_square.to_u64();
                            let mut count = 0;
                            while new_square & edges == 0 {
                                new_square = new_square << 9;
                                diagonals[square_index][0][count] = Square::from_u64(new_square);
                                count += 1;
                                attack_map = attack_map | new_square;
                            }
                        }
                    }


                    {
                        let edges = File::H.to_u64() | Rank::First.to_u64();
                        if current_square.to_u64() & edges == 0 {
                            let mut new_square = current_square.to_u64();
                            let mut count = 0;
                            while new_square & edges == 0 {
                                new_square = new_square >> 7;
                                diagonals[square_index][1][count] = Square::from_u64(new_square);
                                count += 1;
                                attack_map = attack_map | new_square;
                            }
                        }
                    }

                    {
                        let edges = File::A.to_u64() | Rank::First.to_u64();
                        if current_square.to_u64() & edges == 0 {
                            let mut new_square = current_square.to_u64();
                            let mut count = 0;
                            while new_square & edges == 0 {
                                new_square = new_square >> 9;
                                diagonals[square_index][2][count] = Square::from_u64(new_square);
                                count += 1;
                                attack_map = attack_map | new_square;
                            }
                        }
                    }


                    {
                        let edges = File::A.to_u64() | Rank::Eight.to_u64();
                        if current_square.to_u64() & edges == 0 {
                            let mut new_square = current_square.to_u64();
                            let mut count = 0;
                            while new_square & edges == 0 {
                                new_square = new_square << 7;
                                diagonals[square_index][3][count] = Square::from_u64(new_square);
                                count += 1;
                                attack_map = attack_map | new_square;
                            }
                        }
                    }

                    attacks[PieceType::Bishop.precalculated_index()][square_index as usize] = attack_map;
                }
            }

            {
                for square_index in 0..64 {
                    let mut non_blocker_map = 0;
                    let current_square = Square::try_from_primitive(square_index as u8).unwrap();
                    if current_square.to_u64() & File::A.to_u64() == 0 {
                        non_blocker_map = non_blocker_map | File::A.to_u64();
                    }
                    if current_square.to_u64() & File::H.to_u64() == 0 {
                        non_blocker_map = non_blocker_map | File::H.to_u64();
                    }
                    if current_square.to_u64() & Rank::First.to_u64() == 0 {
                        non_blocker_map = non_blocker_map | Rank::First.to_u64();
                    }
                    if current_square.to_u64() & Rank::Eight.to_u64() == 0 {
                        non_blocker_map = non_blocker_map | Rank::Eight.to_u64();
                    }

                    for precalculated_index in 0..3 {
                        let attack_map = attacks[precalculated_index][square_index as usize];
                        let blocker_map = attack_map - (attack_map & non_blocker_map);
                        blockers[precalculated_index][square_index as usize] = blocker_map;
                    }
                }
            }

            {
                for square_index in 0..64 {
                    attacks[PieceType::Queen.precalculated_index()][square_index] 
                        = attacks[PieceType::Rook.precalculated_index()][square_index] | attacks[PieceType::Bishop.precalculated_index()][square_index];
                    blockers[PieceType::Queen.precalculated_index()][square_index] 
                        = blockers[PieceType::Rook.precalculated_index()][square_index] | blockers[PieceType::Bishop.precalculated_index()][square_index];
                }
            }


            PreComputedAttackSets{
                attacks,
                blockers,
                diagonals,
                orthogonals,
            }
        }
    }
}
