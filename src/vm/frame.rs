use std::{collections::{BTreeMap, HashMap}, ptr::null};

use crate::{
	class::constant_pool::ConstantPoolItem, vm::{local::Locals,
		 operand_stack::OperandStack}};

#[derive(Debug, Default)]
pub struct StackFrame {
	/// The frame belonging to the method that invoked the frame to which this one belongs
	pub invoker: Option<Box<StackFrame>>,
	/// Program counter, pointer to position in bytecode
	pub pc: u32,
	/// Operand stack for this frame
	pub operand_stack: OperandStack,
	/// Local variables for the currently running method
	pub locals: Locals,
	/// Reference to class constant pool
	pub constant_pool: BTreeMap<u32, ConstantPoolItem>,
	/// Java bytecode
	pub code: Vec<u8>,
}

impl StackFrame {

	pub fn new() -> StackFrame {
		let frame = StackFrame {
			invoker: Option::None,
			pc: 0,
			operand_stack: OperandStack { max_depth: usize::MAX, stack: Vec::new() },
			locals: Locals { variables: HashMap::new() },
			constant_pool: BTreeMap::new(), code: Vec::new() };
		frame
	}
}