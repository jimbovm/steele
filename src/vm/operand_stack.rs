use std::usize;

#[derive(Debug, Default)]
pub struct OperandStack {
	pub max_depth: usize,
	pub stack: Vec<u8>,
} 

impl OperandStack {

	pub fn new() -> Self {
		OperandStack { max_depth: usize::MAX, stack: Vec::new() }
	}

	pub fn push(&mut self, bytes: &[u8]) {
		if self.stack.len() == self.max_depth as usize { panic!("Stack overflow"); }
		for byte in bytes {
			self.stack.push(*byte);
		}
	}

	pub fn pop(&mut self) -> u8 {
		if self.stack.len() == 0 { panic!("Stack underflow"); }
		self.stack.pop().unwrap()
	}
}