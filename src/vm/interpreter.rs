use std::error::Error;

use crate::make_return;
use crate::vm::{
	errors::ExecutionError,
	types::Int,
};

use crate::{
	isa::opcode::Opcode,
	make_conditional_branches,
	make_float_arithmetic,
	make_float_comparisons,
	make_integer_arithmetic_logic,
	make_pop_load_store,
	make_push,
	vm::{
		frame::StackFrame,
		types::*}};

#[derive(Debug, Default)]
pub struct Interpreter {
	frame: StackFrame,
}

impl Interpreter {
	pub fn new(frame: StackFrame) -> Interpreter {
		Interpreter {
			frame,
		}
	}

	pub fn fetch(&mut self) -> Result<u8, Box<dyn Error>> {
		if self.frame.pc < self.frame.code.len() as u32 {
			let byte = self.frame.code[self.frame.pc as usize];
			self.frame.pc += 1;
			return Ok(byte);
		}
		return Err(Box::new(ExecutionError::EndOfCode(self.frame.pc)));
	}

	fn decode(&mut self, byte: u8) -> Opcode {
		Opcode::try_from(byte).expect("Invalid opcode")
	}

	make_pop_load_store!(i, i32, Int, Int, Int, get_int, 4);
	make_push!(i, i32);
	make_pop_load_store!(l, i64, Long, Long, Long, get_long, 8);
	make_push!(l, i64);
	make_pop_load_store!(f, f32, Float, Float, Float, get_float, 4);
	make_push!(f, f32);
	make_pop_load_store!(d, f64, Double, Double, Double, get_double, 8);
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
	make_conditional_branches!(gt, >);
	make_conditional_branches!(le, <=);
	make_return!(i, i32, Int, I);
	make_return!(l, i64, Long, J);
	make_return!(f, f32, Float, F);
	make_return!(d, f64, Double, D);

	pub fn execute(&mut self) -> Result<Variable, Box<dyn Error>> {
		loop {
			let byte = self.fetch()?;
			let opcode = self.decode(byte);
			match opcode {
				Opcode::Nop => {
					// do nothing
				},
				Opcode::AConstNull => todo!(),
				Opcode::IConstM1 => todo!(),
				Opcode::IConst0 => {
					let item = self.frame.constant_pool.get_int(0)?;
					self.ipush(item.value);
				}
				Opcode::IConst1 => {
					let item = self.frame.constant_pool.get_int(1)?;
					self.ipush(item.value);
				}
				Opcode::IConst2 => {
					let item = self.frame.constant_pool.get_int(2)?;
					self.ipush(item.value);
				}
				Opcode::IConst3 => {
					let item = self.frame.constant_pool.get_int(3)?;
					self.ipush(item.value);
				}
				Opcode::IConst4 => {
					let item = self.frame.constant_pool.get_int(4)?;
					self.ipush(item.value);
				}
				Opcode::IConst5 => {
					let item = self.frame.constant_pool.get_int(5)?;
					self.ipush(item.value);
				}
				Opcode::LConst0 => {
					let item = self.frame.constant_pool.get_long(0)?;
					self.lpush(item.value);
				}
				Opcode::LConst1 => {
					let item = self.frame.constant_pool.get_long(1)?;
					self.lpush(item.value);
				}
				Opcode::FConst0 => {
					let item = self.frame.constant_pool.get_float(0)?;
					self.fpush(item.value);
				}
				Opcode::FConst1 => {
					let item = self.frame.constant_pool.get_float(1)?;
					self.fpush(item.value);
				}
				Opcode::FConst2 => {
					let item = self.frame.constant_pool.get_float(2)?;
					self.fpush(item.value);
				}
				Opcode::DConst0 => {
					let item = self.frame.constant_pool.get_double(0)?;
					self.dpush(item.value);
				}
				Opcode::DConst1 => {
					let item = self.frame.constant_pool.get_double(1)?;
					self.dpush(item.value);
				}
				Opcode::BIpush => {
					let byte = i32::from(self.frame.code[self.frame.pc as usize]);
					self.frame.pc += 1;
					self.ipush(byte);
				}
				Opcode::SIpush => {
					let short_high = self.frame.code[self.frame.pc as usize];
					let short_low = self.frame.code[self.frame.pc as usize];
					let short = i32::from(i16::from_be_bytes([short_high, short_low]));
					self.ipush(short);
				}
				Opcode::Ldc => todo!(),
				Opcode::LdcW => todo!(),
				Opcode::Ldc2W => todo!(),
				Opcode::ALoad => todo!(),
				Opcode::ILoad => {
					let index: u32 = u32::from_be_bytes([0, 0, self.frame.operand_stack.pop(), self.frame.operand_stack.pop()]);
					self.iload(index)?;
				}
				Opcode::ILoad0 => { self.iload(0)?; }
				Opcode::ILoad1 => { self.iload(1)?; }
				Opcode::ILoad2 => {	self.iload(2)?; }
				Opcode::ILoad3 => {	self.iload(3)?; }
				Opcode::LLoad => {
					let index: u32 = u32::from_be_bytes([0, 0, self.frame.operand_stack.pop(), self.frame.operand_stack.pop()]);
					self.lload(index)?;
				},
				Opcode::LLoad0 => {	self.lload(0)?; },
				Opcode::LLoad1 => {	self.lload(1)?; },
				Opcode::LLoad2 => {	self.lload(2)?; },
				Opcode::LLoad3 => { self.lload(3)?; },
				Opcode::FLoad => {
					let index: u32 = u32::from_be_bytes([0, 0, self.frame.operand_stack.pop(), self.frame.operand_stack.pop()]);
					self.fload(index)?;
				},
				Opcode::FLoad0 => {	self.dload(0)?; },
				Opcode::FLoad1 => {	self.dload(1)?; },
				Opcode::FLoad2 => {	self.dload(2)?; },
				Opcode::FLoad3 => {	self.dload(3)?; },
				Opcode::DLoad => {
					let index: u32 = u32::from_be_bytes([0, 0, self.frame.operand_stack.pop(), self.frame.operand_stack.pop()]);
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
				Opcode::IStore0 => {
					let val = self.ipop();
					self.istore(0, val);
				}
				Opcode::IStore1 => {
					let val = self.ipop();
					self.istore(1, val);
				}
				Opcode::IStore2 => {
					let val = self.ipop();
					self.istore(2, val);
				}
				Opcode::IStore3 => {
					let val = self.ipop();
					self.istore(3, val);
				}
				Opcode::LStore0 => {
					let val = self.lpop();
					self.lstore(0, val);
				}
				Opcode::LStore1 => {
					let val = self.lpop();
					self.lstore(1, val);
				}
				Opcode::LStore2 => {
					let val = self.lpop();
					self.lstore(2, val);
				}
				Opcode::LStore3 => {
					let val = self.lpop();
					self.lstore(3, val);
				}
				Opcode::FStore0 => {
					let val = self.fpop();
					self.fstore(0, val);
				}
				Opcode::FStore1 => {
					let val = self.fpop();
					self.fstore(1, val);
				}
				Opcode::FStore2 => {
					let val = self.fpop();
					self.fstore(2, val);
				}
				Opcode::FStore3 => {
					let val = self.fpop();
					self.fstore(3, val);
				}
				Opcode::DStore0 => {
					let val = self.dpop();
					self.dstore(0, val);
				}
				Opcode::DStore1 => {
					let val = self.dpop();
					self.dstore(1, val);
				}
				Opcode::DStore2 => {
					let val = self.dpop();
					self.dstore(2, val);
				}
				Opcode::DStore3 => {
					let val = self.dpop();
					self.dstore(3, val);
				}
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
				Opcode::DupX1 => {
					let value_1 = self.ipop();
					let value_2 = self.ipop();
					self.ipush(value_1);
					self.ipush(value_2);
					self.ipush(value_1);
				},
				Opcode::DupX2 => todo!(),
				Opcode::Dup2 => todo!(),
				Opcode::Dup2X1 => todo!(),
				Opcode::Dup2X2 => todo!(),
				Opcode::Swap => {
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
				Opcode::IRem => { self.irem(); }
				Opcode::LRem => { self.lrem(); }
				Opcode::FRem => { self.frem(); }
				Opcode::DRem => { self.drem(); }
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
				Opcode::I2L => {
					let int = self.ipop();
					self.lpush(int as i64);
				}
				Opcode::I2F => {
					let int = self.ipop();
					self.fpush(int as f32);
				}
				Opcode::I2D => {
					let int = self.ipop();
					self.dpush(int as f64);
				}
				Opcode::L2I => {
					let long = self.lpop().to_be_bytes();
					self.ipush(i32::from_be_bytes([0, 0, long[2], long[3]]));
				}
				Opcode::L2F => {
					let long = self.lpop().to_be_bytes();
					self.fpush(i32::from_be_bytes([0, 0, long[2], long[3]]) as f32);
				}
				Opcode::L2D => {
					let long = self.lpop();
					self.dpush(long as f64);
				}
				Opcode::F2I => {
					let float = self.fpop();
					self.ipush(float.round() as i32);
				}
				Opcode::F2L => {
					let float = self.fpop();
					self.lpush(float.round() as i64);
				}
				Opcode::F2D => {
					let float = self.fpop();
					self.dpush(float as f64);
				}
				Opcode::D2I => {
					let double = self.dpop();
					let rounded = double.round();
					let long = rounded as i64;
					let bytes = long.to_be_bytes();
					self.ipush(i32::from_be_bytes([0, 0, bytes[2], bytes[3]]));
				}
				Opcode::D2L => {
					let double = self.dpop();
					let rounded = double.round();
					self.lpush(rounded as i64);
				}
				Opcode::D2F => {
					let double = self.dpop();
					self.fpush(double as f32);
				}
				Opcode::I2B => {
					let byte = self.ipop() & 0xFF;
					self.ipush(byte);
				}
				Opcode::I2C => {
					let bytes = self.ipop().to_be_bytes();
					let char = i32::from_be_bytes([0, 0, bytes[2], bytes[3]]);
					self.ipush(char);
				}
				Opcode::I2S => {
					let bytes = self.ipop().to_be_bytes();
					let short = i32::from_be_bytes([0, 0, bytes[2], bytes[3]]);
					self.ipush(short);
				}
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
				Opcode::DCmpG => { self.dcmpg(); }
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
				Opcode::Goto => {
					let offset = u32::from(u16::from_be_bytes([self.frame.code[self.frame.pc as usize], self.frame.code[(self.frame.pc + 1) as usize]]));
					self.frame.pc += offset;
				}
				Opcode::Jsr => todo!(),
				Opcode::Ret => todo!(),
				Opcode::TableSwitch => todo!(),
				Opcode::LookupSwitch => todo!(),
				Opcode::IReturn => { return self.ireturn(); } 
				Opcode::LReturn =>  { return self.lreturn(); } 
				Opcode::FReturn => { return self.freturn(); }
				Opcode::DReturn => { return self.lreturn(); }
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
				Opcode::Breakpoint => {
					// do nothing
				},
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
	use std::{collections::{BTreeMap, HashMap}, i32};

	use crate::{
		class::constant_pool::ConstantPool,
		isa::opcode::Opcode,
		vm::frame::StackFrame,
		vm::interpreter::Interpreter,
		vm::local::Locals,
		vm::operand_stack::OperandStack,
		vm::types::*};

	macro_rules! make_arithmetic_logic_test_cases {
		($rust_type:ty,
		 $return_type:ident,
		 $prefix:ident) => {
		fn ${ concat(run_, $prefix, _, test_cases)}(cases: Vec<($rust_type, $rust_type, Opcode, $rust_type)>) {
			for case in cases {
				let value1 = &case.0;
				let value2 = &case.1;
				let opcode = &case.2;
				let expected = &case.3;
				let mut interpreter = Interpreter::new(StackFrame {
					invoker: None,
					code: vec![opcode.clone() as u8],
					return_type: Type::$return_type,
					constant_pool: ConstantPool::new(),
					operand_stack: OperandStack::new(),
					locals: Locals { variables: HashMap::new() },
					pc: 0,
				});
				println!("opcode: {:?} value_1: {} value_2: {}", opcode, value1, value2);
				if vec![Opcode::INeg, Opcode::LNeg, Opcode::FNeg, Opcode::DNeg].contains(&opcode) == false {
					interpreter.${ concat($prefix, push) }(*value2);
				}
				interpreter.${ concat($prefix, push) }(*value1);
				interpreter.execute();
				assert_eq!(interpreter.${ concat($prefix, pop) }(), *expected);
				}
			}
		};
	}

	macro_rules! make_nan_test_cases {
		($rust_type:ty,
		 $return_type:ident,
		 $prefix:ident) => {
		fn ${ concat(run_, $prefix, _, nan_test_cases)}(cases: Vec<($rust_type, $rust_type, Opcode)>) {
			for case in cases {
				let value1 = case.0;
				let value2 = case.1;
				let opcode = case.2;
				let mut interpreter = Interpreter::new(StackFrame {
					invoker: None,
					code: vec![opcode.clone() as u8],
					return_type: Type::$return_type,
					constant_pool: ConstantPool::new(),
					operand_stack: OperandStack::new(),
					locals: Locals { variables: HashMap::new() },
					pc: 0,
				});
				if vec![Opcode::FNeg, Opcode::DNeg].contains(&opcode) == false {
					interpreter.${ concat($prefix, push) }(value2);
				}
				println!("opcode: {:?} value_1: {} value_2: {}", opcode, value1, value2);
				interpreter.${ concat($prefix, push) }(value1);
				interpreter.${ concat($prefix, push) }(value2);
				interpreter.frame.code.push(opcode as u8);
				interpreter.execute();
				assert!((interpreter.${ concat($prefix, pop) }()).is_nan());
				}
			}
		};
	}

	macro_rules! make_return_test_cases {
		($rust_type:ty,
		 $return_type:ident,
		 $jvm_type:ident,
		 $prefix:ident) => {
			fn ${ concat(run_, $prefix, return_, test_cases) }(cases: Vec<(Opcode, $jvm_type)>) {
				for case in cases {
					let opcode = case.0 as u8;
					let return_value = case.1;
					let mut interpreter = Interpreter::new(StackFrame {
						invoker: None,
						code: vec![
							opcode,
						],
						return_type: Type::$return_type,
						constant_pool: ConstantPool::new(),
						operand_stack: OperandStack::new(),
						locals: Locals { variables: HashMap::new() },
						pc: 0,
					});
					println!("opcode: {:?} expected value: {:?}", opcode, return_value);
					interpreter.${ concat($prefix, push) }(return_value.value);
					let ret = interpreter.execute();
					let expected = return_value;
					assert_eq!(ret.unwrap(), Variable::$jvm_type(expected));
				}
			}
		};
	}

	make_arithmetic_logic_test_cases!(i32, I, i);
	make_arithmetic_logic_test_cases!(i64, J, l);
	make_arithmetic_logic_test_cases!(f32, F, f);
	make_arithmetic_logic_test_cases!(f64, D, d);
	make_nan_test_cases!(f32, F, f);
	make_nan_test_cases!(f64, D, d);
	make_return_test_cases!(i32, I, Int, i);

	#[test]
	fn test_int_return() {
		let i_cases: Vec<(Opcode, Int)> = vec![
			(Opcode::IReturn, Int { value: 32 }),
			(Opcode::IReturn, Int { value: -1 }),
			(Opcode::IReturn, Int { value: i32::MAX }),
		];
		run_ireturn_test_cases(i_cases);
	}

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
			(4, 2, Opcode::IRem, 0),
			(4, 3, Opcode::IRem, 1),
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
			(4, 2, Opcode::LRem, 0),
			(4, 3, Opcode::LRem, 1),
			(1, 1, Opcode::LAnd, 1),
			(1, 1, Opcode::LOr, 1),
			(0, 0, Opcode::LOr, 0),
			(1, 0, Opcode::LXor, 1),
			(1, 1, Opcode::LShl, 2),
			(300, 0, Opcode::LNeg, -300),
		];
		run_l_test_cases(l_cases);
	}

	#[test]
	fn test_float_operations() {
		let f_cases: Vec<(f32, f32, Opcode, f32)> = vec![
			(1.0, 1.0, Opcode::FAdd, 2.0),
			(100.0, -0.5, Opcode::FAdd, 99.5),
			(1.0, 1.0, Opcode::FSub, 0.0),
			(0.0, -0.5, Opcode::FSub, 0.5),
			(2.25, 2.0, Opcode::FMul, 4.50),
			(4.0, 2.0, Opcode::FDiv, 2.0),
			(5.0, 2.0, Opcode::FRem, 1.0),
			(1.0, 1.0, Opcode::FRem, 0.0),
			(300.0, 0.0, Opcode::FNeg, -300.0),
		];
		let f_nan_cases: Vec<(f32, f32, Opcode)> = vec![
			(1.0, f32::NAN, Opcode::FAdd),
			(f32::NAN, 1.0, Opcode::FAdd),
			(f32::NAN, f32::NAN, Opcode::FAdd),
			(1.0, f32::NAN, Opcode::FSub),
			(f32::NAN, 1.0, Opcode::FSub),
			(f32::NAN, f32::NAN, Opcode::FSub),
			(1.0, f32::NAN, Opcode::FMul),
			(f32::NAN, 1.0, Opcode::FMul),
			(f32::NAN, f32::NAN, Opcode::FMul),
			(1.0, f32::NAN, Opcode::FDiv),
			(f32::NAN, 1.0, Opcode::FDiv),
			(f32::NAN, f32::NAN, Opcode::FDiv),
		];
		run_f_test_cases(f_cases);
		run_f_nan_test_cases(f_nan_cases);
	}

	#[test]
	fn test_double_operations() {
		let d_cases: Vec<(f64, f64, Opcode, f64)> = vec![
			(1.0, 1.0, Opcode::DAdd, 2.0),
			(100.0, -0.5, Opcode::DAdd, 99.5),
			(1.0, 1.0, Opcode::DSub, 0.0),
			(0.0, -0.5, Opcode::DSub, 0.5),
			(2.25, 2.0, Opcode::DMul, 4.50),
			(4.0, 2.0, Opcode::DDiv, 2.0),
			(5.0, 2.0, Opcode::DRem, 1.0),
			(1.0, 1.0, Opcode::DRem, 0.0),
			(300.0, 0.0, Opcode::DNeg, -300.0),
		];
		let d_nan_cases: Vec<(f64, f64, Opcode)> = vec![
			(1.0, f64::NAN, Opcode::DAdd),
			(f64::NAN, 1.0, Opcode::DAdd),
			(f64::NAN, f64::NAN, Opcode::DAdd),
			(1.0, f64::NAN, Opcode::DSub),
			(f64::NAN, 1.0, Opcode::DSub),
			(f64::NAN, f64::NAN, Opcode::DSub),
			(1.0, f64::NAN, Opcode::DMul),
			(f64::NAN, 1.0, Opcode::DMul),
			(f64::NAN, f64::NAN, Opcode::DMul),
			(1.0, f64::NAN, Opcode::DDiv),
			(f64::NAN, 1.0, Opcode::DDiv),
			(f64::NAN, f64::NAN, Opcode::DDiv),
		];
		run_d_test_cases(d_cases);
		run_d_nan_test_cases(d_nan_cases);
	}

	/// Test zero conditional branches.
	///
	/// If the check results in a branch, the PC will be set to 16 which will break the interpreter loop.
	/// If the check does not result in a branch, execution will continue to byte 3 (nop) which will 
	/// increment the PC to 4.
	#[test]
	fn test_zero_conditional_branches() {
		let cases = vec![
			(Opcode::IfEq, 0, 16),
			(Opcode::IfEq, 1, 4),
			(Opcode::IfGt, 1, 16),
			(Opcode::IfGt, 0, 4),
			(Opcode::IfLt, 0, 4),
			(Opcode::IfLt, -1, 16),
			(Opcode::IfGe, 0, 16),
			(Opcode::IfGe, 1, 16),
			(Opcode::IfGe, -1, 4),
			(Opcode::IfLe, 0, 16),
			(Opcode::IfLe, 1, 4),
			(Opcode::IfLe, -1, 16),
		];
		for case in cases {
			let opcode = case.0 as u8;
			let value = &case.1;
			let expected_pc = case.2;
			let frame = StackFrame::new();
			let mut interpreter = Interpreter::new(frame);
			interpreter.ipush(*value);
			interpreter.frame.code = vec![
				opcode, // 0
				0u8, // 1
				16u8, // 2
				Opcode::Nop as u8 // 3
			];
			let _ = interpreter.execute();
			assert_eq!(interpreter.frame.pc, expected_pc);
		}
	}

	/// Test integer conditional branches.
	///
	/// If the check results in a branch, the PC will be set to 16 which will break the interpreter loop.
	/// If the check does not result in a branch, execution will continue to byte 3 (nop) which will 
	/// increment the PC to 4.
	#[test]
	fn test_integer_conditional_branches() {
		let cases = vec![
			(Opcode::IfICmpEq, 0, 0, 16),
			(Opcode::IfICmpEq, 1, 0, 4),
			(Opcode::IfICmpEq, 0, 1, 4),
			(Opcode::IfICmpGt, 0, 0, 4),
			(Opcode::IfICmpGt, 0, 1, 4),
			(Opcode::IfICmpGt, 1, 0, 16),
			(Opcode::IfICmpLt, 0, 0, 4),
			(Opcode::IfICmpLt, 1, 0, 4),
			(Opcode::IfICmpLt, 0, 1, 16),
			(Opcode::IfICmpGe, 0, 0, 16),
			(Opcode::IfICmpGe, 1, 0, 16),
			(Opcode::IfICmpGe, 0, 1, 4),
			(Opcode::IfICmpLe, 0, 0, 16),
			(Opcode::IfICmpLe, 1, 0, 4),
			(Opcode::IfICmpLe, 0, 1, 16),
		];
		for case in cases {
			let opcode = case.0 as u8;
			let value_1 = &case.1;
			let value_2 = &case.2;
			let expected_pc = case.3;
			let frame = StackFrame::new();
			let mut interpreter = Interpreter::new(frame);
			interpreter.ipush(*value_2);
			interpreter.ipush(*value_1);
			interpreter.frame.code = vec![
				opcode, // 0
				0u8, // 1
				16u8, // 2
				Opcode::Nop as u8 // 3
			];
			let _ = interpreter.execute();
			assert_eq!(interpreter.frame.pc, expected_pc);
		}
	}

	/// Test float comparisons.
	/// 
	/// The fcmpl and fcmpg opcodes should produce an identical top of stack except for 
	/// when one or both compared values are NaN. In that case, fcmpg pushes 1 and fcmpl
	/// pushes -1.
	#[test]
	fn test_float_comparisons() {
		let cases: Vec<(Opcode, f32, f32, i32)> = vec![
			(Opcode::FCmpG, 0.0, 0.0, 0),
			(Opcode::FCmpG, 3.14, 3.14, 0),
			(Opcode::FCmpG, 0.0, -3.14, 1),
			(Opcode::FCmpG, -3.14, 0.0, -1),
			(Opcode::FCmpG, 0.0, -0.0, 0),
			(Opcode::FCmpG, 0.0, f32::NAN, 1),
			(Opcode::FCmpG, f32::NAN, 0.0, 1),
			(Opcode::FCmpG, f32::NAN, f32::NAN, 1),
			(Opcode::FCmpL, 0.0, 0.0, 0),
			(Opcode::FCmpL, 3.14, 3.14, 0),
			(Opcode::FCmpL, 0.0, -3.14, 1),
			(Opcode::FCmpL, -3.14, 0.0, -1),
			(Opcode::FCmpL, 0.0, -0.0, 0),
			(Opcode::FCmpL, 0.0, f32::NAN, -1),
			(Opcode::FCmpL, f32::NAN, 0.0, -1),
			(Opcode::FCmpL, f32::NAN, f32::NAN, -1),
		];
		for case in cases {
			let opcode = case.0 as u8;
			let value_1 = &case.1;
			let value_2 = &case.2;
			let expected = case.3;
			let frame = StackFrame::new();
			let mut interpreter = Interpreter::new(frame);
			interpreter.fpush(*value_2);
			interpreter.fpush(*value_1);
			interpreter.frame.code = vec![opcode];
			println!("opcode: {:?} value_1: {} value_2: {}", Opcode::try_from(opcode).unwrap(), value_1, value_2);
			let _ = interpreter.execute();
			assert_eq!(interpreter.ipop(), expected);
		}
	}

	/// Test double comparisons.
	/// 
	/// The dcmpl and dcmpg opcodes should produce an identical top of stack except for 
	/// when one or both compared values are NaN. In that case, dcmpg pushes 1 and dcmpl
	/// pushes -1.
	#[test]
	fn test_double_comparisons() {
		let cases: Vec<(Opcode, f64, f64, i32)> = vec![
			(Opcode::DCmpG, 0.0, 0.0, 0),
			(Opcode::DCmpG, 3.14, 3.14, 0),
			(Opcode::DCmpG, 0.0, -3.14, 1),
			(Opcode::DCmpG, -3.14, 0.0, -1),
			(Opcode::DCmpG, 0.0, -0.0, 0),
			(Opcode::DCmpG, 0.0, f64::NAN, 1),
			(Opcode::DCmpG, f64::NAN, 0.0, 1),
			(Opcode::DCmpG, f64::NAN, f64::NAN, 1),
			(Opcode::DCmpL, 0.0, 0.0, 0),
			(Opcode::DCmpL, 3.14, 3.14, 0),
			(Opcode::DCmpL, 0.0, -3.14, 1),
			(Opcode::DCmpL, -3.14, 0.0, -1),
			(Opcode::DCmpL, 0.0, -0.0, 0),
			(Opcode::DCmpL, 0.0, f64::NAN, -1),
			(Opcode::DCmpL, f64::NAN, 0.0, -1),
			(Opcode::DCmpL, f64::NAN, f64::NAN, -1),
		];
		for case in cases {
			let opcode = case.0 as u8;
			let value_1 = &case.1;
			let value_2 = &case.2;
			let expected = case.3;
			let frame = StackFrame::new();
			let mut interpreter = Interpreter::new(frame);
			interpreter.dpush(*value_2);
			interpreter.dpush(*value_1);
			println!("opcode: {:?} value_1: {} value_2: {}", Opcode::try_from(opcode).unwrap(), value_1, value_2);
			interpreter.frame.code = vec![opcode];
			interpreter.execute();
			assert_eq!(interpreter.ipop(), expected);
		}
	}
}