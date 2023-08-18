use std::sync::atomic::AtomicBool;

use super::board::Board;
use super::board::Color;
use super::board::Field;
use super::board::KindOfPiece;
use super::board::Piece;
use super::book::Move;

fn is_check(board: &Board, color: &Color) -> bool {
    let enemy_color = color.enemy();
    let mut enemy_positions: Vec<Field> = Vec::new();

    for row in 1..=8 {
        for file in 1..=8 {
            let field = Field::build_unchecked(row, file);
            if let Some(piece) = board.field_content(&field) {
                if piece.color == enemy_color {
                    enemy_positions.push(field);
                }
            }
        }
    }

    let mut enemy_view: Vec<Field> = Vec::new();
    for field in enemy_positions.iter() {
        enemy_view.append(&mut possible_moves_unchecked(field, board));
    }

    for field in enemy_view {
        if *board.field_content(&field)
            == Some(Piece {
                kind_of_piece: KindOfPiece::King,
                color: *color,
            })
        {
            return true;
        }
    }

    false
}

fn possible_moves_unchecked(field: &Field, board: &Board) -> Vec<Field> {
    let field_content = board.field_content(field);
    if *field_content == None {
        return Vec::new();
    }

    let piece = field_content.unwrap();
    match piece.kind_of_piece {
        KindOfPiece::King => king_moves_unchecked(field, board),
        KindOfPiece::Queen => queen_moves_unchecked(field, board),
        KindOfPiece::Bishop => bishop_moves_unchecked(field, board),
        KindOfPiece::Knight => knight_moves_unchecked(field, board),
        KindOfPiece::Rook => rook_moves_unchecked(field, board),
        KindOfPiece::Pawn => pawn_moves_unchecked(field, board),
    }
}

fn possible_moves(field: &Field, board: &Board) -> Vec<Move> {
    let field_content = board.field_content(field);
    if *field_content == None {
        return Vec::new();
    }
    let piece = field_content.unwrap();

    let fields = possible_moves_unchecked(field, board);
    let mut moves: Vec<Move> = Vec::new();

    for target_field in fields.into_iter() {
        if piece.kind_of_piece == KindOfPiece::Pawn
            && (target_field.get_row() == 1 || target_field.get_row() == 8)
        {
            moves.push(Move::build(field.clone(), target_field, Some(KindOfPiece::Queen)).unwrap());
            moves.push(Move::build(field.clone(), target_field, Some(KindOfPiece::Knight)).unwrap());
            // Nie ma co rozważać wieży i gońca bo nie dają nic więcej od hetmana.
        } else {
            moves.push(Move::build(field.clone(), target_field, None).unwrap());
        }
    }

    let mut filtered_moves: Vec<Move> = Vec::new();
    for r#move in moves.into_iter() {
        let mut cloned_board = *board;
        cloned_board.apply_unchecked(&r#move);

        if !is_check(&cloned_board, &piece.color) {
            filtered_moves.push(r#move);
        }
    }

    filtered_moves
}

pub fn get_move(board: &Board, turn: &Color) -> Move {
    let (r#move, _) = minimax_single_thread(board, turn, 2);

    r#move
}

pub static mut STOP_ALL_THREADS: AtomicBool = AtomicBool::new(true);

// TODO: minimax_multithreaded

fn minimax_single_thread(board: &Board, turn: &Color, depth: u8) -> (Move, i32) {
    if depth == 0 {
        let quality = board.eval();

        // Zwracamy niepoprawny ruch, ale to nie ma
        // znaczenia, bo depth powinna być co najmniej
        // 1 przy użyciu z zewnątrz.
        if *turn == Color::White {
            return (Move::try_from(0).unwrap(), quality);
        } else {
            return (Move::try_from(0).unwrap(), -quality);
        }
    }

    let moves_to_consider: Vec<Move> = player_moves(turn, board);
    let mut rated_moves: Vec<(Move, i32)> = Vec::new();
    for r#move in moves_to_consider.into_iter() {
        let mut cloned_board = *board;
        cloned_board.apply_unchecked(&r#move);

        rated_moves.push((r#move,
                          minimax_single_thread(&cloned_board,
                                                &turn.enemy(),
                                                depth - 1).1));
    }

    if rated_moves.len() == 0 {
        // FIXME
        panic!();
    }

    let mut best_move = rated_moves[0];
    for rated_move in rated_moves.into_iter() {
        if rated_move.1 > best_move.1 {
            best_move = rated_move;
        }
    }

    best_move
}

fn player_moves(color: &Color, board: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    for row in 1..=8 {
        for file in 1..=8 {
            let field = Field::build_unchecked(row, file);
            if let Some(piece) = board.field_content(&field) {
                if piece.color == *color {
                    moves.append(&mut possible_moves(&field, board));
                }
            }
        }
    }

    moves
}

fn king_moves_unchecked(field: &Field, board: &Board) -> Vec<Field> {
    let to_filter = match (field.get_row(), field.get_file()) {
        (2..=7, 2..=7) => vec![
            Field::build_unchecked(field.get_row() - 1, field.get_file() - 1),
            Field::build_unchecked(field.get_row() - 1, field.get_file()),
            Field::build_unchecked(field.get_row() - 1, field.get_file() + 1),
            Field::build_unchecked(field.get_row(), field.get_file() - 1),
            Field::build_unchecked(field.get_row(), field.get_file() + 1),
            Field::build_unchecked(field.get_row() + 1, field.get_file() - 1),
            Field::build_unchecked(field.get_row() + 1, field.get_file()),
            Field::build_unchecked(field.get_row() + 1, field.get_file() + 1),
        ],
        (2..=7, 1) => vec![
            Field::build_unchecked(field.get_row() - 1, field.get_file()),
            Field::build_unchecked(field.get_row() - 1, field.get_file() + 1),
            Field::build_unchecked(field.get_row(), field.get_file() + 1),
            Field::build_unchecked(field.get_row() + 1, field.get_file()),
            Field::build_unchecked(field.get_row() + 1, field.get_file() + 1),
        ],
        (2..=7, 8) => vec![
            Field::build_unchecked(field.get_row() - 1, field.get_file() - 1),
            Field::build_unchecked(field.get_row() - 1, field.get_file()),
            Field::build_unchecked(field.get_row(), field.get_file() - 1),
            Field::build_unchecked(field.get_row() + 1, field.get_file() - 1),
            Field::build_unchecked(field.get_row() + 1, field.get_file()),
        ],
        (1, 2..=7) => vec![
            Field::build_unchecked(field.get_row(), field.get_file() - 1),
            Field::build_unchecked(field.get_row(), field.get_file() + 1),
            Field::build_unchecked(field.get_row() + 1, field.get_file() - 1),
            Field::build_unchecked(field.get_row() + 1, field.get_file()),
            Field::build_unchecked(field.get_row() + 1, field.get_file() + 1),
        ],
        (8, 2..=7) => vec![
            Field::build_unchecked(field.get_row() - 1, field.get_file() - 1),
            Field::build_unchecked(field.get_row() - 1, field.get_file()),
            Field::build_unchecked(field.get_row() - 1, field.get_file() + 1),
            Field::build_unchecked(field.get_row(), field.get_file() - 1),
            Field::build_unchecked(field.get_row(), field.get_file() + 1),
        ],
        (1, 1) => vec![
            Field::build_unchecked(field.get_row(), field.get_file() + 1),
            Field::build_unchecked(field.get_row() + 1, field.get_file()),
            Field::build_unchecked(field.get_row() + 1, field.get_file() + 1),
        ],
        (1, 8) => vec![
            Field::build_unchecked(field.get_row(), field.get_file() - 1),
            Field::build_unchecked(field.get_row() + 1, field.get_file() - 1),
            Field::build_unchecked(field.get_row() + 1, field.get_file()),
        ],
        (8, 1) => vec![
            Field::build_unchecked(field.get_row() - 1, field.get_file()),
            Field::build_unchecked(field.get_row() - 1, field.get_file() + 1),
            Field::build_unchecked(field.get_row(), field.get_file() + 1),
        ],
        (8, 8) => vec![
            Field::build_unchecked(field.get_row() - 1, field.get_file() - 1),
            Field::build_unchecked(field.get_row() - 1, field.get_file()),
            Field::build_unchecked(field.get_row(), field.get_file() - 1),
        ],

        // Non-existent field.
        _ => vec![],
    };

    let color = match board.field_content(field) {
        Some(piece) => piece.color,
        None => Color::White,
    };
    let mut moves: Vec<Field> = Vec::new();
    for field in to_filter.into_iter() {
        match board.field_content(&field) {
            Some(Piece {
                kind_of_piece: _,
                color: piece_color,
            }) if *piece_color == color => {}
            _ => {
                moves.push(field);
            }
        }
    }

    moves
}

fn go_in_dir(field: &Field, board: &Board, ns: i32, we: i32) -> Vec<Field> {
    let mut moves: Vec<Field> = Vec::new();
    let (file, row) = (field.get_file(), field.get_row());
    let color = match board.field_content(&field) {
        Some(piece) => piece.color,
        None => Color::White, /* UB */
    };

    for i in 1..=8 {
        if let Some(field) = Field::build(row as i32 + i * ns, file as i32 + i * we) {
            let field_content = *board.field_content(&field);

            if field_content == None {
                moves.push(field);
                continue;
            }

            if field_content.unwrap().color == color {
                break;
            } else {
                moves.push(field);
                break;
            }
        }
    }

    moves
}

fn queen_moves_unchecked(field: &Field, board: &Board) -> Vec<Field> {
    let mut moves: Vec<Field> = Vec::new();

    moves.append(&mut go_in_dir(field, board, 1, 0));
    moves.append(&mut go_in_dir(field, board, 1, 1));
    moves.append(&mut go_in_dir(field, board, 1, -1));
    moves.append(&mut go_in_dir(field, board, 0, 1));
    moves.append(&mut go_in_dir(field, board, 0, -1));
    moves.append(&mut go_in_dir(field, board, -1, 0));
    moves.append(&mut go_in_dir(field, board, -1, 1));
    moves.append(&mut go_in_dir(field, board, -1, -1));

    moves
}

fn bishop_moves_unchecked(field: &Field, board: &Board) -> Vec<Field> {
    let mut moves: Vec<Field> = Vec::new();

    moves.append(&mut go_in_dir(field, board, 1, 1));
    moves.append(&mut go_in_dir(field, board, -1, 1));
    moves.append(&mut go_in_dir(field, board, -1, -1));
    moves.append(&mut go_in_dir(field, board, 1, -1));

    moves
}

fn knight_moves_unchecked(field: &Field, board: &Board) -> Vec<Field> {
    let to_filter = [
        Field::build(field.get_row() as i32 - 1, field.get_file() as i32 - 2),
        Field::build(field.get_row() as i32 - 2, field.get_file() as i32 - 1),
        Field::build(field.get_row() as i32 - 2, field.get_file() as i32 + 1),
        Field::build(field.get_row() as i32 - 1, field.get_file() as i32 + 2),
        Field::build(field.get_row() as i32 + 1, field.get_file() as i32 + 2),
        Field::build(field.get_row() as i32 + 2, field.get_file() as i32 + 1),
        Field::build(field.get_row() as i32 + 2, field.get_file() as i32 - 1),
        Field::build(field.get_row() as i32 + 1, field.get_file() as i32 - 2),
    ];

    let color = match board.field_content(field) {
        Some(piece) => piece.color,
        None => Color::White,
    };
    let mut moves: Vec<Field> = Vec::new();
    for field in to_filter.into_iter() {
        if let Some(field) = field {
            match board.field_content(&field) {
                Some(Piece {
                    kind_of_piece: _,
                    color: piece_color,
                }) if *piece_color == color => {}
                _ => {
                    moves.push(field);
                }
            }
        }
    }

    moves
}

fn rook_moves_unchecked(field: &Field, board: &Board) -> Vec<Field> {
    let mut moves: Vec<Field> = Vec::new();

    moves.append(&mut go_in_dir(field, board, 1, 0));
    moves.append(&mut go_in_dir(field, board, -1, 0));
    moves.append(&mut go_in_dir(field, board, 0, 1));
    moves.append(&mut go_in_dir(field, board, 0, -1));

    moves
}

fn pawn_moves_unchecked(field: &Field, board: &Board) -> Vec<Field> {
    let mut moves: Vec<Field> = Vec::new();
    let color = match board.field_content(field) {
        Some(piece) => piece.color,
        None => Color::White,
    };

    let next_row = match color {
        Color::White => field.get_row() as i32 + 1,
        Color::Black => field.get_row() as i32 - 1,
    };
    let next2_row = match color {
        Color::White => field.get_row() as i32 + 2,
        Color::Black => field.get_row() as i32 - 2,
    };

    if let Some(next_field) = Field::build(next_row, field.get_file() as i32) {
        if *board.field_content(&next_field) == None {
            moves.push(next_field);
            if (color == Color::White && field.get_row() == 2)
                || (color == Color::Black && field.get_row() == 7)
            {
                if let Some(next2_field) = Field::build(next2_row, field.get_file() as i32) {
                    if *board.field_content(&next2_field) == None {
                        moves.push(next2_field);
                    }
                }
            }
        }
    }

    let nw_field = Field::build(next_row as i32, field.get_file() as i32 - 1);
    if let Some(field) = nw_field {
        if let Some(piece) = *board.field_content(&field) {
            if piece.color != color {
                moves.push(field);
            }
        }
    }

    let ne_field = Field::build(next_row as i32, field.get_file() as i32 + 1);
    if let Some(field) = ne_field {
        if let Some(piece) = *board.field_content(&field) {
            if piece.color != color {
                moves.push(field);
            }
        }
    }

    let e_field = Field::build(field.get_row() as i32, field.get_file() as i32 + 1);
    if let Some(field) = e_field {
        if board.can_en_passant(field.get_file())
            && ((color == Color::White && field.get_row() == 5)
                || (color == Color::Black && field.get_row() == 4))
        {
            if let Some(piece) = *board.field_content(&field) {
                if piece.color != color {
                    moves.push(field);
                }
            }
        }
    }

    let w_field = Field::build(field.get_row() as i32, field.get_file() as i32 - 1);
    if let Some(field) = w_field {
        if board.can_en_passant(field.get_file())
            && ((color == Color::White && field.get_row() == 5)
                || (color == Color::Black && field.get_row() == 4))
        {
            if let Some(piece) = *board.field_content(&field) {
                if piece.color != color {
                    moves.push(field);
                }
            }
        }
    }

    moves
}
