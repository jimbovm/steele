use std::{
	fmt::{
		self,
		Error,
		Formatter}};

use binrw::binrw;
use strum_macros;

use crate::class::modified_utf8::ModifiedUtf8String;

/// A control enum used in polymorphic parsing of constant pool entries.
#[binrw]
#[derive(PartialEq, Debug, strum_macros::Display)]
pub enum Item {
	/// Tag for CONSTANT_Utf8 (JVMS17 4.4-B)
	#[br(magic(1u8))]
	Utf8(Utf8),
	/// Tag for CONSTANT_Integer (JVMS17 4.4-B)
	#[br(magic(3u8))]
	Integer(Integer),
	/// Tag for CONSTANT_Float (JVMS17 4.4-B)
	#[br(magic(4u8))]
	Float(Float),
	/// Tag for CONSTANT_Long (JVMS17 4.4-B)
	#[br(magic(5u8))]
	Long(Long),
	/// Tag for CONSTANT_Double (JVMS17 4.4-B)
	#[br(magic(6u8))]
	Double(Double),
	/// Tag for CONSTANT_Class (JVMS17 4.4-B)
	#[br(magic(7u8))]
	Class(Class),
	/// Tag for CONSTANT_String (JVMS17 4.4-B)
	#[br(magic(8u8))]
	String(String),
	/// Tag for CONSTANT_Fieldref (JVMS17 4.4-B)
	#[br(magic(9u8))]
	FieldRef(FieldRef),
	/// Tag for CONSTANT_Methodref (JVMS17 4.4-B)
	#[br(magic(10u8))]
	MethodRef(MethodRef),
	/// Tag for CONSTANT_InterfaceMethod (JVMS17 4.4-B)
	#[br(magic(11u8))]
	InterfaceMethodRef(InterfaceMethodRef),
	/// Tag for CONSTANT_NameAndType (JVMS17 4.4-B)
	#[br(magic(12u8))]
	NameAndType(NameAndType),
}

/// An implementation of CONSTANT_Utf8 (JVMS17 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct Utf8 {
	pub length: u16,
	#[br(count = length)]
	pub bytes: Vec<u8>,
}

impl fmt::Display for Utf8 {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "{}", ModifiedUtf8String::new(self.bytes.clone()))
	}
}

/// An implementation of CONSTANT_Integer (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct Integer {
	pub value: u32,
}

/// An implementation of CONSTANT_Float (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct Float {
	pub value: u32,
}

/// An implementation of CONSTANT_Long (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct Long {
	pub value: u64,
}

/// An implementation of CONSTANT_Double (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct Double {
	pub value: u64,
}

/// An implementation of CONSTANT_Class (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct Class {
	pub index: u16,
}

/// An implementation of CONSTANT_String (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct String {
	pub index: u16,
}

/// An implementation of CONSTANT_Fieldref (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct FieldRef {
	pub class_index: u16,
	pub name_and_type_index: u16,
}

/// An implementation of CONSTANT_Methodref (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct MethodRef {
	pub class_index: u16,
	pub name_and_type_index: u16,
}

/// An implementation of CONSTANT_InterfaceMethodref (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[bw(magic = 11u8)]
#[derive(PartialEq, Debug)]
pub struct InterfaceMethodRef {
	pub class_index: u16,
	pub name_and_type_index: u16,
}

/// An implementation of CONSTANT_NameAndType (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct NameAndType {
	pub name_index: u16,
	pub type_index: u16,
}