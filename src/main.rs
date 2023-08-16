use std::io::{self, Write};
use std::sync::mpsc;
use std::fs::File;

use vendace::executor::executor;

#[tokio::main]
async fn main() {
    println!("Vendace 1.0.0 by Antoni Przybylik");

    let mut logfile = File::create("/tmp/VENDACE_LOG").unwrap();

    let (tx, rx) = mpsc::channel::<String>();
    tokio::spawn(executor(rx));

    loop {
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        logfile.write_all(input.as_bytes()).unwrap();
        let input = input.trim().to_owned();

        let first_token = match input.split_whitespace().next() {
            Some(token) => token,
            None => continue,
        };

        match first_token {
            "help" => {
                println!("\nVendace is a chess engine for playing and analyzing.\n\
                            It is released as free software licensed under the \n\
                            GNU GPLv3 License. Vendace is normally used with a\n\
                            graphical user interface (GUI) and implements the\n\
                            Universal Chess Interface (UCI) protocol to communicate\n\
                            with a GUI, an API, etc.\n");
            },
            "isready" => {
                println!("readyok");
            },
            "ucinewgame" => {
                // Do nothing.
            },
            "uci" => {
                // TODO: Print engine options.
                println!("uciok");
            },
            "position" => {
                tx.send(input).unwrap();
            },
            "go" => {
                tx.send(input).unwrap();
            },
            "quit" => {
                std::process::exit(0);
            },
            "stop" => {
                // TODO: Stop executor thread.
            },
            _ => {
                println!("Unknown command: '{}'. Type help for more information.",
                         input);
            },
        }
    }
}
