mod pi;

use pi::gpio::{Pin, Direction, State};

fn main() {
	let mut x = Pin::new(4);
	x.set_mode(Direction::Out);
	x.write(State::High);
}