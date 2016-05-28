const BOARD_SIZE: usize = 19;

pub type Bpos = u32;

#[derive(Copy, Clone)]
enum Intersection {
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
            Player::Black => Intersection::White,
            Player::White => Intersection::Black,
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

	fn make_move(&self, row : Bpos, column : Bpos) -> GameState {
        let mut g = self.clone();
        let place : usize = (row * self.size + column) as usize;
		g.board[place] = self.turn.to_intersection();
        g.turn = self.turn.other();
        return g
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

pub struct Game {
    pub states : Vec<GameState>,
}

impl Game {
    pub fn new() -> Game {
        let states = vec![GameState::new(19)];
        Game{ states : states }
    }

    pub fn make_move(&mut self, row : Bpos, column : Bpos) {
        //states should always be non empty, so unwrap should be okay.
        let new_state = self.states.last().unwrap().make_move(row, column);
        self.states.push(new_state);
    }
}

