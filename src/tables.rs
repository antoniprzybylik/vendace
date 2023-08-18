use super::board::Field;
use super::board::Piece;
use super::board::KindOfPiece;
use super::board::Color;

static PIECE_VALUES: [i32; 6] = [100, 320, 330, 500, 900, 20000];

static FIELD_VALUES: [[[i32; 8]; 8]; 7] = [
    [ // Pawn
        [ 0,  0,  0,  0,  0,  0,  0,  0 ],
        [ 50, 50, 50, 50, 50, 50, 50, 50 ],
        [ 10, 10, 20, 30, 30, 20, 10, 10 ],
        [ 5,  5, 10, 25, 25, 10,  5,  5 ],
        [ 0,  0,  0, 20, 20,  0,  0,  0 ],
        [ 5, -5,-10,  0,  0,-10, -5,  5 ],
        [ 5, 10, 10,-20,-20, 10, 10,  5 ],
        [ 0,  0,  0,  0,  0,  0,  0,  0 ]
    ],
    [ // Knight
        [ -50,-40,-30,-30,-30,-30,-40,-50 ],
        [ -40,-20,  0,  0,  0,  0,-20,-40 ],
        [ -30,  0, 10, 15, 15, 10,  0,-30 ],
        [ -30,  5, 15, 20, 20, 15,  5,-30 ],
        [ -30,  0, 15, 20, 20, 15,  0,-30 ],
        [ -30,  5, 10, 15, 15, 10,  5,-30 ],
        [ -40,-20,  0,  5,  5,  0,-20,-40 ],
        [ -50,-40,-30,-30,-30,-30,-40,-50 ]
    ],
    [ // Bishop
        [ -20,-10,-10,-10,-10,-10,-10,-20 ],
        [ -10,  0,  0,  0,  0,  0,  0,-10 ],
        [ -10,  0,  5, 10, 10,  5,  0,-10 ],
        [ -10,  5,  5, 10, 10,  5,  5,-10 ],
        [ -10,  0, 10, 10, 10, 10,  0,-10 ],
        [ -10, 10, 10, 10, 10, 10, 10,-10 ],
        [ -10,  5,  0,  0,  0,  0,  5,-10 ],
        [ -20,-10,-10,-10,-10,-10,-10,-20 ],
    ],
    [ // Rook
        [ 0,  0,  0,  0,  0,  0,  0,  0 ],
        [ 5, 10, 10, 10, 10, 10, 10,  5 ],
        [ -5,  0,  0,  0,  0,  0,  0, -5 ],
        [ -5,  0,  0,  0,  0,  0,  0, -5 ],
        [ -5,  0,  0,  0,  0,  0,  0, -5 ],
        [ -5,  0,  0,  0,  0,  0,  0, -5 ],
        [ -5,  0,  0,  0,  0,  0,  0, -5 ],
        [ 0,  0,  0,  5,  5,  0,  0,  0 ]
    ],
    [ // Queen
        [ -20,-10,-10, -5, -5,-10,-10,-20 ],
        [ -10,  0,  0,  0,  0,  0,  0,-10 ],
        [ -10,  0,  5,  5,  5,  5,  0,-10 ],
        [ -5,  0,  5,  5,  5,  5,  0, -5 ],
        [ 0,  0,  5,  5,  5,  5,  0, -5 ],
        [ -10,  5,  5,  5,  5,  5,  0,-10 ],
        [ -10,  0,  5,  0,  0,  0,  0,-10 ],
        [ -20,-10,-10, -5, -5,-10,-10,-20 ]
    ],
    [ // King (at middlegame)
        [ -30,-40,-40,-50,-50,-40,-40,-30 ],
        [ -30,-40,-40,-50,-50,-40,-40,-30 ],
        [ -30,-40,-40,-50,-50,-40,-40,-30 ],
        [ -30,-40,-40,-50,-50,-40,-40,-30 ],
        [ -20,-30,-30,-40,-40,-30,-30,-20 ],
        [ -10,-20,-20,-20,-20,-20,-20,-10 ],
        [ 20, 20,  0,  0,  0,  0, 20, 20 ],
        [ 20, 30, 10,  0,  0, 10, 30, 20 ]

    ],
    [ // King (at the endgame)
        [ -50,-40,-30,-20,-20,-30,-40,-50 ],
        [ -30,-20,-10,  0,  0,-10,-20,-30 ],
        [ -30,-10, 20, 30, 30, 20,-10,-30 ],
        [ -30,-10, 30, 40, 40, 30,-10,-30 ],
        [ -30,-10, 30, 40, 40, 30,-10,-30 ],
        [ -30,-10, 20, 30, 30, 20,-10,-30 ],
        [ -30,-30,  0,  0,  0,  0,-30,-30 ],
        [ -50,-30,-30,-30,-30,-30,-30,-50 ]
    ],
];

pub fn piece_value(field: &Field, piece: &Piece, early_stage: bool) -> i32 {
    let (c1, c2) = match piece.color {
        Color::Black => ((field.get_row()-1) as usize,
                         (field.get_file()-1) as usize),
        Color::White => ((8-field.get_row()) as usize,
                         (field.get_file()-1) as usize),
    };

    let value = match piece.kind_of_piece {
        KindOfPiece::Pawn => PIECE_VALUES[0] +
            FIELD_VALUES[0][c1][c2],
        KindOfPiece::Knight => PIECE_VALUES[1] +
            FIELD_VALUES[1][c1][c2],
        KindOfPiece::Bishop => PIECE_VALUES[2] +
            FIELD_VALUES[2][c1][c2],
        KindOfPiece::Rook => PIECE_VALUES[3] +
            FIELD_VALUES[3][c1][c2],
        KindOfPiece::Queen => PIECE_VALUES[4] +
            FIELD_VALUES[4][c1][c2],
        KindOfPiece::King => PIECE_VALUES[5] +
            if early_stage { FIELD_VALUES[5][c1][c2] }
            else { FIELD_VALUES[6][c1][c2] },
    };

    value * match piece.color {
        Color::White => 1,
        Color::Black => -1,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::board::Board;
    use super::super::book::Move;

    #[test]
    fn test_piece_value_1() {
        let board = Board::new();
        let mut sum: i32 = 0;

        for i in 1..=8 {
            for j in 1..=8 {
                let field = Field::build_unchecked(i, j);
                let value = match board.field_content(&field) {
                    Some(piece) => piece_value(&field, &piece, true),
                    None => 0,
                };

                sum += value;
            }
        }

        assert_eq!(sum, 0i32);
    }

    #[test]
    fn test_piece_value_2() {
        let mut board = Board::new();
        board.apply_unchecked(&Move::try_from("e2e4").unwrap());
        let mut sum: i32 = 0;

        for i in 1..=8 {
            for j in 1..=8 {
                let field = Field::build_unchecked(i, j);
                let value = match board.field_content(&field) {
                    Some(piece) => piece_value(&field, &piece, true),
                    None => 0,
                };

                sum += value;
            }
        }

        assert_eq!(sum, 40i32);
    }
}
