use crate::chess::board::Board;
use crate::chess::board::SideBoard;
use crate::utils::notation;

pub enum Variant {
    Classical,
}

pub enum Side {
    White,
    Black,
}

pub struct Game {
    kind: Variant,
    board: Board,
    active: Side,
    white_castle_kingside: bool,
    white_castle_queenside: bool,
    black_castle_kingside: bool,
    black_castle_queenside: bool,
    ep_square: u8,
    half_move: u8,
    full_move: u16,
}

impl Game {
    pub fn from_fen(fen: String) -> Self {
        let fen_split: Vec<&str> = fen.as_str().split(" ").collect();
        if fen_split.len() != 6 {
            panic!("Invalid FEN {}", fen);
        }

        let fen_pieces = fen_split[0];
        let fen_active = fen_split[1];
        let fen_castle = fen_split[2];
        let fen_ep_square = fen_split[3];
        let fen_half_move = fen_split[4];
        let fen_full_move = fen_split[5];

        let active = match fen_active {
            "w" => Side::White,
            "b" => Side::Black,
            _ => panic!("Invalid fen color: {}", fen_active),
        };

        let mut white_castle_kingside = false;
        let mut white_castle_queenside = false;
        let mut black_castle_kingside = false;
        let mut black_castle_queenside = false;
        if fen_castle != "-" {
            for castle_char in fen_castle.chars() {
                match castle_char {
                    'K' => white_castle_kingside = true,
                    'Q' => white_castle_queenside = true,
                    'k' => black_castle_kingside = true,
                    'q' => black_castle_queenside = true,
                    _ => panic!(),
                }
            }
        }

        let ep_square = match fen_ep_square {
            "-" => 64,
            _ => notation::algebraic_square_to_bit(fen_ep_square),
        };
        let half_move: u8 = fen_half_move.parse().unwrap();
        let full_move: u16 = fen_full_move.parse().unwrap();

        Self {
            kind: Variant::Classical,
            board: Board::from_fen_pieces(fen_pieces.to_string()),
            active: active,
            white_castle_kingside: white_castle_kingside,
            white_castle_queenside: white_castle_queenside,
            black_castle_kingside: black_castle_kingside,
            black_castle_queenside: black_castle_queenside,
            ep_square: ep_square,
            half_move: half_move,
            full_move: full_move,
        }
    }

    pub fn new_classical() -> Self {
        Self::from_fen(String::from(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        ))
    }

    pub fn new_empty() -> Self {
        Self {
            kind: Variant::Classical,
            board: Board::new(SideBoard::new_empty(), SideBoard::new_empty()),
            active: Side::White,
            white_castle_kingside: false,
            white_castle_queenside: false,
            black_castle_kingside: false,
            black_castle_queenside: false,
            ep_square: 64,
            half_move: 0,
            full_move: 1,
        }
    }
}
