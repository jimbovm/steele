use std::{
	fmt::{
		self, Display, Formatter},
	io::{
		Cursor,
		Read}};

pub struct ModifiedUtf8String {
	pub bytes: Vec<u8>,
	parsed: String
}

impl ModifiedUtf8String {

	pub const WIDE_CHARACTER_PADDING: u8 = 0b11101101;

	pub fn new(bytes: Vec<u8>) -> Self {
		Self { bytes: bytes, parsed: String::new() }
	}

	/// See JVMS17 p. 4.4.7.
	fn is_ascii_range(byte: u8) -> bool {
		(byte > 0) && ((byte & 0b1000_0000) == 0)
	}

	/// See JVMS17 p. 4.4.7.
	fn is_null_or_80_to_7ff(byte: u8) -> bool {
		(byte & 0b111_00000) == 0b1100_0000
	}

	/// See JVMS17 p. 4.4.7.
	fn is_0800_to_ffff(byte: u8) -> bool {
		((byte & 0b11100000) == 0b11100000) && byte != Self::WIDE_CHARACTER_PADDING
	}

	/// See JVMS17 p. 4.4.7.
	fn get_null_or_80_to_7ff(x: u8, y: u8) -> char {
		let encoded_value: u32 = 
			u32::from((x & 0b00011111) << 6) +
			u32::from(y & 0b00111111);
		// is safe; can't exceed 0x02FF
		return char::from_u32(encoded_value).unwrap();

	}

	/// See JVMS17 p. 4.4.7.
	fn get_0800_to_ffff(x: u8, y: u8, z: u8) -> char {
		let encoded_value: u32 = 
			(u32::from(x & 0b0000_1111) << 12) +
			(u32::from(y & 0b0011_1111) << 6) +
			(u32::from(z & 0b0011_1111));
		let ch = char::from_u32(encoded_value).unwrap();
		ch
	}

	/// See JVMS17 p. 4.4.7.
	fn get_supplementary(_u: u8, v: u8, w: u8, _x: u8, y: u8, z: u8) -> char {
		let encoded_value: u32 = 
			0x00010000 as u32 +
			((u32::from((v & 0x0F) - 1) << 16)) +
			((u32::from(w & 0x3F) << 10)) +
			((u32::from(y & 0x0F) << 6)) +
			(u32::from(z & 0x3F)) as u32;
		let ch = char::from_u32(encoded_value).unwrap();
		ch
	}

	/// Parses a "modified UTF-8" string, the JVM's internal string representation, into regular UTF-8.
	/// Implements the algorithm shown in JVMS17 4.4.7.
	fn parse(&self) -> Result<String, std::io::Error> {
		if self.parsed != "" {
			return Ok(self.parsed.clone());
		}
		
		let mut output = String::new();
		let mut cursor: Cursor<Vec<u8>> = Cursor::new(self.bytes.clone());
		let mut buf: [u8; 1] = [0u8];

		loop {
			match cursor.read(&mut buf) {
				Ok(0) => break, // EOF, return UTF-8 string
				Ok(_) => { // buffer filled
					if Self::is_ascii_range(buf[0]) {
						// Code point in the range 0x0001..0x007F;
						output.push(buf[0] as char);
					}
					else if Self::is_null_or_80_to_7ff(buf[0]) {
						let x = buf[0].clone();	cursor.read_exact(&mut buf)?;
						let y = buf[0].clone();
						output.push(Self::get_null_or_80_to_7ff(x, y));
					}
					else if Self::is_0800_to_ffff(buf[0]) {
						let x = buf[0].clone();	cursor.read_exact(&mut buf)?;
						let y = buf[0].clone();	cursor.read_exact(&mut buf)?;
						let z = buf[0].clone();
						output.push(Self::get_0800_to_ffff(x, y, z));
					}
					else { // supplementary character
						let u = buf[0].clone(); cursor.read_exact(&mut buf)?;
						let v = buf[0].clone(); cursor.read_exact(&mut buf)?;
						let w = buf[0].clone(); cursor.read_exact(&mut buf)?;
						let x = buf[0].clone(); cursor.read_exact(&mut buf)?;
						let y = buf[0].clone(); cursor.read_exact(&mut buf)?;
						let z = buf[0].clone();
						output.push(Self::get_supplementary(u, v, w, x, y, z));
					}
				},
				Err(e) => return Err(e),
			}
		}
		Ok(output)
	}
}

impl Display for ModifiedUtf8String {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.parse().unwrap_or_default())
	}
}

mod tests {
	
	use crate::class::modified_utf8::ModifiedUtf8String;

	#[test]
	fn test_simple_ascii() {
		let input = b"abcde";
		let output = ModifiedUtf8String::new(input.to_vec()).to_string();
		assert_eq!(String::from_utf8_lossy(input), output);
	}

	#[test]
	fn test_lower_utf_8() {
		let input = ['%' as u8, 0b110_00010, 0b10_10_0011, 0b110_00010, 0b10_10_0011, 0b110_00010, 0b10_10_0011, '$' as u8];
		let output = ModifiedUtf8String::new(input.to_vec()).to_string();
		assert_eq!("%£££$", output);
	}

	#[test]
	fn test_upper_utf_8() {
		let input = [
			0b1110_0010, 0b10_000100, 0b10_111011,
			'M' as u8,
			'A' as u8,
			'R' as u8,
			'I' as u8,
			'O' as u8,
			0b1110_0010, 0b10_000100, 0b10_111011];
		let output = ModifiedUtf8String::new(input.to_vec()).to_string();
		assert_eq!("℻MARIO℻", output);
	}

	#[test]
	fn test_supplementary() {
		let input = [
			'$' as u8, '$' as u8,
			0b110_00010, 0b10_100011,
			ModifiedUtf8String::WIDE_CHARACTER_PADDING,
			0b0000_0001, 0b10_111100,
			ModifiedUtf8String::WIDE_CHARACTER_PADDING,
			0b1011_0010, 0b10_100001,
			0b110_00010, 0b10_100011,
			'$' as u8, '$' as u8];
		let output = ModifiedUtf8String::new(input.to_vec()).to_string();
		assert_eq!("$$£🂡£$$", output);
	}
}