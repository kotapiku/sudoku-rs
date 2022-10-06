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

#[cfg(test)]
mod tests {
    extern crate glob;
    use crate::*;
    use glob::glob;
    #[test]
    fn test1() {
        for benchmark in glob("benchmarks/*").expect("Failed to read glob pattern") {
            let filename = benchmark.unwrap();
            println!("{:?}", &filename);
            let mut board = get_input(Config {
                filename: filename.to_str().unwrap().to_string(),
            });
            let old = board.clone();
            solve(&mut board);
            assert!(check_answer(&old, &board));
        }
    }

    fn check_answer(old: &Board, solved: &Board) -> bool {
        let mut block: [[bool; 9]; 9] = [[false; 9]; 9];
        let mut row: [[bool; 9]; 9] = [[false; 9]; 9];
        let mut column: [[bool; 9]; 9] = [[false; 9]; 9];

        let mut result = true;
        for i in 0..9 {
            for j in 0..9 {
                if old[i][j] != 0 {
                    result = result & (old[i][j] == solved[i][j]);
                };

                let b_1: usize = (solved[i][j] - 1).into();
                block[(i / 3) * 3 + j / 3][b_1] = true;
                row[i][b_1] = true;
                column[j][b_1] = true;
            }
        }

        result & (block == [[true; 9]; 9]) & (row == [[true; 9]; 9]) & (column == [[true; 9]; 9])
    }
}
