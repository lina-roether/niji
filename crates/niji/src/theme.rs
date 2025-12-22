use std::{collections::HashMap, fmt, str::FromStr};

use anyhow::anyhow;
use niji_macros::IntoLua;
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::types::color::Color;

fn colored_square(color: Color) -> String {
	format!("\x1b[48;2;{};{};{}m   \x1b[0m", color.r, color.g, color.b)
}

#[derive(Debug, Clone, IntoLua, Serialize, Deserialize, PartialEq)]
pub struct Palette {
	pub pink: Color,
	pub red: Color,
	pub orange: Color,
	pub yellow: Color,
	pub green: Color,
	pub teal: Color,
	pub blue: Color,
	pub purple: Color,
	pub brown: Color,
	pub black: Color,
	pub white: Color,

	#[serde(flatten, default)]
	pub custom: HashMap<String, Color>,
}

impl Palette {
	pub fn get(&self, name: &str) -> anyhow::Result<Color> {
		let color = match name {
			"pink" => self.pink,
			"red" => self.red,
			"orange" => self.orange,
			"yellow" => self.yellow,
			"green" => self.green,
			"teal" => self.teal,
			"blue" => self.blue,
			"purple" => self.purple,
			"brown" => self.brown,
			"black" => self.black,
			"white" => self.white,
			_ => *self
				.custom
				.get(name)
				.ok_or_else(|| anyhow!("Undefined palette color: `{name}`"))?,
		};
		Ok(color)
	}
}

impl fmt::Display for Palette {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&colored_square(self.pink))?;
		f.write_str(&colored_square(self.red))?;
		f.write_str(&colored_square(self.orange))?;
		f.write_str(&colored_square(self.yellow))?;
		f.write_str(&colored_square(self.green))?;
		f.write_str(&colored_square(self.teal))?;
		f.write_str(&colored_square(self.blue))?;
		f.write_str(&colored_square(self.purple))?;
		f.write_str(&colored_square(self.brown))?;
		f.write_str(&colored_square(self.black))?;
		f.write_str(&colored_square(self.white))?;

		write!(f, " ")?;

		for color in self.custom.values() {
			f.write_str(&colored_square(*color))?;
		}
		Ok(())
	}
}

#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr)]
pub enum ColorRef {
	Named(String),
	Exact(Color),
}

impl ColorRef {
	fn named(name: &str) -> Self {
		Self::Named(name.to_string())
	}

	fn resolve(&self, palette: &Palette) -> anyhow::Result<Color> {
		match self {
			Self::Named(name) => palette.get(name),
			Self::Exact(color) => Ok(*color),
		}
	}
}

impl fmt::Display for ColorRef {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Named(name) => write!(f, "{name}"),
			Self::Exact(color) => write!(f, "{color}"),
		}
	}
}

impl FromStr for ColorRef {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let color_ref = if s.starts_with('#') {
			let color = Color::from_str(s)?;
			Self::Exact(color)
		} else {
			Self::Named(s.to_string())
		};
		Ok(color_ref)
	}
}

impl From<Color> for ColorRef {
	fn from(value: Color) -> Self {
		Self::Exact(value)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSpec {
	pub color: ColorRef,
	pub lighten: Option<f32>,
	pub darken: Option<f32>,
	pub shade: Option<f32>,
	pub alpha: Option<f32>,
}

impl ColorSpec {
	fn resolve(&self, palette: &Palette) -> anyhow::Result<Color> {
		let mut color = self.color.resolve(palette)?;
		if let Some(lightness) = self.shade {
			color = color.shade(lightness);
		}
		if let Some(amount) = self.lighten {
			color = color.lighten(amount);
		}
		if let Some(amount) = self.darken {
			color = color.darken(amount);
		}
		if let Some(alpha) = self.alpha {
			color = color.with_alpha(alpha);
		}
		Ok(color)
	}
}

impl From<ColorRef> for ColorSpec {
	fn from(color: ColorRef) -> Self {
		Self {
			color,
			lighten: None,
			darken: None,
			shade: None,
			alpha: None,
		}
	}
}

impl From<Color> for ColorSpec {
	fn from(color: Color) -> Self {
		Self::from(ColorRef::from(color))
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UiThemeSpec {
	pub background: ColorSpec,
	pub on_background: Option<ColorSpec>,
	pub surface: Option<ColorSpec>,
	pub on_surface: Option<ColorSpec>,
	pub border: Option<ColorSpec>,
	pub shadow: Option<ColorSpec>,

	pub text_light: ColorSpec,
	pub text_dark: ColorSpec,

	pub success: ColorSpec,
	pub on_success: Option<ColorSpec>,
	pub warning: ColorSpec,
	pub on_warning: Option<ColorSpec>,
	pub error: ColorSpec,
	pub on_error: Option<ColorSpec>,
}

impl Default for UiThemeSpec {
	fn default() -> Self {
		Self {
			background: ColorRef::named("black").into(),
			on_background: None,
			surface: None,
			on_surface: None,
			border: None,
			shadow: None,

			text_light: ColorRef::named("white").into(),
			text_dark: ColorRef::named("black").into(),

			success: ColorRef::named("green").into(),
			on_success: None,
			warning: ColorRef::named("yellow").into(),
			on_warning: None,
			error: ColorRef::named("red").into(),
			on_error: None,
		}
	}
}

impl UiThemeSpec {
	const DEFAULT_SHADOW_ALPHA: f32 = 0.3;
	const DEFAULT_SHADOW_DARKEN: f32 = 0.1;

	fn resolve(&self, palette: &Palette) -> anyhow::Result<UiTheme> {
		macro_rules! resolve_or {
			($spec:expr, $default:expr) => {
				Option::as_ref(&$spec)
					.map(|s| s.resolve(palette))
					.transpose()?
					.unwrap_or_else(|| $default)
			};
		}

		let background = self.background.resolve(palette)?;
		let kind = if background.is_light() {
			UiThemeKind::Light
		} else {
			UiThemeKind::Dark
		};
		let surface = resolve_or!(self.surface, background);
		let border = resolve_or!(self.border, background);
		let shadow = resolve_or!(self.border, {
			background
				.darken(Self::DEFAULT_SHADOW_DARKEN)
				.with_alpha(Self::DEFAULT_SHADOW_ALPHA)
		});

		let success = self.success.resolve(palette)?;
		let warning = self.warning.resolve(palette)?;
		let error = self.error.resolve(palette)?;

		let text_light = self.text_light.resolve(palette)?;
		let text_dark = self.text_dark.resolve(palette)?;

		let text_color_for = |color: Color| -> Color {
			if color.is_light() {
				text_dark
			} else {
				text_light
			}
		};

		let on_background = resolve_or!(self.on_background, text_color_for(background));
		let on_surface = resolve_or!(self.on_surface, text_color_for(surface));
		let on_success = resolve_or!(self.on_success, text_color_for(success));
		let on_warning = resolve_or!(self.on_warning, text_color_for(warning));
		let on_error = resolve_or!(self.on_error, text_color_for(error));

		Ok(UiTheme {
			kind,
			background,
			on_background,
			surface,
			on_surface,
			border,
			shadow,
			text_light,
			text_dark,
			success,
			on_success,
			warning,
			on_warning,
			error,
			on_error,
		})
	}
}

#[derive(Debug, Clone, IntoLua)]
pub struct UiTheme {
	pub kind: UiThemeKind,

	pub background: Color,
	pub on_background: Color,
	pub surface: Color,
	pub on_surface: Color,
	pub border: Color,
	pub shadow: Color,

	pub text_light: Color,
	pub text_dark: Color,

	pub success: Color,
	pub on_success: Color,
	pub warning: Color,
	pub on_warning: Color,
	pub error: Color,
	pub on_error: Color,
}

#[derive(Debug, Clone, Copy, IntoLua, PartialEq, Eq)]
#[lua_with("ToString::to_string")]
#[repr(u8)]
pub enum UiThemeKind {
	Light,
	Dark,
}

impl fmt::Display for UiThemeKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Dark => write!(f, "dark"),
			Self::Light => write!(f, "light"),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TerminalThemeSpec {
	pub darken_by: f32,

	pub default: ColorSpec,

	pub black: ColorSpec,
	pub red: ColorSpec,
	pub green: ColorSpec,
	pub yellow: ColorSpec,
	pub blue: ColorSpec,
	pub magenta: ColorSpec,
	pub cyan: ColorSpec,
	pub white: ColorSpec,

	pub bright_black: Option<ColorSpec>,
	pub dark_red: Option<ColorSpec>,
	pub dark_green: Option<ColorSpec>,
	pub dark_yellow: Option<ColorSpec>,
	pub dark_blue: Option<ColorSpec>,
	pub dark_magenta: Option<ColorSpec>,
	pub dark_cyan: Option<ColorSpec>,
	pub dark_white: Option<ColorSpec>,
}

impl Default for TerminalThemeSpec {
	fn default() -> Self {
		Self {
			darken_by: 0.2,
			default: ColorRef::named("white").into(),
			black: ColorRef::named("black").into(),
			red: ColorRef::named("red").into(),
			green: ColorRef::named("green").into(),
			yellow: ColorRef::named("yellow").into(),
			blue: ColorRef::named("blue").into(),
			magenta: ColorRef::named("purple").into(),
			cyan: ColorRef::named("teal").into(),
			white: ColorRef::named("white").into(),

			bright_black: None,
			dark_red: None,
			dark_green: None,
			dark_yellow: None,
			dark_blue: None,
			dark_magenta: None,
			dark_cyan: None,
			dark_white: None,
		}
	}
}

#[derive(Debug, Clone, IntoLua, PartialEq)]
struct TerminalTheme {
	pub default: Color,
	pub dark_black: Color,
	pub dark_red: Color,
	pub dark_green: Color,
	pub dark_yellow: Color,
	pub dark_blue: Color,
	pub dark_magenta: Color,
	pub dark_cyan: Color,
	pub dark_white: Color,
	pub brightblack: Color,
	pub brightred: Color,
	pub bright_green: Color,
	pub bright_yellow: Color,
	pub bright_blue: Color,
	pub bright_magenta: Color,
	pub bright_cyan: Color,
	pub bright_white: Color,
}
