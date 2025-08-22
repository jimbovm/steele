use std::
	io::{
		Read, Seek};

use binrw::{
	binrw,
	BinReaderExt};

use crate::class::{
	attribute::Attribute,
	constant_pool::{ConstantPool, RawConstantPool},
	field::Field,
	method::Method};

pub struct Class {
	pub header: Header,
	pub constants: ConstantPool,
	pub parameters: Parameters,
	// pub fields: Fields,
	// pub methods: Methods,
	// pub attributes: ClassAttributes,
}

impl Class {
	pub fn new<T: Read + Seek>(mut stream: T) -> Class {
		let header: Header = stream.read_be().expect("Could not parse header");
		let raw_constant_pool: RawConstantPool = stream.read_be().expect("Could not parse constant pool");
		let constants = ConstantPool::from(raw_constant_pool);
		let parameters = stream.read_be().expect("Could not parse parameters");
		Class {
			header,
			constants,
			parameters,
		}
	}
}

#[binrw]
#[brw(big)]
#[bw(magic = 0xCAFEBABEu32)]
#[derive(Default)]
pub struct Header {
	/// The magic number 0xCAFEBABEu32.
	pub magic: u32,
	/// The format minor version.
	pub minor_version: u16,
	/// The format major version.
	pub major_version: u16,
	
}

#[binrw]
#[brw(big)]
#[derive(Default)]
pub struct Parameters {
	pub access_flags: u16,
	/// The constant pool index giving this class's name.
	pub this_class: u16,
	/// The constant pool index giving this class's superclass's name.
	pub super_class: u16,
	/// The number of interface entries.
	pub interfaces_count: u16,
	/// Constant pool index numbers giving superinterfaces of this class/interface.
	#[br(count = interfaces_count)]
	pub interfaces: Vec<u16>,
}

#[binrw]
#[brw(big)]
#[derive(Default)]
pub struct Fields {
	// The number of field entries.
	pub fields_count: u16,
	#[br(count = fields_count)]
	// An array of field info structures (JVMS17 4.5).
	pub fields: Vec<Field>,
}

#[binrw]
#[brw(big)]
#[derive(Default)]
pub struct Methods {
	pub method_count: u16,
	#[br(count = method_count)]
	pub methods: Vec<Method>,
}

#[binrw]
#[brw(big)]
#[derive(Default)]
pub struct ClassAttributes {
	pub attribute_count: u16,
	#[br(count = attribute_count as u16)]
	pub attributes: Vec<Attribute>,
}

#[cfg(test)]
mod tests {

use std::fs::File;

use crate::class::{
	class::Class, 
	constant_pool::{
		self,
		ConstantPoolItem}};

const CLASS_FILE_PATH: &str = "tests/resources/Sample.class";
	
	fn get_class() -> Class {
		let mut class_file = File::open(CLASS_FILE_PATH).expect("Couldn't access class file");
		let clazz = Class::new(&mut class_file);
		return clazz;
	}

	#[test]
	fn test_cafebabe() {
		assert_eq!(get_class().header.magic, 0xCAFEBABEu32);
	}

	#[test]
	fn test_minor_version() {
		const MINOR_VERSION: u16 = 0;
		assert_eq!(get_class().header.minor_version, MINOR_VERSION);
	}

	#[test]
	fn test_major_version() {
		const MAJOR_VERSION: u16 = 61;
		assert_eq!(get_class().header.major_version, MAJOR_VERSION);
	}

	#[test]
	fn test_constant_pool() {
		let cases = vec![
			(1u16, ConstantPoolItem::MethodRef(constant_pool::MethodRef { class_index: 2, name_and_type_index: 3 })),
			(2u16, ConstantPoolItem::Class(constant_pool::Class { index: 4 })),
			(3u16, ConstantPoolItem::NameAndType(constant_pool::NameAndType { name_index: 5, type_index: 6 })),
			(4u16, ConstantPoolItem::Utf8(constant_pool::Utf8 { length: 16, bytes: b"java/lang/Object".to_vec() })),
		];

		let clazz = get_class();

		for (index, expected) in cases {
			assert_eq!(clazz.constants.constants.get(&index), Some(&expected));
			// println!("{:?}", clazz.constants);
		}
	}

	/* #[test]
	fn test_access_flags() {
		let clazz = get_class();
		assert!(clazz.access_flags & (access::ClassAccessPropertyFlags::Public as u16) == (access::ClassAccessPropertyFlags::Public as u16));
		assert!(clazz.access_flags & (access::ClassAccessPropertyFlags::Super as u16) == (access::ClassAccessPropertyFlags::Super as u16));
	}

	#[test]
	fn test_canonical_constant_pool() {
		let pool: BTreeMap<u16, constant_pool::ConstantPoolItem> = canonical_constant_pool_from(get_class().constant_pool);
		for key in pool.keys() {
			let item = pool.get(key).unwrap();
			match item {
				constant_pool::ConstantPoolItem::Double(_d) => { println!("[{}] Found a double", key); },
				constant_pool::ConstantPoolItem::Long(_l) => { println!("[{}] Found a long", key); },
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

		assert_eq!(canonical_pool.get(&(field_1.name_index)).unwrap(), &constant_pool::ConstantPoolItem::Utf8(constant_pool::Utf8 { length: b"STATIC_CONST_INT".len() as u16, bytes: b"STATIC_CONST_INT".to_vec() }));
		assert_eq!(canonical_pool.get(&(field_1.descriptor_index)).unwrap(), &constant_pool::ConstantPoolItem::Utf8(constant_pool::Utf8 { length: 1u16, bytes: b"I".to_vec() }));
	}

	#[test]
	fn test_methods() {
		let clazz = get_class();
		let canonical_pool = canonical_constant_pool_from(get_class().constant_pool);

		let cases = [
			(&clazz.methods[0], "<init>", "()V", vec![]),
			(&clazz.methods[1], "someMethod", "()Ljava/lang/String;", vec![]),
			(&clazz.methods[2], "doNothing", "()V", vec![]),
			(&clazz.methods[3], "newInstance", "()LSample;", vec![MethodAccessPropertyFlags::Public, MethodAccessPropertyFlags::Static]),
		];
		
		for (method, name, descriptor, flags) in cases.iter() {
			let method_name_constant = canonical_pool.get(&method.name_index).unwrap();
			let descriptor_constant = canonical_pool.get(&method.descriptor_index).unwrap();
			let method_name = if let constant_pool::ConstantPoolItem::Utf8(utf8) = method_name_constant {
					ModifiedUtf8String::new(utf8.bytes.clone()).to_string()
				}
				else {
					panic!("Expected method name index to point to a Utf8 constant, got something else");
				};
			let method_descriptor = if let constant_pool::ConstantPoolItem::Utf8(utf8) = descriptor_constant {
					ModifiedUtf8String::new(utf8.bytes.clone()).to_string()
				}
				else {
					panic!("Expected method descriptor index to point to a Utf8 constant, got something else");
				};

			assert_eq!(method_name, *name);
			assert_eq!(method_descriptor, *descriptor);
			for flag in flags {
				assert_eq!(method.access_flags & (*flag as u16), *flag as u16);
			}
		}
	} */
}