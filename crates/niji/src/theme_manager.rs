use std::{collections::HashSet, fs, io, path::PathBuf, rc::Rc};

use log::debug;
use thiserror::Error;

use crate::{
	config::{self, Theme},
	files::Files
};

#[derive(Debug, Error)]
pub enum Error {
	#[error("Failed to access theme state: {0}")]
	AccessThemeState(io::Error),

	#[error("Couldn't read theme {0}; {1}")]
	ThemeRead(String, config::Error),

	#[error("Theme \"{0}\" doesn't exist!")]
	UnknownTheme(String),

	#[error("Current theme is \"{0}\", but that theme doesn't exist!")]
	UnknownCurrentTheme(String),

	#[error("No theme is selected")]
	NoThemeSelected
}

pub struct ThemeManager {
	files: Rc<Files>
}

impl ThemeManager {
	pub fn new(files: Rc<Files>) -> Self {
		Self { files }
	}

	pub fn list_themes(&self) -> Vec<String> {
		let mut themes = HashSet::new();
		for location in self.files.iter_themes() {
			if themes.insert(location.name.clone()) {
				debug!(
					"Found theme {} at {}",
					location.name,
					location.path.display()
				);
			}
		}
		themes.into_iter().collect()
	}

	pub fn current_theme(&self) -> Result<Theme, Error> {
		if !self.files.current_theme_file().exists() {
			self.unset_theme()?;
		}

		let current_theme =
			fs::read_to_string(self.files.current_theme_file()).map_err(Error::AccessThemeState)?;

		if current_theme.is_empty() {
			return Err(Error::NoThemeSelected);
		}

		let theme: Option<Theme> = self.read_theme(&current_theme)?;
		let Some(mut theme) = theme else {
			return Err(Error::UnknownCurrentTheme(current_theme));
		};

		theme.name = Some(current_theme);

		Ok(theme)
	}

	pub fn get_theme(&self, name: &str) -> Result<Theme, Error> {
		self.read_theme(name)?
			.ok_or_else(|| Error::UnknownTheme(name.to_string()))
	}

	pub fn set_theme(&self, name: String) -> Result<(), Error> {
		if self.find_theme_path(&name).is_none() {
			return Err(Error::UnknownTheme(name));
		}
		fs::write(self.files.current_theme_file(), name).map_err(Error::AccessThemeState)?;
		Ok(())
	}

	pub fn unset_theme(&self) -> Result<(), Error> {
		fs::write(self.files.current_theme_file(), "").map_err(Error::AccessThemeState)?;
		Ok(())
	}

	fn find_theme_path(&self, name: &str) -> Option<PathBuf> {
		let path = self
			.files
			.iter_themes()
			.find(|l| l.name == name)
			.map(|l| l.path)?;

		Some(path)
	}

	fn read_theme(&self, name: &str) -> Result<Option<Theme>, Error> {
		let Some(path) = self.find_theme_path(name) else {
			return Ok(None);
		};

		debug!("Reading theme \"{name}\" from {}", path.display());

		let mut theme: Theme =
			config::read(path).map_err(|e| Error::ThemeRead(name.to_string(), e))?;

		theme.name = Some(name.to_string());

		Ok(Some(theme))
	}
}
