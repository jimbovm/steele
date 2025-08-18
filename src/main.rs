extern crate regex;
extern crate strum;

use strum::IntoEnumIterator;

use steele::isa::opcode::Opcode;

fn main() {
	let it = Opcode::iter();
	for op in it {
		println!("{}", op);
	}
}
