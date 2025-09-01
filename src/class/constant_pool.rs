use std::{
	collections::BTreeMap,
	error::Error,
	fmt::{
		self, Display, Formatter}};

use binrw::binrw;
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

#[derive(Clone, Debug, Default)]
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

#[derive(Debug)]
pub struct TypeError {
	pub wanted_type: std::string::String,
	pub actual_type: std::string::String,
}

impl Display for TypeError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{} wanted, but got {}", self.wanted_type, self.actual_type)
	}
}

impl Error for TypeError {}

#[derive(Debug)]
pub struct IndexError {
	pub index: u16,
}

impl Display for IndexError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "invalid index {}", self.index)
	}
}

impl Error for IndexError {}

/// Create an accessor for retrieving a ConstantPoolItem as a specific type.
#[macro_export]
macro_rules! make_accessor {
	($fn_name: ident,
	 $constant_pool_item_type: ident,
	 $wanted_type: literal
	) => {
		pub fn $fn_name(&self, index: u16) -> Result<$constant_pool_item_type, Box<dyn Error>> {
			let constant = self.constants.get(&index);
			match constant {
				Some(constant) => {
					match constant {
						ConstantPoolItem::$constant_pool_item_type(itm) => Ok(itm.clone()),
						other_type => Err(TypeError { wanted_type: $wanted_type.to_string(), actual_type: other_type.to_string() }.to_string().into())
					}
				}
				None => Err(IndexError { index }.into())
			}
		}
	};
}

impl ConstantPool {

	pub fn new() -> ConstantPool {
		ConstantPool { length: 0, constants: BTreeMap::new() }
	}

	make_accessor!(get_utf8, Utf8, "Utf8");
	make_accessor!(get_int, Integer, "Integer");
	make_accessor!(get_float, Float, "Float");
	make_accessor!(get_long, Long, "Long");
	make_accessor!(get_double, Double, "Double");
	make_accessor!(get_class, Class, "Class");
	make_accessor!(get_string, String, "String");
	make_accessor!(get_field_ref, FieldRef, "FieldRef");
	make_accessor!(get_method_ref, MethodRef, "MethodRef");
	make_accessor!(get_interface_method_ref, InterfaceMethodRef, "InterfaceMethodRef");
	make_accessor!(get_name_and_type, NameAndType, "NameAndType");

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
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", ModifiedUtf8String::new(self.bytes.clone()))
	}
}

/// An implementation of CONSTANT_Integer (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Integer {
	pub value: i32,
}

/// An implementation of CONSTANT_Float (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Float {
	pub value: f32,
}

/// An implementation of CONSTANT_Long (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Long {
	pub value: i64,
}

/// An implementation of CONSTANT_Double (JVMS 4.4-B)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Double {
	pub value: f64,
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

#[derive(Clone, Debug, Default)]
pub struct ConstantPoolRequiredArgs {
	pub constant_pool: ConstantPool,
} 