pub mod chess;
pub mod utils;

fn main() {
    println!("Hello, world!");

    let empty_game = chess::game::Game::new_empty();
    let game = chess::game::Game::new_classical();

    let e6_index = utils::notation::algebraic_square_to_bit("e6");
    println!("e6_index {}", e6_index);

    let index_notation = utils::notation::bit_to_algebraic_square(e6_index);
    println!("index_notation {}", index_notation);
}
