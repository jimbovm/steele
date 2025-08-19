use std::collections::BTreeMap;

use binrw::binrw;

use crate::class::{constant_pool, field::Field};

/// A representation of a binary Java class file (JVMS17 4.1)
#[binrw]
#[brw(big)]
#[bw(magic = 0xCAFEBABEu32)]
pub struct RawClass {
	/// The magic number 0xCAFEBABEu32.
	pub magic: u32,
	/// The format minor version.
	pub minor_version: u16,
	/// The format major version.
	pub major_version: u16,
	/// The number of constant pool items.
	pub constant_pool_count: u16,
	#[br(count = (constant_pool_count - 2) as u16)]
	/// The constant pool (JVMS17 4.4).
	pub constant_pool: Vec<constant_pool::Item>,
	/// The access and modifier flags for the class.
	pub access_flags: u16,
	/// The constant pool index giving this class's name.
	pub this_class: u16,
	/// The constant pool index giving this class's superclass's name.
	pub super_class: u16,
	/// The number of interface entries.
	pub interfaces_count: u16,
	/// Constant pool index numbers giving superinterfaces of this class/interface.
	#[br(count = interfaces_count as u16)]
	pub interfaces: Vec<u16>,
	// The number of field entries.
	pub fields_count: u16,
	#[br(count = fields_count as u16)]
	// An array of field info structures (JVMS17 4.5).
	pub fields: Vec<Field>,
	// pub method_count: u16,
	// #[br(count = method_count as u16)]
	// pub methods: Vec<Method>,
	// pub attribute_count: u16,
	// #[br(count = attribute_count as u16)]
	// pub attributes: Vec<Attribute>,
}

/// Converts a raw constant pool to canonical form.
/// 
/// Due to a JVM design mistake a double or long stored in the constant pool table "invalidates" the next index.
/// For instance if a double or long is stored as constant pool entry number 10, counting from 1, then the
/// index number number used for the next entry will be 12, and index number 11 will be invalid (JVMS17 4.4.5).
fn canonical_constant_pool_from(raw_constant_pool: Vec<constant_pool::Item>) -> BTreeMap<u16, constant_pool::Item> {
	let mut canonical_constant_pool: BTreeMap<u16, constant_pool::Item> = BTreeMap::new();
	let mut index: u16 = 1;
	for item in raw_constant_pool.into_iter() {
		let increment: u16;
		match item {
			constant_pool::Item::Double(ref _d) => { increment = 2; },
			constant_pool::Item::Long(ref _l) => { increment = 2; },
			_ => { increment = 1; }
		}
		canonical_constant_pool.insert(index, item);
		index += increment;
	}
	return canonical_constant_pool;
}

#[cfg(test)]
mod tests {

use std::{collections::BTreeMap, fs::File};
use binrw::BinReaderExt;

use crate::class::{
	access, class::{canonical_constant_pool_from,
	RawClass,
}, constant_pool};

const CLASS_FILE_PATH: &str = "tests/resources/Sample.class";
	
	fn get_class() -> RawClass {
		let mut class_file = File::open(CLASS_FILE_PATH).expect("Couldn't access class file");
		let clazz: RawClass = class_file.read_be().expect("Couldn't read class file");
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
		assert!(clazz.access_flags & (access::ClassAccessPropertyFlags::Public as u16) == (access::ClassAccessPropertyFlags::Public as u16));
		assert!(clazz.access_flags & (access::ClassAccessPropertyFlags::Super as u16) == (access::ClassAccessPropertyFlags::Super as u16));
	}

	#[test]
	fn test_canonical_constant_pool() {
		let pool: BTreeMap<u16, constant_pool::Item> = canonical_constant_pool_from(get_class().constant_pool);
		for key in pool.keys() {
			let item = pool.get(key).unwrap();
			match item {
				constant_pool::Item::Double(_d) => { println!("[{}] Found a double", key); },
				constant_pool::Item::Long(_l) => { println!("[{}] Found a long", key); },
				_ => { println!("[{}] Found something else", key); }
			}
		}
		assert!(pool.contains_key(&13) == false);
	}

	#[test]
	fn test_fields() {
		let clazz = get_class();
		let canonical_pool = canonical_constant_pool_from(get_class().constant_pool);
		let field_1 = &clazz.fields[0];

		assert_eq!(field_1.name_index, 21);
		assert_eq!(field_1.descriptor_index, 22);
		assert_eq!(field_1.access_flags & access::FieldAccessPropertyFlags::Public as u16, access::FieldAccessPropertyFlags::Public as u16);
		assert_eq!(field_1.access_flags & access::FieldAccessPropertyFlags::Static as u16, access::FieldAccessPropertyFlags::Static as u16);
		assert_eq!(field_1.access_flags & access::FieldAccessPropertyFlags::Final as u16, access::FieldAccessPropertyFlags::Final as u16);
		assert_eq!(field_1.attributes_count, 1);

		assert_eq!(canonical_pool.get(&(field_1.name_index)).unwrap(), &constant_pool::Item::Utf8(constant_pool::Utf8 { length: b"STATIC_CONST_INT".len() as u16, bytes: b"STATIC_CONST_INT".to_vec() }));
		assert_eq!(canonical_pool.get(&(field_1.descriptor_index)).unwrap(), &constant_pool::Item::Utf8(constant_pool::Utf8 { length: 1u16, bytes: b"I".to_vec() }));
	}
}