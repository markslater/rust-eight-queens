use board::Board;

mod board;

fn main() {
    let board_result = Board::new().set(0);
    if let Result::Ok(board) = board_result {
        for i in 1..64 {
            if let Ok(board) = board.set(i) {
                println!("0, {}", i);
                board.output();
                break;
            }
        }
    }
}
