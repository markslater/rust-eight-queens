use board::Board;

mod board;

fn main() {
    let board = Board::new();
    for first in 0..64 {
        if let Ok(board) = board.set(first) {
            for second in first..64 {
                if let Ok(board) = board.set(second) {
                    for third in second..64 {
                        if let Ok(board) = board.set(third) {
                            for fourth in third..64 {
                                if let Ok(board) = board.set(fourth) {
                                    println!("{}, {}, {}, {}", first, second, third, fourth);
                                    board.output();
                                    break;
                                }
                            }
                            break;
                        }
                    }
                    break;
                }
            }
            break;
        }
    }
}
