#![allow(clippy::unused_self)]

use std::process::ExitCode;

use anyhow::anyhow;
use clap::Parser;
use log::LevelFilter;
use niji_console::ColorChoice;
mod syntax;

use crate::{
	app::NijiApp,
	cli::syntax::{
		Accent, AccentCommand, AccentGet, AccentParams, AccentSet, AccentUnset, Apply, ApplyArgs,
		GlobalArgs, Niji, NijiCommand, PaletteColor, Theme, ThemeCommand, ThemeGet, ThemeList,
		ThemePreview, ThemeSet, ThemeUnset, UpdateArgs,
	},
	module_manager::ApplyParams,
	theme::ColorRef,
};

#[must_use]
pub fn run() -> ExitCode {
	let niji = syntax::Niji::parse();
	if let Err(err) = niji.run() {
		log::error!("{err:?}");
		return ExitCode::FAILURE;
	}
	ExitCode::SUCCESS
}

impl Niji {
	fn run(&self) -> anyhow::Result<()> {
		let level = if self.global_args.quiet {
			LevelFilter::Off
		} else if self.global_args.verbose {
			LevelFilter::Debug
		} else {
			LevelFilter::Info
		};

		let color_choice = if self.global_args.no_color {
			ColorChoice::Never
		} else {
			ColorChoice::Auto
		};

		niji_console::init(level, color_choice);

		let mut app = NijiApp::init()?;

		match &self.command {
			NijiCommand::Apply(apply) => apply.run(&app),
			NijiCommand::Theme(theme) => theme.run(&mut app, &self.global_args),
			NijiCommand::Accent(accent) => accent.run(&mut app),
		}
	}
}

impl Apply {
	fn run(&self, app: &NijiApp) -> anyhow::Result<()> {
		let params = self.apply_args.apply_params();
		if self.modules.is_empty() {
			app.apply_default(&params)
		} else {
			app.apply(&params, &self.modules)
		}
	}
}

impl Theme {
	fn run(&self, app: &mut NijiApp, args: &GlobalArgs) -> anyhow::Result<()> {
		match &self.command {
			ThemeCommand::Get(get) => get.run(app),
			ThemeCommand::Set(set) => set.run(app),
			ThemeCommand::List(list) => list.run(app),
			ThemeCommand::Unset(unset) => unset.run(app),
			ThemeCommand::Preview(preview) => preview.run(app, args),
		}
	}
}

impl ThemeGet {
	fn run(&self, app: &NijiApp) -> anyhow::Result<()> {
		let theme = app.get_current_theme()?;
		niji_console::println!("{}", theme.name);
		Ok(())
	}
}

impl ThemePreview {
	fn run(&self, app: &NijiApp, args: &GlobalArgs) -> anyhow::Result<()> {
		if args.no_color {
			return Err(anyhow!(
				"Theme display is not supported in no-color mode. You can query the theme name by \
				 using `niji theme get`."
			));
		}

		let theme = match &self.name {
			Some(name) => app.get_theme(name)?,
			None => app.get_current_theme()?,
		};

		let accent_color = if let Some(color) = self.accent_args.accent_color() {
			color.resolve(&theme.palette)?
		} else {
			if !app.is_accent_set() {
				return Err(anyhow!(
					"No accent color set. Consider using `niji preview <name> --accent <color>` to specify which accent color to use for the preview."
				));
			}
			app.get_current_accent()?.resolve(&theme.palette)?
		};

		niji_console::println!("Theme \"{}\":", theme.name);
		niji_console::println!();
		niji_console::println!("Accent: {}", accent_color.preview());
		niji_console::println!();
		niji_console::println!("{theme}");
		Ok(())
	}
}

impl ThemeSet {
	fn run(&self, app: &mut NijiApp) -> anyhow::Result<()> {
		app.set_current_theme(&self.name)?;
		if let Some(accent) = self.accent_args.accent_color() {
			app.set_current_accent(accent)?;
		}

		if let Some(params) = self.update_args.apply_params() {
			if !app.is_accent_set() {
				return Err(anyhow!(
					"Cannot apply changes since no accent color is set. Consider using `niji theme set <name> --accent <color>` to set an accent color along with the theme, or use `niji theme set --no-apply <name>` to skip this step."
				));
			}
			app.apply_default(&params)?;
		}

		Ok(())
	}
}

impl ThemeList {
	fn run(&self, app: &NijiApp) -> anyhow::Result<()> {
		let themes = app.list_themes();
		if themes.is_empty() {
			return Err(anyhow!("No usable themes were found"));
		}

		for theme in themes {
			niji_console::println!("{theme}");
		}

		Ok(())
	}
}

impl ThemeUnset {
	fn run(&self, app: &mut NijiApp) -> anyhow::Result<()> {
		app.unset_current_theme()
	}
}

impl AccentParams {
	pub fn accent_color(&self) -> Option<ColorRef> {
		self.accent.map(ColorRef::from)
	}
}

impl Accent {
	fn run(&self, app: &mut NijiApp) -> anyhow::Result<()> {
		match &self.command {
			AccentCommand::Get(get) => get.run(app),
			AccentCommand::Set(set) => set.run(app),
			AccentCommand::Unset(unset) => unset.run(app),
		}
	}
}

impl AccentGet {
	fn run(&self, app: &NijiApp) -> anyhow::Result<()> {
		let color = app.get_current_accent()?;
		niji_console::println!("{color}");
		Ok(())
	}
}

impl AccentSet {
	fn run(&self, app: &mut NijiApp) -> anyhow::Result<()> {
		app.set_current_accent(self.color.into())?;
		if let Some(params) = self.update_args.apply_params() {
			if !app.is_theme_set() {
				return Err(anyhow!(
					"Cannot apply changes since no theme is set. Consider setting a theme using `niji theme set <name>`, or use `niji accent set --no-apply <color>` to skip this step."
				));
			}
			app.apply_default(&params)?;
		}
		Ok(())
	}
}

impl AccentUnset {
	fn run(&self, app: &mut NijiApp) -> anyhow::Result<()> {
		app.unset_current_accent()
	}
}

impl From<PaletteColor> for ColorRef {
	fn from(value: PaletteColor) -> Self {
		match value {
			PaletteColor::Pink => ColorRef::named("pink"),
			PaletteColor::Red => ColorRef::named("red"),
			PaletteColor::Orange => ColorRef::named("orange"),
			PaletteColor::Yellow => ColorRef::named("yellow"),
			PaletteColor::Green => ColorRef::named("green"),
			PaletteColor::Teal => ColorRef::named("teal"),
			PaletteColor::Blue => ColorRef::named("blue"),
			PaletteColor::Purple => ColorRef::named("purple"),
			PaletteColor::Black => ColorRef::named("black"),
			PaletteColor::White => ColorRef::named("white"),
		}
	}
}

impl UpdateArgs {
	fn apply_params(&self) -> Option<ApplyParams> {
		if self.no_apply {
			return None;
		}
		Some(self.apply_args.apply_params())
	}
}

impl ApplyArgs {
	fn apply_params(&self) -> ApplyParams {
		ApplyParams {
			reload: !self.no_reload,
			check_deps: !self.ignore_deps,
		}
	}
}
