use std::path::Iter;

use crate::chess::board::Square;

#[derive(Clone, Copy)]
pub struct Bitboard {
    data: u64,
}

impl Bitboard {
    pub fn new() -> Self {
        return Self::from_data(0);
    }

    pub fn from_data(data: u64) -> Self {
        Self { data: data }
    }

    pub fn data(&self) -> u64 {
        self.data
    }

    pub fn is_set(&self, square: Square) -> bool {
        ((self.data >> square.index()) & 1) == 1
    }

    pub fn set(&mut self, square: Square) {
        self.data |= (1 << square.index());
    }

    pub fn unset(&mut self, index: u8) {
        let mask = !(1 << index);
        self.data &= mask;
    }

    pub fn find_first(&self) -> u8 {
        if self.data == 0 {
            panic!();
        }

        self.data.trailing_zeros() as u8
    }

    pub fn find_last(&self) -> u8 {
        if self.data == 0 {
            panic!();
        }

        (63 - self.data.leading_zeros()) as u8
    }
}

impl std::ops::BitAnd<Bitboard> for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self {
            data: (self.data() & rhs.data()),
        }
    }
}

impl std::ops::BitOr<Bitboard> for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self {
            data: (self.data() | rhs.data()),
        }
    }
}

impl std::ops::BitOrAssign<Bitboard> for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.data |= rhs.data();
    }
}

impl std::ops::BitXor<Bitboard> for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Self {
            data: (self.data() ^ rhs.data()),
        }
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self {
        Self { data: !self.data }
    }
}

impl Iterator for Bitboard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        let trailing_zeroes = self.data.trailing_zeros();
        if trailing_zeroes == 64 {
            None
        } else {
            let index = trailing_zeroes as u8;
            let square = Square::new(index);
            self.unset(index);
            Some(square)
        }
    }
}

impl ToString for Bitboard {
    fn to_string(&self) -> String {
        let mut rows = Vec::new();

        for rank in (0..8).rev() {
            let mut row = String::new();
            for file in 0..8 {
                // let bit = (rank * 8) + file;
                let square = Square::new_file_rank(file, rank);
                row.push_str(if self.is_set(square) { "1" } else { "0" });
            }
            rows.push(row);
        }

        rows.join("\n")
    }
}
