fn main() {
    // let mut board = Board::new().set(0).unwrap();
    // board = board.set(12).unwrap();
    // board = board.set(23).unwrap();
    // board = board.set(26).unwrap();
    // board = board.set(29).unwrap();

    eight_queens::place_queens();
    // let mut board = Board::new();
    // let mut place = 0;
    // for queen in 0..6 {
    //     if let Ok((b, p)) = board.place_next(place){
    //         board = b;
    //         place = p;
    //         println!("{} at {}", queen, place)
    //     } else {
    //         println!("Fail :(")
    //     }
    // }
    // board.output();
}
