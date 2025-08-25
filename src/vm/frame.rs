use std::collections::BTreeMap;

use crate::{
	class::constant_pool::ConstantPoolItem, vm::{local::Locals,
		 operand_stack::OperandStack}};

pub struct StackFrame {
	/// Operand stack for this frame
	pub operand_stack: OperandStack,
	/// Local variables for the currently running method
	pub locals: Locals,
	/// Reference to class constant pool
	pub constant_pool: BTreeMap<u32, ConstantPoolItem>,
	/// Java bytecode
	pub code: u32,
}