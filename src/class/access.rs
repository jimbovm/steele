#[repr(u16)]
pub enum ClassAccessFlags {
	Public = 0x0001,
	Final = 0x0010,
	Super = 0x0020,
	Interface = 0x2000,
	Abstract = 0x4000,
}

#[repr(u16)]
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

#[repr(u16)]
pub enum FieldAccessPropertyFlags {
	Public = 0x0001,
	Private = 0x0002,
	Protected = 0x0004,
	Static = 0x0008,
	Synchronized = 0x0020,
	Volatile = 0x0040,
	Transient = 0x0080,
	Synthetic = 0x0100,
	Enum = 0x4000,
}