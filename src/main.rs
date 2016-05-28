extern crate goagain;

use goagain::goban;

fn main() {
    let mut game = goban::Game::new();
    game.make_move(4, 4);
    game.make_move(3, 3);
	println!("{}", game.states[0].to_string());
	println!("{}", game.states[1].to_string());
	println!("{}", game.states[2].to_string());
}
