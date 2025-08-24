use std::{collections::BTreeMap, 
	io::{Read, Seek},
	fmt::Display};

use binrw::{
	binrw, BinRead, BinReaderExt};
use strum::IntoEnumIterator;

use crate::class::{
	access::{self, ClassAccessPropertyFlags}, attribute::Attribute, constant_pool::{self, ConstantPool, ConstantPoolItem, ConstantPoolRequiredArgs, RawConstantPool}, field::Field, method::Method};

/// A high-level container for class data.
/// 
/// This struct abstracts out the class file into a higher-level format that is a lot easier to work with.
#[derive(Debug, Clone)]
pub struct Class {
	pub major_version: u16,
	pub minor_version: u16,
	pub constant_pool: BTreeMap<u16, ConstantPoolItem>,
	pub flags: Vec<ClassAccessPropertyFlags>,
	pub this_class: constant_pool::Class,
	pub super_class: constant_pool::Class,
	pub fields: Fields,
	pub methods: Methods,
	pub attributes: ClassAttributes,
}

impl Display for Class {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl Class {
	fn get_access_flags(flag_word: u16) -> Vec<ClassAccessPropertyFlags> {
		let mut flag_values: Vec<ClassAccessPropertyFlags> = Vec::new();
		for flag in access::ClassAccessPropertyFlags::iter() {
			if (flag_word & (flag as u16)) == flag as u16 {
				flag_values.push(flag);
			}
		}
		flag_values
	}

	pub fn new<T: Read + Seek>(mut stream: T) -> Class {
		let header: Header = stream.read_be().expect("Could not parse header");
		let raw_constant_pool: RawConstantPool = stream.read_be().expect("Could not parse constant pool");
		let constant_pool: ConstantPool = ConstantPool::from(raw_constant_pool);
		let parameters: Parameters = stream.read_be().expect("Could not parse parameters");
		let this_class = &constant_pool.get_class(parameters.this_class).unwrap();
		let super_class = &constant_pool.get_class(parameters.super_class).unwrap();
		let args = ConstantPoolRequiredArgs { constant_pool: constant_pool.clone() };
		let fields: Fields = Fields::read_options(&mut stream, binrw::Endian::Big,  args.clone()).expect("Could not parse fields");
		let methods: Methods = Methods::read_options(&mut stream, binrw::Endian::Big, args.clone()).expect("Could not parse methods");
		let attributes: ClassAttributes = ClassAttributes::read_options(&mut stream, binrw::Endian::Big, args.clone()).expect("Could not parse class attributes");

		Class {
			major_version: header.major_version,
			minor_version: header.minor_version,
			constant_pool: constant_pool.constants,
			flags: Self::get_access_flags(parameters.access_flags),
			this_class: *this_class,
			super_class: *super_class,
			fields: fields,
			methods: methods,
			attributes: attributes,
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
#[derive(Debug, Default)]
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

#[derive(Clone, Debug, Default)]
pub struct Fields {
	// The number of field entries.
	pub fields_count: u16,
	// An array of field info structures (JVMS17 4.5).
	pub fields: Vec<Field>,
}

impl BinRead for Fields {
	type Args<'a> = ConstantPoolRequiredArgs;

	fn read_options<R: Read + Seek>(
		reader: &mut R,
		endian: binrw::Endian,
		args: Self::Args<'_>,
	) -> binrw::BinResult<Self> {
		let fields_count = u16::read_be(reader)?;
		let mut fields: Vec<Field> = Vec::new();
		for _ in 0..fields_count {
			let field = Field::read_options(reader, endian, args.clone())?;
			fields.push(field);
		}
		Ok(Fields {
			fields_count,
			fields,
		})
	}
}

#[derive(Clone, Debug, Default)]
pub struct Methods {
	pub method_count: u16,
	pub methods: Vec<Method>,
}

impl BinRead for Methods {
	type Args<'a> = ConstantPoolRequiredArgs;

	fn read_options<R: Read + Seek>(
		reader: &mut R,
		endian: binrw::Endian,
		args: ConstantPoolRequiredArgs,
	) -> binrw::BinResult<Self> {
		let method_count = u16::read_be(reader)?;
		let mut methods: Vec<Method> = Vec::new();
		for _ in 0..method_count {
			let method = Method::read_options(reader, endian, args.clone())?;
			methods.push(method);
		}
		Ok(Methods {
			method_count,
			methods,
		})
	}
}

#[derive(Clone, Debug, Default)]
pub struct ClassAttributes {
	pub attribute_count: u16,
	pub attributes: Vec<Attribute>,
}

impl BinRead for ClassAttributes {
	type Args<'a> = constant_pool::ConstantPoolRequiredArgs;

	fn read_options<R: Read + Seek>(
		reader: &mut R,
		endian: binrw::Endian,
		args: constant_pool::ConstantPoolRequiredArgs,
	) -> binrw::BinResult<Self> {
		let attribute_count = u16::read_options(reader, endian, ())?;
		let mut attributes: Vec<Attribute> = Vec::new();
		for _ in 0..attribute_count {
			 let attribute = Attribute::read_options(reader, endian, args.clone())?;
			 attributes.push(attribute);
		}
		Ok(ClassAttributes {
			attribute_count,
			attributes,
		})
	}
}

#[cfg(test)]
mod tests {

use std::{collections::BTreeMap, fs::File};

use crate::class::{
	access::{ClassAccessPropertyFlags, FieldAccessPropertyFlags}, class::Class, constant_pool::{
		self,
		ConstantPoolItem}};

const CLASS_FILE_PATH: &str = "tests/resources/Sample.class";
	
	fn get_class() -> Class {
		let mut class_file = File::open(CLASS_FILE_PATH).expect("Couldn't access class file");
		let clazz = Class::new(&mut class_file);
		println!("{}", clazz);
		return clazz;
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
		let cases = vec![
			(1u16, ConstantPoolItem::MethodRef(constant_pool::MethodRef { class_index: 2, name_and_type_index: 3 })),
			(2u16, ConstantPoolItem::Class(constant_pool::Class { index: 4 })),
			(3u16, ConstantPoolItem::NameAndType(constant_pool::NameAndType { name_index: 5, type_index: 6 })),
			(4u16, ConstantPoolItem::Utf8(constant_pool::Utf8 { length: 16, bytes: b"java/lang/Object".to_vec() })),
		];

		let clazz = get_class();

		for (index, expected) in cases {
			assert_eq!(clazz.constant_pool.get(&index), Some(&expected));
		}
	}

	#[test]
	fn test_access_flags() {
		let clazz = get_class();
		assert!(clazz.flags.contains(&ClassAccessPropertyFlags::Public));
		assert!(clazz.flags.contains(&ClassAccessPropertyFlags::Super));
	}

	#[test]
	fn test_canonical_constant_pool() {
		let pool: BTreeMap<u16, constant_pool::ConstantPoolItem> = get_class().constant_pool;
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
		let canonical_pool = get_class().constant_pool;
		let field_1 = &clazz.fields.fields.get(0).unwrap();

		assert_eq!(field_1.name_index, 21);
		assert_eq!(field_1.descriptor_index, 22);
		assert_eq!(field_1.access_flags & FieldAccessPropertyFlags::Public as u16, FieldAccessPropertyFlags::Public as u16);
		assert_eq!(field_1.access_flags & FieldAccessPropertyFlags::Static as u16, FieldAccessPropertyFlags::Static as u16);
		assert_eq!(field_1.access_flags & FieldAccessPropertyFlags::Final as u16, FieldAccessPropertyFlags::Final as u16);
		assert_eq!(field_1.attributes_count, 1);

		assert_eq!(canonical_pool.get(&(field_1.name_index)).unwrap(), &constant_pool::ConstantPoolItem::Utf8(constant_pool::Utf8 { length: b"STATIC_CONST_INT".len() as u16, bytes: b"STATIC_CONST_INT".to_vec() }));
		assert_eq!(canonical_pool.get(&(field_1.descriptor_index)).unwrap(), &constant_pool::ConstantPoolItem::Utf8(constant_pool::Utf8 { length: 1u16, bytes: b"I".to_vec() }));
	}

	/* #[test]
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