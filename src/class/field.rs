use binrw::BinRead;

use crate::{
	class::{
		attribute::Attribute},
	generate_pool_context_read,
};

/// An implementation of a field_info structure (JVMS17 4.5)
#[derive(Clone, Debug, PartialEq)]
pub struct Field {
	pub access_flags: u16,
	pub name_index: u16,
	pub descriptor_index: u16,
	pub attributes_count: u16,
	pub attributes: Vec<Attribute>,
}

generate_pool_context_read!(Field, Field);