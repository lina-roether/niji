use std::fmt::Debug;

use strfmt::{strfmt_map, DisplayStr, Formatter};
use thiserror::Error;

pub enum FmtValue {
	String(String),
	Int(i64),
	Float(f64),
}

macro_rules! fmt_value_from_int {
    ($($ty:ty),*) => {
        $(
            impl From<$ty> for FmtValue {
                fn from(value: $ty) -> Self {
                    Self::Int(value as i64)
                }
            }
         )*
    };
}

macro_rules! fmt_value_from_float {
    ($($ty:ty),*) => {
        $(
            impl From<$ty> for FmtValue {
                fn from(value: $ty) -> Self {
                    Self::Float(value as f64)
                }
            }
         )*
    };
}

fmt_value_from_int!(i8, u8, i16, u16, i32, u32, i64, u64);
fmt_value_from_float!(f32, f64);

impl From<String> for FmtValue {
	fn from(value: String) -> Self {
		Self::String(value)
	}
}

impl DisplayStr for FmtValue {
	fn display_str(&self, f: &mut Formatter) -> strfmt::Result<()> {
		match self {
			Self::String(string) => string.display_str(f),
			Self::Int(int) => int.display_str(f),
			Self::Float(float) => float.display_str(f),
		}
	}
}

#[derive(Debug, Error)]
#[error("Failed to format {type_name}: {inner}")]
pub struct FmtError {
	type_name: &'static str,
	inner: strfmt::FmtError,
}

impl FmtError {
	fn new(type_name: &'static str, inner: strfmt::FmtError) -> Self {
		Self { type_name, inner }
	}
}

pub trait Format: Debug {
	fn type_name(&self) -> &'static str;

	fn default_fmtstr(&self) -> &'static str;

	fn get_placeholder(&self, name: &str) -> Option<FmtValue>;

	fn format(&self, fmtstr: Option<&str>) -> Result<String, FmtError> {
		let fmtstr = fmtstr.unwrap_or_else(|| self.default_fmtstr());
		let result = strfmt_map(fmtstr, |mut fmt| {
			let value = self
				.get_placeholder(fmt.key)
				.ok_or_else(|| strfmt::FmtError::KeyError(fmt.key.to_string()))?;
			value.display_str(&mut fmt)?;
			Ok(())
		})
		.map_err(|inner| FmtError::new(self.type_name(), inner))?;

		Ok(result)
	}
}
