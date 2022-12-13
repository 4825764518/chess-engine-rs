use super::super::utils::bitboard::Bitboard;
use super::board::SideBoard;
use super::board::Square;
use super::game::Game;
use super::game::Side;
use super::move_tables::pawn_double_move_board;

pub struct Move {
    from: Square,
    to: Square,
    capture: bool,
    en_passant: bool,
    promotion: u8,
}

pub fn generate_ray_moves(
    from: Square,
    our_pieces: &SideBoard,
    opponent_pieces: &SideBoard,
    attack_table: Bitboard,
) -> Vec<Move> {
    let mut result = Vec::new();

    let our_occupied = our_pieces.occupied();
    let opponent_occupied = opponent_pieces.occupied();
    let attacked = attack_table & !our_occupied;

    for to_square in attacked {
        let capture = opponent_occupied.is_set(to_square);
        result.push(Move {
            from: from,
            to: to_square,
            capture: capture,
            en_passant: false,
            promotion: 0,
        });
    }

    result
}

pub fn generate_pawn_moves(
    our_pieces: &SideBoard,
    opponent_pieces: &SideBoard,
    occupied: Bitboard,
    side: Side,
    ep_square: Option<Square>,
) -> Vec<Move> {
    let mut result = Vec::new();

    let our_pawns = our_pieces.pawns();
    let opponent_occupied = opponent_pieces.occupied();

    for from_square in our_pawns {
        let mut move_board = super::move_tables::pawn_move_board(side, from_square) & !occupied;
        if move_board.data() != 0 {
            move_board |= pawn_double_move_board(side, from_square) & !occupied;
        }

        let capture_board =
            super::move_tables::pawn_attack_board(side, from_square) & opponent_occupied;

        let mut ep_board = Bitboard::new();
        match ep_square {
            Some(ep_square) => {
                let side_offset = if side == Side::White { -1 } else { 1 };
                let ep_square_w = Square::new_file_rank(
                    (from_square.file() as i8 + -1) as u8,
                    (from_square.rank() as i8 + side_offset) as u8,
                );
                let ep_square_e = Square::new_file_rank(
                    (from_square.file() as i8 + 1) as u8,
                    (from_square.rank() as i8 + side_offset) as u8,
                );
                if ep_square_w.index() == ep_square.index()
                    || ep_square_e.index() == ep_square.index()
                {
                    ep_board.set(ep_square);
                }
            }
            _ => {}
        }

        let promotion_rank = if side == Side::White { 7 } else { 0 };

        // pushes
        for to_square in move_board {
            if to_square.rank() == promotion_rank {
                for promote in 0..3 {
                    // TODO
                    result.push(Move {
                        from: from_square,
                        to: to_square,
                        capture: false,
                        en_passant: false,
                        promotion: promote,
                    });
                }
            } else {
                result.push(Move {
                    from: from_square,
                    to: to_square,
                    capture: false,
                    en_passant: false,
                    promotion: 0,
                });
            }
        }

        // normal captures
        for to_square in capture_board {
            if to_square.rank() == promotion_rank {
                for promote in 0..3 {
                    // TODO
                    result.push(Move {
                        from: from_square,
                        to: to_square,
                        capture: true,
                        en_passant: false,
                        promotion: promote,
                    });
                }
            } else {
                result.push(Move {
                    from: from_square,
                    to: to_square,
                    capture: true,
                    en_passant: false,
                    promotion: 0,
                });
            }
        }

        // en passant capture
        for to_square in ep_board {
            result.push(Move {
                from: from_square,
                to: to_square,
                capture: true,
                en_passant: true,
                promotion: 0,
            });
        }
    }

    result
}

pub fn generate_knight_moves(
    our_pieces: &SideBoard,
    opponent_pieces: &SideBoard,
    occupied: Bitboard,
) -> Vec<Move> {
    let mut result = Vec::new();

    let our_knights = our_pieces.knights();
    let our_occupied = our_pieces.occupied();
    let opponent_occupied = opponent_pieces.occupied();

    for from_square in our_knights {
        let attacked =
            super::move_tables::attack_table_knight(occupied, from_square) & !our_occupied;
        for to_square in attacked {
            let capture = opponent_occupied.is_set(to_square);
            result.push(Move {
                from: from_square,
                to: to_square,
                capture: capture,
                en_passant: false,
                promotion: 0,
            });
        }
    }

    result
}

pub fn generate_bishop_moves(
    our_pieces: &SideBoard,
    opponent_pieces: &SideBoard,
    occupied: Bitboard,
) -> Vec<Move> {
    let mut result = Vec::new();

    let our_bishops = our_pieces.bishops();
    for from_square in our_bishops {
        let attacked = super::move_tables::attack_table_bishop(occupied, from_square);
        result.append(&mut generate_ray_moves(
            from_square,
            our_pieces,
            opponent_pieces,
            attacked,
        ));
    }

    result
}

pub fn generate_rook_moves(
    our_pieces: &SideBoard,
    opponent_pieces: &SideBoard,
    occupied: Bitboard,
) -> Vec<Move> {
    let mut result = Vec::new();

    let our_bishops = our_pieces.rooks();
    for from_square in our_bishops {
        let attacked = super::move_tables::attack_table_rook(occupied, from_square);
        result.append(&mut generate_ray_moves(
            from_square,
            our_pieces,
            opponent_pieces,
            attacked,
        ));
    }

    result
}

pub fn generate_queen_moves(
    our_pieces: &SideBoard,
    opponent_pieces: &SideBoard,
    occupied: Bitboard,
) -> Vec<Move> {
    let mut result = Vec::new();

    let our_bishops = our_pieces.queens();
    for from_square in our_bishops {
        let attacked = super::move_tables::attack_table_queen(occupied, from_square);
        result.append(&mut generate_ray_moves(
            from_square,
            our_pieces,
            opponent_pieces,
            attacked,
        ));
    }

    result
}

pub fn generate_king_moves(
    our_pieces: &SideBoard,
    opponent_pieces: &SideBoard,
    occupied: Bitboard,
) -> Vec<Move> {
    let mut result = Vec::new();

    let our_kings = our_pieces.kings();
    let our_occupied = our_pieces.occupied();
    let opponent_occupied = opponent_pieces.occupied();

    for from_square in our_kings {
        let attacked = super::move_tables::attack_table_king(occupied, from_square) & !our_occupied;
        for to_square in attacked {
            let capture = opponent_occupied.is_set(to_square);
            result.push(Move {
                from: from_square,
                to: to_square,
                capture: capture,
                en_passant: false,
                promotion: 0,
            });
        }
    }

    result
}

pub fn generate_pseudolegal_moves(game: &Game) -> Vec<Move> {
    let side = game.active();
    let board = game.board();
    let occupied = board.occupied();
    let our_pieces = board.side_pieces(side);
    let ep_square = game.ep_square();
    let opponent_pieces = if side == Side::White {
        board.black_pieces()
    } else {
        board.white_pieces()
    };

    let mut result = Vec::new();

    result.append(&mut generate_pawn_moves(
        our_pieces,
        opponent_pieces,
        occupied,
        side,
        ep_square,
    ));
    result.append(&mut generate_knight_moves(
        our_pieces,
        opponent_pieces,
        occupied,
    ));
    result.append(&mut generate_bishop_moves(
        our_pieces,
        opponent_pieces,
        occupied,
    ));
    result.append(&mut generate_rook_moves(
        our_pieces,
        opponent_pieces,
        occupied,
    ));
    result.append(&mut generate_queen_moves(
        our_pieces,
        opponent_pieces,
        occupied,
    ));
    result.append(&mut generate_king_moves(
        our_pieces,
        opponent_pieces,
        occupied,
    ));

    result
}
