#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Chess {
    Empty,
    Black,
    White,
}

impl std::fmt::Display for Chess {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Chess::Empty => write!(f, "{}", "."),
            Chess::Black => write!(f, "{}", "x"),
            Chess::White => write!(f, "{}", "o"),
        }
    }
}

#[derive(Clone)]
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
            coord: [Chess::Empty; 19 * 19],
            size: i,
            mode,
        }
    }

    pub fn is(&self, x: usize, y: usize, t: Chess) -> Result<bool, String> {
        let i = self.point_to_index(x, y)?;
        Ok(self.coord[i] == t)
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
        self.coord[i] = Chess::Empty;
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
                    Chess::Empty => {
                        write!(f, "{}", "。")?;
                    }
                    _other => {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_mode_has_correct_star_position() {
        let g = Board::new(BoardSize::Normal);
        // star points
        assert!(g.is_star_position(4, 4).unwrap());
        assert!(g.is_star_position(4, 10).unwrap());
        assert!(g.is_star_position(4, 16).unwrap());
        assert!(g.is_star_position(10, 4).unwrap());
        assert!(g.is_star_position(10, 10).unwrap());
        assert!(g.is_star_position(10, 16).unwrap());
        assert!(g.is_star_position(16, 4).unwrap());
        assert!(g.is_star_position(16, 10).unwrap());
        assert!(g.is_star_position(16, 16).unwrap());
        // not star points
        assert!(!g.is_star_position(1, 1).unwrap());
        assert!(!g.is_star_position(3, 3).unwrap());
        assert!(!g.is_star_position(7, 7).unwrap());
        let g = Board::new(BoardSize::Medium);
        // star points
        assert!(g.is_star_position(4, 4).unwrap());
        assert!(g.is_star_position(4, 7).unwrap());
        assert!(g.is_star_position(4, 10).unwrap());
        assert!(g.is_star_position(7, 4).unwrap());
        assert!(g.is_star_position(7, 7).unwrap());
        assert!(g.is_star_position(7, 10).unwrap());
        assert!(g.is_star_position(10, 4).unwrap());
        assert!(g.is_star_position(10, 7).unwrap());
        assert!(g.is_star_position(10, 10).unwrap());
        // not star points
        assert!(!g.is_star_position(1, 1).unwrap());
        assert!(!g.is_star_position(3, 3).unwrap());
        assert!(!g.is_star_position(5, 5).unwrap());
        let g = Board::new(BoardSize::Small);
        // star points
        assert!(g.is_star_position(3, 3).unwrap());
        assert!(g.is_star_position(3, 5).unwrap());
        assert!(g.is_star_position(3, 7).unwrap());
        assert!(g.is_star_position(5, 3).unwrap());
        assert!(g.is_star_position(5, 5).unwrap());
        assert!(g.is_star_position(5, 7).unwrap());
        assert!(g.is_star_position(7, 3).unwrap());
        assert!(g.is_star_position(7, 5).unwrap());
        assert!(g.is_star_position(7, 7).unwrap());
        // not star points
        assert!(!g.is_star_position(1, 1).unwrap());
        assert!(!g.is_star_position(4, 4).unwrap());
    }

    #[test]
    fn each_mode_has_correct_add_position() {
        let mut g = Board::new(BoardSize::Normal);
        g.add(Chess::Black, 1, 1).unwrap();
        assert!(g.coord[0] == Chess::Black);
        assert!(g.coord[1] == Chess::Empty);
        g.add(Chess::White, 2, 1).unwrap();
        assert!(g.coord[1] == Chess::White);
        assert!(g.coord[2] == Chess::Empty);
        assert!(g.coord[19] == Chess::Empty);
        g.add(Chess::Black, 1, 2).unwrap();
        assert!(g.coord[0] == Chess::Black);
        assert!(g.coord[1] == Chess::White);
        assert!(g.coord[19] == Chess::Black);
        g.add(Chess::White, 19, 19).unwrap();
        assert!(g.coord[0] == Chess::Black);
        assert!(g.coord[1] == Chess::White);
        assert!(g.coord[19] == Chess::Black);
        assert!(g.coord[360] == Chess::White);
        g.del(1, 2).unwrap();
        assert!(g.coord[19] == Chess::Empty);
        for i in 2..359 {
            if g.coord[i] != Chess::Empty {
                println!("index {} is not empty", i);
            }
            assert!(g.coord[i] == Chess::Empty);
        }
    }
}
