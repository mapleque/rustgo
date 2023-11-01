#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Stone {
    Empty,
    Black,
    White,
}

impl std::fmt::Display for Stone {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Stone::Empty => write!(f, "{}", "."),
            Stone::Black => write!(f, "{}", "x"),
            Stone::White => write!(f, "{}", "o"),
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
    coord: [Stone; 19 * 19],
    size: usize,
}

impl Board {
    // new board with size
    pub fn new(size: BoardSize) -> Board {
        let i = match size {
            BoardSize::Normal => 19,
            BoardSize::Medium => 13,
            BoardSize::Small => 9,
        };
        Board {
            coord: [Stone::Empty; 19 * 19],
            size: i,
        }
    }

    pub fn size(&self) -> BoardSize {
        match self.size {
            9 => BoardSize::Small,
            13 => BoardSize::Medium,
            _ => BoardSize::Normal,
        }
    }

    pub fn is(&self, x: usize, y: usize, t: Stone) -> Result<bool, String> {
        let i = self.point_to_index(x, y)?;
        Ok(self.coord[i] == t)
    }

    // add a stone to the point
    pub fn add(&mut self, r: Stone, x: usize, y: usize) -> Result<(), String> {
        let i = self.point_to_index(x, y)?;
        self.coord[i] = r;
        Ok(())
    }
    // del a stone from the point
    pub fn del(&mut self, x: usize, y: usize) -> Result<(), String> {
        let i = self.point_to_index(x, y)?;
        self.coord[i] = Stone::Empty;
        Ok(())
    }

    // point (x, y) is star position
    fn is_star_position(&self, x: usize, y: usize) -> Result<bool, String> {
        let _ = self.point_to_index(x, y)?;
        match self.size() {
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
                    Stone::Empty => {
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

// zip board data with follow rule:
//  - 2 bit as 1 position
//  - 0 for empty, 1 for black stone, 2 for white stone, 3 is illegal
#[derive(PartialEq, Debug)]
pub enum BoardZip {
    // 19*19*2 = 361*2 = 128+128+128+128+128+128-46
    NormalBoardZip(u128, u128, u128, u128, u128, u128),
    // 13*13*2 = 169*2 = 128+128+128-46
    MediumBoardZip(u128, u128, u128),
    // 9*9*2 = 81*2 = 128+64-30
    SmallBoardZip(u128, u64),
}

pub fn zip_board(board: &Board) -> BoardZip {
    match board.size() {
        BoardSize::Normal => zip_board_normal(board),
        BoardSize::Medium => zip_board_medium(board),
        BoardSize::Small => zip_board_small(board),
    }
}

fn zip_board_normal(board: &Board) -> BoardZip {
    let d1 = zip_stone_128(board, 64 * 0, 64 * 1);
    let d2 = zip_stone_128(board, 64 * 1, 64 * 2);
    let d3 = zip_stone_128(board, 64 * 2, 64 * 3);
    let d4 = zip_stone_128(board, 64 * 3, 64 * 4);
    let d5 = zip_stone_128(board, 64 * 4, 64 * 5);
    let d6 = zip_stone_128(board, 64 * 5, 361);
    BoardZip::NormalBoardZip(d1, d2, d3, d4, d5, d6)
}

fn zip_board_medium(board: &Board) -> BoardZip {
    let d1 = zip_stone_128(board, 64 * 0, 64 * 1);
    let d2 = zip_stone_128(board, 64 * 1, 64 * 2);
    let d3 = zip_stone_128(board, 64 * 2, 169);
    BoardZip::MediumBoardZip(d1, d2, d3)
}
fn zip_board_small(board: &Board) -> BoardZip {
    let d1 = zip_stone_128(board, 64 * 0, 64 * 1);
    let d2 = zip_stone_128(board, 64 * 1, 81) as u64;
    BoardZip::SmallBoardZip(d1, d2)
}

fn zip_stone_128(board: &Board, from: usize, to: usize) -> u128 {
    let mut d: u128 = 0;
    for i in from..to - 1 {
        d += zip_stone_to_val(board.coord[i]) as u128;
        d = d << 2;
    }
    d += zip_stone_to_val(board.coord[to - 1]) as u128;
    d
}

fn zip_stone_to_val(stone: Stone) -> u8 {
    match stone {
        Stone::Empty => 0,
        Stone::Black => 1,
        Stone::White => 2,
    }
}

pub fn unzip_board(zip: &BoardZip) -> Board {
    match zip {
        BoardZip::NormalBoardZip(d1, d2, d3, d4, d5, d6) => {
            return unzip_board_normal(d1, d2, d3, d4, d5, d6);
        }
        BoardZip::MediumBoardZip(d1, d2, d3) => unzip_board_medium(d1, d2, d3),
        BoardZip::SmallBoardZip(d1, d2) => unzip_board_small(d1, d2),
    }
}

fn unzip_board_normal(d1: &u128, d2: &u128, d3: &u128, d4: &u128, d5: &u128, d6: &u128) -> Board {
    let mut b = Board::new(BoardSize::Normal);
    unzip_stone_128(&mut b, d1, 64 * 0, 64 * 1);
    unzip_stone_128(&mut b, d2, 64 * 1, 64 * 2);
    unzip_stone_128(&mut b, d3, 64 * 2, 64 * 3);
    unzip_stone_128(&mut b, d4, 64 * 3, 64 * 4);
    unzip_stone_128(&mut b, d5, 64 * 4, 64 * 5);
    unzip_stone_128(&mut b, d6, 64 * 5, 361);
    b
}
fn unzip_board_medium(d1: &u128, d2: &u128, d3: &u128) -> Board {
    let mut b = Board::new(BoardSize::Medium);
    unzip_stone_128(&mut b, d1, 64 * 0, 64 * 1);
    unzip_stone_128(&mut b, d2, 64 * 1, 64 * 2);
    unzip_stone_128(&mut b, d3, 64 * 2, 169);
    b
}
fn unzip_board_small(d1: &u128, d2: &u64) -> Board {
    let mut b = Board::new(BoardSize::Small);
    unzip_stone_128(&mut b, d1, 64 * 0, 64 * 1);
    unzip_stone_128(&mut b, &(*d2 as u128), 64 * 1, 81);
    b
}

fn unzip_stone_128(b: &mut Board, d: &u128, from: usize, to: usize) {
    let mut r = d.clone();
    for i in from..to {
        b.coord[to - i - 1 + from] = unzip_val_to_stone(r as u8 % 4);
        r = r >> 2;
    }
}

fn unzip_val_to_stone(d: u8) -> Stone {
    match d % 4 {
        1 => Stone::Black,
        2 => Stone::White,
        _ => Stone::Empty,
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
        g.add(Stone::Black, 1, 1).unwrap();
        assert!(g.coord[0] == Stone::Black);
        assert!(g.coord[1] == Stone::Empty);
        g.add(Stone::White, 2, 1).unwrap();
        assert!(g.coord[1] == Stone::White);
        assert!(g.coord[2] == Stone::Empty);
        assert!(g.coord[19] == Stone::Empty);
        g.add(Stone::Black, 1, 2).unwrap();
        assert!(g.coord[0] == Stone::Black);
        assert!(g.coord[1] == Stone::White);
        assert!(g.coord[19] == Stone::Black);
        g.add(Stone::White, 19, 19).unwrap();
        assert!(g.coord[0] == Stone::Black);
        assert!(g.coord[1] == Stone::White);
        assert!(g.coord[19] == Stone::Black);
        assert!(g.coord[360] == Stone::White);
        g.del(1, 2).unwrap();
        assert!(g.coord[19] == Stone::Empty);
        for i in 2..359 {
            if g.coord[i] != Stone::Empty {
                println!("index {} is not empty", i);
            }
            assert!(g.coord[i] == Stone::Empty);
        }
    }

    #[test]
    fn zip_and_unzip() {
        let mut g1 = Board::new(BoardSize::Normal);
        g1.add(Stone::Black, 1, 1).unwrap();
        assert!(g1.coord[0] == Stone::Black);
        let mut g2 = Board::new(BoardSize::Normal);
        g2.add(Stone::Black, 1, 1).unwrap();
        let z1 = zip_board(&g1);
        let z2 = zip_board(&g2);
        assert!(z1 == z2);
        let uz1 = unzip_board(&z1);
        println!("{:?}", uz1.coord);
        assert!(uz1.coord[0] == Stone::Black);
        assert!(uz1.coord[1] == Stone::Empty);
    }
}
