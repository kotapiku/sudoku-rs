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
    let mut remain_index: Vec<(usize, usize)> = Vec::with_capacity(9);

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

    let mut possibles: Vec<Vec<Vec<u8>>> = vec![vec![Vec::with_capacity(9); 9]; 9];
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

    // while !remain_index.is_empty() {
        remain_index.sort_by(|a, b| possibles[b.0][b.1].len().cmp(&possibles[a.0][a.1].len()));
        println!("{:?}", possibles);
        println!("{:?}", remain_index);
        // split remain_index to remain_index (len > 1), one_index (len == 1)
        let idx = remain_index.partition_point(|n| possibles[n.0][n.1].len() != 1);
        let one_index = remain_index.split_off(idx);
        // update determined numbers
        for &(i, j) in one_index.iter() {
            board[i][j] = possibles[i][j].pop().unwrap();
            println!("changed {} {}", i, j);
        }


        println!("{:?}", remain_index);

    board
}
