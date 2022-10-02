use sudoku_rs::*;
use std::{
    env,
    process,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let input = get_input(config);

    println!("init:");
    print_board(input);

    let solved = solve(input);
    println!("solved:");
    print_board(solved)
}