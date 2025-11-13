use anyhow::{Context, anyhow};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{fmt, mem::transmute, str::FromStr};

use crate::utils::{lerp, oklch::OklchColor};

// If the channel is negative something went *very* wrong somewhere else
#[allow(clippy::cast_sign_loss)]
// The truncation is intended in this case; we only have a limited number of colors we can
// represent after all
#[allow(clippy::cast_possible_truncation)]
fn discretize(channel: f32) -> u8 {
	(channel * 255.0) as u8
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, SerializeDisplay, DeserializeFromStr)]
#[repr(C, align(4))]
pub struct Color {
	pub a: u8,
	pub b: u8,
	pub g: u8,
	pub r: u8,
}

impl Color {
	pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
		Self { a, b, g, r }
	}

	#[inline]
	pub fn alpha(self) -> f32 {
		f32::from(self.a) / 255.0
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
			discretize(out_alpha),
		)
	}

	pub fn mix(col1: Self, col2: Self) -> Self {
		Self::blend(col1, col2, 0.5)
	}

	pub fn with_alpha(self, alpha: f32) -> Self {
		Self::new_rgba(self.r, self.g, self.b, discretize(alpha))
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
		Self::from(0x00_00_00_ff)
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

impl FromStr for Color {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let Some(s) = s.strip_prefix('#') else {
			return Err(anyhow!(
				"Color strings must start with a '#'! (got \"{s}\")"
			));
		};

		let parsed_num: u32 =
			u32::from_str_radix(s, 16).context("\"{s}\" is not a valid hexadecimal number")?;

		let col: u32 = match s.len() {
			3 => (parsed_num << 20) | (parsed_num << 8) | 0xff,
			6 => parsed_num << 8 | 0xff,
			8 => parsed_num,
			_ => {
				return Err(anyhow!(
					"Colors must have 3, 6, or 8 hex digits! (got {})",
					s.len()
				));
			}
		};

		Ok(Self::from(col))
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn should_construct_from_int() {
		let col = Color::from(0x0a_0b_0c_0d);

		assert_eq!(col.r, 0x0a);
		assert_eq!(col.g, 0x0b);
		assert_eq!(col.b, 0x0c);
		assert_eq!(col.a, 0x0d);
	}

	#[test]
	fn should_display_correctly() {
		let col = Color::from(0xf1_02_34_ff);

		assert_eq!(col.to_string(), String::from("#f10234ff"));
	}

	#[test]
	fn should_parse_3_len() {
		assert_eq!(Color::from_str("#222").unwrap(), Color::from(0x22_22_22_ff));
	}

	#[test]
	fn should_parse_6_len() {
		assert_eq!(
			Color::from_str("#abcdef").unwrap(),
			Color::from(0xab_cd_ef_ff)
		);
	}

	#[test]
	fn should_parse_8_len() {
		assert_eq!(
			Color::from_str("#abcdef80").unwrap(),
			Color::from(0xab_cd_ef_80)
		);
	}
}
