use std::sync::mpsc;

use super::board::Board;
use super::board::FENString;
use super::book::Move;
use super::book::Book;
use super::book::BookEntry;

lazy_static! {
    static ref BOOK: Book = Book::load("/usr/share/gnuchess/smallbook.bin").unwrap();
}

pub async fn executor(rx: mpsc::Receiver<String>) {
    let mut board: Board = Board::new();

    for cmd in rx.iter() {
        let tokens = cmd.split_whitespace().collect::<Vec<&str>>();

        match tokens[0] {
            "position" => {
                let mut pos_set: bool = false;
                let mut mov_set: bool = false;

                let mut i: usize = 1;
                'parse_commands: while i < tokens.len() {
                    match tokens[i] {
                        "fen" => {
                            if pos_set || mov_set {
                                println!("Error: Malformed `position` command string.");
                                break;
                            }

                            let fs = match FENString::try_from(tokens[i+1..=i+4]
                                                               .to_vec()) {
                                Ok(fs) => fs,
                                Err(()) => {
                                    println!("Error: Malformed fenstring.");
                                    break;
                                },
                            };

                            // Przesuwamy iterator za fenstring.
                            i += 4;

                            board = match Board::try_from(fs) {
                                Ok(board) => board,
                                Err(()) => {
                                    println!("Error: Malformed fenstring.");
                                    break;
                                },
                            };
                            pos_set = true;
                        },
                        "startpos" => {
                            if pos_set || mov_set {
                                println!("Error: Malformed `position` command string.");
                                break;
                            }

                            board = Board::new();
                            pos_set = true;
                        },
                        "moves" => {
                            i += 1;
                            while i < tokens.len() {
                                let token = tokens[i];

                                let r#move = match Move::try_from(token) {
                                    Ok(r#move) => r#move,
                                    Err(()) => {
                                        println!("Invalid move in `position` command string.");
                                        break 'parse_commands;
                                    },
                                };

                                board.apply_unchecked(&r#move);
                                i += 1;
                            }

                            mov_set = true;
                        },
                        _ => {},
                    }

                    i += 1;
                }
            },
            "go" => {
                // TODO: Zwracaj uwagę na parametry.

                if let Some(moves) = BOOK.get(&board.hash()) {
                    let (mut best_move, mut best_weight) = (0u16, 0u16);
                    for BookEntry { r#move, weight } in moves.iter() {
                        if *weight > best_weight {
                            (best_move, best_weight) = (*r#move, *weight);
                        }
                    }

                    println!("bestmove {}", Move::try_from(best_move).unwrap());
                } else {
                    // TODO: Losowe ruchy.
                }
            },
            _ => unreachable!(),
        }
    }
}
