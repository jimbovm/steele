use binrw::binrw;

use super::attribute::ConstantValue;

/// An implementation of a field_info structure (JVMS17 4.5)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct Field {
	pub access_flags: u16,
	pub name_index: u16,
	pub descriptor_index: u16,
	pub attributes_count: u16,
	#[br(count = attributes_count as u16)]
	pub attributes: Vec<ConstantValue>,
}