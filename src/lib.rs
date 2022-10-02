use std::{
    fs,
    io::{prelude::*, BufReader},
};

pub const LEN: usize = 9;
pub type Board = [[u8; LEN]; LEN];
pub fn print_board(board: Board) {
    for line in board.iter() {
        println!("{:?}", line);
    }
}


pub fn get_input(config: Config) -> Board {
    let f = fs::File::open(config.filename).expect("file not found");
    let buf = BufReader::new(f);

    buf.lines()
        .map(|line| line.unwrap().split(",").map(|s| s.trim().parse::<u8>().unwrap_or(0)).collect::<Vec<u8>>().try_into().expect("wrong row length"))
        .collect::<Vec<[u8; LEN]>>().try_into().expect("wrong column length")
}

pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("please enter filename");
        }

        let filename = args[1].clone();
        Ok(Config { filename })
    }
}


pub fn solve(board: Board) -> Board {
    board
}
