use board::Board;

mod board;

fn main() {
    let board_result = Board::new().set(0);
    if let Result::Ok(board) = board_result {
        board.output();
    }
}
