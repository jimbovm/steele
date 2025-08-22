use strum_macros::EnumIter;

/// An implementation of JVM class access and property flags (JVMS17 Table 4.1-B)
#[repr(u16)]
#[derive(PartialEq, Debug, Clone, Copy, EnumIter)]
pub enum ClassAccessPropertyFlags {
	Public = 0x0001,
	Final = 0x0010,
	Super = 0x0020,
	Interface = 0x2000,
	Abstract = 0x4000,
}

/// An implementation of JVM method access and property flags (JVMS17 Table 4.6-A)
#[repr(u16)]
#[derive(PartialEq, Debug, Clone, Copy, EnumIter)]
pub enum MethodAccessPropertyFlags {
	Public = 0x0001,
	Private = 0x0002,
	Protected = 0x0004,
	Static = 0x0008,
	Synchronized = 0x0020,
	Bridge = 0x0040,
	VarArgs = 0x0080,
	Native = 0x0100,
	Abstract = 0x0400,
	Strict = 0x8000,
	Synthetic = 0x1000,
}

/// An implementation of JVM field access and property flags (JVMS17 Table 4.5-A)
#[repr(u16)]
#[derive(PartialEq, Debug, Clone, Copy, EnumIter)]
pub enum FieldAccessPropertyFlags {
	Public = 0x0001,
	Private = 0x0002,
	Protected = 0x0004,
	Static = 0x0008,
	Final = 0x0010,
	Synchronized = 0x0020,
	Volatile = 0x0040,
	Transient = 0x0080,
	Synthetic = 0x0100,
	Enum = 0x4000,
}