use std::{fs, rc::Rc};

use anyhow::{Context, anyhow};
use serde::{Deserialize, Serialize};

use crate::files::Files;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
struct State {
	theme: Option<String>,
	accent: Option<String>,
}

#[derive(Debug)]
pub struct StateManager {
	files: Rc<Files>,
	state: State,
}

impl StateManager {
	pub fn new(files: Rc<Files>) -> anyhow::Result<Self> {
		let mut state = State::default();
		if files.state_file().exists() {
			let state_str =
				fs::read_to_string(files.state_file()).context("Failed to read state file")?;
			match toml::from_str(&state_str) {
				Ok(s) => state = s,
				Err(err) => log::error!("Invalid state file: {err}\nResetting to default state."),
			}
		}

		Ok(Self { files, state })
	}

	pub fn get_theme(&self) -> anyhow::Result<&str> {
		self.state.theme.as_deref().ok_or(anyhow!(
			"No theme set; use `niji theme set <name>` to specify a theme."
		))
	}

	pub fn get_accent(&self) -> anyhow::Result<&str> {
		self.state.accent.as_deref().ok_or(anyhow!(
			"No accent color set; use `niji accent set <name>` to specify an accent color."
		))
	}

	pub fn set_theme(&mut self, theme: String) -> anyhow::Result<()> {
		self.state.theme = Some(theme);
		self.write_state()
	}

	pub fn set_accent(&mut self, accent: String) -> anyhow::Result<()> {
		self.state.accent = Some(accent);
		self.write_state()
	}

	pub fn unset_theme(&mut self) -> anyhow::Result<()> {
		self.state.theme = None;
		self.write_state()
	}

	pub fn unset_accent(&mut self) -> anyhow::Result<()> {
		self.state.accent = None;
		self.write_state()
	}

	fn write_state(&self) -> anyhow::Result<()> {
		let state_str = toml::to_string(&self.state)?;
		fs::write(self.files.state_file(), state_str).context("Failed to write state file")?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use tempfile::tempdir;

	use crate::utils::xdg::XdgDirs;

	use super::*;

	#[test]
	fn get_state() {
		let tempdir = tempdir().unwrap();
		let xdg = XdgDirs::in_tempdir(&tempdir);
		let files = Rc::new(Files::new(&xdg).unwrap());

		fs::write(
			xdg.state_home.join("niji/state.toml"),
			"theme = \"some_theme\"\naccent = \"some_color\"",
		)
		.unwrap();

		let state_manager = StateManager::new(files).unwrap();
		assert_eq!(state_manager.get_theme().unwrap(), "some_theme");
		assert_eq!(state_manager.get_accent().unwrap(), "some_color");
	}

	#[test]
	fn set_state() {
		let tempdir = tempdir().unwrap();
		let xdg = XdgDirs::in_tempdir(&tempdir);
		let files = Rc::new(Files::new(&xdg).unwrap());

		fs::write(
			xdg.state_home.join("niji/state.toml"),
			"theme = \"some_theme\"\naccent = \"some_color\"",
		)
		.unwrap();

		let mut state_manager = StateManager::new(files).unwrap();
		state_manager
			.set_theme("some_other_theme".to_string())
			.unwrap();
		state_manager
			.set_accent("some_other_color".to_string())
			.unwrap();

		assert_eq!(
			fs::read_to_string(xdg.state_home.join("niji/state.toml")).unwrap(),
			"theme = \"some_other_theme\"\naccent = \"some_other_color\"\n",
		);
	}

	#[test]
	fn get_initial_state() {
		let tempdir = tempdir().unwrap();
		let xdg = XdgDirs::in_tempdir(&tempdir);
		let state_manager = StateManager::new(Rc::new(Files::new(&xdg).unwrap())).unwrap();

		assert!(state_manager.get_theme().is_err());
		assert!(state_manager.get_accent().is_err());
	}

	#[test]
	fn set_initial_state() {
		let tempdir = tempdir().unwrap();
		let xdg = XdgDirs::in_tempdir(&tempdir);
		let mut state_manager = StateManager::new(Rc::new(Files::new(&xdg).unwrap())).unwrap();

		state_manager.set_theme("some_theme".to_string()).unwrap();
		state_manager.set_accent("some_color".to_string()).unwrap();

		assert_eq!(
			fs::read_to_string(xdg.state_home.join("niji/state.toml")).unwrap(),
			"theme = \"some_theme\"\naccent = \"some_color\"\n",
		);
	}

	#[test]
	fn unset_current_theme() {
		let tempdir = tempdir().unwrap();
		let xdg = XdgDirs::in_tempdir(&tempdir);
		let files = Rc::new(Files::new(&xdg).unwrap());
		fs::write(
			xdg.state_home.join("niji/state.toml"),
			"theme = \"some_theme\"\naccent = \"some_color\"",
		)
		.unwrap();

		let mut state_manager = StateManager::new(files).unwrap();
		state_manager.unset_theme().unwrap();
		state_manager.unset_accent().unwrap();

		assert_eq!(
			fs::read_to_string(xdg.state_home.join("niji/state.toml")).unwrap(),
			"",
		);
	}
}
