use crate::board::Board;

mod board;

pub fn place_queens() {
    // attempt_to_place_queen_at(&Board::new(), 0, 0).unwrap();
    let results: Vec<Vec<usize>> = Board::new().place_queens(0, 8).collect();
    println!("Found {} results", results.len());
    for result in results {
        println!("{:?}", result)
    }
}

fn attempt_to_place_queen_at(board: &Board, start_at: usize, queen_number: i32) -> Result<(), ()> {
    if queen_number == 8 {
        board.output();
        Ok(())
    } else {
        let mut place = start_at;
        loop {
            if place == 64 {
                break Err(())
            } else {
                let (candidate_board, candidate_place) = board.place_next(place)?;
                if attempt_to_place_queen_at(&candidate_board, candidate_place, queen_number + 1).is_ok() {
                    println!("{}: {}, {}", place, place / 8, place % 8);
                    break Ok(())
                }
            }
            place = place + 1;
        }
    }
}