use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{fmt, mem::transmute, num::ParseIntError, str::FromStr};
use thiserror::Error;

use crate::utils::{lerp, oklch::OklchColor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, SerializeDisplay, DeserializeFromStr)]
#[repr(C, align(4))]
pub struct Color {
	pub a: u8,
	pub b: u8,
	pub g: u8,
	pub r: u8
}

impl Color {
	pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
		Self { r, g, b, a }
	}

	#[inline]
	pub fn alpha(self) -> f32 {
		self.a as f32 / 255.0
	}

	pub fn lighten(self, amount: f32) -> Self {
		Self::from_oklch(self.into_oklch().lighten(amount), self.a)
	}

	pub fn darken(self, amount: f32) -> Self {
		Self::from_oklch(self.into_oklch().darken(amount), self.a)
	}

	pub fn shade(self, lightness: f32) -> Self {
		Self::from_oklch(self.into_oklch().shade(lightness), self.a)
	}

	pub fn blend(col1: Self, col2: Self, t: f32) -> Self {
		let alpha1 = col1.alpha();
		let alpha2 = col2.alpha();
		let out_alpha = lerp(alpha1, alpha2, t);

		Self::from_oklch(
			OklchColor::blend(col1.into_oklch(), col2.into_oklch(), t),
			f32::round(out_alpha * 255.0) as u8
		)
	}

	pub fn mix(col1: Self, col2: Self) -> Self {
		Self::blend(col1, col2, 0.5)
	}

	pub fn with_alpha(self, alpha: f32) -> Self {
		Self::new_rgba(self.r, self.g, self.b, (alpha * 255.0) as u8)
	}

	fn into_oklch(self) -> OklchColor {
		OklchColor::from_srgb(self.r, self.g, self.b)
	}

	fn from_oklch(color: OklchColor, a: u8) -> Self {
		let (r, g, b) = color.into_srgb();
		Self::new_rgba(r, g, b, a)
	}
}

impl Default for Color {
	fn default() -> Self {
		Self::from(0x000000ff)
	}
}

impl From<u32> for Color {
	fn from(value: u32) -> Self {
		unsafe { transmute(value) }
	}
}

impl From<Color> for u32 {
	fn from(value: Color) -> Self {
		unsafe { transmute(value) }
	}
}

impl fmt::Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "#{:08x}", u32::from(*self))
	}
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ColorParseError {
	#[error("Color strings must start with a '#'! (got \"{0}\")")]
	MissingHashtag(String),

	#[error("\"{0}\" is not a valid hexadecimal number: {1}")]
	InvalidHexNumber(String, ParseIntError),

	#[error("Colors must have 3, 6, or 8 hex digits! (got {0})")]
	InvalidLength(usize)
}

impl FromStr for Color {
	type Err = ColorParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let Some(s) = s.strip_prefix('#') else {
			return Err(ColorParseError::MissingHashtag(s.to_string()));
		};

		let parsed_num: u32 = u32::from_str_radix(s, 16)
			.map_err(|err| ColorParseError::InvalidHexNumber(s.to_string(), err))?;

		let col: u32 = match s.len() {
			3 => (parsed_num << 20) | (parsed_num << 8) | 0xff,
			6 => parsed_num << 8 | 0xff,
			8 => parsed_num,
			_ => return Err(ColorParseError::InvalidLength(s.len()))
		};

		Ok(Self::from(col))
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn should_construct_from_int() {
		let col = Color::from(0x0a0b0c0d);

		assert_eq!(col.r, 0x0a);
		assert_eq!(col.g, 0x0b);
		assert_eq!(col.b, 0x0c);
		assert_eq!(col.a, 0x0d);
	}

	#[test]
	fn should_display_correctly() {
		let col = Color::from(0xf10234ff);

		assert_eq!(col.to_string(), String::from("#f10234ff"))
	}

	#[test]
	fn should_parse_3_len() {
		assert_eq!(Color::from_str("#222"), Ok(Color::from(0x222222ff)))
	}

	#[test]
	fn should_parse_6_len() {
		assert_eq!(Color::from_str("#abcdef"), Ok(Color::from(0xabcdefff)));
	}

	#[test]
	fn should_parse_8_len() {
		assert_eq!(Color::from_str("#abcdef80"), Ok(Color::from(0xabcdef80)));
	}
}
