use std::collections::HashSet;
use std::fmt;

pub type Bpos = (u32, u32);

pub enum MoveError {
    Suicide,
    Ko,
    SuperKo,
    OnTopOfOther,
    NotOnBoard,
}

impl fmt::Display for MoveError {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MoveError::Suicide => write!(f, "Invalid Move: Suicide"),
            MoveError::Ko => write!(f, "Invalid Move: Ko"),
            MoveError::SuperKo=> write!(f, "Invalid Move: Super Ko"),
            MoveError::OnTopOfOther=> write!(f, "Invalid Move: On top of other stone"),
            MoveError::NotOnBoard=> write!(f, "Invalid Move: Not on the board"),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
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

#[derive(Debug)]
struct Group {
    color : Intersection,
    members : HashSet<Bpos>,
    alive : bool,
}

impl Group {
    fn new(color : Intersection) -> Group {
        Group{
            color : color,
            members : HashSet::new(),
            alive : false
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

    fn group_at(&self, pos: Bpos) -> Group {
        let color = self.intersection(pos);
        let mut group = Group::new(color);
        let mut stack = Vec::<Bpos>::new();

        stack.push(pos);
        group.members.insert(pos);

        while let Some(p) = stack.pop() {
            let (x, y) = (p.0 as i32,  p.1 as i32);
            let candidates : [(i32, i32); 4] =
                [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

            let adj_new : Vec<Bpos> = candidates.iter()
                .filter(|pos| {
                    pos.0 < self.size as i32 && pos.0 >= 0 &&
                    pos.1 < self.size as i32 && pos.1 >= 0
                })
                .map(|pos| { (pos.0 as u32, pos.1 as u32) })
                .filter(|pos| {
                    !group.members.contains(pos)
                }).collect();

            for x in adj_new {
                if self.intersection(x) == Intersection::Empty {
                    group.alive = true;
                } else if self.intersection(x) == color {
                    stack.push(x);
                    group.members.insert(x);
                }
            }
        }

        return group;
    }

    fn remove_group(&mut self, g : &Group) {
        for pos in &g.members {
            let place : usize = (pos.0 * self.size + pos.1) as usize;
            self.board[place] = Intersection::Empty;
        }
    }

	pub fn make_move(&self, pos : Bpos) -> Result<GameState, MoveError> {

        if pos.0 >= self.size || pos.1 >= self.size {
            return Err(MoveError::NotOnBoard);
        }

        if self.intersection(pos) != Intersection::Empty {
            return Err(MoveError::OnTopOfOther);
        }

        let mut g = self.clone();
        let place : usize = (pos.0 * self.size + pos.1) as usize;
		g.board[place] = self.turn.to_intersection();
        g.turn = self.turn.other();

        //get neighbor groups

        let (x, y) = (pos.0 as i32,  pos.1 as i32);
        let candidates : [(i32, i32); 4] =
            [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

        let dead_groups : Vec<Group> = candidates.iter()
            .filter(|pos| {
                pos.0 < self.size as i32 && pos.0 >= 0 &&
                pos.1 < self.size as i32 && pos.1 >= 0
            })
            .map(|a| {
                //TODO This breaks if we change Bpos
                (a.0 as u32, a.1 as u32)
            })
            .filter(|a| {
                g.intersection(*a) == g.turn.to_intersection()
            })
            .map(|a| {g.group_at(a)})
            .filter(|g| {!g.alive})
            .collect();

        //remove dead groups

        for dead in dead_groups {
            g.remove_group(&dead);
            println!("{:?}", dead);
        }

        let new_group = g.group_at(pos);
        if !new_group.alive {
            return Err(MoveError::Suicide);
        }


        return Ok(g);
	}

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn intersection(&self, pos : Bpos) -> Intersection {
        let place : usize = (pos.0 * self.size + pos.1) as usize;
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

    pub fn make_move(&mut self, pos : Bpos) -> Result<(), MoveError> {
        //states should always be non empty, so unwrap should be okay.
        let new_state = try!(self.states.last().unwrap().make_move(pos));
        self.states.push(new_state);
        self.current_state = self.states.len() - 1;
        Ok(())
    }

    pub fn current_state(&self) -> &GameState {
        let i = self.current_state;
        &self.states[i]
    }

    pub fn forwards(&mut self, delta : usize) {
        let proposed_state = self.current_state + delta;
        if proposed_state < self.states.len() {
            self.current_state = proposed_state;
        }
    }

    pub fn backwards(&mut self, delta : usize) {
        self.current_state = self.current_state.saturating_sub(delta);
    }
}
