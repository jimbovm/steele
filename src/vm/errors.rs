use thiserror::Error;

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
}