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

    pub fn is_set(&self, index: u8) -> bool {
        ((self.data >> index) & 1) == 1
    }

    pub fn set(&mut self, index: u8) {
        println!("setting bit {}", index);
        self.data |= (1 as u64) << index;
    }
}

impl ToString for Bitboard {
    fn to_string(&self) -> String {
        let mut rows = Vec::new();

        for rank in 0..8 {
            let mut row = String::new();
            for file in 0..8 {
                let bit = (rank * 8) + file;
                row.push_str(if self.is_set(bit) { "1" } else { "0" });
            }
            rows.push(row);
        }

        rows.join("\n")
    }
}
