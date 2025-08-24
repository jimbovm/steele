/// Generate a custom BinRead implementation for use by methods and fields.
#[macro_export]
macro_rules! generate_pool_context_read {
	($for_type: ty, $target_type: ident) => {
		impl BinRead for $for_type {

			type Args<'a> = crate::class::constant_pool::ConstantPoolRequiredArgs;

			fn read_options<R: std::io::Read + std::io::Seek>(
				reader: &mut R,
				endian: binrw::Endian,
				args: crate::class::constant_pool::ConstantPoolRequiredArgs,
			) -> binrw::BinResult<Self> {
				let access_flags = u16::read_options(reader, endian, ())?;
				let name_index = u16::read_options(reader, endian, ())?;
				let descriptor_index = u16::read_options(reader, endian, ())?;
				let attributes_count = u16::read_options(reader, endian, ())?;

				let mut attributes: Vec<Attribute> = Vec::new();
				for _ in 0..attributes_count {
					let attribute = Attribute::read_options(reader, endian, args.clone())?;
					attributes.push(attribute);
				}

				Ok($target_type {
					access_flags,
					name_index,
					descriptor_index,
					attributes_count,
					attributes
				})
			}
		}
	};
}