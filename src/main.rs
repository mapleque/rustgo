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
            Player::Black => println!("Black (aa-ss or pass):"),
            Player::White => println!("White (aa-ss or pass):"),
        };
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        match buffer.trim() {
            "undo" => g.undo().unwrap_or_else(|err| {
                println!("can not undo: {}", err);
            }),
            "pass" => g.next(Cmd::Pass).unwrap(),
            "redo" => {
                let redo_list = g.redo_list();
                if redo_list.len() == 0 {
                    println!("can not redo {}", "no redo steps");
                } else {
                    let mut steps = String::from("");
                    for i in 0..redo_list.len() {
                        let cmd = match &redo_list[i] {
                            Cmd::Pass => String::from("pass"),
                            Cmd::Step(s) => s.clone(),
                            Cmd::Start => String::from("start"),
                        };
                        steps = format!("{} {}:{} ", steps, i, cmd);
                    }
                    println!("select redo step-> {}", steps);
                    let mut buffer = String::new();
                    io::stdin().read_line(&mut buffer).unwrap();
                    let index: usize = buffer.trim().parse::<usize>().unwrap();
                    if index >= redo_list.len() {
                        println!("invalid step: {}", index);
                    } else {
                        g.redo(index).unwrap_or_else(|err| {
                            println!("can not redo: {}", err);
                        });
                    }
                }
            }
            other => {
                if other.len() != 2 {
                    println!("invalid input: {:?}", other.as_bytes());
                }
                g.next(Cmd::Step(other.to_string())).unwrap_or_else(|err| {
                    println!("invalid input with err: {:?}", err);
                });
            }
        };
        print!("{}", g);
    }
}
