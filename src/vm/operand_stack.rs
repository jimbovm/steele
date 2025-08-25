use crate::isa::opcode::Opcode;

pub struct OperandStack {
	pub max_depth: usize,
	pub stack: Vec<u8>,
} 

impl OperandStack {

	pub fn push(&mut self, byte: &dyn Stackable) {
		if self.stack.len() == self.max_depth as usize { panic!("Stack overflow"); }
		let b = byte.as_u8();
		match b { 
			Some(b) => { self.stack.push(b); }, // is a plain old byte
			None => { self.stack.push(b.unwrap()); }, // is an opcode
		};
	}

	pub fn pop_opcode(&mut self) -> Opcode {
		if self.stack.len() == 0 {	panic!("Stack underflow"); }
		let popped = self.stack.pop().unwrap();
		Opcode::try_from(popped).expect("Invalid opcode")
	}

	pub fn pop_operand(&mut self) -> u8 {
		if self.stack.len() == 0 {	panic!("Stack underflow"); }
		self.stack.pop().unwrap()
	}

	pub fn pop_wide_operand(&mut self) -> u16 {
		if self.stack.len() == 0 {	panic!("Stack underflow"); }
		let high = self.stack.pop().unwrap();
		if self.stack.len() == 0 {	panic!("Stack underflow"); }
		let low = self.stack.pop().unwrap();
		u16::from_be_bytes([high, low])
	}
}

trait Stackable {
	fn as_u8(&self) -> Option<u8>;
	fn as_opcode(&self) -> Option<Opcode>;
}

impl Stackable for Opcode {
	fn as_u8(&self) -> Option<u8> {
		None
	}
	fn as_opcode(&self) -> Option<Opcode> {
		Some(self.clone())
	}
}

impl Stackable for u8 {
	fn as_u8(&self) -> Option<u8> {
		Some(*self)
	}
	fn as_opcode(&self) -> Option<Opcode> {
		None
	}
}