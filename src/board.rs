pub struct Board {
    squares: [bool; 64], // TODO could be an unsigned 64 bit integer
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [false; 64]
        }
    }

    fn place_queens(&self, start_at: usize, number_to_place: u32) -> impl Iterator<Item = usize> {
        match number_to_place {
            0 => vec![1usize; 0],
            _ => (start_at..64).filter(|square| !self.squares[*square]).collect::<Vec<usize>>(),
        }.into_iter()
    }

    fn set(&self, square: usize) -> Result<Board, ()> { // TODO should we use a smaller argument?  Does it matter?
        if self.squares[square] { // TODO should we improve error when index is out of bounds?
            Err(())
        } else {
            let mut squares = self.squares;
            for i in square..64 {
                if (i % 8 == square % 8)
                    || (i / 8 == square / 8)
                    || ((i - square) % 9 == 0 && i % 8 >= square % 8)
                    || ((i - square) % 7 == 0 && i % 8 <= square % 8)
                {
                    squares[i] = true;
                }
            }
            Ok(
                Board {
                    squares
                }
            )
        }
    }

    pub fn place_next(&self, start_at: usize) -> Result<(Board, usize), ()> {
        for i in start_at..64 {
            if let Ok(board) = self.set(i) {
                return Ok((board, i));
            }
        }
        Err(())
    }

    pub fn output(&self) {
        for y in 0..8 {
            print!("{}|", y);
            for x in 0..8 {
                if self.squares[(y * 8) + x] {
                    print!(" X ");
                } else {
                    print!("   ");
                };
            };
            println!();
        };
        print!("  ");
        for _x in 0..8 {
            print!(" - ");
        }
        println!();
        print!("  ");
        for x in 0..8 {
            print!(" {} ", x);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;

    #[test]
    fn can_place_zero_queens() {
        let places = Board::new().place_queens(0, 0);
        assert_eq!(places.collect::<Vec<usize>>(), vec![]);
    }

    #[test]
    fn can_place_one_queen() {
        let places = Board::new().place_queens(0, 1);
        assert_eq!(places.collect::<Vec<usize>>(), (0..64).collect::<Vec<usize>>());
    }

    #[test]
    fn set_rejects_placement_on_same_square_twice() {
        let square = 0;
        let setup = Board::new().set(square).unwrap();
        assert!(setup.set(square).is_err())
    }
}