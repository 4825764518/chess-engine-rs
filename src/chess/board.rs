use crate::utils::bitboard::Bitboard;

use super::game::Side;

#[derive(Clone, Copy)]
pub struct Square {
    index: u8,
}

pub struct SideBoard {
    pawns: Bitboard,
    knights: Bitboard,
    bishops: Bitboard,
    rooks: Bitboard,
    queens: Bitboard,
    kings: Bitboard,
}

pub struct Board {
    white: SideBoard,
    black: SideBoard,
}

impl Square {
    fn file_rank_to_index(file: u8, rank: u8) -> u8 {
        (rank & 7) << 3 | (file & 7)
    }

    pub fn new(index: u8) -> Self {
        Self { index: index }
    }

    pub fn new_file_rank(file: u8, rank: u8) -> Self {
        Self::new(Self::file_rank_to_index(file, rank))
    }

    pub fn index(&self) -> u8 {
        self.index
    }

    pub fn file(&self) -> u8 {
        self.index & 7
    }

    pub fn rank(&self) -> u8 {
        (self.index >> 3) & 7
    }
}

impl SideBoard {
    pub fn new(
        pawns: Bitboard,
        knights: Bitboard,
        bishops: Bitboard,
        rooks: Bitboard,
        queens: Bitboard,
        kings: Bitboard,
    ) -> Self {
        Self {
            pawns: pawns,
            knights: knights,
            bishops: bishops,
            rooks: rooks,
            queens: queens,
            kings: kings,
        }
    }

    pub fn new_empty() -> Self {
        Self::new(
            Bitboard::new(),
            Bitboard::new(),
            Bitboard::new(),
            Bitboard::new(),
            Bitboard::new(),
            Bitboard::new(),
        )
    }

    pub fn pawns(&self) -> Bitboard {
        self.pawns
    }

    pub fn knights(&self) -> Bitboard {
        self.knights
    }

    pub fn bishops(&self) -> Bitboard {
        self.bishops
    }

    pub fn rooks(&self) -> Bitboard {
        self.rooks
    }

    pub fn queens(&self) -> Bitboard {
        self.queens
    }

    pub fn kings(&self) -> Bitboard {
        self.kings
    }

    pub fn occupied(&self) -> Bitboard {
        self.pawns | self.knights | self.bishops | self.rooks | self.queens | self.kings
    }
}

impl Board {
    pub fn new(white: SideBoard, black: SideBoard) -> Self {
        Self {
            white: white,
            black: black,
        }
    }

    pub fn from_fen_pieces(fen: String) -> Self {
        let row_strings: Vec<&str> = fen.as_str().split("/").collect();
        if row_strings.len() != 8 {
            panic!("Invalid piece FEN {}", fen);
        }

        let mut white_pawns = Bitboard::new();
        let mut black_pawns = Bitboard::new();
        let mut white_knights = Bitboard::new();
        let mut black_knights = Bitboard::new();
        let mut white_bishops = Bitboard::new();
        let mut black_bishops = Bitboard::new();
        let mut white_rooks = Bitboard::new();
        let mut black_rooks = Bitboard::new();
        let mut white_queens = Bitboard::new();
        let mut black_queens = Bitboard::new();
        let mut white_kings = Bitboard::new();
        let mut black_kings = Bitboard::new();

        for rank in (0..8).rev() {
            let row_string = row_strings[7 - rank];
            let mut file = 0 as u8;
            for row_char in row_string.chars() {
                let square = Square::new_file_rank(file, rank as u8);
                if row_char.is_numeric() {
                    let row_number = row_char.to_digit(10);
                    match row_number {
                        Some(x) => file += x as u8,
                        None => panic!("Invalid piece fen {}", fen),
                    }
                } else {
                    match row_char {
                        'p' => black_pawns.set(square),
                        'n' => black_knights.set(square),
                        'b' => black_bishops.set(square),
                        'r' => black_rooks.set(square),
                        'q' => black_queens.set(square),
                        'k' => black_kings.set(square),
                        'P' => white_pawns.set(square),
                        'N' => white_knights.set(square),
                        'B' => white_bishops.set(square),
                        'R' => white_rooks.set(square),
                        'Q' => white_queens.set(square),
                        'K' => white_kings.set(square),
                        _ => panic!("Invalid piece character {}", row_char),
                    }

                    file += 1;
                }

                if file == 8 {
                    break;
                }
            }
        }

        let white_pieces = SideBoard::new(
            white_pawns,
            white_knights,
            white_bishops,
            white_rooks,
            white_queens,
            white_kings,
        );
        let black_pieces = SideBoard::new(
            black_pawns,
            black_knights,
            black_bishops,
            black_rooks,
            black_queens,
            black_kings,
        );

        println!("Black pawns:");
        println!("{}", black_pieces.pawns().to_string());
        println!("Black knights:");
        println!("{}", black_pieces.knights().to_string());
        println!("Black bishops:");
        println!("{}", black_pieces.bishops().to_string());
        println!("Black rooks:");
        println!("{}", black_pieces.rooks().to_string());
        println!("Black queens:");
        println!("{}", black_pieces.queens().to_string());
        println!("Black kings:");
        println!("{}", black_pieces.kings().to_string());

        println!("White pawns:");
        println!("{}", white_pieces.pawns().to_string());
        println!("White knights:");
        println!("{}", white_pieces.knights().to_string());
        println!("White bishops:");
        println!("{}", white_pieces.bishops().to_string());
        println!("White rooks:");
        println!("{}", white_pieces.rooks().to_string());
        println!("White queens:");
        println!("{}", white_pieces.queens().to_string());
        println!("White kings:");
        println!("{}", white_pieces.kings().to_string());

        Self {
            white: white_pieces,
            black: black_pieces,
        }
    }

    pub fn white_pieces(&self) -> &SideBoard {
        &self.white
    }

    pub fn black_pieces(&self) -> &SideBoard {
        &self.black
    }

    pub fn side_pieces(&self, side: Side) -> &SideBoard {
        if side == Side::White {
            self.white_pieces()
        } else {
            self.black_pieces()
        }
    }

    pub fn occupied(&self) -> Bitboard {
        self.white.occupied() | self.black.occupied()
    }
}
