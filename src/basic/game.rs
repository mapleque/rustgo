use crate::basic::{Board, BoardSize, Stone};
use crate::util::{LinkedTree, LinkedTreeOperation};

pub struct Game {
    current_board: Board,
    current_player: Player,
    history_cmd: LinkedTree<Cmd>,
    current_cmd: LinkedTree<Cmd>,
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
        let cmd = LinkedTree::new_tree(Cmd::Start);
        Game {
            current_board: Board::new(size.clone()),
            current_player: Player::Black,
            history_cmd: cmd.ptr(),
            current_cmd: cmd.ptr(),
        }
    }

    pub fn next(&mut self, cmd: Cmd) -> Result<(), String> {
        self.add_history(cmd.clone());
        match cmd {
            Cmd::Pass => self.change_player(),
            Cmd::Step(p) => self.step(p),
            other => Err(format!("invalid next cmd: {:?}", other)),
        }
    }

    pub fn undo(&mut self) -> Result<(), String> {
        if self.current_cmd.parent().is_none() {
            return Err(format!("can not undo"));
        }
        let cmd = self.current_cmd.val();
        self.current_cmd = self.current_cmd.parent().unwrap().ptr();
        match cmd {
            Cmd::Pass => self.change_player(),
            Cmd::Step(p) => self.unstep(p),
            other => Err(format!("invalid undo cmd: {:?}", other)),
        }
    }

    pub fn redo(&mut self, index: usize) -> Result<(), String> {
        if index >= self.current_cmd.child_len() {
            return Err(format!("no redo steps {:?}", index));
        }
        let cmd = self.current_cmd.child(index).unwrap().ptr();
        self.current_cmd = cmd.ptr();
        match cmd.val() {
            Cmd::Pass => self.change_player(),
            Cmd::Step(p) => self.step(p),
            other => Err(format!("invalid redo cmd: {:?}", other)),
        }
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

    pub fn get_history(&self) -> LinkedTree<Cmd> {
        self.history_cmd.ptr()
    }

    fn add_history(&mut self, cmd: Cmd) {
        let node = self.current_cmd.add_child(cmd);
        self.current_cmd = node;
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
        self.current_board.add(stone, x, y)?;
        self.change_player()
    }

    fn unstep(&mut self, cmd: String) -> Result<(), String> {
        let (x, y) = Cmd::cmd_to_point(cmd)?;
        self.current_board.del(x, y)?;
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
        // TODO assert!(g.current_board.is(1, 1, Stone::Empty).unwrap());
        assert!(g.step_count() == 4);
        assert!(g.next_player() == Player::Black);
        g.undo().unwrap();
        assert!(g.current_board.is(1, 2, Stone::Empty).unwrap());
        // TODO assert!(g.current_board.is(1, 1, Stone::Black).unwrap());
        assert!(g.step_count() == 3);
        assert!(g.next_player() == Player::White);
        assert!(g.redo_list().len() == 1);
        g.redo(0).unwrap();
        assert!(g.current_board.is(1, 2, Stone::White).unwrap());
        // TODO assert!(g.current_board.is(1, 1, Stone::Empty).unwrap());
        assert!(g.step_count() == 4);
        assert!(g.next_player() == Player::Black);
        assert!(g.redo_list().len() == 0);
    }
}
