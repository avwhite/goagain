const BOARD_SIZE: usize = 19;

pub type Bpos = u32;

#[derive(Copy, Clone)]
pub enum Intersection {
	Empty,
	Black,
	White,
}

impl ToString for Intersection {
	fn to_string(&self) -> String {
		match *self {
			Intersection::Empty => ".".to_string(),
			Intersection::Black => "B".to_string(),
			Intersection::White => "W".to_string(),
		}
	}
}

#[derive(Clone)]
enum Player {
	Black,
	White,
}

impl Player {
    fn other(&self) -> Player{
        match *self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }

    fn to_intersection(&self) -> Intersection {
        match *self {
            Player::Black => Intersection::Black,
            Player::White => Intersection::White,
        }
    }
}

#[derive(Clone)]
pub struct GameState {
	board : Vec<Intersection>,
	size : u32,
	turn : Player,
}

impl GameState {
	fn new(size : u32) -> GameState {
		GameState{
			board : vec![Intersection::Empty ; (size*size) as usize],
			size : size,
			turn : Player::Black
		}
	}

	pub fn make_move(&self, row : Bpos, column : Bpos) -> GameState {
        let mut g = self.clone();
        let place : usize = (row * self.size + column) as usize;
		g.board[place] = self.turn.to_intersection();
        g.turn = self.turn.other();
        return g;
	}

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn intersection(&self, row : Bpos, column: Bpos) -> Intersection {
        let place : usize = (row * self.size + column) as usize;
        return self.board[place];
    }
}

impl ToString for GameState {
	fn to_string(&self) -> String {
		let mut i = 0;
		let mut res = String::new();
		for intersec in self.board.iter() {
			if i == self.size {
				res.push_str("\n");
				i = 1;
			} else {
				i = i + 1;
			}
			res.push_str(&intersec.to_string());
		}
		return res;
	}
}

pub struct GameModel {
    states : Vec<GameState>,
    current_state : usize,
}

impl GameModel {
    pub fn new() -> GameModel {
        let states = vec![GameState::new(19)];
        GameModel{
            states : states,
            current_state : 0,
        }
    }

    pub fn make_move(&mut self, row : Bpos, column : Bpos) {
        //states should always be non empty, so unwrap should be okay.
        let new_state = self.states.last().unwrap().make_move(row, column);
        self.states.push(new_state);
        self.current_state = self.states.len() - 1;
    }

    pub fn current_state(&self) -> &GameState {
        let i = self.current_state;
        &self.states[i]
    }
}
