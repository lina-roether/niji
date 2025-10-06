use std::fmt::Debug;

use anyhow::Context;
use strfmt::{strfmt_map, DisplayStr, Formatter};

pub enum FmtValue {
	String(String),
	Int(i64),
	Float(f64)
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
			Self::Float(float) => float.display_str(f)
		}
	}
}

pub trait Format: Debug {
	fn type_name(&self) -> &'static str;

	fn default_fmtstr(&self) -> &'static str;

	fn get_placeholder(&self, name: &str) -> Option<FmtValue>;

	fn format(&self, fmtstr: Option<&str>) -> anyhow::Result<String> {
		let fmtstr = fmtstr.unwrap_or_else(|| self.default_fmtstr());
		let result = strfmt_map(fmtstr, |mut fmt| {
			let value = self
				.get_placeholder(fmt.key)
				.ok_or_else(|| strfmt::FmtError::KeyError(fmt.key.to_string()))?;
			value.display_str(&mut fmt)?;
			Ok(())
		})
		.context(format!("Failed to format {}", self.type_name()))?;

		Ok(result)
	}

	fn display(&self) -> anyhow::Result<String> {
		self.format(Some(self.default_fmtstr()))
	}
}

#[cfg(test)]
pub(crate) mod test_utils {
	use super::*;

	#[derive(Debug)]
	pub struct TestFormat;

	impl Format for TestFormat {
		fn type_name(&self) -> &'static str {
			"test"
		}

		fn default_fmtstr(&self) -> &'static str {
			"default: {string}"
		}

		fn get_placeholder(&self, name: &str) -> Option<FmtValue> {
			match name {
				"string" => Some(FmtValue::from("STRING VALUE :)".to_string())),
				"int" => Some(FmtValue::from(69)),
				"float" => Some(FmtValue::from(12.3)),
				_ => None
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test_utils::*;

	#[test]
	fn format_string() {
		let result = TestFormat.format(Some("## {string} ##")).unwrap();
		assert_eq!(result, "## STRING VALUE :) ##");
	}

	#[test]
	fn format_int() {
		let result = TestFormat.format(Some("## {int} ##")).unwrap();
		assert_eq!(result, "## 69 ##");
	}

	#[test]
	fn format_float() {
		let result = TestFormat.format(Some("## {float} ##")).unwrap();
		assert_eq!(result, "## 12.3 ##");
	}

	#[test]
	fn format_default() {
		let result = TestFormat.format(None).unwrap();
		assert_eq!(result, "default: STRING VALUE :)");
	}

	#[test]
	fn display() {
		let result = TestFormat.display().unwrap();
		assert_eq!(result, "default: STRING VALUE :)");
	}
}
