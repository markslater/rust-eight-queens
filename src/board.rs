pub struct Board {
    squares: [bool; 64], // TODO could be an unsigned 64 bit integer
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [false; 64]
        }
    }

    pub fn set(&self, square: usize) -> Result<Board, ()> { // TODO should we use a smaller argument?  Does it matter?
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
