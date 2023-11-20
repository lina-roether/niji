use std::{collections::HashMap, rc::Rc};

use thiserror::Error;

use crate::{
	config::{self, Config, GeneralConfig, ModuleConfig, Theme},
	console,
	file_manager::{self, FileManager},
	files::{self, Files},
	module_manager::{self, ModuleManager, ModuleManagerInit},
	theme_manager::{self, NamedTheme, ThemeManager},
	utils::xdg::{self, XdgDirs}
};

#[derive(Debug, Error)]
#[error("{0}")]
pub enum Error {
	Xdg(#[from] xdg::Error),
	Files(#[from] files::Error),
	Config(#[from] config::Error),
	FileManager(#[from] file_manager::Error),
	ThemeManager(#[from] theme_manager::Error),
	ModuleManager(#[from] module_manager::Error)
}

pub struct NijiApp {
	xdg: Rc<XdgDirs>,
	files: Rc<Files>,
	config: Rc<Config>,
	file_manager: Rc<FileManager>,
	theme_manager: Rc<ThemeManager>,
	module_manager: Rc<ModuleManager>
}

impl NijiApp {
	pub fn init() -> Result<Self, Error> {
		let xdg = Rc::new(XdgDirs::new()?);
		let files = Rc::new(Files::new(&xdg)?);
		let config = Rc::<Config>::new(config::read(files.config_file())?);
		let file_manager = Rc::new(FileManager::new(Rc::clone(&files))?);
		let theme_manager = Rc::new(ThemeManager::new(Rc::clone(&files)));
		let module_manager = Rc::new(ModuleManager::new(ModuleManagerInit {
			xdg: Rc::clone(&xdg),
			files: Rc::clone(&files),
			config: Rc::clone(&config),
			file_manager: Rc::clone(&file_manager)
		})?);

		Ok(Self {
			xdg,
			files,
			config,
			file_manager,
			theme_manager,
			module_manager
		})
	}

	pub fn current_theme(&self) -> Result<Option<NamedTheme>, Error> {
		Ok(self.theme_manager.current_theme()?)
	}

	#[inline]
	pub fn general_config(&self) -> &GeneralConfig {
		&self.config.general
	}

	#[inline]
	pub fn config_for(&self, module: &str) -> Option<&ModuleConfig> {
		self.config.module_config.get(module)
	}

	pub fn apply_config(&self) -> Result<(), Error> {
		self.module_manager.configure(&self.config.general)?;
		Ok(())
	}

	pub fn apply_theme(&self) -> Result<(), Error> {
		let Some(theme) = self.current_theme()? else {
			console::warn!("No theme is currently set; theme application will be skipped");
			return Ok(());
		};
		self.module_manager.apply(&theme.values)?;
		Ok(())
	}

	pub fn reset_theme(&self) -> Result<(), Error> {
		Ok(self.theme_manager.reset_theme()?)
	}

	pub fn apply(&self) -> Result<(), Error> {
		self.apply_config()?;
		self.apply_theme()?;
		Ok(())
	}

	pub fn set_theme(&self, name: &str) -> Result<(), Error> {
		self.theme_manager.set_theme(name.to_string())?;
		self.apply_theme()?;
		Ok(())
	}
}
