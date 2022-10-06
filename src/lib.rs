use std::{
    fs,
    io::{prelude::*, BufReader},
};

pub type Board = [[u8; 9]; 9];
pub fn print_board(board: Board) {
    for line in board.iter() {
        println!("{:?}", line);
    }
}

pub fn get_input(config: Config) -> Board {
    let f = fs::File::open(config.filename).expect("file not found");
    let buf = BufReader::new(f);

    buf.lines()
        .map(|line| {
            line.unwrap()
                .split(',')
                .map(|s| s.trim().parse::<u8>().unwrap_or(0))
                .collect::<Vec<u8>>()
                .try_into()
                .expect("wrong row length")
        })
        .collect::<Vec<[u8; 9]>>()
        .try_into()
        .expect("wrong column length")
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

pub fn solve(board: &mut Board) -> bool {
    let (mut possibles, mut remain_index) = possibles(board);
    solve2(board, &mut possibles, &mut remain_index)
}

type Possibles = Vec<Vec<Vec<u8>>>;
type RemainIndex = Vec<(usize, usize)>;

pub fn possibles(board: &Board) -> (Possibles, RemainIndex) {
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
                let b_1: usize = (b - 1).into();
                possible_block[(i / 3) * 3 + j / 3][b_1] = false;
                possible_row[i][b_1] = false;
                possible_column[j][b_1] = false;
            }
        }
    }

    let mut possibles: Possibles = vec![vec![Vec::with_capacity(9); 9]; 9];
    for &(i, j) in remain_index.iter() {
        for k in 0..9 {
            if possible_block[(i / 3) * 3 + j / 3][k] && possible_row[i][k] && possible_column[j][k]
            {
                possibles[i][j].push((k + 1).try_into().unwrap());
            }
        }
    }
    (possibles, remain_index)
}

fn solve2(board: &mut Board, possibles: &mut Possibles, remain_index: &mut RemainIndex) -> bool {
    remain_index.sort_by(|a, b| possibles[b.0][b.1].len().cmp(&possibles[a.0][a.1].len()));

    // split remain_index to remain_index (len > 1), one_index (len == 1 or 0)
    let idx = remain_index.partition_point(|n| possibles[n.0][n.1].len() > 1);
    let one_index = remain_index.split_off(idx);

    // update determined numbers (len == 1 or 0)
    for &(i, j) in one_index.iter() {
        match possibles[i][j].pop() {
            None => return false,
            Some(n) => {
                board[i][j] = n;
                println!("changed: {} at ({}, {})", n, i, j);
                print_board(*board);

                for k in 0..9 {
                    possibles[k][j].retain(|&v| v != n);
                    possibles[i][k].retain(|&v| v != n);
                    possibles[(i / 3) * 3 + k / 3][(j / 3) * 3 + k % 3].retain(|&v| v != n);
                }
            }
        }
    }

    // do kari-oki for remain_index (len > 1)
    match remain_index.pop() {
        None => return true,
        Some(last) => {
            // let elements = possibles[last.0][last.1].clone();
            for &kari in possibles[last.0][last.1].iter() {
                let mut board2 = *board;
                let mut possibles2 = possibles.clone();
                let mut remain_index2 = remain_index.clone();
                board2[last.0][last.1] = kari;
                println!("kari-oki {:?} at {:?}", kari, last);

                // update possibles
                for i in 0..9 {
                    possibles2[i][last.1].retain(|&v| v != kari);
                    possibles2[last.0][i].retain(|&v| v != kari);
                    possibles2[((last.0) / 3) * 3 + i / 3][((last.1) / 3) * 3 + i % 3]
                        .retain(|&v| v != kari);
                }

                if solve2(&mut board2, &mut possibles2, &mut remain_index2) {
                    *board = board2;
                    return true;
                };
                println!("kari-oki failed: {:?} at {:?}", kari, last);
            }
        }
    };
    false
}
