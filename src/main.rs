use board::Board;

mod board;

fn main() {
    let first = 0;
    let board_result = Board::new().set(first);
    if let Ok(board) = board_result {
        for second in first..64 {
            if let Ok(board) = board.set(second) {
                for third in second..64 {
                    if let Ok(board) = board.set(third) {
                        println!("{}, {}, {}", first, second, third);
                        board.output();
                        break;
                    }
                }
                break;
            }
        }
    }
}
