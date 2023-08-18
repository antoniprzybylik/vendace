use std::str::FromStr;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::time::Duration;

use super::board::Board;
use super::board::Color;
use super::board::FENString;
use super::book::Book;
use super::book::BookEntry;
use super::book::Move;

use super::moves::get_move;
use super::moves::STOP_ALL_THREADS;

lazy_static! {
    static ref BOOK: Book = Book::load("/usr/share/gnuchess/smallbook.bin").unwrap();
}

static mut JOB_COUNTER: AtomicU8 = AtomicU8::new(0);

pub fn executor(rx: mpsc::Receiver<String>) {
    let mut board: Board = Board::new();

    for cmd in rx.iter() {
        let tokens = cmd.split_whitespace().collect::<Vec<&str>>();

        match tokens[0] {
            "debug_quality" => {
                println!("Quality = {}", board.eval());
            }
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

                            let fs = match FENString::try_from(tokens[i + 1..=i + 4].to_vec()) {
                                Ok(fs) => fs,
                                Err(()) => {
                                    println!("Error: Malformed fenstring.");
                                    break;
                                }
                            };

                            // Przesuwamy iterator za fenstring.
                            i += 4;

                            board = match Board::try_from(fs) {
                                Ok(board) => board,
                                Err(()) => {
                                    println!("Error: Malformed fenstring.");
                                    break;
                                }
                            };
                            pos_set = true;
                        }
                        "startpos" => {
                            if pos_set || mov_set {
                                println!("Error: Malformed `position` command string.");
                                break;
                            }

                            board = Board::new();
                            pos_set = true;
                        }
                        "moves" => {
                            i += 1;
                            while i < tokens.len() {
                                let token = tokens[i];

                                let r#move = match Move::try_from(token) {
                                    Ok(r#move) => r#move,
                                    Err(()) => {
                                        println!("Invalid move in `position` command string.");
                                        break 'parse_commands;
                                    }
                                };

                                board.apply_unchecked(&r#move);
                                board.next_turn();
                                i += 1;
                            }

                            mov_set = true;
                        }
                        _ => {}
                    }

                    i += 1;
                }
            }
            "go" => {
                unsafe {
                    let _ = JOB_COUNTER.fetch_add(1, Ordering::SeqCst);
                }

                // Odczytaj wtime/btime
                let mut my_time: u64 = 8000;
                let mut i = 1;
                while i < tokens.len() {
                    let token = tokens[i];

                    match token {
                        "wtime" => {
                            if board.which_turn() == Color::White {
                                if tokens.len() == i + 1 {
                                    // FIXME: Malformed command error.
                                    break;
                                }

                                if let Ok(time) = FromStr::from_str(tokens[i + 1]) {
                                    my_time = time;
                                } else {
                                    // FIXME: Malformed command error.
                                    break;
                                }
                            }
                        }
                        "btime" => {
                            if board.which_turn() == Color::Black {
                                if tokens.len() == i + 1 {
                                    // FIXME: Malformed command error.
                                    break;
                                }

                                if let Ok(time) = FromStr::from_str(tokens[i + 1]) {
                                    my_time = time;
                                } else {
                                    // FIXME: Malformed command error.
                                    break;
                                }
                            }
                        }
                        _ => {}
                    };

                    i += 1;
                }

                std::thread::spawn(move || {
                    let job_num = unsafe { JOB_COUNTER.load(Ordering::SeqCst) };
                    std::thread::sleep(Duration::from_millis(my_time));

                    if job_num == unsafe { JOB_COUNTER.load(Ordering::SeqCst) } {
                        unsafe {
                            *STOP_ALL_THREADS.get_mut() = true;
                        }
                    }
                });

                // Odblokuj obliczenia.
                unsafe {
                    *STOP_ALL_THREADS.get_mut() = false;
                }

                if let Some(moves) = BOOK.get(&board.hash()) {
                    let (mut best_move, mut best_weight) = (0u16, 0u16);
                    for BookEntry { r#move, weight } in moves.iter() {
                        if *weight > best_weight {
                            (best_move, best_weight) = (*r#move, *weight);
                        }
                    }

                    println!("bestmove {}", Move::try_from(best_move).unwrap());
                } else {
                    let r#move = get_move(&board, &board.which_turn());

                    println!("bestmove {}", r#move);
                }
            }
            _ => unreachable!(),
        }
    }
}
