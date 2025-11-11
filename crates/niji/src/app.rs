use std::rc::Rc;

use crate::{
	config::{self, Config, Theme},
	files::Files,
	module_manager::{ModuleManager, ModuleManagerInit},
	theme_manager::ThemeManager,
	utils::xdg::XdgDirs,
};

pub struct NijiApp {
	_xdg: Rc<XdgDirs>,
	_files: Rc<Files>,
	config: Rc<Config>,
	theme_manager: Rc<ThemeManager>,
	module_manager: Rc<ModuleManager>,
}

impl NijiApp {
	pub fn init() -> anyhow::Result<Self> {
		let xdg = Rc::new(XdgDirs::new()?);
		let files = Rc::new(Files::new(&xdg)?);
		let config = Rc::<Config>::new(config::read(files.config_file())?);
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
			theme_manager,
			module_manager,
		})
	}

	pub fn get_current_theme(&self) -> anyhow::Result<Theme> {
		self.theme_manager.get_current_theme()
	}

	pub fn get_theme(&self, name: &str) -> anyhow::Result<Theme> {
		self.theme_manager.get_theme(name)
	}

	pub fn list_themes(&self) -> Vec<String> {
		self.theme_manager.list_themes()
	}

	pub fn apply(&self, reload: bool, modules: Option<&[String]>) -> anyhow::Result<()> {
		let theme = self.get_current_theme()?;
		self.module_manager
			.apply(&self.config, &theme, reload, modules)?;
		Ok(())
	}

	pub fn unset_current_theme(&self) -> anyhow::Result<()> {
		self.theme_manager.unset_current_theme()
	}

	pub fn set_current_theme(&self, name: &str) -> anyhow::Result<()> {
		self.theme_manager.set_current_theme(name.to_string())?;
		Ok(())
	}
}
