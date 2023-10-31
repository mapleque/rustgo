mod basic;
mod util;

use crate::basic::{BoardSize, Cmd, Game, Player};
use std::env;
use std::io;

fn show_usage() {
    println!("Usage:");
    println!("\t{} {}", "normal or empty", "use 19 * 19 board size");
    println!("\t{} {}", "medium", "use 13 * 13 board size");
    println!("\t{} {}", "small", "use 9 * 9 board size");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut g = if args.len() < 2 || args[1].eq("normal") {
        Game::new(BoardSize::Normal)
    } else {
        let arg = &args[1];
        match &arg[..] {
            "medium" => Game::new(BoardSize::Medium),
            "small" => Game::new(BoardSize::Small),
            _ => {
                show_usage();
                panic!("invalid args");
            }
        }
    };

    print!("{}", g);

    loop {
        println!();
        match g.next_player() {
            Player::Black => println!("Black (aa-zz or pass):"),
            Player::White => println!("White (aa-zz or pass):"),
        };
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        if buffer == "pass\n" {
            g.next(Cmd::Pass).unwrap();
        } else if buffer.len() == 3 {
            buffer.truncate(2);
            g.next(Cmd::Step(buffer)).unwrap();
        } else {
            println!("invalid input: {:?}", buffer.as_bytes());
        }
        print!("{}", g);
    }
}
