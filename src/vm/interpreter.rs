use std::error::Error;

use crate::{
	isa::opcode::Opcode, vm::{
		frame::StackFrame, interpreter, types::*}};

#[derive(Debug, Default)]
pub struct Interpreter {
	pub frame: StackFrame,
}

impl Interpreter {
	pub fn new() -> Interpreter {
		Interpreter { frame: StackFrame::new() }
	}

	pub fn fetch(&mut self) -> u8 {
		let byte = self.frame.code[self.frame.pc as usize];
		self.frame.pc += 1;
		byte
	}

	fn decode(&mut self, byte: u8) -> Opcode {
		Opcode::try_from(byte).expect("Invalid opcode")
	}

	pub fn ipop(&mut self) -> i32 {
		i32::from_be_bytes([
			self.frame.operand_stack.pop(),
			self.frame.operand_stack.pop(),
			self.frame.operand_stack.pop(),
			self.frame.operand_stack.pop()
		])
	}

	pub fn ipush(&mut self, value: i32) {
		self.frame.operand_stack.push(&i32::to_le_bytes(value));
	}

	fn iload(&mut self, index: u16) -> Result<Int, Box<dyn Error>> {
		let local = self.frame.locals.get_int(index)?;
		self.frame.operand_stack.push(&local.value.to_be_bytes());
		Ok(local)
	}

	fn lload(&mut self, index: u16) -> Result<Long, Box<dyn Error>> {
		let local = self.frame.locals.get_long(index)?;
		self.frame.operand_stack.push(&local.value.to_be_bytes());
		Ok(local)
	}

	fn fload(&mut self, index: u16) -> Result<Float, Box<dyn Error>> {
		let local = self.frame.locals.get_float(index)?;
		self.frame.operand_stack.push(&local.value.to_be_bytes());
		Ok(local)
	}

	fn dload(&mut self, index: u16) -> Result<Double, Box<dyn Error>> {
		let local = self.frame.locals.get_double(index)?;
		self.frame.operand_stack.push(&local.value.to_be_bytes());
		Ok(local)
	}

	pub fn execute(&mut self) -> Result<usize, Box<dyn Error>> {
		loop {
			if self.frame.pc == self.frame.code.len() as u32 {
				break Ok(0);
			}
			let byte = self.fetch();
			let opcode = self.decode(byte);
			match opcode {
				Opcode::Nop => {
					// do nothing
				},
				Opcode::AConstNull => todo!(),
				Opcode::IConstM1 => todo!(),
				Opcode::IConst0 => todo!(),
				Opcode::IConst1 => todo!(),
				Opcode::IConst2 => todo!(),
				Opcode::IConst3 => todo!(),
				Opcode::IConst4 => todo!(),
				Opcode::IConst5 => todo!(),
				Opcode::LConst0 => todo!(),
				Opcode::LConst1 => todo!(),
				Opcode::FConst0 => todo!(),
				Opcode::FConst1 => todo!(),
				Opcode::FConst3 => todo!(),
				Opcode::DConst0 => todo!(),
				Opcode::DConst1 => todo!(),
				Opcode::BIpush => todo!(),
				Opcode::SIpush => todo!(),
				Opcode::Ldc => todo!(),
				Opcode::LdcW => todo!(),
				Opcode::Ldc2W => todo!(),
				Opcode::ALoad => todo!(),
				Opcode::ILoad => {
					let index: u16 = u16::from_be_bytes([self.frame.operand_stack.pop(), self.frame.operand_stack.pop()]);
					self.iload(index)?;
				}
				Opcode::ILoad0 => {
					self.iload(0)?;
				}
				Opcode::ILoad1 => {
					self.iload(1)?;
				}
				Opcode::ILoad2 => {
					self.iload(2)?;
				}
				Opcode::ILoad3 => {
					self.iload(3)?;
				}
				Opcode::LLoad => {
					let index: u16 = u16::from_be_bytes([self.frame.operand_stack.pop(), self.frame.operand_stack.pop()]);
					self.lload(index)?;
				},
				Opcode::LLoad0 => {
					self.lload(0)?;
				},
				Opcode::LLoad1 => {
					self.lload(1)?;
				},
				Opcode::LLoad2 => {
					self.lload(2)?;
				},
				Opcode::LLoad3 => {
					self.lload(3)?;
				},
				Opcode::FLoad => {
					let index: u16 = u16::from_be_bytes([self.frame.operand_stack.pop(), self.frame.operand_stack.pop()]);
					self.fload(index)?;
				},
				Opcode::FLoad0 => {
					self.dload(0)?;
				},
				Opcode::FLoad1 => {
					self.dload(1)?;
				},
				Opcode::FLoad2 => {
					self.dload(2)?;
				},
				Opcode::FLoad3 => {
					self.dload(3)?;
				},
				Opcode::DLoad => {
					let index: u16 = u16::from_be_bytes([self.frame.operand_stack.pop(), self.frame.operand_stack.pop()]);
					self.dload(index)?;
				},
				Opcode::DLoad0 => {
					self.dload(0)?;
				},
				Opcode::DLoad1 => {
					self.dload(1)?;
				},
				Opcode::DLoad2 => {
					self.dload(2)?;
				},
				Opcode::DLoad3 => {
					self.dload(3)?;
				},
				Opcode::ALoad0 => todo!(),
				Opcode::ALoad1 => todo!(),
				Opcode::ALoad2 => todo!(),
				Opcode::ALoad3 => todo!(),
				Opcode::IALoad => todo!(),
				Opcode::LALoad => todo!(),
				Opcode::FALoad => todo!(),
				Opcode::DALoad => todo!(),
				Opcode::AALoad => todo!(),
				Opcode::BALoad => todo!(),
				Opcode::CALoad => todo!(),
				Opcode::SALoad => todo!(),
				Opcode::IStore => todo!(),
				Opcode::LStore => todo!(),
				Opcode::FStore => todo!(),
				Opcode::DStore => todo!(),
				Opcode::AStore => todo!(),
				Opcode::IStore0 => todo!(),
				Opcode::IStore1 => todo!(),
				Opcode::IStore2 => todo!(),
				Opcode::IStore3 => todo!(),
				Opcode::LStore0 => todo!(),
				Opcode::LStore1 => todo!(),
				Opcode::LStore2 => todo!(),
				Opcode::LStore3 => todo!(),
				Opcode::FStore0 => todo!(),
				Opcode::FStore1 => todo!(),
				Opcode::FStore2 => todo!(),
				Opcode::FStore3 => todo!(),
				Opcode::DStore0 => todo!(),
				Opcode::DStore1 => todo!(),
				Opcode::DStore2 => todo!(),
				Opcode::DStore3 => todo!(),
				Opcode::AStore0 => todo!(),
				Opcode::AStore1 => todo!(),
				Opcode::AStore2 => todo!(),
				Opcode::AStore3 => todo!(),
				Opcode::IAStore => todo!(),
				Opcode::LAStore => todo!(),
				Opcode::FAStore => todo!(),
				Opcode::DAStore => todo!(),
				Opcode::AAStore => todo!(),
				Opcode::BAStore => todo!(),
				Opcode::CAStore => todo!(),
				Opcode::SAStore => todo!(),
				Opcode::Pop => todo!(),
				Opcode::Pop2 => todo!(),
				Opcode::Dup => todo!(),
				Opcode::DupX1 => todo!(),
				Opcode::DupX2 => todo!(),
				Opcode::Dup2 => todo!(),
				Opcode::Dup2X1 => todo!(),
				Opcode::Dup2X2 => todo!(),
				Opcode::Swap => todo!(),
				Opcode::IAdd => {
					let lhs = self.ipop();
					let rhs = self.ipop();
					self.ipush(lhs.wrapping_add(rhs));
				}
				Opcode::LAdd => todo!(),
				Opcode::FAdd => todo!(),
				Opcode::DAdd => todo!(),
				Opcode::ISub => {
					let lhs = self.ipop();
					let rhs = self.ipop();
					self.ipush(lhs.wrapping_sub(rhs));
				},
				Opcode::LSub => todo!(),
				Opcode::FSub => todo!(),
				Opcode::DSub => todo!(),
				Opcode::IMul => {
					let lhs = self.ipop();
					let rhs = self.ipop();
					self.ipush(lhs.wrapping_mul(rhs));
				},
				Opcode::LMul => todo!(),
				Opcode::FMul => todo!(),
				Opcode::DMul => todo!(),
				Opcode::IDiv => {
					let lhs = self.ipop();
					let rhs = self.ipop();
					self.ipush(lhs.wrapping_div(rhs));
				},
				Opcode::LDiv => todo!(),
				Opcode::FDiv => todo!(),
				Opcode::DDiv => todo!(),
				Opcode::IRem => todo!(),
				Opcode::LRem => todo!(),
				Opcode::FRem => todo!(),
				Opcode::DRem => todo!(),
				Opcode::INeg => {
					let val = self.ipop();
					self.ipush(-val);
				},
				Opcode::LNeg => todo!(),
				Opcode::FNeg => todo!(),
				Opcode::DNeg => todo!(),
				Opcode::IShl => todo!(),
				Opcode::LShl => {
					let lhs = self.ipop();
					let rhs = self.ipop();
					self.ipush(lhs << (rhs & 0x1F));
				}
				Opcode::IShr => {
					let lhs = self.ipop();
					let rhs = self.ipop();
					self.ipush(lhs >> (rhs & 0x1F));
				},
				Opcode::LShr => todo!(),
				Opcode::IUShr => todo!(),
				Opcode::LUShr => todo!(),
				Opcode::IAnd => {
					let lhs = self.ipop();
					let rhs = self.ipop();
					self.ipush(lhs & rhs);
				},
				Opcode::LAnd => todo!(),
				Opcode::IOr => {
					let lhs = self.ipop();
					let rhs = self.ipop();
					self.ipush(lhs | rhs);
				}
				Opcode::LOr => todo!(),
				Opcode::IXor => {
					let lhs = self.ipop();
					let rhs = self.ipop();
					self.ipush(lhs ^ rhs);
				}
				Opcode::LXor => todo!(),
				Opcode::IInc => todo!(),
				Opcode::I2L => todo!(),
				Opcode::I2F => todo!(),
				Opcode::I2D => todo!(),
				Opcode::L2I => todo!(),
				Opcode::L2F => todo!(),
				Opcode::L2D => todo!(),
				Opcode::F2I => todo!(),
				Opcode::F2L => todo!(),
				Opcode::F2D => todo!(),
				Opcode::D2I => todo!(),
				Opcode::D2L => todo!(),
				Opcode::D2F => todo!(),
				Opcode::I2B => {todo!()},
				Opcode::I2C => todo!(),
				Opcode::I2S => todo!(),
				Opcode::LCmp => todo!(),
				Opcode::FCmpL => todo!(),
				Opcode::FCmpG => todo!(),
				Opcode::DCmpL => todo!(),
				Opcode::DCmpG => todo!(),
				Opcode::IfEq => todo!(),
				Opcode::IfNe => todo!(),
				Opcode::IfLt => todo!(),
				Opcode::IfGe => todo!(),
				Opcode::IfGt => todo!(),
				Opcode::IfLe => todo!(),
				Opcode::IfICmpEq => todo!(),
				Opcode::IfICmpNe => todo!(),
				Opcode::IfICmpLt => todo!(),
				Opcode::IfICmpGe => todo!(),
				Opcode::IfICmpGt => todo!(),
				Opcode::IfICmpLe => todo!(),
				Opcode::IfACmpEq => todo!(),
				Opcode::IfACmpNe => todo!(),
				Opcode::Goto => todo!(),
				Opcode::Jsr => todo!(),
				Opcode::Ret => todo!(),
				Opcode::TableSwitch => todo!(),
				Opcode::LookupSwitch => todo!(),
				Opcode::IReturn => todo!(),
				Opcode::LReturn => todo!(),
				Opcode::FReturn => todo!(),
				Opcode::DReturn => todo!(),
				Opcode::AReturn => todo!(),
				Opcode::Return => todo!(),
				Opcode::GetStatic => todo!(),
				Opcode::PutStatic => todo!(),
				Opcode::GetField => todo!(),
				Opcode::PutField => todo!(),
				Opcode::InvokeVirtual => todo!(),
				Opcode::InvokeSpecial => todo!(),
				Opcode::InvokeStatic => todo!(),
				Opcode::InvokeInterface => todo!(),
				Opcode::InvokeDynamic => todo!(),
				Opcode::New => todo!(),
				Opcode::NewArray => todo!(),
				Opcode::ANewArray => todo!(),
				Opcode::ArrayLength => todo!(),
				Opcode::AThrow => todo!(),
				Opcode::CheckCast => todo!(),
				Opcode::InstanceOf => todo!(),
				Opcode::MonitorEnter => todo!(),
				Opcode::MonitorExit => todo!(),
				Opcode::Wide => todo!(),
				Opcode::MultiANewArray => todo!(),
				Opcode::IfNull => todo!(),
				Opcode::IfNonNull => todo!(),
				Opcode::GotoW => todo!(),
				Opcode::JsrW => todo!(),
				Opcode::Breakpoint => todo!(),
				Opcode::Impdep1 => todo!(),
				Opcode::Impdep2 => todo!(),
			}
		}
	}
}

mod tests {
    use std::{collections::{BTreeMap, HashMap}};

    use crate::{
		isa::opcode::Opcode,
		vm::interpreter::Interpreter};


	#[test]
	fn test_int_operations() {
		let cases: [(i32, i32, Opcode, i32); 6] = [
			(1, 1, Opcode::IAdd, 2),
			(1, 1, Opcode::ISub, 0),
			(0, 1, Opcode::ISub, -1),
			(2, 2, Opcode::IMul, 4),
			(4, 2, Opcode::IDiv, 2),
			(300, 0, Opcode::INeg, -300),
		];

		for case in cases {
			let mut interpreter = Interpreter::new();
			let value1 = case.0;
			let value2 = case.1;
			let opcode = case.2;
			let expected = case.3;
			if opcode != Opcode::INeg {
				interpreter.ipush(value2);
			}
			interpreter.ipush(value1);
			interpreter.frame.code.push(opcode as u8);
			interpreter.execute();
			assert_eq!(interpreter.ipop(), expected);
		}
	}
}