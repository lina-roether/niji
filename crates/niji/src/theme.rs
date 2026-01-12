use std::{collections::HashMap, fmt, fs, path::Path, str::FromStr};

use anyhow::anyhow;
use niji_macros::IntoLua;
use serde::Deserialize;
use serde_with::DeserializeFromStr;

use crate::types::color::Color;

fn colored_square(color: Color) -> String {
	format!("\x1b[48;2;{};{};{}m   \x1b[0m", color.r, color.g, color.b)
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, DeserializeFromStr)]
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

#[derive(Debug, Clone, Deserialize)]
pub struct DerivedColor {
	color: ColorRef,
	lighten: Option<f32>,
	darken: Option<f32>,
	shade: Option<f32>,
	alpha: Option<f32>,
}

impl DerivedColor {
	fn new(color: ColorRef) -> Self {
		Self {
			color,
			lighten: None,
			darken: None,
			shade: None,
			alpha: None,
		}
	}

	fn named(name: &str) -> Self {
		Self::new(ColorRef::named(name))
	}

	fn lighten(mut self, amount: f32) -> Self {
		self.lighten = Some(amount);
		self
	}

	fn darken(mut self, amount: f32) -> Self {
		self.darken = Some(amount);
		self
	}

	fn shade(mut self, lightness: f32) -> Self {
		self.shade = Some(lightness);
		self
	}

	fn alpha(mut self, alpha: f32) -> Self {
		self.alpha = Some(alpha);
		self
	}
}

impl DerivedColor {
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

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ColorSpec {
	Color(ColorRef),
	Derived(DerivedColor),
}

impl ColorSpec {
	fn resolve(&self, palette: &Palette) -> anyhow::Result<Color> {
		match self {
			Self::Color(color) => color.resolve(palette),
			Self::Derived(derived) => derived.resolve(palette),
		}
	}
}

impl From<ColorRef> for ColorSpec {
	fn from(color: ColorRef) -> Self {
		Self::Color(color)
	}
}

impl From<Color> for ColorSpec {
	fn from(color: Color) -> Self {
		Self::from(ColorRef::from(color))
	}
}

impl From<DerivedColor> for ColorSpec {
	fn from(color: DerivedColor) -> Self {
		Self::Derived(color)
	}
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct UiThemeSpec {
	pub background: ColorSpec,
	pub surface: ColorSpec,
	pub border: Option<ColorSpec>,
	pub shadow: Option<ColorSpec>,

	pub text_light: ColorSpec,
	pub text_dark: ColorSpec,

	pub success: ColorSpec,
	pub warning: ColorSpec,
	pub error: ColorSpec,
}

impl Default for UiThemeSpec {
	fn default() -> Self {
		Self::default_dark()
	}
}

impl UiThemeSpec {
	fn default_dark() -> Self {
		Self {
			background: ColorRef::named("black").into(),
			surface: DerivedColor::named("black").lighten(0.1).into(),
			border: None,
			shadow: None,

			text_light: ColorRef::named("white").into(),
			text_dark: ColorRef::named("black").into(),

			success: ColorRef::named("green").into(),
			warning: ColorRef::named("yellow").into(),
			error: ColorRef::named("red").into(),
		}
	}

	fn default_light() -> Self {
		Self {
			background: ColorRef::named("white").into(),
			surface: DerivedColor::named("white").into(),
			border: None,
			shadow: None,

			text_light: ColorRef::named("white").into(),
			text_dark: ColorRef::named("black").into(),

			success: ColorRef::named("green").into(),
			warning: ColorRef::named("yellow").into(),
			error: ColorRef::named("red").into(),
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
		let surface = self.surface.resolve(palette)?;
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

		Ok(UiTheme {
			background,
			surface,
			border,
			shadow,
			text_light,
			text_dark,
			success,
			warning,
			error,
		})
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct UiTheme {
	pub background: Color,
	pub surface: Color,
	pub border: Color,
	pub shadow: Color,

	pub text_light: Color,
	pub text_dark: Color,

	pub success: Color,
	pub warning: Color,
	pub error: Color,
}

impl UiTheme {
	pub fn text_color_on(&self, background: Color) -> Color {
		let dark_contrast = self.text_dark.contrast(background);
		let light_contrast = self.text_light.contrast(background);

		if dark_contrast >= light_contrast {
			self.text_dark
		} else {
			self.text_light
		}
	}
}

fn color_display(text: &str, bg_col: Color, fg_col: Color) -> String {
	format!(
		"\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m {text} \x1b[0m",
		bg_col.r, bg_col.g, bg_col.b, fg_col.r, fg_col.g, fg_col.b
	)
}

impl fmt::Display for UiTheme {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(
			f,
			"{}",
			color_display(
				"Background",
				self.background,
				self.text_color_on(self.background)
			)
		)?;
		writeln!(
			f,
			"{}",
			color_display("Surface", self.surface, self.text_color_on(self.surface))
		)?;
		writeln!(f, "Border: {}", colored_square(self.border))?;

		writeln!(f)?;

		writeln!(
			f,
			"{}",
			color_display("Success", self.success, self.text_color_on(self.success))
		)?;
		writeln!(
			f,
			"{}",
			color_display("Warning", self.warning, self.text_color_on(self.warning))
		)?;
		writeln!(
			f,
			"{}",
			color_display("Error", self.error, self.text_color_on(self.error))
		)?;

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, IntoLua, Deserialize, PartialEq, Eq)]
#[lua_with("ToString::to_string")]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum ThemeKind {
	Light,
	Dark,
}

impl fmt::Display for ThemeKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Dark => write!(f, "dark"),
			Self::Light => write!(f, "light"),
		}
	}
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct TerminalThemeSpec {
	pub shade_difference: f32,

	pub default: ColorSpec,

	pub bright_black: Option<ColorSpec>,
	pub bright_red: Option<ColorSpec>,
	pub bright_green: Option<ColorSpec>,
	pub bright_yellow: Option<ColorSpec>,
	pub bright_blue: Option<ColorSpec>,
	pub bright_magenta: Option<ColorSpec>,
	pub bright_cyan: Option<ColorSpec>,
	pub bright_white: Option<ColorSpec>,

	pub dark_black: Option<ColorSpec>,
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
		Self::default_dark()
	}
}

impl TerminalThemeSpec {
	fn default_dark() -> Self {
		Self {
			shade_difference: 0.2,
			default: ColorRef::named("white").into(),
			dark_black: Some(ColorRef::named("black").into()),
			bright_red: Some(ColorRef::named("red").into()),
			bright_green: Some(ColorRef::named("green").into()),
			bright_yellow: Some(ColorRef::named("yellow").into()),
			bright_blue: Some(ColorRef::named("blue").into()),
			bright_magenta: Some(ColorRef::named("purple").into()),
			bright_cyan: Some(ColorRef::named("teal").into()),
			bright_white: Some(ColorRef::named("white").into()),

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

	fn default_light() -> Self {
		Self {
			shade_difference: 0.2,
			default: ColorRef::named("black").into(),
			dark_black: Some(ColorRef::named("black").into()),
			bright_red: Some(ColorRef::named("red").into()),
			bright_green: Some(ColorRef::named("green").into()),
			bright_yellow: Some(ColorRef::named("yellow").into()),
			bright_blue: Some(ColorRef::named("blue").into()),
			bright_magenta: Some(ColorRef::named("purple").into()),
			bright_cyan: Some(ColorRef::named("teal").into()),
			bright_white: Some(ColorRef::named("white").into()),

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

impl TerminalThemeSpec {
	fn resolve(&self, palette: &Palette) -> anyhow::Result<TerminalTheme> {
		macro_rules! resolve_pair {
			($name:expr, $bright:expr, $dark:expr) => {
				match (Option::as_ref(&$bright), Option::as_ref(&$dark)) {
					(Some(bright), Some(dark)) => {
						(bright.resolve(palette)?, dark.resolve(palette)?)
					}
					(Some(bright), None) => {
						let bright = bright.resolve(palette)?;
						(bright, bright.darken(self.shade_difference))
					}
					(None, Some(dark)) => {
						let dark = dark.resolve(palette)?;
						(dark.lighten(self.shade_difference), dark)
					}
					(None, None) => {
						return Err(anyhow!(
							"Must specify one of \"bright_{}\" or \"dark_{}\"",
							$name,
							$name
						))
					}
				}
			};
		}

		let default = self.default.resolve(palette)?;
		let (bright_black, dark_black) = resolve_pair!("black", self.bright_black, self.dark_black);
		let (bright_red, dark_red) = resolve_pair!("red", self.bright_red, self.dark_red);
		let (bright_green, dark_green) = resolve_pair!("green", self.bright_green, self.dark_green);
		let (bright_yellow, dark_yellow) =
			resolve_pair!("yellow", self.bright_yellow, self.dark_yellow);
		let (bright_blue, dark_blue) = resolve_pair!("bblue", self.bright_blue, self.dark_blue);
		let (bright_magenta, dark_magenta) =
			resolve_pair!("magenta", self.bright_magenta, self.dark_magenta);
		let (bright_cyan, dark_cyan) = resolve_pair!("cyan", self.bright_cyan, self.dark_cyan);
		let (bright_white, dark_white) = resolve_pair!("white", self.bright_white, self.dark_white);

		Ok(TerminalTheme {
			default,
			dark_black,
			dark_red,
			dark_green,
			dark_yellow,
			dark_blue,
			dark_magenta,
			dark_cyan,
			dark_white,
			bright_black,
			bright_red,
			bright_green,
			bright_yellow,
			bright_blue,
			bright_magenta,
			bright_cyan,
			bright_white,
		})
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct TerminalTheme {
	pub default: Color,
	pub dark_black: Color,
	pub dark_red: Color,
	pub dark_green: Color,
	pub dark_yellow: Color,
	pub dark_blue: Color,
	pub dark_magenta: Color,
	pub dark_cyan: Color,
	pub dark_white: Color,
	pub bright_black: Color,
	pub bright_red: Color,
	pub bright_green: Color,
	pub bright_yellow: Color,
	pub bright_blue: Color,
	pub bright_magenta: Color,
	pub bright_cyan: Color,
	pub bright_white: Color,
}

impl fmt::Display for TerminalTheme {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&colored_square(self.default))?;

		writeln!(f)?;

		f.write_str(&colored_square(self.dark_black))?;
		f.write_str(&colored_square(self.dark_red))?;
		f.write_str(&colored_square(self.dark_green))?;
		f.write_str(&colored_square(self.dark_yellow))?;
		f.write_str(&colored_square(self.dark_blue))?;
		f.write_str(&colored_square(self.dark_magenta))?;
		f.write_str(&colored_square(self.dark_cyan))?;
		f.write_str(&colored_square(self.dark_white))?;

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

#[derive(Debug, Clone, Deserialize)]
pub struct ThemeSpec {
	pub kind: ThemeKind,

	pub palette: Palette,

	#[serde(default)]
	pub ui: Option<UiThemeSpec>,

	#[serde(default)]
	pub terminal: Option<TerminalThemeSpec>,
}

impl ThemeSpec {
	fn resolve(self, name: String) -> anyhow::Result<Theme> {
		let ui = self
			.ui
			.unwrap_or_else(|| match self.kind {
				ThemeKind::Dark => UiThemeSpec::default_dark(),
				ThemeKind::Light => UiThemeSpec::default_light(),
			})
			.resolve(&self.palette)?;
		let terminal = self
			.terminal
			.unwrap_or_else(|| match self.kind {
				ThemeKind::Dark => TerminalThemeSpec::default_dark(),
				ThemeKind::Light => TerminalThemeSpec::default_light(),
			})
			.resolve(&self.palette)?;
		Ok(Theme {
			name,
			palette: self.palette,
			ui,
			terminal,
		})
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
	pub name: String,
	pub palette: Palette,
	pub ui: UiTheme,
	pub terminal: TerminalTheme,
}

impl fmt::Display for Theme {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"Theme '{}':\n{}\n{}\n{}",
			self.name, self.palette, self.ui, self.terminal
		)
	}
}

pub fn read_theme(name: String, path: impl AsRef<Path>) -> anyhow::Result<Theme> {
	let theme_str = fs::read_to_string(&path)?;
	let theme_spec: ThemeSpec = toml::from_str(&theme_str)?;
	let theme = theme_spec.resolve(name)?;
	Ok(theme)
}

#[cfg(test)]
pub mod test_utils {
	use super::*;

	pub const TEST_THEME_STR: &str = include_str!("test_theme.toml");

	pub fn test_theme() -> Theme {
		let theme_spec: ThemeSpec = toml::from_str(TEST_THEME_STR).unwrap();
		theme_spec.resolve("test_theme".to_string()).unwrap()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn palette_get_builtin() {
		let palette = Palette {
			pink: Color::from_str("#000000").unwrap(),
			red: Color::from_str("#111111").unwrap(),
			orange: Color::from_str("#222222").unwrap(),
			yellow: Color::from_str("#333333").unwrap(),
			green: Color::from_str("#444444").unwrap(),
			teal: Color::from_str("#555555").unwrap(),
			blue: Color::from_str("#666666").unwrap(),
			purple: Color::from_str("#777777").unwrap(),
			brown: Color::from_str("#888888").unwrap(),
			black: Color::from_str("#999999").unwrap(),
			white: Color::from_str("#aaaaaa").unwrap(),
			custom: HashMap::new(),
		};
		assert_eq!(palette.get("pink").unwrap().to_string(), "#000000ff");
		assert_eq!(palette.get("red").unwrap().to_string(), "#111111ff");
		assert_eq!(palette.get("orange").unwrap().to_string(), "#222222ff");
		assert_eq!(palette.get("yellow").unwrap().to_string(), "#333333ff");
		assert_eq!(palette.get("green").unwrap().to_string(), "#444444ff");
		assert_eq!(palette.get("teal").unwrap().to_string(), "#555555ff");
		assert_eq!(palette.get("blue").unwrap().to_string(), "#666666ff");
		assert_eq!(palette.get("purple").unwrap().to_string(), "#777777ff");
		assert_eq!(palette.get("brown").unwrap().to_string(), "#888888ff");
		assert_eq!(palette.get("black").unwrap().to_string(), "#999999ff");
		assert_eq!(palette.get("white").unwrap().to_string(), "#aaaaaaff");
		assert!(palette.get("dsfsdfgaqsdea").is_err());
		assert!(palette.get("custom_color").is_err());
	}

	#[test]
	fn palette_get_custom() {
		let mut palette = Palette {
			pink: Color::from_str("#000000").unwrap(),
			red: Color::from_str("#111111").unwrap(),
			orange: Color::from_str("#222222").unwrap(),
			yellow: Color::from_str("#333333").unwrap(),
			green: Color::from_str("#444444").unwrap(),
			teal: Color::from_str("#555555").unwrap(),
			blue: Color::from_str("#666666").unwrap(),
			purple: Color::from_str("#777777").unwrap(),
			brown: Color::from_str("#888888").unwrap(),
			black: Color::from_str("#999999").unwrap(),
			white: Color::from_str("#aaaaaa").unwrap(),
			custom: HashMap::new(),
		};
		palette.custom.insert(
			"custom_color".to_string(),
			Color::from_str("#bbbbbb").unwrap(),
		);
		assert_eq!(
			palette.get("custom_color").unwrap().to_string(),
			"#bbbbbbff"
		);
		assert!(palette.get("dsfsdfgaqsdea").is_err());
	}
}
