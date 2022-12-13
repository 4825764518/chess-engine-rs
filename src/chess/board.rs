use crate::utils::bitboard::Bitboard;

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

        for row in (0..8).rev() {
            let bit_rank = (row * 8) as u8;
            let row_string = row_strings[row];
            let mut tile = 0 as u8;
            for row_char in row_string.chars() {
                let bit = bit_rank + tile as u8;
                if row_char.is_numeric() {
                    let row_number = row_char.to_digit(10);
                    match row_number {
                        Some(x) => tile += x as u8,
                        None => panic!("Invalid piece fen {}", fen),
                    }
                } else {
                    match row_char {
                        'p' => black_pawns.set(bit),
                        'n' => black_knights.set(bit),
                        'b' => black_bishops.set(bit),
                        'r' => black_rooks.set(bit),
                        'q' => black_queens.set(bit),
                        'k' => black_kings.set(bit),
                        'P' => white_pawns.set(bit),
                        'N' => white_knights.set(bit),
                        'B' => white_bishops.set(bit),
                        'R' => white_rooks.set(bit),
                        'Q' => white_queens.set(bit),
                        'K' => white_kings.set(bit),
                        _ => panic!("Invalid piece character {}", row_char),
                    }

                    tile += 1;
                }

                if tile == 8 {
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
}
