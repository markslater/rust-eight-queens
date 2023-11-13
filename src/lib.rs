use crate::board::Board;

mod board;

pub fn place_queens() {
    let results: Vec<Vec<usize>> = Board::new().place_queens(0, 8).collect();
    println!("Found {} results", results.len());
    for result in results {
        println!("{:?}", result)
    }
}
