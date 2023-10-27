mod basic;
mod util;

use crate::basic::{BoardSize, Cmd, Game, Player};
use std::io;

fn main() {
    let mut g = loop {
        println!("Please choose board size:");
        println!("  1: 19*19, 2: 13*13, 3: 9*9");
        println!("your choose is (1 or 2 or 3):");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        if buffer == "1\n" {
            break Game::new(BoardSize::Normal);
        } else if buffer == "2\n" {
            break Game::new(BoardSize::Medium);
        } else if buffer == "3\n" {
            break Game::new(BoardSize::Small);
        } else {
            println!("invalid input: {:?}", buffer.as_bytes());
        }
    };

    print!("{}", g);

    loop {
        println!();
        match g.next_player() {
            Player::Black => println!("turn to Black (aa-zz or pass):"),
            Player::White => println!("turn to White (aa-zz or pass):"),
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
