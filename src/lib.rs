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

fn possibles(board: &Board) -> (Vec<Vec<Vec<u8>>>, Vec<(usize, usize)>) {
    // possible_block[i in 0~8]: (i//3)*3~(i//3)*3+2, (i%3)*3~(i%3)*3+2
    let mut possible_block: [[bool; 9]; 9] = [[true; 9]; 9];
    // possible_row[i in 0~8]
    let mut possible_row: [[bool; 9]; 9] = [[true; 9]; 9];
    // possible_column[i in 0~8]
    let mut possible_column: [[bool; 9]; 9] = [[true; 9]; 9];
    let mut remain_index: Vec<(usize, usize)> = Vec::new();

    for i in 0..9 {
        for j in 0..9 {
            let b = board[i][j];

            if board[i][j] == 0 {
                remain_index.push((i, j));
            } else {
                let b_1: usize = (b-1).into();
                possible_block[(i/3)*3+j/3][b_1] = false;
                possible_row[i][b_1] = false;
                possible_column[j][b_1] = false;
            }
        }
    }

    let mut possibles: Vec<Vec<Vec<u8>>> = vec![vec![Vec::new(); 9]; 9];
    for &(i, j) in remain_index.iter() {
        for k in 0..9 {
            if possible_block[(i/3)*3+j/3][k]
            && possible_row[i][k]
            && possible_column[j][k] {
                possibles[i][j].push((k+1).try_into().unwrap());
            }
        }
    }
    (possibles, remain_index)
}

pub fn solve(board: &mut Board) -> &mut Board {
    let (mut possibles, mut remain_index) = possibles(&board);

    for &(i, j) in remain_index.iter() {
        if let &[k] = possibles[i][j].as_slice() {
            board[i][j] = k.try_into().unwrap();
            println!("changed {} {}", i, j);
        }
    }
    println!("{:?}", possibles);
    board
}
