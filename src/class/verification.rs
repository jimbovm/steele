use binrw::binrw;

/// An implementation of verification_type_info (JVMS17 4.74).
#[binrw]
#[derive(Clone, Debug, PartialEq)]
pub enum VerificationTypeInfo {
	#[br(magic(0u8))]
	TopVariableInfo(TopVariableInfo),
	#[br(magic(1u8))]
	IntegerVariableInfo(IntegerVariableInfo),
	#[br(magic(2u8))]
	FloatVariableInfo(FloatVariableInfo),
	#[br(magic(3u8))]
	DoubleVariableInfo(DoubleVariableInfo),
	#[br(magic(4u8))]
	LongVariableInfo(LongVariableInfo),
	#[br(magic(5u8))]
	NullVariableInfo(NullVariableInfo),
	#[br(magic(6u8))]
	UninitializedThisVariableInfo(UninitializedThisVariableInfo),
	#[br(magic(7u8))]
	ObjectVariableInfo(ObjectVariableInfo),
	#[br(magic(8u8))]
	UninitializedVariableInfo(UninitializedVariableInfo)
}

/// See JVMS17 4.74 p. 119.
#[binrw]
#[derive(Clone, Debug, PartialEq)]
struct TopVariableInfo {
	tag: u8,
}

/// See JVMS17 4.74 p. 120.
#[binrw]
#[derive(Clone, Debug, PartialEq)]
struct IntegerVariableInfo {
	tag: u8,
}

/// See JVMS17 4.74 p. 120.
#[binrw]
#[derive(Clone, Debug, PartialEq)]
struct FloatVariableInfo {
	tag: u8,
}

/// See JVMS17 4.74 p. 121.
#[binrw]
#[derive(Clone, Debug, PartialEq)]
struct DoubleVariableInfo {
	tag: u8,
}

/// See JVMS17 4.74 p. 121.
#[binrw]
#[derive(Clone, Debug, PartialEq)]
struct LongVariableInfo {
	tag: u8,
}

/// See JVMS17 4.74 p. 120.
#[binrw]
#[derive(Clone, Debug, PartialEq)]
struct NullVariableInfo {
	tag: u8,
}

/// See JVMS17 4.74 p. 120.
#[binrw]
#[derive(Clone, Debug, PartialEq)]
struct UninitializedThisVariableInfo {
	tag: u8,
}

/// See JVMS17 4.74 p. 120.
#[binrw]
#[derive(Clone, Debug, PartialEq)]
struct ObjectVariableInfo {
	tag: u8,
	constant_pool_index: u16,
}

/// See JVMS17 4.74 p. 120.
#[binrw]
#[derive(Clone, Debug, PartialEq)]
struct UninitializedVariableInfo {
	tag: u8,
	offset: u16,
}