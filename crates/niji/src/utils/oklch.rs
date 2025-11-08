use oklab::{oklab_to_srgb, srgb_to_oklab, Oklab, Rgb};

use crate::utils::lerp;

#[derive(Debug, Clone, Copy)]
pub struct OklchColor {
	l: f32,
	c: f32,
	h: f32,
}

impl OklchColor {
	pub const fn new(l: f32, c: f32, h: f32) -> Self {
		Self { l, c, h }
	}

	pub fn from_srgb(r: u8, g: u8, b: u8) -> Self {
		let lab = srgb_to_oklab(Rgb { r, g, b });

		// Convert to polar representation
		let c = f32::sqrt(lab.a.powi(2) + lab.b.powi(2));
		let h = f32::atan2(lab.b, lab.a);

		Self::new(lab.l, c, h)
	}

	pub fn into_srgb(self) -> (u8, u8, u8) {
		// Convert to cartesian representation
		let a = self.c * f32::cos(self.h);
		let b = self.c * f32::sin(self.h);
		let lab = Oklab { l: self.l, a, b };

		let rgb = oklab_to_srgb(lab);

		(rgb.r, rgb.g, rgb.b)
	}

	#[inline]
	pub fn lightness(self) -> f32 {
		self.l
	}

	#[inline]
	pub fn chroma(self) -> f32 {
		self.c
	}

	#[inline]
	pub fn hue(self) -> f32 {
		self.h
	}

	pub fn shade(self, lightness: f32) -> Self {
		let mut result = self;
		result.l = lightness;
		result
	}

	pub fn lighten(self, amount: f32) -> Self {
		self.shade(self.lightness() + amount)
	}

	pub fn darken(self, amount: f32) -> Self {
		self.shade(self.lightness() - amount)
	}

	pub fn blend(col1: Self, col2: Self, t: f32) -> Self {
		Self::new(
			lerp(col1.lightness(), col2.lightness(), t),
			lerp(col1.chroma(), col2.chroma(), t),
			lerp(col1.hue(), col2.hue(), t),
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn srgb_conversion() {
		let color = OklchColor::from_srgb(174, 49, 39);

		assert_eq!(color.into_srgb(), (174, 49, 39))
	}
}
