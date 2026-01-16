use std::{collections::HashSet, path::PathBuf, rc::Rc};

use anyhow::{Context, anyhow};
use log::debug;

use crate::{
	files::Files,
	theme::{self, Theme},
};

pub struct ThemeManager {
	files: Rc<Files>,
}

impl ThemeManager {
	pub fn new(files: Rc<Files>) -> Self {
		Self { files }
	}

	pub fn list_themes(&self) -> Vec<String> {
		let mut themes_set = HashSet::new();
		for location in self.files.iter_themes() {
			if themes_set.insert(location.name.clone()) {
				debug!(
					"Found theme {} at {}",
					location.name,
					location.path.display()
				);
			}
		}
		let mut themes: Vec<String> = themes_set.into_iter().collect();
		themes.sort();
		themes
	}

	pub fn get_theme(&self, name: &str) -> anyhow::Result<Theme> {
		self.read_theme(name)?
			.ok_or_else(|| anyhow!("Theme \"{name}\" doesn't exist!"))
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

		let theme: Theme = theme::read_theme(name.to_string(), path)
			.context(format!("Couldn't read theme {name}"))?;

		Ok(Some(theme))
	}
}

#[cfg(test)]
mod tests {
	use std::fs;

	use tempfile::tempdir;

	use crate::{
		theme::test_utils::{TEST_THEME_STR, test_theme},
		utils::xdg::XdgDirs,
	};

	use super::*;

	#[test]
	fn list_themes() {
		let tempdir = tempdir().unwrap();
		let xdg = XdgDirs::in_tempdir(&tempdir);
		let theme_manager = ThemeManager::new(Rc::new(Files::new(&xdg).unwrap()));

		fs::write(xdg.config_home.join("niji/themes/theme1.toml"), "").unwrap();
		fs::write(xdg.config_home.join("niji/themes/theme2.toml"), "").unwrap();
		fs::write(xdg.config_home.join("niji/themes/theme3.toml"), "").unwrap();

		assert_eq!(
			theme_manager.list_themes(),
			vec!["theme1", "theme2", "theme3"]
		);
	}

	#[test]
	fn get_theme() {
		let tempdir = tempdir().unwrap();
		let xdg = XdgDirs::in_tempdir(&tempdir);
		let theme_manager = ThemeManager::new(Rc::new(Files::new(&xdg).unwrap()));

		fs::write(
			xdg.config_home.join("niji/themes/test_theme.toml"),
			TEST_THEME_STR,
		)
		.unwrap();

		assert_eq!(theme_manager.get_theme("test_theme").unwrap(), test_theme());
	}

	#[test]
	fn get_nonexistent_theme() {
		let tempdir = tempdir().unwrap();
		let xdg = XdgDirs::in_tempdir(&tempdir);
		let theme_manager = ThemeManager::new(Rc::new(Files::new(&xdg).unwrap()));

		theme_manager.get_theme("theme1").unwrap_err();
	}
}
