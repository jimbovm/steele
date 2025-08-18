use binrw::binrw;

#[binrw]
#[derive(PartialEq, Debug)]
pub enum Item {
	#[br(magic(1u8))]
	Utf8(Utf8),
	#[br(magic(3u8))]
	Integer(Integer),
	#[br(magic(4u8))]
	Float(Float),
	#[br(magic(5u8))]
	Long(Long),
	#[br(magic(6u8))]
	Double(Double),
	#[br(magic(7u8))]
	Class(Class),
	#[br(magic(8u8))]
	String(String),
	#[br(magic(9u8))]
	FieldRef(FieldRef),
	#[br(magic(10u8))]
	MethodRef(MethodRef),
	#[br(magic(11u8))]
	InterfaceMethodRef(InterfaceMethodRef),
	#[br(magic(12u8))]
	NameAndType(NameAndType),
}

#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct Utf8 {
	pub length: u16,
	#[br(count = length)]
	pub bytes: Vec<u8>,
}

#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct Integer {
	pub value: u32,
}

#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct Float {
	pub value: u32,
}

#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct Long {
	pub value: u64,
}

#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct Double {
	pub value: u64,
}

#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct Class {
	pub index: u16,
}

#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct String {
	pub index: u16,
}

#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct FieldRef {
	pub class_index: u16,
	pub name_and_type_index: u16,
}

#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct MethodRef {
	pub class_index: u16,
	pub name_and_type_index: u16,
}

#[binrw]
#[brw(big)]
#[bw(magic = 11u8)]
#[derive(PartialEq, Debug)]
pub struct InterfaceMethodRef {
	pub class_index: u16,
	pub name_and_type_index: u16,
}

#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct NameAndType {
	pub name_index: u16,
	pub type_index: u16,
}