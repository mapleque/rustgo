#[derive(Copy, Clone, Debug)]
pub enum Chess {
    IsEmpty,
    Black,
    White,
}

impl std::fmt::Display for Chess {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Chess::IsEmpty => write!(f, "{}", "."),
            Chess::Black => write!(f, "{}", "x"),
            Chess::White => write!(f, "{}", "o"),
        }
    }
}

pub enum BoardSize {
    Normal,
    Medium,
    Small,
}

pub struct Board {
    coord: [Chess; 19 * 19],
    size: usize,
    mode: BoardSize,
}

impl Board {
    // new board with size
    pub fn new(mode: BoardSize) -> Board {
        let i = match mode {
            BoardSize::Normal => 19,
            BoardSize::Medium => 13,
            BoardSize::Small => 9,
        };
        Board {
            coord: [Chess::IsEmpty; 19 * 19],
            size: i,
            mode,
        }
    }

    // add a chess to the point
    pub fn add(&mut self, r: Chess, x: usize, y: usize) -> Result<(), String> {
        let i = self.point_to_index(x, y)?;
        self.coord[i] = r;
        Ok(())
    }
    // del a chess from the point
    pub fn del(&mut self, x: usize, y: usize) -> Result<(), String> {
        let i = self.point_to_index(x, y)?;
        self.coord[i] = Chess::IsEmpty;
        Ok(())
    }

    // point (x, y) is star position
    fn is_star_position(&self, x: usize, y: usize) -> Result<bool, String> {
        let _ = self.point_to_index(x, y)?;
        match self.mode {
            BoardSize::Normal => {
                return Ok((x == 4 || x == 10 || x == 16) && (y == 4 || y == 10 || y == 16));
            }
            BoardSize::Medium => Ok((x == 4 || x == 7 || x == 10) && (y == 4 || y == 7 || y == 10)),
            BoardSize::Small => Ok((x == 3 || x == 5 || x == 7) && (y == 3 || y == 5 || y == 7)),
        }
    }

    // change index to point
    fn index_to_point(&self, i: usize) -> Result<(usize, usize), String> {
        if i >= self.size * self.size {
            return Err(format!(
                "index {} is too large for current board size {}",
                i, self.size,
            ));
        }
        Ok((i % self.size + 1, i / self.size + 1))
    }

    // change point to index
    fn point_to_index(&self, x: usize, y: usize) -> Result<usize, String> {
        if x < 1 || x > self.size || y < 1 || y > self.size {
            return Err(format!(
                "point ({}, {}) is not match current board size {}",
                x, y, self.size,
            ));
        }
        Ok((y - 1) * self.size + x - 1)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "  ")?;
        for i in 0..self.size {
            write!(f, "{} ", ('a' as u8 + i as u8) as char)?;
        }

        for i in 0..self.size * self.size {
            let v = self.coord[i];
            let (x, y) = self.index_to_point(i).unwrap();
            if x == 1 {
                write!(f, "{}", "\n")?;
                write!(f, "{} ", ('a' as u8 + (y - 1) as u8) as char)?;
            }
            if self.is_star_position(x, y).unwrap() {
                match v {
                    Chess::IsEmpty => {
                        write!(f, "{}", "。")?;
                    }
                    other => {
                        write!(f, "{} ", v)?;
                    }
                }
            } else {
                write!(f, "{} ", v)?;
            }
            if x == self.size {
                write!(f, "{}", ('a' as u8 + (y - 1) as u8) as char)?;
            }
        }
        write!(f, "{}", "\n")?;
        write!(f, "  ")?;
        for i in 0..self.size {
            write!(f, "{} ", ('a' as u8 + i as u8) as char)?;
        }
        write!(f, "{}", "\n")
    }
}
