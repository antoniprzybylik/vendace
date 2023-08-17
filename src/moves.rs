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

pub fn player_moves(color: &Color, board: &Board) -> Vec<Move> {
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

fn king_moves_unchecked(field: &Field, _board: &Board) -> Vec<Field> {
    match (field.get_row(), field.get_file()) {
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
    }
}

fn go_in_dir(field: &Field, board: &Board, we: i32, ns: i32) -> Vec<Field> {
    let mut moves: Vec<Field> = Vec::new();
    let (file, row) = (field.get_file(), field.get_row());
    let color = match board.field_content(&field) {
        Some(piece) => piece.color,
        None => Color::White, /* UB */
    };

    for i in 1..=8 {
        if let Some(field) = Field::build(file as i32 + i * we, row as i32 + i * ns) {
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

fn knight_moves_unchecked(field: &Field, _board: &Board) -> Vec<Field> {
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

    let mut moves: Vec<Field> = Vec::new();
    for field in to_filter.into_iter() {
        if let Some(field) = field {
            moves.push(field);
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
        } else if (color == Color::White && field.get_row() == 2)
            || (color == Color::Black && field.get_row() == 7)
        {
            if let Some(next2_field) = Field::build(next2_row, field.get_file() as i32) {
                if *board.field_content(&next2_field) == None {
                    moves.push(next2_field);
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
        if board.can_en_passant(field.get_file() + 1)
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
        if board.can_en_passant(field.get_file() - 1)
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
