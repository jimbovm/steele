use binrw::binrw;

use crate::class::constant_pool;

#[binrw]
#[brw(big)]
#[bw(magic = 0xCAFEBABEu32)]
pub struct Class {
	pub magic: u32,
	pub minor_version: u16,
	pub major_version: u16,
	#[bw(try_calc(u16::try_from(constant_pool.len())))]
	pub constant_pool_count: u16,
	#[br(count = (constant_pool_count - 2) as u16)]
	pub constant_pool: Vec<constant_pool::Item>,
	pub access_flags: u16,
	pub this_class: u16,
	pub super_class: u16,
	pub interfaces_count: u16
}

#[repr(u16)]
pub enum AccessFlags {
	Public = 0x0001,
	Final = 0x0010,
	Super = 0x0020,
	Interface = 0x2000,
	Abstract = 0x4000,
}

#[cfg(test)]
mod tests {

use crate::class::constant_pool;

use super::*;
use std::fs::File;
use binrw::BinReaderExt;

	const CLASS_FILE_PATH: &str = "tests/resources/Sample.class";
	
	fn get_class() -> Class {
		let mut class_file = File::open(CLASS_FILE_PATH).expect("Couldn't access class file `CLASS_FILE`");
		let clazz: Class = class_file.read_be().expect("Couldn't read class file `CLASS_FILE`");
		return clazz;
	}

	#[test]
	fn test_cafebabe() {
		const CAFEBABE: u32 = 0xCAFEBABEu32;
		assert_eq!(get_class().magic, CAFEBABE);
	}

	#[test]
	fn test_minor_version() {
		const MINOR_VERSION: u16 = 0;
		assert_eq!(get_class().minor_version, MINOR_VERSION);
	}

	#[test]
	fn test_major_version() {
		const MAJOR_VERSION: u16 = 61;
		assert_eq!(get_class().major_version, MAJOR_VERSION);
	}

	#[test]
	fn test_constant_pool() {
		let clazz = get_class();

		let expected_results = vec![
			constant_pool::Item::MethodRef(constant_pool::MethodRef { class_index: 2, name_and_type_index: 3 }),
			constant_pool::Item::Class(constant_pool::Class { index: 4 }),
			constant_pool::Item::NameAndType(constant_pool::NameAndType { name_index: 5, type_index: 6 }),
			constant_pool::Item::Utf8(constant_pool::Utf8 { length: 16, bytes: b"java/lang/Object".to_vec() }),
		];

		for i in 0..(expected_results.len()) {
			assert_eq!(clazz.constant_pool[i], expected_results[i]);
		}
	}

	#[test]
	fn test_access_flags() {
		let clazz = get_class();
		assert!(clazz.access_flags & (AccessFlags::Public as u16) == (AccessFlags::Public as u16));
		assert!(clazz.access_flags & (AccessFlags::Super as u16) == (AccessFlags::Super as u16));
	}
}