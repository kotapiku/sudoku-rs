use sudoku_rs::*;
use std::{
    env,
    process,
    fs,
    io::{prelude::*, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let f = fs::File::open(config.filename).expect("file not found");
    let buf = BufReader::new(f);

    let input: Board =
        buf.lines()
            .map(|line| line.unwrap().split(",").map(|s| s.trim().parse::<u8>().unwrap_or(0)).collect::<Vec<u8>>().try_into().unwrap())
            .collect::<Vec<[u8; length]>>().try_into().unwrap();

    println!("init:");
    print_board(input);

    let solved = solve(input);
    println!("solved:");
    print_board(solved)
}

struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("please enter filename");
        }

        let filename = args[1].clone();
        Ok(Config { filename })
    }
}