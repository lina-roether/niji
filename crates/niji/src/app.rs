use std::rc::Rc;

use anyhow::{Context, anyhow};

use crate::{
	config::{self, Config},
	files::Files,
	module_manager::{ApplyParams, ModuleManager, ModuleManagerInit},
	state_manager::StateManager,
	theme::{ColorRef, Theme},
	theme_manager::ThemeManager,
	utils::xdg::XdgDirs,
};

pub struct NijiApp {
	_xdg: Rc<XdgDirs>,
	_files: Rc<Files>,
	config: Rc<Config>,
	state_manager: StateManager,
	theme_manager: Rc<ThemeManager>,
	module_manager: Rc<ModuleManager>,
}

impl NijiApp {
	pub fn init() -> anyhow::Result<Self> {
		let xdg = Rc::new(XdgDirs::new()?);
		let files = Rc::new(Files::new(&xdg)?);
		let config = Rc::new(config::read_config(files.config_file())?);
		let state_manager = StateManager::new(Rc::clone(&files))?;
		let theme_manager = Rc::new(ThemeManager::new(Rc::clone(&files)));
		let module_manager = Rc::new(ModuleManager::new(ModuleManagerInit {
			xdg: Rc::clone(&xdg),
			files: Rc::clone(&files),
			config: Rc::clone(&config),
		})?);

		Ok(Self {
			_xdg: xdg,
			_files: files,
			config,
			state_manager,
			theme_manager,
			module_manager,
		})
	}

	pub fn get_current_theme(&self) -> anyhow::Result<Theme> {
		let theme_name = self.state_manager.get_theme()?;
		self.theme_manager
			.get_theme(theme_name)
			.context("Cannot get current theme")
	}

	pub fn get_current_accent(&self) -> anyhow::Result<ColorRef> {
		let name = self.state_manager.get_accent()?;
		Ok(ColorRef::named(name))
	}

	pub fn get_theme(&self, name: &str) -> anyhow::Result<Theme> {
		self.theme_manager.get_theme(name)
	}

	pub fn list_themes(&self) -> Vec<String> {
		self.theme_manager.list_themes()
	}

	pub fn apply(&self, params: &ApplyParams, modules: Option<&[String]>) -> anyhow::Result<()> {
		let theme = self.get_current_theme()?;
		let accent = self
			.get_current_accent()?
			.resolve(&theme.palette)
			.context("Invalid accent color set")?;

		self.module_manager
			.apply(&self.config, &theme, accent, params, modules)?;
		Ok(())
	}

	pub fn unset_current_theme(&mut self) -> anyhow::Result<()> {
		self.state_manager.unset_theme()
	}

	pub fn unset_current_accent(&mut self) -> anyhow::Result<()> {
		self.state_manager.unset_accent()
	}

	pub fn set_current_theme(&mut self, name: &str) -> anyhow::Result<()> {
		self.state_manager.set_theme(name.to_string())
	}

	pub fn set_current_accent(&mut self, color: ColorRef) -> anyhow::Result<()> {
		let ColorRef::Named(name) = color else {
			return Err(anyhow!(
				"Setting non-palette accent colors is not supported!"
			));
		};
		self.state_manager.set_accent(name)
	}
}
