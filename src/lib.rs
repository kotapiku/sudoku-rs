pub const length: usize = 9;
pub type Board = [[u8; length]; length];
pub fn print_board(board: Board) {
    for line in board.iter() {
        println!("{:?}", line);
    }
}

pub fn solve(board: Board) -> Board {
  board
}
