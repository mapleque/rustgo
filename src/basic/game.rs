use crate::basic::*;
use crate::util::{LinkedTree, LinkedTreeOperation};

pub struct Game {
    current_board: Board,
    current_player: Player,
    current_cmd: LinkedTree<Cmd>,
    current_zip_board: LinkedTree<BoardZip>,
}

#[derive(Clone, PartialEq)]
pub enum Player {
    Black,
    White,
}

#[derive(Clone, Debug)]
pub enum Cmd {
    Start,
    Pass,
    Step(String),
}

impl Cmd {
    fn cmd_to_point(cmd: String) -> Result<(usize, usize), String> {
        if cmd.len() != 2 {
            println!("invalid cmd: {:?} with length {}", cmd, cmd.len());
            return Err(format!("invalid cmd: {}", cmd));
        }
        let arr = cmd.as_bytes();
        let x = arr[0] as usize - 'a' as usize + 1;
        let y = arr[1] as usize - 'a' as usize + 1;
        Ok((x, y))
    }
}

impl Game {
    pub fn new(size: BoardSize) -> Game {
        let cmd_history = LinkedTree::new_tree(Cmd::Start);
        let b = Board::new(size.clone());
        let zb = zip_board(&b);
        let zb_history = LinkedTree::new_tree(zb).ptr();
        Game {
            current_board: b,
            current_player: Player::Black,
            current_cmd: cmd_history.ptr(),
            current_zip_board: zb_history.ptr(),
        }
    }

    pub fn next(&mut self, cmd: Cmd) -> Result<(), String> {
        match cmd.clone() {
            Cmd::Pass => self.change_player()?,
            Cmd::Step(p) => self.step(p)?,
            other => {
                return Err(format!("invalid next cmd: {:?}", other));
            }
        };
        self.add_cmd_history(cmd);
        self.add_board_history();
        Ok(())
    }

    pub fn undo(&mut self) -> Result<(), String> {
        if self.current_cmd.parent().is_none() {
            return Err(format!("can not undo"));
        }
        self.current_cmd = self.current_cmd.parent().unwrap().ptr();
        self.current_zip_board = self.current_zip_board.parent().unwrap().ptr();
        self.current_board = unzip_board(&self.current_zip_board.val());
        self.change_player()
    }

    pub fn redo(&mut self, index: usize) -> Result<(), String> {
        if index >= self.current_cmd.child_len() {
            return Err(format!("no redo steps {:?}", index));
        }
        self.current_cmd = self.current_cmd.child(index).unwrap().ptr();
        self.current_zip_board = self.current_zip_board.child(index).unwrap().ptr();
        self.current_board = unzip_board(&self.current_zip_board.val());
        self.change_player()
    }

    pub fn redo_list(&self) -> Vec<Cmd> {
        let mut ret: Vec<Cmd> = vec![];
        for i in 0..self.current_cmd.child_len() {
            let cmd = self.current_cmd.child(i).unwrap().ptr();
            ret.push(cmd.val());
        }
        ret
    }

    pub fn step_count(&self) -> usize {
        self.current_cmd.deepth()
    }

    pub fn next_player(&self) -> Player {
        self.current_player.clone()
    }

    fn add_cmd_history(&mut self, cmd: Cmd) {
        let node = self.current_cmd.add_child(cmd);
        self.current_cmd = node;
    }

    fn add_board_history(&mut self) {
        let zb = zip_board(&self.current_board);
        let node = self.current_zip_board.add_child(zb);
        self.current_zip_board = node;
    }

    fn change_player(&mut self) -> Result<(), String> {
        match self.current_player {
            Player::Black => self.current_player = Player::White,
            Player::White => self.current_player = Player::Black,
        };
        Ok(())
    }

    fn step(&mut self, cmd: String) -> Result<(), String> {
        let (x, y) = Cmd::cmd_to_point(cmd)?;
        let stone = match self.current_player {
            Player::Black => Stone::Black,
            Player::White => Stone::White,
        };
        check_if_empty(&self.current_board, x, y)?;
        self.current_board = check_if_never_repeat_with_new_stone(
            &self.current_board,
            stone,
            x,
            y,
            self.current_zip_board.list_parents(),
        )?;
        self.change_player()
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\n")?;
        let indent = match self.current_board.size() {
            BoardSize::Normal => "        ",
            BoardSize::Medium => "  ",
            BoardSize::Small => "",
        };
        match self.current_player {
            Player::Black => write!(f, "{}White(o)   [{}] > Black(x)", indent, self.step_count())?,
            Player::White => write!(f, "{}White(o) < [{}]   Black(x)", indent, self.step_count())?,
        };
        write!(f, "\n")?;
        write!(f, "\n")?;
        write!(f, "{}", self.current_board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmd_to_point() {
        assert!(Cmd::cmd_to_point(String::from("aa")).unwrap() == (1, 1));
        assert!(Cmd::cmd_to_point(String::from("aa")).unwrap() != (1, 2));
        assert!(Cmd::cmd_to_point(String::from("ss")).unwrap() == (19, 19));
        assert!(Cmd::cmd_to_point(String::from("as")).unwrap() == (1, 19));
        assert!(Cmd::cmd_to_point(String::from("sa")).unwrap() == (19, 1));
    }

    #[test]
    fn a_normal_game() {
        let mut g = Game::new(BoardSize::Normal);
        g.next(Cmd::Step("aa".to_string())).unwrap();
        assert!(g.current_board.is(1, 1, Stone::Black).unwrap());
        assert!(g.step_count() == 1);
        assert!(g.next_player() == Player::White);
        g.next(Cmd::Step("ba".to_string())).unwrap();
        assert!(g.current_board.is(2, 1, Stone::White).unwrap());
        assert!(g.step_count() == 2);
        assert!(g.next_player() == Player::Black);
        g.next(Cmd::Pass).unwrap();
        assert!(g.step_count() == 3);
        assert!(g.next_player() == Player::White);
        g.next(Cmd::Step("ab".to_string())).unwrap();
        assert!(g.current_board.is(1, 2, Stone::White).unwrap());
        assert!(g.current_board.is(1, 1, Stone::Empty).unwrap());
        assert!(g.step_count() == 4);
        assert!(g.next_player() == Player::Black);
        g.undo().unwrap();
        assert!(g.current_board.is(1, 2, Stone::Empty).unwrap());
        assert!(g.current_board.is(1, 1, Stone::Black).unwrap());
        assert!(g.step_count() == 3);
        assert!(g.next_player() == Player::White);
        assert!(g.redo_list().len() == 1);
        g.redo(0).unwrap();
        assert!(g.current_board.is(1, 2, Stone::White).unwrap());
        assert!(g.current_board.is(1, 1, Stone::Empty).unwrap());
        assert!(g.step_count() == 4);
        assert!(g.next_player() == Player::Black);
        assert!(g.redo_list().len() == 0);
    }
}
