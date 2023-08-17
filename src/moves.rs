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
            let field = Field { row, file };
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
                        && (target_field.row == 1 || target_field.row
                             == 8) {
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
            let field = Field { row, file };
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
    match (field.row, field.file) {
        (1..=6, 1..=6) => vec![
            Field {
                row: field.row - 1,
                file: field.file - 1,
            },
            Field {
                row: field.row - 1,
                file: field.file,
            },
            Field {
                row: field.row - 1,
                file: field.file + 1,
            },
            Field {
                row: field.row,
                file: field.file - 1,
            },
            Field {
                row: field.row,
                file: field.file + 1,
            },
            Field {
                row: field.row + 1,
                file: field.file - 1,
            },
            Field {
                row: field.row + 1,
                file: field.file,
            },
            Field {
                row: field.row + 1,
                file: field.file + 1,
            },
        ],
        (1..=6, 0) => vec![
            Field {
                row: field.row - 1,
                file: field.file,
            },
            Field {
                row: field.row - 1,
                file: field.file + 1,
            },
            Field {
                row: field.row,
                file: field.file + 1,
            },
            Field {
                row: field.row + 1,
                file: field.file,
            },
            Field {
                row: field.row + 1,
                file: field.file + 1,
            },
        ],
        (1..=6, 7) => vec![
            Field {
                row: field.row - 1,
                file: field.file - 1,
            },
            Field {
                row: field.row - 1,
                file: field.file,
            },
            Field {
                row: field.row,
                file: field.file - 1,
            },
            Field {
                row: field.row + 1,
                file: field.file - 1,
            },
            Field {
                row: field.row + 1,
                file: field.file,
            },
        ],
        (0, 1..=6) => vec![
            Field {
                row: field.row,
                file: field.file - 1,
            },
            Field {
                row: field.row,
                file: field.file + 1,
            },
            Field {
                row: field.row + 1,
                file: field.file - 1,
            },
            Field {
                row: field.row + 1,
                file: field.file,
            },
            Field {
                row: field.row + 1,
                file: field.file + 1,
            },
        ],
        (7, 1..=6) => vec![
            Field {
                row: field.row - 1,
                file: field.file - 1,
            },
            Field {
                row: field.row - 1,
                file: field.file,
            },
            Field {
                row: field.row - 1,
                file: field.file + 1,
            },
            Field {
                row: field.row,
                file: field.file - 1,
            },
            Field {
                row: field.row,
                file: field.file + 1,
            },
        ],
        (0, 0) => vec![
            Field {
                row: field.row,
                file: field.file + 1,
            },
            Field {
                row: field.row + 1,
                file: field.file,
            },
            Field {
                row: field.row + 1,
                file: field.file + 1,
            },
        ],
        (0, 7) => vec![
            Field {
                row: field.row,
                file: field.file - 1,
            },
            Field {
                row: field.row + 1,
                file: field.file - 1,
            },
            Field {
                row: field.row + 1,
                file: field.file,
            },
        ],
        (7, 0) => vec![
            Field {
                row: field.row - 1,
                file: field.file,
            },
            Field {
                row: field.row - 1,
                file: field.file + 1,
            },
            Field {
                row: field.row,
                file: field.file + 1,
            },
        ],
        (7, 7) => vec![
            Field {
                row: field.row - 1,
                file: field.file - 1,
            },
            Field {
                row: field.row - 1,
                file: field.file,
            },
            Field {
                row: field.row,
                file: field.file - 1,
            },
        ],

        // Non-existent field.
        _ => vec![],
    }
}

fn queen_moves_unchecked(field: &Field, board: &Board) -> Vec<Field> {
    let mut moves: Vec<Field> = Vec::new();
    let (file, row) = (field.file, field.row);
    let color = match board.field_content(&field) {
        Some(piece) => piece.color,
        None => Color::White, /* UB */
    };

    for i in 1..=8 {
        let field = Field { file, row: row + i };
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

    for i in 1..=8 {
        let field = Field {
            file: file + i,
            row: row + i,
        };
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

    for i in 1..=8 {
        let field = Field { file, row: row - i };
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

    for i in 1..=8 {
        let field = Field {
            file: file + i,
            row: row - i,
        };
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

    for i in 1..=8 {
        let field = Field {
            file: file + i,
            row,
        };
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

    for i in 1..=8 {
        let field = Field {
            file: file - i,
            row: row - i,
        };
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

    for i in 1..=8 {
        let field = Field {
            file: file - i,
            row: row + 1,
        };
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

    moves
}

fn bishop_moves_unchecked(field: &Field, board: &Board) -> Vec<Field> {
    let mut moves: Vec<Field> = Vec::new();
    let (file, row) = (field.file, field.row);
    let color = match board.field_content(&field) {
        Some(piece) => piece.color,
        None => Color::White, /* UB */
    };

    for i in 1..=8 {
        let field = Field {
            file: file + i,
            row: row + i,
        };
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

    for i in 1..=8 {
        let field = Field {
            file: file - i,
            row: row - i,
        };
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

    for i in 1..=8 {
        let field = Field {
            file: file + i,
            row: row - i,
        };
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

    for i in 1..=8 {
        let field = Field {
            file: file - i,
            row: row + 1,
        };
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

    moves
}

fn knight_moves_unchecked(field: &Field, _board: &Board) -> Vec<Field> {
    let to_filter = [
        Field {
            row: field.row - 1,
            file: field.file - 2,
        },
        Field {
            row: field.row - 2,
            file: field.file - 1,
        },
        Field {
            row: field.row - 2,
            file: field.file + 1,
        },
        Field {
            row: field.row - 1,
            file: field.file + 2,
        },
        Field {
            row: field.row + 1,
            file: field.file + 2,
        },
        Field {
            row: field.row + 2,
            file: field.file + 1,
        },
        Field {
            row: field.row + 2,
            file: field.file - 1,
        },
        Field {
            row: field.row + 1,
            file: field.file - 2,
        },
    ];

    let mut moves: Vec<Field> = Vec::new();
    for field in to_filter.into_iter() {
        if field.on_board() {
            moves.push(field);
        }
    }

    moves
}

fn rook_moves_unchecked(field: &Field, board: &Board) -> Vec<Field> {
    let mut moves: Vec<Field> = Vec::new();
    let (file, row) = (field.file, field.row);
    let color = match board.field_content(&field) {
        Some(piece) => piece.color,
        None => Color::White, /* UB */
    };

    for i in 1..=8 {
        let field = Field { file, row: row + i };
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

    for i in 1..=8 {
        let field = Field { file, row: row - i };
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

    for i in 1..=8 {
        let field = Field {
            file: file + i,
            row,
        };
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

    for i in 1..=8 {
        let field = Field {
            file: file - i,
            row,
        };
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

    moves
}

fn pawn_moves_unchecked(field: &Field, board: &Board) -> Vec<Field> {
    let mut moves: Vec<Field> = Vec::new();
    let color = match board.field_content(field) {
        Some(piece) => piece.color,
        None => Color::White,
    };

    let next_row = match color {
        Color::White => field.row + 1,
        Color::Black => field.row - 1,
    };

    let next_field = Field {
        row: next_row,
        file: field.file,
    };
    if *board.field_content(&next_field) == None {
        moves.push(next_field);
    }

    let nw_field = Field {
        row: next_row,
        file: field.file - 1,
    };
    if nw_field.on_board() {
        if let Some(piece) = *board.field_content(&nw_field) {
            if piece.color != color {
                moves.push(nw_field);
            }
        }
    }

    let ne_field = Field {
        row: next_row,
        file: field.file + 1,
    };
    if ne_field.on_board() {
        if let Some(piece) = *board.field_content(&ne_field) {
            if piece.color != color {
                moves.push(ne_field);
            }
        }
    }

    let e_field = Field {
        row: field.row,
        file: field.file + 1,
    };
    if e_field.on_board()
        && board.can_en_passant(field.file + 1)
        && ((color == Color::White && field.row == 5) || (color == Color::Black && field.row == 4))
    {
        if let Some(piece) = *board.field_content(&e_field) {
            if piece.color != color {
                moves.push(e_field);
            }
        }
    }

    let w_field = Field {
        row: field.row,
        file: field.file - 1,
    };
    if w_field.on_board()
        && board.can_en_passant(field.file - 1)
        && ((color == Color::White && field.row == 5) || (color == Color::Black && field.row == 4))
    {
        if let Some(piece) = *board.field_content(&w_field) {
            if piece.color != color {
                moves.push(w_field);
            }
        }
    }

    moves
}
