use super::{board::Square, game::Side};
use crate::utils::bitboard::Bitboard;

lazy_static! {
    static ref PAWN_MOVE_TABLES: [[Bitboard; 64]; 2] = compute_pawn_move_tables();
    static ref PAWN_DOUBLE_MOVE_TABLES: [[Bitboard; 64]; 2] = compute_pawn_double_move_tables();
    static ref PAWN_CAPTURE_TABLES: [[Bitboard; 64]; 2] = compute_pawn_capture_tables();
    static ref KNIGHT_ATTACK_TABLE: [Bitboard; 64] = compute_knight_table();
    static ref KING_ATTACK_TABLE: [Bitboard; 64] = compute_king_table();
    static ref RAY_TABLE_NORTH: [Bitboard; 64] = compute_ray_table::<0, 1>();
    static ref RAY_TABLE_SOUTH: [Bitboard; 64] = compute_ray_table::<0, -1>();
    static ref RAY_TABLE_EAST: [Bitboard; 64] = compute_ray_table::<1, 0>();
    static ref RAY_TABLE_WEST: [Bitboard; 64] = compute_ray_table::<-1, 0>();
    static ref RAY_TABLE_NORTHEAST: [Bitboard; 64] = compute_ray_table::<1, 1>();
    static ref RAY_TABLE_NORTHWEST: [Bitboard; 64] = compute_ray_table::<-1, 1>();
    static ref RAY_TABLE_SOUTHEAST: [Bitboard; 64] = compute_ray_table::<1, -1>();
    static ref RAY_TABLE_SOUTHWEST: [Bitboard; 64] = compute_ray_table::<-1, -1>();
}

fn compute_pawn_move_tables() -> [[Bitboard; 64]; 2] {
    let mut result = [[Bitboard::new(); 64]; 2];

    for side in 0..2 {
        let mut side_result = [Bitboard::new(); 64];
        let direction = if side == 0 { 1 } else { -1 };

        for index in 0..64 {
            let square = Square::new(index);
            let file = square.file();
            let rank = square.rank();
            let mut board = Bitboard::new();

            let rank_offset = rank as i8 + direction;
            if rank_offset < 0 || rank_offset > 7 {
                continue;
            }

            board.set(Square::new_file_rank(file, rank_offset as u8));
            side_result[index as usize] = board;
        }

        result[side] = side_result;
    }

    result
}

fn compute_pawn_double_move_tables() -> [[Bitboard; 64]; 2] {
    let mut result = [[Bitboard::new(); 64]; 2];

    for side in 0..2 {
        let mut side_result = [Bitboard::new(); 64];
        let direction = if side == 0 { 2 } else { -2 };
        let double_move_rank = if side == 0 { 1 } else { 6 };

        for index in 0..64 {
            let square = Square::new(index);
            let file = square.file();
            let rank = square.rank();
            let mut board = Bitboard::new();

            if rank == double_move_rank {
                let rank_offset = rank as i8 + direction;
                board.set(Square::new_file_rank(file, rank_offset as u8));
            }

            side_result[index as usize] = board;
        }

        result[side] = side_result;
    }

    result
}

fn compute_pawn_capture_tables() -> [[Bitboard; 64]; 2] {
    const offsets: [i8; 2] = [-1, 1];
    let mut result = [[Bitboard::new(); 64]; 2];

    for side in 0..2 {
        let mut side_result = [Bitboard::new(); 64];
        let direction = if side == 0 { 1 } else { -1 };

        for index in 0..64 {
            let square = Square::new(index);
            let file = square.file();
            let rank = square.rank();
            let mut board = Bitboard::new();

            for offset in offsets {
                let file_offset = file as i8 + offset;
                let rank_offset = rank as i8 + direction;
                if file_offset < 0 || file_offset > 7 {
                    continue;
                }
                if rank_offset < 0 || rank_offset > 7 {
                    continue;
                }

                board.set(Square::new_file_rank(file_offset as u8, rank_offset as u8));
            }

            side_result[index as usize] = board;
        }

        result[side] = side_result;
    }

    result
}

fn compute_knight_table() -> [Bitboard; 64] {
    const offsets: [(i8, i8); 8] = [
        (-2, 1),
        (-1, 2),
        (1, 2),
        (2, 1),
        (2, -1),
        (1, -2),
        (-1, -2),
        (-2, -1),
    ];

    let mut result = [Bitboard::new(); 64];

    for index in 0..64 {
        let square = Square::new(index);
        let file = square.file() as i8;
        let rank = square.rank() as i8;
        let mut board = Bitboard::new();

        for (file_offset, rank_offset) in offsets {
            let attack_file = file + file_offset;
            let attack_rank = rank + rank_offset;
            if attack_file < 0 || attack_file > 7 {
                continue;
            }
            if attack_rank < 0 || attack_rank > 7 {
                continue;
            }

            board.set(Square::new_file_rank(attack_file as u8, attack_rank as u8));
        }

        result[index as usize] = board;
    }

    result
}

fn compute_king_table() -> [Bitboard; 64] {
    const offsets: [(i8, i8); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];

    let mut result = [Bitboard::new(); 64];

    for index in 0..64 {
        let square = Square::new(index);
        let file = square.file() as i8;
        let rank = square.rank() as i8;
        let mut board = Bitboard::new();

        for (file_offset, rank_offset) in offsets {
            let attack_file = file + file_offset;
            let attack_rank = rank + rank_offset;
            if attack_file < 0 || attack_file > 7 {
                continue;
            }
            if attack_rank < 0 || attack_rank > 7 {
                continue;
            }

            board.set(Square::new_file_rank(attack_file as u8, attack_rank as u8));
        }

        result[index as usize] = board;
    }

    result
}

fn compute_ray_table<const FILE_DIRECTION: i8, const RANK_DIRECTION: i8>() -> [Bitboard; 64] {
    let mut result = [Bitboard::new(); 64];
    for index in 0..64 {
        let square = Square::new(index);
        let mut board = Bitboard::new();
        let mut file = square.file() as i8 + FILE_DIRECTION;
        let mut rank = square.rank() as i8 + RANK_DIRECTION;

        loop {
            if file < 0 || file > 7 {
                break;
            }
            if rank < 0 || rank > 7 {
                break;
            }

            board.set(Square::new_file_rank(file as u8, rank as u8));

            file = file + FILE_DIRECTION;
            rank = rank + RANK_DIRECTION;
        }

        result[square.index() as usize] = board;
    }

    result
}

fn ray_attack_table<const GENERATE_DIAGONAL: bool, const GENERATE_HORIZONTAL: bool>(
    occupied: Bitboard,
    from: Square,
) -> Bitboard {
    let mut attacked = Bitboard::new();
    let mut blockers = Bitboard::new();
    let mut blocking_square = Square::new(0);

    if GENERATE_DIAGONAL {
        let mut northeast_attacked = RAY_TABLE_NORTHEAST[from.index() as usize];
        blockers = (northeast_attacked & occupied) | Bitboard::from_data(0x8000000000000000);
        blocking_square = Square::new(blockers.find_first());
        northeast_attacked =
            northeast_attacked ^ RAY_TABLE_NORTHEAST[blocking_square.index() as usize];

        let mut northwest_attacked = RAY_TABLE_NORTHWEST[from.index() as usize];
        blockers = (northwest_attacked & occupied) | Bitboard::from_data(0x8000000000000000);
        blocking_square = Square::new(blockers.find_first());
        northwest_attacked =
            northwest_attacked ^ RAY_TABLE_NORTHWEST[blocking_square.index() as usize];

        let mut southeast_attacked = RAY_TABLE_SOUTHEAST[from.index() as usize];
        blockers = (southeast_attacked & occupied) | Bitboard::from_data(1);
        blocking_square = Square::new(blockers.find_last());
        southeast_attacked =
            southeast_attacked ^ RAY_TABLE_SOUTHEAST[blocking_square.index() as usize];

        let mut southwest_attacked = RAY_TABLE_SOUTHWEST[from.index() as usize];
        blockers = (southwest_attacked & occupied) | Bitboard::from_data(1);
        blocking_square = Square::new(blockers.find_last());
        southwest_attacked =
            southwest_attacked ^ RAY_TABLE_SOUTHWEST[blocking_square.index() as usize];

        attacked = attacked
            | northeast_attacked
            | northwest_attacked
            | southeast_attacked
            | southwest_attacked;
    }

    if GENERATE_HORIZONTAL {
        let mut north_attacked = RAY_TABLE_NORTH[from.index() as usize];
        blockers = (north_attacked & occupied) | Bitboard::from_data(0x8000000000000000);
        blocking_square = Square::new(blockers.find_first());
        north_attacked = north_attacked ^ RAY_TABLE_NORTH[blocking_square.index() as usize];

        let mut east_attacked = RAY_TABLE_EAST[from.index() as usize];
        blockers = (east_attacked & occupied) | Bitboard::from_data(0x8000000000000000);
        blocking_square = Square::new(blockers.find_first());
        east_attacked = east_attacked ^ RAY_TABLE_EAST[blocking_square.index() as usize];

        let mut south_attacked = RAY_TABLE_SOUTH[from.index() as usize];
        blockers = (south_attacked & occupied) | Bitboard::from_data(1);
        blocking_square = Square::new(blockers.find_last());
        south_attacked = south_attacked ^ RAY_TABLE_SOUTH[blocking_square.index() as usize];

        let mut west_attacked = RAY_TABLE_WEST[from.index() as usize];
        blockers = (west_attacked & occupied) | Bitboard::from_data(1);
        blocking_square = Square::new(blockers.find_last());
        west_attacked = west_attacked ^ RAY_TABLE_WEST[blocking_square.index() as usize];

        attacked = attacked | north_attacked | east_attacked | south_attacked | west_attacked;
    }

    attacked
}

pub fn pawn_move_board(side: Side, square: Square) -> Bitboard {
    PAWN_MOVE_TABLES[side as usize][square.index() as usize]
}

pub fn pawn_double_move_board(side: Side, square: Square) -> Bitboard {
    PAWN_DOUBLE_MOVE_TABLES[side as usize][square.index() as usize]
}

pub fn pawn_attack_board(side: Side, square: Square) -> Bitboard {
    PAWN_CAPTURE_TABLES[side as usize][square.index() as usize]
}

pub fn attack_table_knight(occupied: Bitboard, from: Square) -> Bitboard {
    KNIGHT_ATTACK_TABLE[from.index() as usize]
}

pub fn attack_table_bishop(occupied: Bitboard, from: Square) -> Bitboard {
    ray_attack_table::<true, false>(occupied, from)
}

pub fn attack_table_rook(occupied: Bitboard, from: Square) -> Bitboard {
    ray_attack_table::<false, true>(occupied, from)
}

pub fn attack_table_queen(occupied: Bitboard, from: Square) -> Bitboard {
    ray_attack_table::<true, true>(occupied, from)
}

pub fn attack_table_king(occupied: Bitboard, from: Square) -> Bitboard {
    KING_ATTACK_TABLE[from.index() as usize]
}
