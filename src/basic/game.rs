use crate::basic::{Board, BoardSize, Chess};

pub struct Game {
    board: Board,
    pub currentPlayer: Player,
}

pub enum Player {
    Black,
    White,
}

pub enum Cmd {
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
        Game {
            board: Board::new(size),
            currentPlayer: Player::Black,
        }
    }

    pub fn next(&mut self, cmd: Cmd) -> Result<(), String> {
        match cmd {
            Cmd::Pass => self.change_player(),
            Cmd::Step(p) => self.step(p),
        };
        Ok(())
    }

    pub fn back(&mut self) -> Result<(), String> {
        Ok(())
    }

    pub fn estimate() {}

    fn change_player(&mut self) -> Result<(), String> {
        match self.currentPlayer {
            Player::Black => self.currentPlayer = Player::White,
            Player::White => self.currentPlayer = Player::Black,
        };
        Ok(())
    }

    fn step(&mut self, cmd: String) -> Result<(), String> {
        let (x, y) = Cmd::cmd_to_point(cmd)?;
        let chess = match self.currentPlayer {
            Player::Black => Chess::Black,
            Player::White => Chess::White,
        };
        self.board.add(chess, x, y)?;
        self.change_player()
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\n")?;
        match self.currentPlayer {
            Player::Black => write!(f, "White   {} > Black", 0)?,
            Player::White => write!(f, "White < {}   Black", 0)?,
        };
        write!(f, "\n")?;
        write!(f, "\n")?;
        write!(f, "{}", self.board)
    }
}
