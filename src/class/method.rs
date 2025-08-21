use binrw::binrw;

use super::attribute::Attribute;

#[binrw]
#[brw(big)]
pub struct Method {
	pub access_flags: u16,
	pub name_index: u16,
	pub descriptor_index: u16,
	pub attributes_count: u16,
	#[br(count = attributes_count)]
	pub attributes: Vec<Attribute>,
}