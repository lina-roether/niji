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
