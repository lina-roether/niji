use std::{fs, path::Path};

use anyhow::Context;
use niji_templates::{FmtValue, Template};

use crate::types::color::Color;

impl niji_templates::Format for Color {
	fn type_name(&self) -> &'static str {
		"color"
	}

	fn default_fmtstr(&self) -> &'static str {
		"#{rx}{gx}{bx}{ax}"
	}

	fn get_placeholder(&self, name: &str) -> Option<FmtValue> {
		match name {
			"r" => Some(self.r.into()),
			"g" => Some(self.g.into()),
			"b" => Some(self.b.into()),
			"a" => Some(self.a.into()),
			"rx" => Some(format!("{:02x}", self.r).into()),
			"gx" => Some(format!("{:02x}", self.g).into()),
			"bx" => Some(format!("{:02x}", self.b).into()),
			"ax" => Some(format!("{:02x}", self.a).into()),
			"rf" => Some((self.r as f32 / 255.0).into()),
			"gf" => Some((self.g as f32 / 255.0).into()),
			"bf" => Some((self.b as f32 / 255.0).into()),
			"af" => Some((self.a as f32 / 255.0).into()),
			_ => None,
		}
	}
}

pub fn load_template<P>(path: P) -> anyhow::Result<Template>
where
	P: AsRef<Path>,
{
	let path_name = path.as_ref().display().to_string();
	let source = fs::read_to_string(path).context(format!("Failed to load {}", path_name))?;

	Ok(source.parse()?)
}

#[cfg(test)]
mod tests {
	use niji_templates::Format;

	use super::*;

	#[test]
	fn format_color() {
		let color = Color::new_rgba(17, 51, 85, 255);
		let result = color
			.format(Some(
				"r={r}; g={g}; b={b}; a={a}; rx={rx}; gx={gx}; bx={bx}; ax={ax}; rf={rf}; \
				 gf={gf}; bf={bf}; af={af}",
			))
			.unwrap();
		assert_eq!(
			result,
			"r=17; g=51; b=85; a=255; rx=11; gx=33; bx=55; ax=ff; rf=0.06666667014360428; \
			 gf=0.20000000298023224; bf=0.3333333432674408; af=1"
		)
	}
}
