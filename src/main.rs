use std::{env, process};
use sudoku_rs::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut board = get_input(config);

    println!("init:");
    print_board(board);

    if solve(&mut board) {
        println!("solved:");
        print_board(board);
    } else {
        println!("Failed to solve");
        print_board(board);
        process::exit(1);
    };
}
