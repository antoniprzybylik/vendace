use std::cmp;
use std::fmt;

use super::book::Move;
use super::polyglot_data::RANDOM_CASTLE;
use super::polyglot_data::RANDOM_EN_PASSANT;
use super::polyglot_data::RANDOM_PIECE;
use super::polyglot_data::RANDOM_TURN;
use super::tables::piece_value;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum KindOfPiece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl fmt::Display for KindOfPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                KindOfPiece::Pawn => "p",
                KindOfPiece::Knight => "n",
                KindOfPiece::Bishop => "b",
                KindOfPiece::Rook => "r",
                KindOfPiece::Queen => "q",
                KindOfPiece::King => "k",
            }
        )
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn enemy(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Castle {
    Short,
    Long,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Piece {
    pub kind_of_piece: KindOfPiece,
    pub color: Color,
}

const WHITE_ROOK: Piece = Piece {
    kind_of_piece: KindOfPiece::Rook,
    color: Color::White,
};
const BLACK_ROOK: Piece = Piece {
    kind_of_piece: KindOfPiece::Rook,
    color: Color::Black,
};
const WHITE_KNIGHT: Piece = Piece {
    kind_of_piece: KindOfPiece::Knight,
    color: Color::White,
};
const BLACK_KNIGHT: Piece = Piece {
    kind_of_piece: KindOfPiece::Knight,
    color: Color::Black,
};
const WHITE_BISHOP: Piece = Piece {
    kind_of_piece: KindOfPiece::Bishop,
    color: Color::White,
};
const BLACK_BISHOP: Piece = Piece {
    kind_of_piece: KindOfPiece::Bishop,
    color: Color::Black,
};
const WHITE_QUEEN: Piece = Piece {
    kind_of_piece: KindOfPiece::Queen,
    color: Color::White,
};
const BLACK_QUEEN: Piece = Piece {
    kind_of_piece: KindOfPiece::Queen,
    color: Color::Black,
};
const WHITE_KING: Piece = Piece {
    kind_of_piece: KindOfPiece::King,
    color: Color::White,
};
const BLACK_KING: Piece = Piece {
    kind_of_piece: KindOfPiece::King,
    color: Color::Black,
};
const WHITE_PAWN: Piece = Piece {
    kind_of_piece: KindOfPiece::Pawn,
    color: Color::White,
};
const BLACK_PAWN: Piece = Piece {
    kind_of_piece: KindOfPiece::Pawn,
    color: Color::Black,
};

impl Piece {
    pub fn code(&self) -> u8 {
        let code = match self.kind_of_piece {
            KindOfPiece::Pawn => 0,
            KindOfPiece::Knight => 1,
            KindOfPiece::Bishop => 2,
            KindOfPiece::Rook => 3,
            KindOfPiece::Queen => 4,
            KindOfPiece::King => 5,
        };

        match self.color {
            Color::Black => code * 2,
            Color::White => code * 2 + 1,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Board {
    fields: [[Option<Piece>; 8]; 8],
    castle: [bool; 4],
    en_passant: [bool; 8],
    turn: Color,
}

impl Board {
    /// Constructor.
    pub fn new() -> Self {
        Board {
            fields: [
                [
                    Some(WHITE_ROOK),
                    Some(WHITE_KNIGHT),
                    Some(WHITE_BISHOP),
                    Some(WHITE_QUEEN),
                    Some(WHITE_KING),
                    Some(WHITE_BISHOP),
                    Some(WHITE_KNIGHT),
                    Some(WHITE_ROOK),
                ],
                [
                    Some(WHITE_PAWN),
                    Some(WHITE_PAWN),
                    Some(WHITE_PAWN),
                    Some(WHITE_PAWN),
                    Some(WHITE_PAWN),
                    Some(WHITE_PAWN),
                    Some(WHITE_PAWN),
                    Some(WHITE_PAWN),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some(BLACK_PAWN),
                    Some(BLACK_PAWN),
                    Some(BLACK_PAWN),
                    Some(BLACK_PAWN),
                    Some(BLACK_PAWN),
                    Some(BLACK_PAWN),
                    Some(BLACK_PAWN),
                    Some(BLACK_PAWN),
                ],
                [
                    Some(BLACK_ROOK),
                    Some(BLACK_KNIGHT),
                    Some(BLACK_BISHOP),
                    Some(BLACK_QUEEN),
                    Some(BLACK_KING),
                    Some(BLACK_BISHOP),
                    Some(BLACK_KNIGHT),
                    Some(BLACK_ROOK),
                ],
            ],
            castle: [true, true, true, true],
            en_passant: [false, false, false, false, false, false, false, false],
            turn: Color::White,
        }
    }

    /// Zobrist hash of the board.
    pub fn hash(&self) -> u64 {
        let mut key: u64 = 0;

        for r in 0..self.fields.len() {
            for c in 0..self.fields[r].len() {
                let field = Field {
                    row: r as u8,
                    file: c as u8,
                };

                if let Some(ref piece) = self.fields[r][c] {
                    key ^= field_hash(piece, &field);
                }
            }
        }

        for i in 0..4 {
            if self.castle[i] {
                key ^= RANDOM_CASTLE[i];
            }
        }

        for i in 0..8 {
            if self.en_passant[i] {
                key ^= RANDOM_EN_PASSANT[i];
            }
        }

        match self.turn {
            Color::White => key ^ RANDOM_TURN[0],
            Color::Black => key,
        }
    }

    /// Can player of color `color` castle on side `side`.
    pub fn can_castle(&self, color: Color, side: Castle) -> bool {
        match (color, side) {
            (Color::White, Castle::Short) => self.castle[0],
            (Color::White, Castle::Long) => self.castle[1],
            (Color::Black, Castle::Short) => self.castle[2],
            (Color::Black, Castle::Long) => self.castle[3],
        }
    }

    /// Can a pawn in file `file` be captured en passant.
    pub fn can_en_passant(&self, file: u8) -> bool {
        self.en_passant[(file - 1) as usize]
    }

    /// Apply move without performing checks.
    pub fn apply_unchecked(&mut self, r#move: &Move) {
        let field_content =
            self.fields[(r#move.from_row() - 1) as usize][(r#move.from_file_number() - 1) as usize];

        self.fields[(r#move.from_row() - 1) as usize][(r#move.from_file_number() - 1) as usize] =
            None;

        if let Some(piece_kind) = r#move.promotion() {
            self.fields[(r#move.to_row() - 1) as usize][(r#move.to_file_number() - 1) as usize] =
                Some(Piece {
                    kind_of_piece: piece_kind,
                    color: if let Some(piece) = field_content {
                        piece.color
                    } else {
                        panic!("Failed to apply move.")
                    },
                });
        } else {
            self.fields[(r#move.to_row() - 1) as usize][(r#move.to_file_number() - 1) as usize] =
                field_content;
        }

        if let Some(piece) = field_content {
            // Krótka roszada.
            // Przesuwamy wieżę.
            if piece.kind_of_piece == KindOfPiece::King
                && r#move.from_file_number() == 5
                && r#move.to_file_number() == 7
            {
                self.fields[(r#move.from_row() - 1) as usize][7] = None;
                self.fields[(r#move.from_row() - 1) as usize][5] = Some(Piece {
                    kind_of_piece: KindOfPiece::Rook,
                    color: piece.color,
                });
            }

            // Długa roszada.
            // Przesuwamy wieżę.
            if piece.kind_of_piece == KindOfPiece::King
                && r#move.from_file_number() == 5
                && r#move.to_file_number() == 3
            {
                self.fields[(r#move.from_row() - 1) as usize][0] = None;
                self.fields[(r#move.from_row() - 1) as usize][3] = Some(Piece {
                    kind_of_piece: KindOfPiece::Rook,
                    color: piece.color,
                });
            }

            // Jeżeli ruszył się król:
            // Unieważniamy roszady tego
            // koloru.
            if piece.kind_of_piece == KindOfPiece::King {
                match piece.color {
                    Color::White => {
                        self.castle[0] = false;
                        self.castle[1] = false;
                    }
                    Color::Black => {
                        self.castle[2] = false;
                        self.castle[3] = false;
                    }
                }
            }

            // Biały bije przelotem.
            if piece.color == Color::White
                && self.en_passant[(r#move.to_file_number() - 1) as usize]
                && r#move.to_row() == 6
            {
                let above_content = &mut self.fields[(r#move.to_row() - 2) as usize]
                    [(r#move.to_file_number() - 1) as usize];
                if let Some(piece) = above_content {
                    if *piece
                        == (Piece {
                            kind_of_piece: KindOfPiece::Pawn,
                            color: Color::Black,
                        })
                    {
                        *above_content = None;
                    }
                }
            }

            // Czarny bije przelotem.
            if piece.color == Color::Black
                && self.en_passant[(r#move.to_file_number() - 1) as usize]
                && r#move.to_row() == 3
            {
                let under_content = &mut self.fields[(r#move.to_row()) as usize]
                    [(r#move.to_file_number() - 1) as usize];
                if let Some(piece) = under_content {
                    if *piece
                        == (Piece {
                            kind_of_piece: KindOfPiece::Pawn,
                            color: Color::White,
                        })
                    {
                        *under_content = None;
                    }
                }
            }

            // Unieważniamy możliwe bicia przelotem z poprzedniej rundy.
            self.en_passant = [false; 8];

            // Biały może być zbity przelotem.
            if piece.color == Color::White && r#move.to_row() == 4 {
                if (r#move.to_file_number() == 1
                    || self.fields[(r#move.to_row() - 1) as usize]
                        [(r#move.to_file_number() - 2) as usize]
                        == Some(Piece {
                            kind_of_piece: KindOfPiece::Pawn,
                            color: Color::Black,
                        }))
                    && (r#move.to_file_number() == 8
                        || self.fields[(r#move.to_row() - 1) as usize]
                            [(r#move.to_file_number()) as usize]
                            == Some(Piece {
                                kind_of_piece: KindOfPiece::Pawn,
                                color: Color::Black,
                            }))
                {
                    self.en_passant[(r#move.to_file_number() - 1) as usize] = true;
                }
            }

            // Czarny może być zbity przelotem.
            if piece.color == Color::Black && r#move.to_row() == 5 {
                if (r#move.to_file_number() == 1
                    || self.fields[(r#move.to_row() - 1) as usize]
                        [(r#move.to_file_number() - 2) as usize]
                        == Some(Piece {
                            kind_of_piece: KindOfPiece::Pawn,
                            color: Color::White,
                        }))
                    && (r#move.to_file_number() == 8
                        || self.fields[(r#move.to_row() - 1) as usize]
                            [(r#move.to_file_number()) as usize]
                            == Some(Piece {
                                kind_of_piece: KindOfPiece::Pawn,
                                color: Color::White,
                            }))
                {
                    self.en_passant[(r#move.to_file_number() - 1) as usize] = true;
                }
            }
        }
    }

    /// Content of field `field`.
    pub fn field_content(&self, field: &Field) -> &Option<Piece> {
        &self.fields[(field.row - 1) as usize][(field.file - 1) as usize]
    }

    /// Which turn.
    pub fn which_turn(&self) -> Color {
        self.turn
    }

    /// Next turn.
    pub fn next_turn(&mut self) {
        self.turn = self.turn.enemy();
    }

    /// Evaluate board.
    pub fn eval(&self) -> i32 {
        let mut sum: i32 = 0;

        for i in 1..=8 {
            for j in 1..=8 {
                let field = Field::build_unchecked(i, j);
                let value = match self.field_content(&field) {
                    Some(piece) => piece_value(&field, &piece, true),
                    None => 0,
                };

                sum += value;
            }
        }

        sum
    }
}

// TODO: Test apply_unchecked()

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string: String = String::new();

        for row in 0..8 {
            for file in 0..8 {
                let field_content = self.fields[row][file];
                if let Some(piece) = field_content {
                    string += match piece.color {
                        Color::White => "w",
                        Color::Black => "b",
                    };

                    string += match piece.kind_of_piece {
                        KindOfPiece::King => "k",
                        KindOfPiece::Queen => "q",
                        KindOfPiece::Bishop => "b",
                        KindOfPiece::Knight => "n",
                        KindOfPiece::Rook => "r",
                        KindOfPiece::Pawn => "p",
                    };
                } else {
                    string += "  ";
                }
            }
            string += "\n";
        }

        write!(f, "{}", string)
    }
}

impl std::convert::TryFrom<FENString> for Board {
    type Error = ();

    /// Create `Board` object from FENString.
    fn try_from(f: FENString) -> Result<Self, Self::Error> {
        let mut fields: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
        let mut castle: [bool; 4] = [false; 4];
        let mut en_passant: [bool; 8] = [false; 8];
        let turn: Color;

        let f = f;
        for i in 0..f.rows.len() {
            let row = &f.rows[7 - i].as_bytes();

            let mut pos = 0usize;
            for p in row.iter() {
                match p {
                    num @ b'1'..=b'8' => {
                        let num = *num - b'1' + 1;
                        let end = num as usize + pos;
                        for j in pos..end {
                            fields[i][j] = None;
                        }
                        pos += num as usize;
                    }
                    b'p' => {
                        fields[i][pos] = Some(BLACK_PAWN);
                        pos += 1;
                    }
                    b'P' => {
                        fields[i][pos] = Some(WHITE_PAWN);
                        pos += 1;
                    }
                    b'r' => {
                        fields[i][pos] = Some(BLACK_ROOK);
                        pos += 1;
                    }
                    b'R' => {
                        fields[i][pos] = Some(WHITE_ROOK);
                        pos += 1;
                    }
                    b'n' => {
                        fields[i][pos] = Some(BLACK_KNIGHT);
                        pos += 1;
                    }
                    b'N' => {
                        fields[i][pos] = Some(WHITE_KNIGHT);
                        pos += 1;
                    }
                    b'b' => {
                        fields[i][pos] = Some(BLACK_BISHOP);
                        pos += 1;
                    }
                    b'B' => {
                        fields[i][pos] = Some(WHITE_BISHOP);
                        pos += 1;
                    }
                    b'q' => {
                        fields[i][pos] = Some(BLACK_QUEEN);
                        pos += 1;
                    }
                    b'Q' => {
                        fields[i][pos] = Some(WHITE_QUEEN);
                        pos += 1;
                    }
                    b'k' => {
                        fields[i][pos] = Some(BLACK_KING);
                        pos += 1;
                    }
                    b'K' => {
                        fields[i][pos] = Some(WHITE_KING);
                        pos += 1;
                    }
                    _ => return Err(()),
                }
            }

            if pos != 8 {
                return Err(());
            }
        }

        turn = match f.turn {
            b'b' => Color::Black,
            b'w' => Color::White,
            _ => return Err(()),
        };

        for p in f.castle {
            match p {
                b'A' => {
                    castle[0] = true;
                }
                b'H' => {
                    castle[1] = true;
                }
                b'a' => {
                    castle[2] = true;
                }
                b'h' => {
                    castle[3] = true;
                }
                b'-' => {}
                _ => return Err(()),
            };
        }

        if f.en_passant != "-" {
            let mut string: &str = f.en_passant.as_str();
            let mut pos: &str;
            while !string.is_empty() {
                (pos, string) = string.split_at(cmp::min(2, string.len()));

                let field = Field::try_from(pos)?;
                if field.row != 3 && field.row != 6 {
                    return Err(());
                }

                en_passant[(field.file - 1) as usize] = true;
            }
        }

        Ok(Board {
            fields,
            castle,
            en_passant,
            turn,
        })
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Field {
    row: u8,
    file: u8,
}

impl Field {
    pub fn build_unchecked(row: u8, file: u8) -> Self {
        match (file, row) {
            (1..=8, 1..=8) => Field {
                file: file as u8,
                row: row as u8,
            },
            _ => panic!(),
        }
    }

    pub fn build(row: i32, file: i32) -> Option<Self> {
        match (file, row) {
            (1..=8, 1..=8) => Some(Field {
                file: file as u8,
                row: row as u8,
            }),
            _ => None,
        }
    }

    pub fn get_file(&self) -> u8 {
        self.file
    }

    pub fn get_row(&self) -> u8 {
        self.row
    }
}

impl std::convert::TryFrom<&str> for Field {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let pos = s.as_bytes();
        if pos.len() != 2 {
            return Err(());
        }

        let file: u8 = if pos[0] >= b'a' && pos[0] <= b'h' {
            pos[0] - b'a' + 1
        } else if pos[0] >= b'A' && pos[0] <= b'H' {
            pos[0] - b'A' + 1
        } else {
            return Err(());
        };

        let row: u8 = if pos[1] >= b'1' && pos[1] <= b'8' {
            pos[1] - b'0'
        } else {
            return Err(());
        };

        Ok(Field { file, row })
    }
}

#[test]
fn test_field() {
    assert_eq!(Field::try_from("a"), Err(()));
    assert_eq!(Field::try_from("a23"), Err(()));
    assert_eq!(Field::try_from("a2"), Ok(Field { file: 1, row: 2 }));
    assert_eq!(Field::try_from("A2"), Ok(Field { file: 1, row: 2 }));
    assert_eq!(Field::try_from("a0"), Err(()));
    assert_eq!(Field::try_from("a9"), Err(()));
    assert_eq!(Field::try_from("@2"), Err(()));
    assert_eq!(Field::try_from("{2"), Err(()));
}

pub fn field_code(piece: &Piece, field: &Field) -> u16 {
    64u16 * (piece.code() as u16) + 8u16 * (field.row as u16) + (field.file as u16)
}

pub fn field_hash(piece: &Piece, field: &Field) -> u64 {
    RANDOM_PIECE[field_code(piece, field) as usize]
}

#[derive(Debug)]
pub struct FENString {
    rows: [String; 8],
    turn: u8,
    castle: [u8; 4],
    en_passant: String,
}

impl std::convert::TryFrom<&str> for FENString {
    type Error = ();

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let tokens = string.split_whitespace().collect::<Vec<&str>>();

        <FENString as TryFrom<Vec<&str>>>::try_from(tokens)
    }
}

impl std::convert::TryFrom<Vec<&str>> for FENString {
    type Error = ();

    fn try_from(tokens: Vec<&str>) -> Result<Self, Self::Error> {
        if tokens.len() < 4 || tokens.len() > 6 {
            return Err(());
        }

        let rows = tokens[0].split('/').collect::<Vec<&str>>();
        if rows.len() != 8 {
            return Err(());
        }

        let turn: &[u8] = tokens[1].as_bytes();
        if turn.len() != 1 {
            return Err(());
        }

        let castle: &[u8] = tokens[2].as_bytes();
        if castle.len() != 4 {
            return Err(());
        }

        Ok(FENString {
            rows: rows
                .into_iter()
                .map(ToOwned::to_owned)
                .collect::<Vec<String>>()
                .try_into()
                .unwrap(),
            turn: turn[0],
            castle: castle.try_into().unwrap(),
            en_passant: tokens[3].to_owned(),
        })
    }
}

#[test]
fn test_fenstring() {
    let f = match FENString::try_from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w AHah -") {
        Ok(f) => f,
        Err(()) => panic!(),
    };

    let board = match Board::try_from(f) {
        Ok(board) => board,
        Err(()) => panic!(),
    };

    assert_eq!(board, Board::new());
}
