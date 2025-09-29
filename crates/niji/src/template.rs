use std::{fs, io, path::Path};

use niji_templates::{FmtValue, Template};
use thiserror::Error;

use crate::types::color::Color;

#[derive(Debug, Error)]
pub enum LoadError {
	#[error("Failed to load {0}: {1}")]
	Load(String, io::Error),

	#[error(transparent)]
	Parse(#[from] niji_templates::ParseError)
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct RenderError(#[from] niji_templates::RenderError);

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
			_ => None
		}
	}
}

pub fn load_template<P>(path: P) -> Result<Template, LoadError>
where
	P: AsRef<Path>
{
	let path_name = path.as_ref().display().to_string();
	let source = fs::read_to_string(path).map_err(|e| LoadError::Load(path_name, e))?;

	Ok(source.parse()?)
}
