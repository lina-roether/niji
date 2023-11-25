use std::{collections::HashMap, fmt, fs, io, path::Path};

use niji_macros::IntoLua;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::types::color::Color;

#[derive(Debug, Clone, IntoLua, Serialize, Deserialize)]
#[lua_with("ToString::to_string")]
#[serde(rename_all = "lowercase")]
pub enum ColorScheme {
	Light,
	Dark
}

impl fmt::Display for ColorScheme {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Light => write!(f, "light"),
			Self::Dark => write!(f, "dark")
		}
	}
}

#[derive(Debug, Clone, IntoLua, Serialize, Deserialize)]
pub struct UiTheme {
	pub color_scheme: ColorScheme,
	pub background: Color,
	pub surface: Color,
	pub primary: Color,
	pub secondary: Color,
	pub border: Color,
	pub shadow: Color,
	pub text_background: Color,
	pub text_surface: Color,
	pub text_primary: Color,
	pub success: Color,
	pub info: Color,
	pub warning: Color,
	pub error: Color,
	pub text_success: Color,
	pub text_info: Color,
	pub text_warning: Color,
	pub text_error: Color
}

fn color_display(text: &str, bg_col: Color, fg_col: Color) -> String {
	format!(
		"\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m {text} \x1b[0m",
		bg_col.r, bg_col.g, bg_col.b, fg_col.r, fg_col.g, fg_col.b
	)
}

impl fmt::Display for UiTheme {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "Color scheme: {}", self.color_scheme)?;
		writeln!(
			f,
			"{}",
			color_display("Background", self.background, self.text_background)
		)?;
		writeln!(
			f,
			"{}",
			color_display("Surface", self.surface, self.text_surface)
		)?;
		writeln!(
			f,
			"{}",
			color_display("Primary", self.primary, self.text_primary)
		)?;
		writeln!(f, "Secondary: {}", colored_square(self.secondary))?;
		writeln!(f, "Border: {}", colored_square(self.border))?;

		writeln!(f)?;

		writeln!(f, "{}", color_display("Info", self.info, self.text_info))?;
		writeln!(
			f,
			"{}",
			color_display("Warning", self.warning, self.text_warning)
		)?;
		writeln!(f, "{}", color_display("Error", self.error, self.text_error))?;

		Ok(())
	}
}

#[derive(Debug, Clone, IntoLua, Serialize, Deserialize)]
pub struct Terminal {
	pub black: Color,
	pub red: Color,
	pub green: Color,
	pub yellow: Color,
	pub blue: Color,
	pub magenta: Color,
	pub cyan: Color,
	pub white: Color,
	pub bright_black: Color,
	pub bright_red: Color,
	pub bright_green: Color,
	pub bright_yellow: Color,
	pub bright_blue: Color,
	pub bright_magenta: Color,
	pub bright_cyan: Color,
	pub bright_white: Color
}

fn colored_square(color: Color) -> String {
	format!("\x1b[48;2;{};{};{}m   \x1b[0m", color.r, color.g, color.b)
}

impl fmt::Display for Terminal {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&colored_square(self.black))?;
		f.write_str(&colored_square(self.red))?;
		f.write_str(&colored_square(self.green))?;
		f.write_str(&colored_square(self.yellow))?;
		f.write_str(&colored_square(self.blue))?;
		f.write_str(&colored_square(self.magenta))?;
		f.write_str(&colored_square(self.cyan))?;
		f.write_str(&colored_square(self.white))?;

		writeln!(f)?;

		f.write_str(&colored_square(self.bright_black))?;
		f.write_str(&colored_square(self.bright_red))?;
		f.write_str(&colored_square(self.bright_green))?;
		f.write_str(&colored_square(self.bright_yellow))?;
		f.write_str(&colored_square(self.bright_blue))?;
		f.write_str(&colored_square(self.bright_magenta))?;
		f.write_str(&colored_square(self.bright_cyan))?;
		f.write_str(&colored_square(self.bright_white))?;

		Ok(())
	}
}

#[derive(Debug, Clone, IntoLua, Serialize, Deserialize)]
pub struct Theme {
	pub ui: UiTheme,
	pub terminal: Terminal
}

impl fmt::Display for Theme {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}\n{}", self.ui, self.terminal)
	}
}

#[derive(Debug, Default, Clone, IntoLua, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModuleConfigValue {
	#[default]
	Nil,
	String(String),
	Int(i64),
	Float(f64),
	Bool(bool),
	Vec(Vec<ModuleConfigValue>),
	Map(HashMap<String, ModuleConfigValue>)
}

pub type ModuleConfig = HashMap<String, ModuleConfigValue>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DisableReloads {
	#[default]
	None,

	All,

	#[serde(untagged)]
	Blacklist(Vec<String>)
}

impl DisableReloads {
	pub fn is_disabled(&self, name: &str) -> bool {
		match self {
			Self::None => false,
			Self::All => true,
			Self::Blacklist(blacklist) => blacklist.contains(&name.to_string())
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	pub modules: Vec<String>,

	#[serde(default)]
	pub disable_reloads: DisableReloads,

	#[serde(default)]
	pub global: ModuleConfig,

	#[serde(flatten)]
	pub module_config: HashMap<String, ModuleConfig>
}

#[derive(Debug, Error)]
pub enum Error {
	#[error("Failed to read {0}: {1}")]
	Read(String, io::Error),

	#[error("Invalid syntax in {0}: {1}")]
	Parse(String, Box<toml::de::Error>)
}

pub fn read<C, P>(path: P) -> Result<C, Error>
where
	C: for<'de> Deserialize<'de>,
	P: AsRef<Path>
{
	let config_str = fs::read_to_string(&path)
		.map_err(|e| Error::Read(path.as_ref().display().to_string(), e))?;
	let config = toml::from_str(&config_str)
		.map_err(|e| Error::Parse(path.as_ref().display().to_string(), Box::new(e)))?;
	Ok(config)
}
