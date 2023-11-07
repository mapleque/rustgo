mod basic;
mod util;

use crate::basic::{BoardSize, Cmd, Game, Player};
use std::env;
use std::io;
use std::process::exit;

fn show_usage() {
    println!("Usage: <command> [options]");
    println!("");
    println!("option list:");
    println!("\t{}: \t\t{}", "new", "start 19 * 19 game");
    println!("\t{}: \t{}", "medium", "start 13 * 13 game");
    println!("\t{}: \t\t{}", "small", "start 9 * 9 game");
    println!("");
    println!(
        "\t{}: \t{}",
        "load <dump-file-path>", "start by loading a dumped file"
    );
}

fn show_operator_usage() {
    println!("System Operators:");
    println!("\t{}: \t{}", "help", "show this.");
    println!(
        "\t{}: \t{}",
        "exit", "exit game immediately, without saving."
    );
    println!(
        "\t{}: {}",
        "dump", "dump current steps, which can be load anytime."
    );
    println!("Game Operators:");
    println!(
        "\t{}: \t{}",
        "pass", "let another player step without any stone put in."
    );
    println!("\t{}: \t{}", "undo", "get back stone just put in.");
    println!("\t{}: \t{}", "redo", "redo the undo step.");
    println!(
        "\t{}: \t{}",
        "**", "like aa, bc, etc., put the stone on that point."
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut g = if args.len() < 2 {
        show_usage();
        exit(0)
    } else {
        let arg = &args[1];
        match &arg[..] {
            "load" => {
                if args.len() < 3 {
                    show_usage();
                    panic!("invalid args");
                }
                let filename = &args[2];
                Game::load(filename.to_string()).unwrap()
            }
            "new" => Game::new(BoardSize::Normal),
            "medium" => Game::new(BoardSize::Medium),
            "small" => Game::new(BoardSize::Small),
            _ => {
                println!("invalid args");
                show_usage();
                exit(0)
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
            "help" => {
                show_operator_usage();
                continue;
            }
            "exit" => exit(0),
            "dump" => {
                g.dump();
                continue;
            }
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
