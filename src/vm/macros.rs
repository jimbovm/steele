#[macro_export]
macro_rules! make_pop_load {
	($prefix:ident,
	 $rust_type:ty,
	 $variable_type:ty,
	 $getter:ident,
	 $byte_len:literal
	) => {
		pub fn ${ concat($prefix, load) } (&mut self, index: u16) -> Result<$variable_type, Box<dyn Error>> {
			let local = self.frame.locals.$getter(index)?;
			self.frame.operand_stack.push(&local.value.to_be_bytes());
			Ok(local)
		}

		pub fn $ { concat($prefix, pop) } (&mut self) -> $rust_type {
			let mut popped = [0u8; $byte_len];
			for i in 0..$byte_len {	popped[i] = self.frame.operand_stack.pop();	}
			<$rust_type>::from_be_bytes(popped)
		}
	}
}

#[macro_export]
macro_rules! make_push {
	($prefix:ident,
	 $rust_type:ty
	) => {
		pub fn ${ concat($prefix, push) } (&mut self, value: $rust_type) {
			self.frame.operand_stack.push(&<$rust_type>::to_le_bytes(value));
		}
	};
}

/// For shorts and bytes, which are represented as ints internally
#[macro_export]
macro_rules! make_push_extend {
	($prefix:ident,
	 $rust_type:ty
	) => {
		pub fn ${ concat($prefix, push) } (&mut self, value: $rust_type) {
			self.frame.operand_stack.push(&u32:to_le_bytes(value));
		}
	};
}

#[macro_export]
macro_rules! make_integer_arithmetic_logic {
	($prefix:ident,
	 $rust_type:ty
	) => {
		pub fn ${ concat($prefix, add) } (&mut self) -> $rust_type {
			let val: $rust_type = self.${ concat($prefix, pop) }().wrapping_add(self.${ concat($prefix, pop) }());
			self.${ concat($prefix, push) }(val as $rust_type);
			val
		}
		pub fn ${ concat($prefix, sub) } (&mut self) -> $rust_type {
			let val: $rust_type = self.${ concat($prefix, pop) }().wrapping_sub(self.${ concat($prefix, pop) }());
			self.${ concat($prefix, push) }(val as $rust_type);
			val
		}
		pub fn ${ concat($prefix, mul) } (&mut self) -> $rust_type {
			let val = self.${ concat($prefix, pop) }().wrapping_mul(self.${ concat($prefix, pop) }());
			self.${ concat($prefix, push) }(val as $rust_type);
			val
		}
		pub fn ${ concat($prefix, div) } (&mut self) -> $rust_type {
			let val = self.${ concat($prefix, pop) }().wrapping_div(self.${ concat($prefix, pop) }());
			self.${ concat($prefix, push) }(val as $rust_type);
			val
		}
		pub fn ${ concat($prefix, neg) } (&mut self) -> $rust_type {
			let val = -self.${ concat($prefix, pop) }();
			self.${ concat($prefix, push) }(val as $rust_type);
			val
		}
		pub fn ${ concat($prefix, and) } (&mut self) -> $rust_type {
			let val = self.${ concat($prefix, pop) }() & self.${ concat($prefix, pop) }();
			self.${ concat($prefix, push) }(val as $rust_type);
			val
		}
		pub fn ${ concat($prefix, or) } (&mut self) -> $rust_type {
			let val = self.${ concat($prefix, pop) }() | self.${ concat($prefix, pop) }();
			self.${ concat($prefix, push) }(val as $rust_type);
			val
		}
		pub fn ${ concat($prefix, xor) } (&mut self) -> $rust_type {
			let val = self.${ concat($prefix, pop) }() ^ self.${ concat($prefix, pop) }();
			self.${ concat($prefix, push) }(val as $rust_type);
			val
		}
		pub fn ${ concat($prefix, shl) } (&mut self) -> $rust_type {
			let val = self.${ concat($prefix, pop) }() << (self.${ concat($prefix, pop) }() & 0x1F);
			self.${ concat($prefix, push) }(val as $rust_type);
			val
		}
		pub fn ${ concat($prefix, shr) } (&mut self) -> $rust_type {
			let val = self.${ concat($prefix, pop) }() >> (self.${ concat($prefix, pop) }() & 0x1F);
			self.${ concat($prefix, push) }(val as $rust_type);
			val
		}
	};
}

#[macro_export]
macro_rules! make_float_arithmetic {
	($prefix:ident,
	 $rust_type:ty
	) => {
		pub fn ${ concat($prefix, add) } (&mut self) -> $rust_type {
			let val: $rust_type = self.${ concat($prefix, pop) }() + self.${ concat($prefix, pop) }();
			self.${ concat($prefix, push) }(val as $rust_type);
			val
		}
		pub fn ${ concat($prefix, sub) } (&mut self) -> $rust_type {
			let val: $rust_type = self.${ concat($prefix, pop) }() - self.${ concat($prefix, pop) }();
			self.${ concat($prefix, push) }(val as $rust_type);
			val
		}
		pub fn ${ concat($prefix, mul) } (&mut self) -> $rust_type {
			let val: $rust_type = self.${ concat($prefix, pop) }() * self.${ concat($prefix, pop) }();
			self.${ concat($prefix, push) }(val as $rust_type);
			val
		}
		pub fn ${ concat($prefix, div) } (&mut self) -> $rust_type {
			let val: $rust_type = self.${ concat($prefix, pop) }() / self.${ concat($prefix, pop) }();
			self.${ concat($prefix, push) }(val as $rust_type);
			val
		}
		pub fn ${ concat($prefix, neg) } (&mut self) -> $rust_type {
			let val = -self.${ concat($prefix, pop) }();
			self.${ concat($prefix, push) }(val as $rust_type);
			val
		}
	};
}

#[macro_export]
macro_rules! make_conditional_branches {
	($condition_name:ident,
	 $operator:tt) => {
		pub fn ${concat (if_icmp, $condition_name)}(&mut self) {
			let offset_high = self.frame.code[(self.frame.pc) as usize];
			let offset_low = self.frame.code[(self.frame.pc+1) as usize];
			let offset = i16::from_be_bytes([offset_high, offset_low]);
			self.frame.pc += if (self.ipop() $operator self.ipop()) { ((offset-1) as u32) } else { 2 };
		}

		pub fn ${concat (if_, $condition_name)}(&mut self) {
			let offset_high = self.frame.code[(self.frame.pc) as usize];
			let offset_low = self.frame.code[(self.frame.pc+1) as usize];
			let offset = i16::from_be_bytes([offset_high, offset_low]);
			self.frame.pc += if (self.ipop() $operator 0) { ((offset-1) as u32) } else { 2 };
		}
	}
}

#[macro_export]
macro_rules! make_float_comparisons {
	($prefix:ident,
	 $rust_type:ty) => {
		fn ${ concat($prefix, "cmp") }(&mut self, value_1: $rust_type, value_2: $rust_type) -> i32 {
			if value_1 > value_2 {
				return 1;
			} else if value_1 < value_2 {
				return -1;
			}
			else if value_1 == value_2 {
				return 0;
			}
			// if we're here, we have at least one NaN
			return -2;
		}

		pub fn ${ concat($prefix, "cmpg") }(&mut self) {
			let value_1: $rust_type = self.${ concat($prefix, "pop") }();
			let value_2: $rust_type = self.${ concat($prefix, "pop") }();
			match self.${ concat($prefix, "cmp") }(value_1, value_2) {
				-2 => { self.ipush(1); }
				anything_else => { self.ipush(anything_else); }
			}
		}

		pub fn ${ concat($prefix, "cmpl") }(&mut self) {
			let value_1: $rust_type = self.${ concat($prefix, "pop") }();
			let value_2: $rust_type = self.${ concat($prefix, "pop") }();
			match self.${ concat($prefix, "cmp") }(value_1, value_2) {
				-2 => { self.ipush(-1); }
				anything_else => { self.ipush(anything_else); }
			}
		}
	};
}