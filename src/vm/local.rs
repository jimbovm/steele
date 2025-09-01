use std::{
	collections::HashMap,
	error::Error};

use crate::{
	make_local_accessor,
	error::*,
	vm::types::*,
};

/// An encapsulation of a local variable array for a stack frame.
#[derive(Debug, Default)]
pub struct Locals {
	pub variables: HashMap<u32, Variable>
}

impl Locals {

	/// Create a new local variable map, observing the rules for doubles and longs.
	pub fn new(raw_variables: Vec<Variable>) -> Result<Locals, VariableError> {
		let mut variables: HashMap<u32, Variable> = HashMap::new();
		let mut modifier = 0;
		for i in 0..raw_variables.len() {
			let var = raw_variables.get(i);
			match var {
				Some(Variable::Double(d)) => {
					modifier += 1;
					variables.insert((i + modifier) as u32, Variable::Double(Double { value: d.value }));
				},
				Some(Variable::Long(l)) => {
					modifier += 1;
					variables.insert((i + modifier) as u32, Variable::Long(Long { value: l.value }));
				},
				Some(other_variable) => {
					variables.insert((i + modifier) as u32, other_variable.clone());
				},
				None => {
					return Err(VariableError { msg: "unknown error creating locals object".to_string() });
				}
			};
		}
		Ok(Locals { variables })
	}

	make_local_accessor!(get_boolean, Boolean, "Boolean");
	make_local_accessor!(get_byte, Byte, "Byte");
	make_local_accessor!(get_char, Char, "Char");
	make_local_accessor!(get_int, Int, "Int");
	make_local_accessor!(get_float, Float, "Float");
	make_local_accessor!(get_long, Long, "Long");
	make_local_accessor!(get_short, Short, "Short");
	make_local_accessor!(get_double, Double, "Double");
	make_local_accessor!(get_return_address, ReturnAddress, "ReturnAddress");
	make_local_accessor!(get_class_reference, ClassReference, "ClassReference");
	make_local_accessor!(get_array_reference, ArrayReference, "ArrayReference");

}

#[macro_export]
macro_rules! make_local_accessor {
	($fn_name: ident,
	 $variable_type: ident,
	 $wanted_type: literal
	) => {
		pub fn $fn_name(&self, index: u32) -> Result<$variable_type, Box<dyn Error>> {
			let variable = self.variables.get(&index);
			match variable {
				Some(variable) => {
					match variable {
						Variable::$variable_type(itm) => Ok(itm.clone()),
						other_type => Err(crate::class::constant_pool::TypeError { wanted_type: $wanted_type.to_string(), actual_type: other_type.to_string() }.to_string().into())
					}
				}
				None => Err(crate::class::constant_pool::IndexError { index: index as u16 }.into())
			}
		}
	};
}