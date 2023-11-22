use std::fmt;
use std::fmt::Formatter;

pub struct Board {
    squares: [bool; 64], // TODO could be an unsigned 64 bit integer
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [false; 64]
        }
    }

    pub fn place_queens(&self, start_at: usize, number_to_place: u32) -> impl Iterator<Item=Vec<usize>> {
        match number_to_place {
            0 => vec![vec![1usize; 0]; 1],
            _ => (start_at..64)
                .map(|square| (square, self.set(square)))
                .filter_map(|(square, result)| result.map(|board| (square, board)).ok())
                .map(|(square, board)| (square, board.place_queens(square + 1, number_to_place - 1)))
                .flat_map(|(square, results)| results.map(move |result| [vec![square], result].concat()))
                .collect::<Vec<Vec<usize>>>(),
        }.into_iter()
    }

    fn set(&self, square: usize) -> Result<Board, ()> { // TODO should we use a smaller argument?  Does it matter?
        if self.squares[square] { // TODO should we improve error when index is out of bounds?
            Err(())
        } else {
            let mut squares = self.squares;
            for i in square..64 {
                squares[i] = squares[i]
                    || (i % 8 == square % 8)
                    || (i / 8 == square / 8)
                    || ((i - square) % 9 == 0 && i % 8 >= square % 8)
                    || ((i - square) % 7 == 0 && i % 8 <= square % 8);
            }
            Ok(Board { squares })
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..8 {
            writeln!(
                f,
                "{}|{}",
                y,
                (0..8)
                    .map(|x| if self.squares[(y * 8) + x] {
                        " X "
                    } else {
                        "   "
                    })
                    .collect::<Vec<&str>>()
                    .join("")
            )?;
        };
        writeln!(f, "  {}", (0..8).map(|_x| " - ").collect::<Vec<&str>>().join(""))?;
        writeln!(f, "  {}", (0..8).map(|x| format!(" {} ", x)).collect::<Vec<String>>().join(""))
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;

    #[test]
    fn can_place_zero_queens() {
        let places = Board::new().place_queens(0, 0);
        assert_eq!(places.collect::<Vec<Vec<usize>>>(), [[]]);
    }

    #[test]
    fn can_place_one_queen() {
        let places = Board::new().place_queens(0, 1);
        assert_eq!(places.collect::<Vec<Vec<usize>>>(), (0..64).map(|square| vec![square]).collect::<Vec<Vec<usize>>>());
    }

    #[test]
    fn can_place_one_queen_starting_after_first_square() {
        let places = Board::new().place_queens(1, 1);
        assert_eq!(places.collect::<Vec<Vec<usize>>>(), (1..64).map(|square| vec![square]).collect::<Vec<Vec<usize>>>());
    }
}