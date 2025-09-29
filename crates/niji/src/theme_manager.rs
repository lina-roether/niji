use std::{collections::HashSet, fs, path::PathBuf, rc::Rc};

use anyhow::{anyhow, Context};
use log::debug;

use crate::{
	config::{self, Theme},
	files::Files,
};

pub struct ThemeManager {
	files: Rc<Files>,
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

	pub fn current_theme(&self) -> anyhow::Result<Theme> {
		if !self.files.current_theme_file().exists() {
			self.unset_theme()?;
		}

		let current_theme = fs::read_to_string(self.files.current_theme_file())
			.context("Failed to access theme state")?;

		if current_theme.is_empty() {
			return Err(anyhow!("No theme is selected"));
		}

		let theme: Option<Theme> = self.read_theme(&current_theme)?;
		let Some(mut theme) = theme else {
			return Err(anyhow!(
				"Current theme is \"{}\", but that theme doesn't exist!",
				current_theme
			));
		};

		theme.name = Some(current_theme);

		Ok(theme)
	}

	pub fn get_theme(&self, name: &str) -> anyhow::Result<Theme> {
		self.read_theme(name)?
			.ok_or_else(|| anyhow!("Theme \"{name}\" doesn't exist!"))
	}

	pub fn set_theme(&self, name: String) -> anyhow::Result<()> {
		if self.find_theme_path(&name).is_none() {
			return Err(anyhow!("Theme \"{name}\" doesn't exist!"));
		}
		fs::write(self.files.current_theme_file(), name).context("Failed to access theme state")?;
		Ok(())
	}

	pub fn unset_theme(&self) -> anyhow::Result<()> {
		fs::write(self.files.current_theme_file(), "").context("Failed to access theme state")?;
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

	fn read_theme(&self, name: &str) -> anyhow::Result<Option<Theme>> {
		let Some(path) = self.find_theme_path(name) else {
			return Ok(None);
		};

		debug!("Reading theme \"{name}\" from {}", path.display());

		let mut theme: Theme = config::read(path).context(format!("Couldn't read theme {name}"))?;

		theme.name = Some(name.to_string());

		Ok(Some(theme))
	}
}
