use board::Board;

mod board;

fn main() {
    let mut board = Board::new();
    let mut place = 0;
    for queen in 0..6 {
        if let Ok(t) = board.place_next(place){
            board = t.0;
            place = t.1;
            println!("{} at {}", queen, place)
        } else {
            println!("Fail :(")
        }
    }
    board.output();
}
