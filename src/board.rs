use std::fmt;
use std::fmt::Formatter;

pub struct Board(u64);

impl Board {
    pub fn new() -> Board {
        Board(0)
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
        if ((self.0 >> square) & 1) != 0 { // TODO should we improve error when index is out of bounds?
            Err(())
        } else {
            Ok(Board(self.0
                | 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111 << (8 * (square / 8))
                | 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001 << (square % 8)
                | 0b10000000_01000000_00100000_00010000_00001000_00000100_00000010_00000001 << (8 * (square % 8)) >> (8 * (square % 8)) << square
                | 0b00000000_00000010_00000100_00001000_00010000_00100000_01000000_10000001 << (63 - 8 * (square % 8)) >> (63 - 8 * (square % 8)) << square
            ))
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
                    .map(|x| if ((self.0 >> ((y * 8) + x)) & 1) == 1 {
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

    #[test]
    fn can_set_square_0() {
        assert_eq!(Board::new().set(0).unwrap().0, 9313761861428380671)
    }

    #[test]
    fn can_set_square_7() {
        assert_eq!(Board::new().set(7).unwrap().0, 9332167099941961983)
    }

    #[test]
    fn can_set_square_56() {
        assert_eq!(Board::new().set(56).unwrap().0, 18374969058471772417)
    }

    #[test]
    fn can_set_square_63() {
        assert_eq!(Board::new().set(63).unwrap().0, 18410856566090662016)
    }
}