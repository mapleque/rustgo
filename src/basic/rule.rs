use crate::basic::{unzip_board, zip_board};
use crate::basic::{Board, BoardZip, Stone};
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
pub struct Point {
    stone: Stone,
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(stone: Stone, x: usize, y: usize) -> Point {
        Point { stone, x, y }
    }
}

// check target point is empty
pub fn check_if_empty(board: &Board, x: usize, y: usize) -> Result<(), String> {
    if !board.is(x, y, Stone::Empty)? {
        return Err(format!("this position ({},{}) is not empty", x, y));
    }
    Ok(())
}
// can not same to history, know as ko
pub fn check_if_never_repeat_with_new_stone(
    board: &Board,
    stone: Stone,
    x: usize,
    y: usize,
    board_history: Vec<BoardZip>,
) -> Result<Board, String> {
    let zb = zip_board(board);
    let mut nb = unzip_board(&zb);
    nb.add(stone, x, y)?;
    remove_lose_liberty_stones(&mut nb, x, y)?;
    let nzb = zip_board(&nb);
    for b in board_history {
        println!("check repeat\n{}\n{}", unzip_board(&nzb), unzip_board(&b));
        if nzb == b {
            return Err(format!("this point ({},{}) has same scene before", x, y));
        }
    }
    Ok(nb)
}

// remove lose liberty stones relatate current stone positon
pub fn remove_lose_liberty_stones(board: &mut Board, x: usize, y: usize) -> Result<(), String> {
    let stone = board.at(x, y)?;
    let mut remove_flag: bool = false;
    for p in neighbour_at(&board, x, y) {
        if p.stone == stone.another() {
            if calc_liberty(&board, p.x, p.y) == 0 {
                remove_block(board, p.x, p.y);
                remove_flag = true;
            }
        }
    }
    if calc_liberty(&board, x, y) == 0 && !remove_flag {
        return Err(format!("this point ({},{}) has no liberty", x, y));
    }
    Ok(())
}

// only considered with size edge
fn neighbour_at(board: &Board, x: usize, y: usize) -> Vec<Point> {
    let mut ret = vec![];
    for (nx, ny) in vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
        match board.at(nx, ny) {
            Ok(stone) => ret.push(Point::new(stone, nx, ny)),
            Err(_) => continue,
        }
    }
    ret
}

// calculate liberty of the block start from target point
fn calc_liberty(board: &Board, x: usize, y: usize) -> usize {
    let block = get_block(board, x, y);
    let mut liberty: HashSet<Point> = HashSet::new();
    for bp in block.iter() {
        for np in neighbour_at(board, bp.x, bp.y) {
            if board.is(np.x, np.y, Stone::Empty).unwrap() {
                liberty.insert(Point::new(Stone::Empty, np.x, np.y));
            }
        }
    }
    liberty.len()
}

// return a set of stones combine to target point, which stones are same
fn get_block(board: &Board, x: usize, y: usize) -> HashSet<Point> {
    let mut ret: HashSet<Point> = HashSet::new();
    let stone = board.at(x, y).unwrap();
    ret.insert(Point::new(stone, x, y));
    for p in neighbour_at(board, x, y) {
        if p.stone == stone {
            ret.insert(p);
        }
    }
    ret
}

// remove all block stones start from target point
fn remove_block(board: &mut Board, x: usize, y: usize) {
    let block = get_block(&board, x, y);
    for bp in block.iter() {
        board.del(bp.x, bp.y).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::basic::BoardSize;

    #[test]
    fn test_neighbour_at() {
        let b = Board::new(BoardSize::Small);
        let ret = neighbour_at(&b, 1, 1);
        assert!(ret.len() == 2);
        assert!(ret[0] == Point::new(Stone::Empty, 2, 1));
        assert!(ret[1] == Point::new(Stone::Empty, 1, 2));
        let ret = neighbour_at(&b, 1, 2);
        assert!(ret.len() == 3);
        assert!(ret[0] == Point::new(Stone::Empty, 2, 2));
        assert!(ret[1] == Point::new(Stone::Empty, 1, 1));
        assert!(ret[2] == Point::new(Stone::Empty, 1, 3));
        let ret = neighbour_at(&b, 2, 2);
        assert!(ret.len() == 4);
        assert!(ret[0] == Point::new(Stone::Empty, 1, 2));
        assert!(ret[1] == Point::new(Stone::Empty, 3, 2));
        assert!(ret[2] == Point::new(Stone::Empty, 2, 1));
        assert!(ret[3] == Point::new(Stone::Empty, 2, 3));
    }

    #[test]
    fn test_remove_block() {
        let mut b = Board::new(BoardSize::Small);
        b.add(Stone::Black, 1, 1);
        b.add(Stone::Black, 1, 2);
        b.add(Stone::White, 2, 1);
        b.add(Stone::White, 2, 2);
        b.add(Stone::White, 1, 3);
        remove_block(&mut b, 1, 1);
        assert!(b.is(1, 1, Stone::Empty).unwrap());
        assert!(b.is(1, 2, Stone::Empty).unwrap());
        assert!(b.is(2, 1, Stone::White).unwrap());
        assert!(b.is(2, 2, Stone::White).unwrap());
        assert!(b.is(1, 3, Stone::White).unwrap());
        remove_block(&mut b, 1, 3);
        assert!(b.is(1, 1, Stone::Empty).unwrap());
        assert!(b.is(1, 2, Stone::Empty).unwrap());
        assert!(b.is(2, 1, Stone::White).unwrap());
        assert!(b.is(2, 2, Stone::White).unwrap());
        assert!(b.is(1, 3, Stone::Empty).unwrap());
        remove_block(&mut b, 2, 2);
        assert!(b.is(1, 1, Stone::Empty).unwrap());
        assert!(b.is(1, 2, Stone::Empty).unwrap());
        assert!(b.is(2, 1, Stone::Empty).unwrap());
        assert!(b.is(2, 2, Stone::Empty).unwrap());
        assert!(b.is(1, 3, Stone::Empty).unwrap());
    }

    #[test]
    fn test_calc_liberty() {
        let mut b = Board::new(BoardSize::Small);
        b.add(Stone::Black, 1, 1);
        assert!(calc_liberty(&b, 1, 1) == 2);
        b.add(Stone::Black, 1, 2);
        assert!(calc_liberty(&b, 1, 1) == 3);
        b.add(Stone::White, 2, 1);
        assert!(calc_liberty(&b, 1, 1) == 2);
        b.add(Stone::Black, 2, 3);
        assert!(calc_liberty(&b, 1, 1) == 2);
    }

    fn test_remove_lose_liberty_stones() {
        let mut b = Board::new(BoardSize::Small);
        b.add(Stone::Black, 1, 1);
        b.add(Stone::White, 2, 1);
        b.add(Stone::White, 1, 2);
        remove_lose_liberty_stones(&mut b, 1, 2).unwrap();
        assert!(b.is(1, 1, Stone::Empty).unwrap());
        b.add(Stone::Black, 2, 2);
        b.add(Stone::Black, 1, 3);
        b.add(Stone::Black, 1, 1);
        remove_lose_liberty_stones(&mut b, 1, 1).unwrap();
        assert!(b.is(1, 1, Stone::Black).unwrap());
        assert!(b.is(1, 2, Stone::Empty).unwrap());
    }

    #[test]
    fn test_check_if_never_repeat_with_new_stone() {
        let mut b = Board::new(BoardSize::Small);
        let mut his = vec![zip_board(&b)];
        b.add(Stone::Black, 1, 1);
        his.push(zip_board(&b));
        b.add(Stone::White, 4, 1);
        his.push(zip_board(&b));
        b.add(Stone::Black, 3, 1);
        his.push(zip_board(&b));
        b.add(Stone::White, 3, 2);
        his.push(zip_board(&b));
        b.add(Stone::Black, 2, 2);
        his.push(zip_board(&b));
        b.add(Stone::White, 2, 1);
        b.del(3, 1);
        his.push(zip_board(&b));

        match check_if_never_repeat_with_new_stone(&b, Stone::Black, 3, 1, his) {
            Ok(_) => assert!(false),
            Err(err) => assert!(err == "this point (3,1) has same scene before"),
        }
    }
}
