use std::{
	fmt::Display,
	fmt::Debug,
	error::Error};

use crate::{
	make_error,
};

#[macro_export]
macro_rules! make_error {
	($err_name: ident) => {
		#[derive(Debug)]
		pub struct $err_name {
			pub msg: String,	
		}

		impl Display for $err_name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "{}", self.msg)
			}
		}

		impl Error for $err_name {}
	};
}

make_error!(StackError);
make_error!(VariableError);
make_error!(FetchError);
make_error!(DecodeError);