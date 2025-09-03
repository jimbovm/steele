use strum_macros::Display;

#[derive(Clone, Debug, Default, Display, PartialEq)]
pub enum Type {
	Z, // boolean
	B, // byte
	C, // char
	D, // double
	F, // float
	I, // int
	L(String), // object
	J, // long
	S, // short
	A, // array dimension
	#[default]
	V, // void
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Variable {
	Boolean(Boolean),
	Byte(Byte),
	Char(Char),
	Double(Double),
	Float(Float),
	Int(Int),
	Long(Long),
	Short(Short),
	ClassReference(ClassReference),
	ArrayReference(ArrayReference),
	ReturnAddress(ReturnAddress),
	Void(Void),
}

impl Default for Variable {
	fn default() -> Self {
		Variable::Void(Void {})
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct Void {}

#[derive(Clone, Debug, PartialEq)]
pub struct Boolean {
	pub value: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Byte {
	pub value: i8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Char {
	pub value: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Float {
	pub value: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Double {
	pub value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Int {
	pub value: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Long {
	pub value: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Short {
	pub value: i16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClassReference {
	pub class_name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayReference {
	pub dimensions: u32,
	pub class_name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnAddress {
	pub value: u32,
}