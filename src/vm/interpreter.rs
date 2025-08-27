use std::error::Error;

use crate::{
	isa::opcode::Opcode, 
	make_conditional_branches,
	make_float_arithmetic,
	make_float_comparisons,
	make_integer_arithmetic_logic,
	make_pop_load,
	make_push, vm::{
		frame::StackFrame,
		types::*}};

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

	make_pop_load!(i, i32, Int, get_int, 4);
	make_push!(i, i32);
	make_pop_load!(l, i64, Long, get_long, 8);
	make_push!(l, i64);
	make_pop_load!(f, f32, Float, get_float, 4);
	make_push!(f, f32);
	make_pop_load!(d, f64, Double, get_double, 8);
	make_push!(d, f64);
	make_integer_arithmetic_logic!(i, i32);
	make_integer_arithmetic_logic!(l, i64);
	make_float_arithmetic!(f, f32);
	make_float_arithmetic!(d, f64);

	make_float_comparisons!(f, f32);
	make_float_comparisons!(d, f64);

	make_conditional_branches!(eq, ==);
	make_conditional_branches!(ne, !=);
	make_conditional_branches!(lt, <);
	make_conditional_branches!(ge, >=);
	make_conditional_branches!(gt, <);
	make_conditional_branches!(le, <=);

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
				Opcode::ILoad0 => { self.iload(0)?; }
				Opcode::ILoad1 => { self.iload(1)?; }
				Opcode::ILoad2 => {	self.iload(2)?; }
				Opcode::ILoad3 => {	self.iload(3)?; }
				Opcode::LLoad => {
					let index: u16 = u16::from_be_bytes([self.frame.operand_stack.pop(), self.frame.operand_stack.pop()]);
					self.lload(index)?;
				},
				Opcode::LLoad0 => {	self.lload(0)?; },
				Opcode::LLoad1 => {	self.lload(1)?; },
				Opcode::LLoad2 => {	self.lload(2)?; },
				Opcode::LLoad3 => { self.lload(3)?; },
				Opcode::FLoad => {
					let index: u16 = u16::from_be_bytes([self.frame.operand_stack.pop(), self.frame.operand_stack.pop()]);
					self.fload(index)?;
				},
				Opcode::FLoad0 => {	self.dload(0)?; },
				Opcode::FLoad1 => {	self.dload(1)?; },
				Opcode::FLoad2 => {	self.dload(2)?; },
				Opcode::FLoad3 => {	self.dload(3)?; },
				Opcode::DLoad => {
					let index: u16 = u16::from_be_bytes([self.frame.operand_stack.pop(), self.frame.operand_stack.pop()]);
					self.dload(index)?;
				},
				Opcode::DLoad0 => { self.dload(0)?; },
				Opcode::DLoad1 => {	self.dload(1)?; },
				Opcode::DLoad2 => { self.dload(2)?; },
				Opcode::DLoad3 => { self.dload(3)?; },
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
				Opcode::Pop => {
					self.ipop();
				},
				Opcode::Pop2 => {
					self.ipop();
					self.ipop();
				},
				Opcode::Dup => {
					let val = self.ipop();
					self.ipush(val);
					self.ipush(val);
				}
				Opcode::DupX1 => todo!(),
				Opcode::DupX2 => todo!(),
				Opcode::Dup2 => todo!(),
				Opcode::Dup2X1 => todo!(),
				Opcode::Dup2X2 => todo!(),
				Opcode::Swap =>{
					let value_2 = self.ipop();
					let value_1 = self.ipop();
					self.ipush(value_2);
					self.ipush(value_1);
				},
				Opcode::IAdd => { self.iadd(); }
				Opcode::LAdd => { self.ladd(); }
				Opcode::FAdd => { self.fadd(); }
				Opcode::DAdd => { self.dadd(); }
				Opcode::ISub => { self.isub(); }
				Opcode::LSub => { self.lsub(); }
				Opcode::FSub => { self.fsub(); }
				Opcode::DSub => { self.dsub(); }
				Opcode::IMul => { self.imul(); }
				Opcode::LMul => { self.lmul(); }
				Opcode::FMul => { self.fmul(); }
				Opcode::DMul => { self.dmul(); }
				Opcode::IDiv => { self.idiv(); }
				Opcode::LDiv => { self.ldiv(); }
				Opcode::FDiv => { self.fdiv(); }
				Opcode::DDiv => { self.ddiv(); }
				Opcode::IRem => todo!(),
				Opcode::LRem => todo!(),
				Opcode::FRem => todo!(),
				Opcode::DRem => todo!(),
				Opcode::INeg => { self.ineg(); }
				Opcode::LNeg => { self.lneg(); }
				Opcode::FNeg => { self.fneg(); }
				Opcode::DNeg => { self.dneg(); }
				Opcode::IShl => { self.ishl(); }
				Opcode::LShl => { self.lshl(); }
				Opcode::IShr => { self.ishr(); }
				Opcode::LShr => { self.lshr(); }
				Opcode::IUShr => {
					let value_1 = self.ipop();
					let value_2 = self.ipop();
					let s = value_2 & 0x1F;
					if value_1 > 0 {
						self.ipush(value_1 >> s);
					}
					else {
						self.ipush((value_1 >> s) + (2 << !s));
					}
				},
				Opcode::LUShr => {
					let value_1 = self.lpop();
					let value_2 = self.lpop();
					let s = value_2 & 0x3F;
					if value_1 > 0 {
						self.lpush(value_1 >> s);
					}
					else {
						self.lpush((value_1 >> s) + (2 << !s));
					}
				},
				Opcode::IAnd => { self.iand(); }
				Opcode::LAnd => { self.land(); }
				Opcode::IOr => { self.ior(); }
				Opcode::LOr => { self.lor(); }
				Opcode::IXor => { self.ixor(); }
				Opcode::LXor => { self.lxor(); }
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
				Opcode::LCmp => {
					let value_1 = self.lpop();
					let value_2 = self.lpop();
					if value_1 > value_2 { self.ipush(1); }
					else if value_1 < value_2 {	self.ipush(-1);	}
					else { self.ipush(0); }
				},
				Opcode::FCmpL => { self.fcmpl(); }
				Opcode::FCmpG => { self.fcmpg(); }
				Opcode::DCmpL => { self.dcmpl(); }
				Opcode::DCmpG => { self.dcmpl(); }
				Opcode::IfEq => { self.if_eq(); }
				Opcode::IfNe => { self.if_ne(); }
				Opcode::IfLt => { self.if_lt(); }
				Opcode::IfGe => { self.if_ge(); }
				Opcode::IfGt => { self.if_gt(); }
				Opcode::IfLe => { self.if_le(); }
				Opcode::IfICmpEq => { self.if_icmpeq(); }
				Opcode::IfICmpNe => { self.if_icmpne(); }
				Opcode::IfICmpLt => { self.if_icmplt(); }
				Opcode::IfICmpGe => { self.if_icmpge(); }
				Opcode::IfICmpGt => { self.if_icmpgt(); }
				Opcode::IfICmpLe => { self.if_icmple(); }
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
				Opcode::Impdep1 => {
					// do nothing
				},
				Opcode::Impdep2 => {
					// do nothing
				},
			}
		}
	}
}

mod tests {
	use std::{collections::{BTreeMap, HashMap}};

	use crate::{
		isa::opcode::Opcode,
		vm::interpreter::Interpreter};


	macro_rules! integer_test_cases {
		($rust_type:ty,
		 $prefix:ident) => {
		fn ${ concat(run_, $prefix, _, test_cases)}(cases: Vec<($rust_type, $rust_type, Opcode, $rust_type)>) {
			for case in cases {
				let mut interpreter = Interpreter::new();
				let value1 = case.0;
				let value2 = case.1;
				let opcode = case.2;
				let expected = case.3;
				if opcode != Opcode::INeg {
					interpreter.${ concat($prefix, push) }(value2);
				}
				interpreter.${ concat($prefix, push) }(value1);
				interpreter.frame.code.push(opcode as u8);
				interpreter.execute();
				assert_eq!(interpreter.${ concat($prefix, pop) }(), expected);
				}
			}
		};
	}

	integer_test_cases!(i32, i);
	integer_test_cases!(i64, l);

	#[test]
	fn test_int_operations() {
		let i_cases: Vec<(i32, i32, Opcode, i32)> = vec![
			(1, 1, Opcode::IAdd, 2),
			(100, -1, Opcode::IAdd, 99),
			(1, 1, Opcode::ISub, 0),
			(0, 1, Opcode::ISub, -1),
			(i32::MIN, 1, Opcode::ISub, i32::MAX),
			(2, 2, Opcode::IMul, 4),
			(4, 2, Opcode::IDiv, 2),
			(1, 1, Opcode::IAnd, 1),
			(1, 1, Opcode::IOr, 1),
			(0, 0, Opcode::IOr, 0),
			(1, 0, Opcode::IXor, 1),
			(1, 1, Opcode::IShl, 2),
			(300, 0, Opcode::INeg, -300),
		];
		run_i_test_cases(i_cases);
	}

	#[test]
	fn test_long_operations() {
		let l_cases: Vec<(i64, i64, Opcode, i64)> = vec![
			(1, 1, Opcode::LAdd, 2),
			(100, -1, Opcode::LAdd, 99),
			(1, 1, Opcode::LSub, 0),
			(0, 1, Opcode::LSub, -1),
			(i64::MIN, 1, Opcode::LSub, i64::MAX),
			(2, 2, Opcode::LMul, 4),
			(4, 2, Opcode::LDiv, 2),
			(1, 1, Opcode::LAnd, 1),
			(1, 1, Opcode::LOr, 1),
			(0, 0, Opcode::LOr, 0),
			(1, 0, Opcode::LXor, 1),
			(1, 1, Opcode::LShl, 2),
			(300, 0, Opcode::LNeg, -300),
		];
		run_l_test_cases(l_cases);
	}
}