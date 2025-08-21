use std::collections::BTreeMap;

use binrw::
	binrw;

use crate::class::constant_pool;

#[binrw]
#[brw(big)]
pub struct Attribute {
	name_index: u16,
	length: u32,
	#[br(count = length)]
	attribute_info: Vec<u8>
}

pub enum AttributeInfo {
	ConstantValue(ConstantValue),
	Code(Code),
	ExceptionHandler(ExceptionHandler),
	LineNumberTable(LineNumberTable),
	StackMapTable(StackMapTable),
	BootstrapMethods(BootstrapMethods),
	NestHost(NestHost),
	NestMembers(NestMembers),
	PermittedSubclasses(PermittedSubclasses),
}


struct AttributeReadArgs {
	constant_pool:  BTreeMap<u16, constant_pool::Item>,
}

// An implementation of a ConstantValue attribute (JVMS17 4.7.2)
#[binrw]
#[brw(big)]
#[derive(PartialEq, Debug)]
pub struct ConstantValue {
	pub attribute_name_index: u16,
	pub attribute_length: u32,
	pub constant_value_index: u16
}

#[binrw]
#[brw(big)]
pub struct Code {
	pub attribute_name_index: u16,
	pub attribute_length: u32,
	pub max_stack: u16,
	pub max_locals: u16,
	pub code_count: u32,
	#[br(count = code_count as u16)]
	pub code: Vec<u8>,
	pub handler_count: u16,
	#[br(count = handler_count as u16)]
	pub handlers: Vec<ExceptionHandler>,
	pub attributes_count: u16,
	#[br(count = attributes_count as u16)]
	pub attributes: Vec<Attribute>,
}

#[binrw]
#[brw(big)]
pub struct ExceptionHandler {
	pub start_pc: u16,
	pub end_pc: u16,
	pub handler_pc: u16,
	pub catch_type_index: u16,
}

#[binrw]
#[brw(big)]
pub struct LineNumberTable {
	pub attribute_name_index: u16,
	pub attribute_length: u32,
	pub lines_count: u32,
	#[br(count = lines_count as u16)]
	pub lines: Vec<Line>,
}

#[binrw]
#[brw(big)]
pub struct Line {
	pub start_pc: u16,
	pub line_number: u16,
}

#[binrw]
#[brw(big)]
pub struct StackMapTable {
	attribute_name_index: u8,
	attribute_length: u16,
	number_of_entries: u8,
	#[br(count = number_of_entries)]
	entries: Vec<StackMapFrame>
}

#[binrw]
#[brw(big)]
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
pub struct SameFrame {
	frame_type: u8,
}

#[binrw]
#[brw(big)]
#[br(assert((63..127).contains(&frame_type)))]
pub struct SameLocals1StackItemFrame {
	frame_type: u8,
	// verification_type_info: VerificationTypeInfo
}

#[binrw]
#[brw(big)]
#[br(assert(frame_type == 247))]
pub struct SameLocals1StackItemFrameExtended {
	frame_type: u8,
	offset_delta: u16,
	// verification_type_info: VerificationTypeInfo
}

#[binrw]
#[brw(big)]
#[br(assert((248..=250).contains(&frame_type)))]
pub struct ChopFrame {
	frame_type: u8,
	offset_delta: u16,
}

#[binrw]
#[brw(big)]
#[br(assert(frame_type == 251))]
pub struct SameFrameExtended {
	frame_type: u8,
	offset_delta: u16
}

#[binrw]
#[brw(big)]
#[br(assert((252..254).contains(&frame_type)))]
pub struct AppendFrame {
	frame_type: u8,
	offset_delta: u16
	// [br()]
	// locals: Vec<VerificationTypeInfo>
}

#[binrw]
#[brw(big)]
#[br(assert(frame_type == 255))]
pub struct FullFrame {
	frame_type: u8,
	offset_delta: u16,
	number_of_locals: u16,
	// [br(count = number_of_locals)]
	// locals: Vec<VerificationTypeInfo>
	number_of_stack_items: u16,
	// [br(count = number_of_stack_items)]
	// stack: Vec<VerificationTypeInfo>
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
pub struct BootstrapMethods {
	attribute_name_index: u16,
	attribute_length: u32,
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
pub struct NestHost {
	attribute_name_index: u16,
	attribute_length: u32,
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
pub struct NestMembers {
	attribute_name_index: u16,
	attribute_length: u32,
	number_of_classes: u16,
	#[br(count = number_of_classes)]
	classes: Vec<u16>
}

/// An implementation of PermittedSubclasses (JVMS17 4.7.31).
#[binrw]
#[brw(big)]
pub struct PermittedSubclasses {
	attribute_name_index: u16,
	attribute_length: u32,
	number_of_classes: u16,
	#[br(count = number_of_classes)]
	classes: Vec<u16>
}