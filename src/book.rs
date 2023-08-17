use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::fmt;

use std::io::prelude::*;

use super::board::KindOfPiece;
use super::board::Field;

struct BookFileEntry {
    key: u64,
    r#move: u16,
    weight: u16,
    learn: u32,
}

fn read_entry(file: &mut File, entry: &mut BookFileEntry) -> Result<(), Box<dyn Error>> {
    let mut buf = [0; 16];
    file.read_exact(&mut buf)?;

    entry.key = u64::from_be_bytes(buf[0..8].try_into()?);
    entry.r#move = u16::from_be_bytes(buf[8..10].try_into()?);
    entry.weight = u16::from_be_bytes(buf[10..12].try_into()?);
    entry.learn = u32::from_be_bytes(buf[12..16].try_into()?);

    if entry.r#move & 0x8000 != 0 {
        return Err(anyhow::anyhow!("Malformed data.").into());
    }

    Ok(())
}

#[derive(Debug)]
pub struct BookEntry {
    pub r#move: u16,
    pub weight: u16,
    // We don't use information in `learn`.
}

pub struct Book(HashMap<u64, Vec<BookEntry>>);

impl Deref for Book {
    type Target = HashMap<u64, Vec<BookEntry>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Book {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Book {
    pub fn load<P: AsRef<Path>>(path: P) -> Option<Self> {
        let mut file = match File::open(&path) {
            Err(_) => return None,
            Ok(file) => file,
        };

        let mut book = Book(HashMap::new());
        let mut entry = BookFileEntry {
            key: 0,
            r#move: 0,
            weight: 0,
            learn: 0,
        };
        while read_entry(&mut file, &mut entry).is_ok() {
            if book.get(&entry.key).is_none() {
                book.insert(entry.key, Vec::new());
            }

            book.get_mut(&entry.key).unwrap().push(
                BookEntry {
                    r#move: entry.r#move,
                    weight: entry.weight,
                },
            );
        }

        Some(book)
    }
}

#[derive(Debug, PartialEq)]
pub struct Move(u16);

impl Move {
    pub fn from_file_number(&self) -> u8 {
        (((self.0 & 0x1C0) >> 6) + 1) as u8
    }

    pub fn from_file(&self) -> char {
        const ALPHABET: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        char::from(ALPHABET[(self.from_file_number()-1) as usize])
    }

    pub fn from_row(&self) -> u8 {
        (((self.0 & 0xE00) >> 9) + 1) as u8
    }

    pub fn from_field(&self) -> Field {
        Field {
            row: self.from_row(),
            file: self.from_file_number(),
        }
    }

    pub fn to_file_number(&self) -> u8 {
        ((self.0 & 0x7) + 1) as u8
    }

    pub fn to_file(&self) -> char {
        const ALPHABET: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        char::from(ALPHABET[(self.to_file_number()-1) as usize])
    }

    pub fn to_row(&self) -> u8 {
        (((self.0 & 0x38) >> 3) + 1) as u8
    }

    pub fn to_field(&self) -> Field {
        Field {
            row: self.to_row(),
            file: self.to_file_number(),
        }
    }

    pub fn promotion(&self) -> Option<KindOfPiece> {
        match (self.0 & 0x7000) >> 12 {
            0 => None,
            1 => Some(KindOfPiece::Knight),
            2 => Some(KindOfPiece::Bishop),
            3 => Some(KindOfPiece::Rook),
            4 => Some(KindOfPiece::Queen),
            _ => unreachable!(),
        }
    }

    pub fn build(f1: Field, f2: Field, piece: Option<KindOfPiece>) -> Result<Self, ()> {
        let mut code: u16 = 0;

        code |= (f2.file-1) as u16;
        code |= ((f2.row-1) as u16) << 3;
        code |= ((f1.file-1) as u16) << 6;
        code |= ((f1.row-1) as u16) << 9;

        code |= match piece {
            None => 0,
            Some(KindOfPiece::Knight) => 1,
            Some(KindOfPiece::Bishop) => 2,
            Some(KindOfPiece::Rook) => 3,
            Some(KindOfPiece::Queen) => 4,
            _ => return Err(()),
        } << 12;

        Ok(Move(code))
    }
}

impl std::convert::TryFrom<u16> for Move {
    type Error = ();

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        if code & 0x8000 != 0 {
            return Err(());
        }

        if (code & 0x7000) >> 12 > 4 {
            return Err(());
        }

        Ok(Move(code))
    }
}

impl std::convert::TryFrom<&str> for Move {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() != 4 &&
           s.len() != 5 {
            return Err(());
        }

        let (s1, mut s2) = s.split_at(2);
        let mut m: &str = &String::new();
        if s2.len() == 3 {
            (s2, m) = s2.split_at(2);
        }

        let f1 = match Field::try_from(s1) {
            Ok(field) => field,
            Err(()) => return Err(()),
        };
        let f2 = match Field::try_from(s2) {
            Ok(field) => field,
            Err(()) => return Err(()),
        };

        let piece: Option<KindOfPiece> = match m {
            "" => None,
            "q" => Some(KindOfPiece::Queen), 
            "b" => Some(KindOfPiece::Bishop), 
            "n" => Some(KindOfPiece::Knight), 
            "r" => Some(KindOfPiece::Rook), 
            _ => return Err(()),
        };

        Move::build(f1, f2, piece)
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(piece) = self.promotion() {
            write!(f, "{}{}{}{}{}",
                   self.from_file(),
                   self.from_row(),
                   self.to_file(),
                   self.to_row(), piece)
        } else {
            write!(f, "{}{}{}{}",
                   self.from_file(),
                   self.from_row(),
                   self.to_file(),
                   self.to_row())
        }
    }
}

#[test]
fn test_move() {
    assert_eq!(Move::try_from(0x8000u16), Err(()));
    assert_eq!(Move::try_from(0xF000u16), Err(()));
    assert_eq!(Move::try_from(0x7000u16), Err(()));

    let m = match Move::try_from(0x031Cu16) {
        Ok(m) => m,
        Err(_) => panic!(),
    };
    assert_eq!(m.from_file_number(), 5);
    assert_eq!(m.from_file(), 'e');
    assert_eq!(m.from_row(), 2);
    assert_eq!(m.to_file_number(), 5);
    assert_eq!(m.to_file(), 'e');
    assert_eq!(m.to_row(), 4);
    assert_eq!(m.promotion(), None);

    assert_eq!(Move::try_from(0x031Cu16),
               Move::try_from("e2e4"));
    assert_eq!(Move::try_from(0x4D3Cu16),
               Move::try_from("e7e8q"));
}
