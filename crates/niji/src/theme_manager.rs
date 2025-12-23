use std::{collections::HashSet, fs, path::PathBuf, rc::Rc};

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

	pub fn get_current_theme(&self) -> anyhow::Result<Theme> {
		if !self.files.current_theme_file().exists() {
			self.unset_current_theme()?;
		}

		let current_theme = fs::read_to_string(self.files.current_theme_file())
			.context("Failed to access theme state")?;

		if current_theme.is_empty() {
			return Err(anyhow!("No theme is selected"));
		}

		let theme: Option<Theme> = self.read_theme(&current_theme)?;
		let Some(theme) = theme else {
			return Err(anyhow!(
				"Current theme is \"{current_theme}\", but that theme doesn't exist!",
			));
		};
		assert_eq!(theme.name, current_theme);

		Ok(theme)
	}

	pub fn get_theme(&self, name: &str) -> anyhow::Result<Theme> {
		self.read_theme(name)?
			.ok_or_else(|| anyhow!("Theme \"{name}\" doesn't exist!"))
	}

	pub fn set_current_theme(&self, name: String) -> anyhow::Result<()> {
		if self.find_theme_path(&name).is_none() {
			return Err(anyhow!("Theme \"{name}\" doesn't exist!"));
		}
		fs::write(self.files.current_theme_file(), name).context("Failed to access theme state")?;
		Ok(())
	}

	pub fn unset_current_theme(&self) -> anyhow::Result<()> {
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

		let theme: Theme =
			theme::read_theme(path).context(format!("Couldn't read theme {name}"))?;

		Ok(Some(theme))
	}
}

#[cfg(test)]
mod tests {
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
			toml::to_string(TEST_THEME_STR).unwrap(),
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

	#[test]
	fn get_current_theme() {
		let tempdir = tempdir().unwrap();
		let xdg = XdgDirs::in_tempdir(&tempdir);
		let theme_manager = ThemeManager::new(Rc::new(Files::new(&xdg).unwrap()));

		fs::write(xdg.state_home.join("niji/current_theme.txt"), "test_theme").unwrap();
		fs::write(
			xdg.config_home.join("niji/themes/test_theme.toml"),
			toml::to_string(TEST_THEME_STR).unwrap(),
		)
		.unwrap();

		assert_eq!(theme_manager.get_current_theme().unwrap(), test_theme());
	}

	#[test]
	fn get_nonexistent_current_theme() {
		let tempdir = tempdir().unwrap();
		let xdg = XdgDirs::in_tempdir(&tempdir);
		let theme_manager = ThemeManager::new(Rc::new(Files::new(&xdg).unwrap()));

		fs::write(xdg.state_home.join("niji/current_theme.txt"), "test_theme").unwrap();

		theme_manager.get_current_theme().unwrap_err();
	}

	#[test]
	fn get_unset_current_theme() {
		let tempdir = tempdir().unwrap();
		let xdg = XdgDirs::in_tempdir(&tempdir);
		let theme_manager = ThemeManager::new(Rc::new(Files::new(&xdg).unwrap()));

		fs::write(xdg.state_home.join("niji/current_theme.txt"), "").unwrap();
		fs::write(
			xdg.config_home.join("niji/themes/theme1.toml"),
			toml::to_string(TEST_THEME_STR).unwrap(),
		)
		.unwrap();

		theme_manager.get_current_theme().unwrap_err();
	}

	#[test]
	fn set_current_theme() {
		let tempdir = tempdir().unwrap();
		let xdg = XdgDirs::in_tempdir(&tempdir);
		let theme_manager = ThemeManager::new(Rc::new(Files::new(&xdg).unwrap()));

		fs::write(
			xdg.config_home.join("niji/themes/test_theme.toml"),
			toml::to_string(TEST_THEME_STR).unwrap(),
		)
		.unwrap();

		theme_manager
			.set_current_theme("test_theme".to_string())
			.unwrap();

		assert_eq!(
			fs::read_to_string(xdg.state_home.join("niji/current_theme.txt")).unwrap(),
			"test_theme"
		);
	}

	#[test]
	fn set_current_theme_to_nonexistent() {
		let tempdir = tempdir().unwrap();
		let xdg = XdgDirs::in_tempdir(&tempdir);
		let theme_manager = ThemeManager::new(Rc::new(Files::new(&xdg).unwrap()));

		fs::write(xdg.state_home.join("niji/current_theme.txt"), "").unwrap();
		theme_manager
			.set_current_theme("theme1".to_string())
			.unwrap_err();

		assert_eq!(
			fs::read_to_string(xdg.state_home.join("niji/current_theme.txt")).unwrap(),
			""
		);
	}

	#[test]
	fn unset_current_theme() {
		let tempdir = tempdir().unwrap();
		let xdg = XdgDirs::in_tempdir(&tempdir);
		let theme_manager = ThemeManager::new(Rc::new(Files::new(&xdg).unwrap()));

		fs::write(xdg.state_home.join("niji/current_theme.txt"), "aaaaaaa").unwrap();
		theme_manager.unset_current_theme().unwrap();

		assert_eq!(
			fs::read_to_string(xdg.state_home.join("niji/current_theme.txt")).unwrap(),
			""
		);
	}
}
