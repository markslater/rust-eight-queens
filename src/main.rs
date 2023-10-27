use board::Board;

mod board;

fn main() {
    let mut board = Board::new();
    let mut place = 0;
    for queen in 0..6 {
        if let Ok((b, p)) = board.place_next(place){
            board = b;
            place = p;
            println!("{} at {}", queen, place)
        } else {
            println!("Fail :(")
        }
    }
    board.output();
}
