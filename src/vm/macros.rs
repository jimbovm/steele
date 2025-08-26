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