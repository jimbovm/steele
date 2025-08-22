use std::{
	collections::BTreeMap, fmt::{
		self,
		Error,
		Formatter}, io::{Read, Seek}};

use binrw::{
	binrw,
	BinRead};
use strum_macros;

use crate::class::modified_utf8::ModifiedUtf8String;

#[binrw]
#[brw(big)]
#[derive(Default)]
pub struct RawConstantPool {
	pub constant_pool_count: u16,
	#[br(count = constant_pool_count - 2)]
	pub constants: Vec<ConstantPoolItem>,
}

#[derive(Debug, Default)]
pub struct ConstantPool {
	pub length: u16,
	pub constants: BTreeMap<u16, ConstantPoolItem>,
}

impl From<RawConstantPool> for ConstantPool {
	fn from(raw: RawConstantPool) -> Self {
		let canonical_pool = Self::canonical_constant_pool_from(raw.constants);
		Self {
			length: raw.constant_pool_count,
			constants: canonical_pool,
		}
	}
}

impl ConstantPool {

	/// Converts a raw constant pool to canonical form.
	/// 
	/// Due to a JVM design mistake a double or long stored in the constant pool table "invalidates" the next index.
	/// For instance if a double or long is stored as constant pool entry number 10, counting from 1, then the
	/// index number number used for the next entry will be 12, and index number 11 will be invalid (JVMS17 4.4.5).
	fn canonical_constant_pool_from(raw_constant_pool: Vec<ConstantPoolItem>) -> BTreeMap<u16, ConstantPoolItem> {
		let mut canonical_constant_pool: BTreeMap<u16, ConstantPoolItem> = BTreeMap::new();
		let mut index: u16 = 1;
		for item in raw_constant_pool.into_iter() {
			let increment: u16;
			match item {
				ConstantPoolItem::Double(ref _d) => { increment = 2; },
				ConstantPoolItem::Long(ref _l) => { increment = 2; },
				_ => { increment = 1; }
			}
			canonical_constant_pool.insert(index, item);
			index += increment;
		}
		return canonical_constant_pool;
	}

}

/// A control enum used in polymorphic parsing of constant pool entries.
#[binrw]
#[derive(PartialEq, Debug, Clone, strum_macros::Display)]
pub enum ConstantPoolItem {
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
#[derive(PartialEq, Debug, Clone)]
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
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Integer {
	pub value: u32,
}

/// An implementation of CONSTANT_Float (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Float {
	pub value: u32,
}

/// An implementation of CONSTANT_Long (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Long {
	pub value: u64,
}

/// An implementation of CONSTANT_Double (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Double {
	pub value: u64,
}

/// An implementation of CONSTANT_Class (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Class {
	pub index: u16,
}

/// An implementation of CONSTANT_String (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct String {
	pub index: u16,
}

/// An implementation of CONSTANT_Fieldref (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct FieldRef {
	pub class_index: u16,
	pub name_and_type_index: u16,
}

/// An implementation of CONSTANT_Methodref (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct MethodRef {
	pub class_index: u16,
	pub name_and_type_index: u16,
}

/// An implementation of CONSTANT_InterfaceMethodref (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[bw(magic = 11u8)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct InterfaceMethodRef {
	pub class_index: u16,
	pub name_and_type_index: u16,
}

/// An implementation of CONSTANT_NameAndType (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct NameAndType {
	pub name_index: u16,
	pub type_index: u16,
}