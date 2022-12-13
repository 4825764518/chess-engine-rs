#[macro_use]
extern crate lazy_static;

pub mod chess;
pub mod utils;

fn main() {
    println!("Hello, world!");

    let game = chess::game::Game::new_classical();

    let e6_index = utils::notation::algebraic_square_to_bit("e6");
    println!("e6_index {}", e6_index);

    let index_notation = utils::notation::bit_to_algebraic_square(e6_index);
    println!("index_notation {}", index_notation);

    let empty_board = utils::bitboard::Bitboard::from_data(0);
    let e7_occupied_board = utils::bitboard::Bitboard::from_data(0x10000000000000);
    let e6_square = chess::board::Square::new(e6_index);
    println!(
        "e6_square index {} file {} rank {}",
        e6_square.index(),
        e6_square.file(),
        e6_square.rank()
    );
    println!("e7_occupied_board");
    println!("{}", e7_occupied_board.to_string());
    let bishop_moves_e6 = chess::move_tables::attack_table_bishop(empty_board, e6_square);
    let rook_moves_e6 = chess::move_tables::attack_table_rook(empty_board, e6_square);
    let queen_moves_e6 = chess::move_tables::attack_table_queen(empty_board, e6_square);
    let queen_moves_e6_occupied =
        chess::move_tables::attack_table_queen(e7_occupied_board, e6_square);

    println!("e6 bishop moves:");
    println!("{}", bishop_moves_e6.to_string());
    println!("e6 rook moves:");
    println!("{}", rook_moves_e6.to_string());
    println!("e6 queen moves:");
    println!("{}", queen_moves_e6.to_string());
    println!("e6 queen moves occupied:");
    println!("{}", queen_moves_e6_occupied.to_string());

    let rook_move_a1_game = chess::game::Game::from_fen(String::from("8/8/8/8/8/8/8/R7 w - - 0 1"));
    let rook_capture_a1_game =
        chess::game::Game::from_fen(String::from("8/8/8/8/q7/q7/q7/Rqqq4 w - - 0 1"));
    let moves_rook_move_a1 = chess::moves::generate_pseudolegal_moves(&rook_move_a1_game);
    let moves_rook_capture_a1 = chess::moves::generate_pseudolegal_moves(&rook_capture_a1_game);

    println!("move count {}", moves_rook_move_a1.len());
    println!("move count {}", moves_rook_capture_a1.len());
}
