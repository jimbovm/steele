
use binrw::BinRead;

use crate::class::{attribute::Attribute, constant_pool::ConstantPoolRequiredArgs};


/// An implementation of a field_info structure (JVMS17 4.5)
#[derive(PartialEq, Debug)]
pub struct Field {
	pub access_flags: u16,
	pub name_index: u16,
	pub descriptor_index: u16,
	pub attributes_count: u16,
	// pub attributes: Vec<Attribute>,
	pub attributes: Vec<Attribute>,
}

impl BinRead for Field {
	type Args<'a> = ConstantPoolRequiredArgs;

	fn read_options<R: std::io::Read + std::io::Seek>(
		reader: &mut R,
		endian: binrw::Endian,
		args: ConstantPoolRequiredArgs,
	) -> binrw::BinResult<Self> {
		let access_flags = u16::read_options(reader, endian, ())?;
		let name_index = u16::read_options(reader, endian, ())?;
		let descriptor_index = u16::read_options(reader, endian, ())?;
		let attributes_count = u16::read_options(reader, endian, ())?;

		let constant_pool = &(args.constant_pool);

		let mut attributes: Vec<Attribute> = Vec::new();
		for _ in 0..attributes_count {
			let attribute = Attribute::read_options(reader, endian, args.clone())?;
			attributes.push(attribute);
		}

		Ok(Field {
			access_flags,
			name_index,
			descriptor_index,
			attributes_count,
			attributes
		})
	}
}