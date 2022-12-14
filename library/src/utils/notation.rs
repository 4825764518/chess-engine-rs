pub fn algebraic_square_to_bit(square: &str) -> u8 {
    let file_char = square.chars().nth(0).unwrap();
    let rank_char = square.chars().nth(1).unwrap();

    let file = file_char as u8 - 'a' as u8;
    let rank = rank_char.to_digit(10).unwrap() as u8 - 1;
    return (rank * 8) + file;
}

pub fn bit_to_algebraic_square(index: u8) -> String {
    let rank = ((index & 0b00111000) / 8) + 1;
    let file = index & 0b00000111;
    let file_char = ('a' as u8 + file) as char;
    let rank_char = char::from_digit(rank as u32, 10).unwrap();
    return String::from(format!("{}{}", file_char, rank_char));
}
