use binrw::BinRead;

use crate::{
	class::{
		attribute::Attribute},
	generate_pool_context_read
};

/// An implementation of a method_info structure (JVMS17 4.6)
#[derive(Clone, Debug, PartialEq)]
pub struct Method {
	pub access_flags: u16,
	pub name_index: u16,
	pub descriptor_index: u16,
	pub attributes_count: u16,
	pub attributes: Vec<Attribute>,
}

generate_pool_context_read!(Method, Method);