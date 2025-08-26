#[derive(Debug, Default)]
pub struct OperandStack {
	pub max_depth: usize,
	pub stack: Vec<u8>,
} 

impl OperandStack {

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

	fn pop_wide_operand(&mut self) -> i64 {
		if self.stack.len() == 0 {	panic!("Stack underflow"); }
		let high = self.stack.pop().unwrap();
		if self.stack.len() == 0 {	panic!("Stack underflow"); }
		let low = self.stack.pop().unwrap();
		let operand: i64 = ((high as i64) << 32) & (low as i64);
		operand
	}
}