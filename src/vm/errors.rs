use thiserror::Error;

use crate::vm::types::Type;

#[derive(Error, Debug)]
pub enum ExecutionError {
	#[error("illegal opcode: {0}")]
	#[from(TryFromPrimitiveError)]
	DecodeError(u8),
	#[error("stack overflow")]
	StackOverflow,
	#[error("stack underflow")]
	StackUnderflow,
	#[error("premature end of code at {0}")]
	EndOfCode(u32),
	#[error("jump out of bounds to {0}, code max byte address is {1}")]
	JumpOutOfBounds(u32, usize),
	#[error("Attempt to return {0} from method type {1}")]
	BadReturnType(Type, Type),
}