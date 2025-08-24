use binrw::{
	binread, binrw, BinRead};

use crate::class::{
	attribute, constant_pool::ConstantPoolRequiredArgs, modified_utf8::ModifiedUtf8String, verification::*};

#[derive(Clone, Debug, PartialEq)]
pub struct Attribute {
	name_index: u16,
	length: u32,
	attribute_info: AttributeInfo,
}

impl BinRead for Attribute {
	type Args<'a> = ConstantPoolRequiredArgs;

	fn read_options<R: std::io::Read + std::io::Seek>(
		reader: &mut R,
		endian: binrw::Endian,
		args: ConstantPoolRequiredArgs,
	) -> binrw::BinResult<Self> {
		let name_index = u16::read_options(reader, endian, ())?;
		let length = u32::read_options(reader, endian, ())?;

		let attribute_type_constant = args.constant_pool.get_utf8(name_index).unwrap();

		let attribute_type = ModifiedUtf8String::new(attribute_type_constant.bytes).to_string();
		let attribute_info: Result<AttributeInfo, binrw::Error> = match attribute_type.as_str() {
			"BootstrapMethods" => Ok(AttributeInfo::BootstrapMethods(BootstrapMethods::read_options(reader, endian, ())?)),
			"Code" => Ok(AttributeInfo::Code(Code::read_options(reader, endian, args.clone())?)),
			"ConstantValue" => Ok(AttributeInfo::ConstantValue(ConstantValue::read_options(reader, endian, ())?)),
			"LineNumberTable" => Ok(AttributeInfo::LineNumberTable(LineNumberTable::read_options(reader, endian, ())?)),
			"NestHost" => Ok(AttributeInfo::NestHost(NestHost::read_options(reader, endian, ())?)),
			"NestMembers" => Ok(AttributeInfo::NestMembers(NestMembers::read_options(reader, endian, ())?)),
			"PermittedSubclass" => Ok(AttributeInfo::PermittedSubclasses(PermittedSubclasses::read_options(reader, endian, ())?)),
			"SourceFile" => Ok(AttributeInfo::SourceFile(SourceFile::read_options(reader, endian, ())?)),
			"StackMapTable" => Ok(AttributeInfo::StackMapTable(StackMapTable::read_options(reader, endian, ())?)),
			unrecognised => {
				let name = String::from(unrecognised).into_bytes();
				Ok(AttributeInfo::UnrecognisedAttribute(UnrecognisedAttribute { length: name.len() as u16, attribute_name: name }))
			}
		};
		return Ok(Attribute {
			name_index,
			length,
			attribute_info: attribute_info.unwrap(),
		});
	}
}

#[binread]
#[derive(Clone, Debug, PartialEq)]
pub enum AttributeInfo {
	BootstrapMethods(BootstrapMethods),
	Code(Code),
	ConstantValue(ConstantValue),
	LineNumberTable(LineNumberTable),
	NestHost(NestHost),
	NestMembers(NestMembers),
	PermittedSubclasses(PermittedSubclasses),
	SourceFile(SourceFile),
	StackMapTable(StackMapTable),
	UnrecognisedAttribute(UnrecognisedAttribute),
}

/// Dummy struct to represent unimplemented or unrecognised attributes.
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq)]
pub struct UnrecognisedAttribute {
	pub length: u16,
	#[br(count(length as u16))]
	pub attribute_name: Vec<u8>,
}

/// An implementation of a ConstantValue attribute (JVMS17 4.7.2)
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq)]
pub struct ConstantValue {
	pub constant_value_index: u16
}

#[derive(Clone, Debug, PartialEq)]
pub struct Code {
	pub max_stack: u16,
	pub max_locals: u16,
	pub code_length: u32,
	pub code: Vec<u8>,
	pub handler_count: u16,
	pub handlers: Vec<ExceptionHandler>,
	pub attributes_count: u16,
	pub attributes: Vec<Attribute>,
}

/// Implementation of an exception_table (JVMS17 4.7 p. 166)
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq)]
pub struct ExceptionHandler {
	pub start_pc: u16,
	pub end_pc: u16,
	pub handler_pc: u16,
	pub catch_type_index: u16,
}


impl BinRead for Code {
	type Args<'a> = ConstantPoolRequiredArgs;

	fn read_options<R: std::io::Read + std::io::Seek>(
		reader: &mut R,
		endian: binrw::Endian,
		args: ConstantPoolRequiredArgs,
	) -> binrw::BinResult<Self> {
		let max_stack = u16::read_options(reader, endian, ())?;
		let max_locals = u16::read_options(reader, endian, ())?;
		let code_length = u32::read_options(reader, endian, ())?;
		let mut code: Vec<u8> = Vec::new();
		for _ in 0..code_length {
			let byte = u8::read_options(reader, endian, ())?;
			code.push(byte);
		}
		let handler_count = u16::read_options(reader, endian, ())?;
		let mut handlers: Vec<ExceptionHandler> = Vec::new();
		for _ in 0..handler_count {
			let handler = ExceptionHandler::read_options(reader, endian, ())?;
			handlers.push(handler);
		}
		let attributes_count = u16::read_options(reader, endian, ())?;
		let mut attributes: Vec<Attribute> = Vec::new();
		for _ in 0..attributes_count {
			let attribute = Attribute::read_options(reader, endian, args.clone())?;
			attributes.push(attribute);
		}
		Ok(Code {
				max_stack,
				max_locals,
				code_length,
				code,
				handler_count,
				handlers,
				attributes_count,
				attributes,
		})
	}
}


#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq)]
pub struct LineNumberTable {
	pub table_length: u16,
	#[br(count = table_length)]
	pub lines: Vec<Line>,
}

#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq)]
pub struct Line {
	pub start_pc: u16,
	pub line_number: u16,
}

#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq)]
pub struct SourceFile {
	pub source_file_index: u16,
}

#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq)]
pub struct StackMapTable {
	number_of_entries: u8,
	#[br(count = number_of_entries)]
	entries: Vec<StackMapFrame>
}

#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq)]
pub enum StackMapFrame {
	SameFrame(SameFrame),
	SameLocals1StackItemFrame(SameLocals1StackItemFrame),
	SameLocals1StackItemFrameExtended(SameLocals1StackItemFrameExtended),
	ChopFrame(ChopFrame),
	SameFrameExtended(SameFrameExtended),
	AppendFrame(AppendFrame),
	FullFrame(FullFrame)
}

#[binrw]
#[brw(big)]
#[br(assert(frame_type <= 63))]
#[derive(Clone, Debug, PartialEq)]
pub struct SameFrame {
	frame_type: u8,
}

#[binrw]
#[brw(big)]
#[br(assert((63..127).contains(&frame_type)))]
#[derive(Clone, Debug, PartialEq)]
pub struct SameLocals1StackItemFrame {
	frame_type: u8,
	verification_type_info: VerificationTypeInfo
}

#[binrw]
#[brw(big)]
#[br(assert(frame_type == 247))]
#[derive(Clone, Debug, PartialEq)]
pub struct SameLocals1StackItemFrameExtended {
	frame_type: u8,
	offset_delta: u16,
	verification_type_info: VerificationTypeInfo
}

#[binrw]
#[brw(big)]
#[br(assert((248..=250).contains(&frame_type)))]
#[derive(Clone, Debug, PartialEq)]
pub struct ChopFrame {
	frame_type: u8,
	offset_delta: u16,
}

#[binrw]
#[brw(big)]
#[br(assert(frame_type == 251))]
#[derive(Clone, Debug, PartialEq)]
pub struct SameFrameExtended {
	frame_type: u8,
	offset_delta: u16
}

#[binrw]
#[brw(big)]
#[br(assert((252..254).contains(&frame_type)))]
#[derive(Clone, Debug, PartialEq)]
pub struct AppendFrame {
	frame_type: u8,
	offset_delta: u16,
	#[br(count = frame_type - 251)]
	locals: Vec<VerificationTypeInfo>
}

#[binrw]
#[brw(big)]
#[br(assert(frame_type == 255))]
#[derive(Clone, Debug, PartialEq)]
pub struct FullFrame {
	frame_type: u8,
	offset_delta: u16,
	number_of_locals: u16,
	#[br(count = number_of_locals)]
	locals: Vec<VerificationTypeInfo>,
	number_of_stack_items: u16,
	#[br(count = number_of_stack_items)]
	stack: Vec<VerificationTypeInfo>
}

/// An implementation of BootstrapMethods_attribute (JVMS17 4.723).
/// BootstrapMethods_attribute {
/// u2 attribute_name_index;
/// u4 attribute_length;
/// u2 num_bootstrap_methods;
/// { 	u2 bootstrap_method_ref;
/// 	u2 num_bootstrap_arguments;
/// 	u2 bootstrap_arguments[num_bootstrap_arguments];
/// } bootstrap_methods[num_bootstrap_methods];
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq)]
pub struct BootstrapMethods {
	num_bootstrap_methods: u16,
	#[br(count = num_bootstrap_methods)]
	bootstrap_methods: Vec<BootstrapMethodEntry>,
}

/// An implementation of BootstrapMethods_attribute.bootstrap_methods (JVMS17 4.7.23).
/// { 	u2 bootstrap_method_ref;
/// 	u2 num_bootstrap_arguments;
/// 	u2 bootstrap_arguments[num_bootstrap_arguments];
/// } bootstrap_methods[num_bootstrap_methods];
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq)]
pub struct BootstrapMethodEntry {
	bootstrap_method_ref: u16,
	num_bootstrap_arguments: u16,
	#[br(count = num_bootstrap_arguments)]
	bootstrap_arguments: Vec<u16>,
}

/// An implementation of NestHost (JVMS17 4.7.28).
/// NestHost_attribute {
///		u2 attribute_name_index;
///		u4 attribute_length;
///		u2 host_class_index;
///	}
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq)]
pub struct NestHost {
	host_class_index: u16,
}

/// An implementation of NestMembers (JVMS17 4.7.29).
///
/// NestMembers_attribute {
/// 	u2 attribute_name_index;
/// 	u4 attribute_length;
/// 	u2 number_of_classes;
/// 	u2 classes[number_of_classes];
/// }
///
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq)]
pub struct NestMembers {
	number_of_classes: u16,
	#[br(count = number_of_classes)]
	classes: Vec<u16>
}

/// An implementation of PermittedSubclasses (JVMS17 4.7.31).
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq)]
pub struct PermittedSubclasses {
	number_of_classes: u16,
	#[br(count = number_of_classes)]
	classes: Vec<u16>
}