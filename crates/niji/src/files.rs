use std::{
	fs,
	path::{Path, PathBuf},
};

use anyhow::Context;

use crate::utils::{
	fs::{find_dirs, find_files},
	xdg::XdgDirs,
};

#[derive(Debug)]
pub struct Files {
	config_file: PathBuf,
	state_file: PathBuf,
	output_dir: PathBuf,
	themes_dirs: Vec<PathBuf>,
	modules_dirs: Vec<PathBuf>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
	pub name: String,
	pub path: PathBuf,
}

impl Files {
	const PREFIX: &'static str = "niji";
	const CONFIG_FILE: &'static str = "config.toml";
	const STATE_FILE: &'static str = "state.toml";
	const THEMES_DIR: &'static str = "themes";
	const THEME_MAIN_FILE_NAME: &'static str = "theme.toml";
	const MODULES_DIR: &'static str = "modules";

	pub fn new(xdg: &XdgDirs) -> anyhow::Result<Self> {
		let config_dir = xdg.config_home.join(Self::PREFIX);
		let data_dir = xdg.data_home.join(Self::PREFIX);
		let state_dir = xdg.state_home.join(Self::PREFIX);

		init_dir(&config_dir)?;
		init_dir(&data_dir)?;
		init_dir(&state_dir)?;

		let config_file = config_dir.join(Self::CONFIG_FILE);
		let state_file = state_dir.join(Self::STATE_FILE);
		let custom_themes_dir = config_dir.join(Self::THEMES_DIR);
		let custom_modules_dir = config_dir.join(Self::MODULES_DIR);

		init_dir(&custom_themes_dir)?;
		init_dir(&custom_modules_dir)?;

		let mut themes_dirs = vec![custom_themes_dir];
		let mut modules_dirs = vec![custom_modules_dir];

		let data_dirs = xdg
			.data_dirs
			.iter()
			.map(|d| d.join(Self::PREFIX))
			.collect::<Vec<_>>();

		let builtin_themes_dirs = data_dirs.iter().map(|dir| dir.join(Self::THEMES_DIR));
		themes_dirs.extend(builtin_themes_dirs);

		let builtin_modules_dirs = data_dirs.iter().map(|dir| dir.join(Self::MODULES_DIR));
		modules_dirs.extend(builtin_modules_dirs);

		log::debug!("Config file location is {}", config_file.display());
		log::debug!("Output directory is {}", data_dir.display());
		log::debug!("State file is {}", state_file.display());
		log::debug!(
			"Theme directories are {}",
			themes_dirs
				.iter()
				.map(|p| p.to_string_lossy().into_owned())
				.collect::<Vec<_>>()
				.join(", ")
		);
		log::debug!(
			"Module directories are {}",
			modules_dirs
				.iter()
				.map(|p| p.to_string_lossy().into_owned())
				.collect::<Vec<_>>()
				.join(", ")
		);

		Ok(Self {
			config_file,
			output_dir: data_dir,
			state_file,
			themes_dirs,
			modules_dirs,
		})
	}

	#[inline]
	pub fn config_file(&self) -> &Path {
		&self.config_file
	}

	#[inline]
	pub fn state_file(&self) -> &Path {
		&self.state_file
	}

	#[inline]
	pub fn output_dir(&self) -> &Path {
		&self.output_dir
	}

	pub fn iter_themes(&self) -> impl Iterator<Item = Location> + '_ {
		let toplevel_themes = find_files(&self.themes_dirs).map(|f| Location {
			name: f
				.file_stem()
				.unwrap_or_default()
				.to_string_lossy()
				.to_string(),
			path: f,
		});
		let nested_themes = find_dirs(&self.themes_dirs)
			.map(|d| Location {
				name: d
					.file_name()
					.unwrap_or_default()
					.to_string_lossy()
					.to_string(),
				path: d.join(Self::THEME_MAIN_FILE_NAME),
			})
			.filter(|l| l.path.exists());
		toplevel_themes.chain(nested_themes)
	}

	pub fn iter_modules(&self) -> impl Iterator<Item = Location> + '_ {
		find_dirs(&self.modules_dirs).map(|d| Location {
			name: d
				.file_name()
				.unwrap_or_default()
				.to_string_lossy()
				.to_string(),
			path: d,
		})
	}
}

fn init_dir(dir: &Path) -> anyhow::Result<()> {
	log::debug!("Initializing {}...", dir.display());
	fs::create_dir_all(dir).context(format!("Failed to create {}", dir.display()))
}

#[cfg(test)]
mod tests {
	use tempfile::tempdir;

	use super::*;

	#[test]
	fn init() {
		let tempdir = tempdir().unwrap();
		let xdg_dirs = XdgDirs::in_tempdir(&tempdir);
		let files = Files::new(&xdg_dirs).unwrap();

		assert!(xdg_dirs.config_home.join("niji").exists());
		assert!(xdg_dirs.config_home.join("niji/themes").exists());
		assert!(xdg_dirs.config_home.join("niji/modules").exists());
		assert!(xdg_dirs.data_home.join("niji").exists());
		assert!(xdg_dirs.state_home.join("niji").exists());

		assert_eq!(
			files.config_file(),
			xdg_dirs.config_home.join("niji/config.toml")
		);
		assert_eq!(
			files.state_file(),
			xdg_dirs.state_home.join("niji/state.toml")
		);
		assert_eq!(files.output_dir(), xdg_dirs.data_home.join("niji"));
	}

	#[test]
	fn iter_theme_files() {
		let tempdir = tempdir().unwrap();
		let xdg_dirs = XdgDirs::in_tempdir(&tempdir);
		let files = Files::new(&xdg_dirs).unwrap();

		fs::create_dir_all(xdg_dirs.data_dirs[0].join("niji/themes")).unwrap();
		fs::write(xdg_dirs.data_dirs[0].join("niji/themes/test.toml"), "").unwrap();

		fs::write(xdg_dirs.config_home.join("niji/themes/test2.toml"), "").unwrap();

		let themes: Vec<Location> = files.iter_themes().collect();
		assert_eq!(
			themes,
			vec![
				Location {
					name: "test2".to_string(),
					path: xdg_dirs.config_home.join("niji/themes/test2.toml")
				},
				Location {
					name: "test".to_string(),
					path: xdg_dirs.data_dirs[0].join("niji/themes/test.toml")
				}
			]
		);
	}

	#[test]
	fn iter_theme_folders() {
		let tempdir = tempdir().unwrap();
		let xdg_dirs = XdgDirs::in_tempdir(&tempdir);
		let files = Files::new(&xdg_dirs).unwrap();

		fs::create_dir_all(xdg_dirs.data_dirs[0].join("niji/themes/test")).unwrap();
		fs::write(
			xdg_dirs.data_dirs[0].join("niji/themes/test/theme.toml"),
			"",
		)
		.unwrap();
		fs::create_dir(xdg_dirs.config_home.join("niji/themes/test2")).unwrap();
		fs::write(
			xdg_dirs.config_home.join("niji/themes/test2/theme.toml"),
			"",
		)
		.unwrap();

		let themes: Vec<Location> = files.iter_themes().collect();
		assert_eq!(
			themes,
			vec![
				Location {
					name: "test2".to_string(),
					path: xdg_dirs.config_home.join("niji/themes/test2/theme.toml")
				},
				Location {
					name: "test".to_string(),
					path: xdg_dirs.data_dirs[0].join("niji/themes/test/theme.toml")
				}
			]
		);
	}

	#[test]
	fn iter_empty_theme_folders() {
		let tempdir = tempdir().unwrap();
		let xdg_dirs = XdgDirs::in_tempdir(&tempdir);
		let files = Files::new(&xdg_dirs).unwrap();

		fs::create_dir_all(xdg_dirs.data_dirs[0].join("niji/themes/test")).unwrap();
		fs::create_dir(xdg_dirs.config_home.join("niji/themes/test2")).unwrap();

		let themes: Vec<Location> = files.iter_themes().collect();
		assert_eq!(themes, vec![]);
	}

	#[test]
	fn iter_module_folders() {
		let tempdir = tempdir().unwrap();
		let xdg_dirs = XdgDirs::in_tempdir(&tempdir);
		let files = Files::new(&xdg_dirs).unwrap();

		fs::create_dir_all(xdg_dirs.data_dirs[0].join("niji/modules/test")).unwrap();
		fs::create_dir(xdg_dirs.config_home.join("niji/modules/test2")).unwrap();

		let themes: Vec<Location> = files.iter_modules().collect();
		assert_eq!(
			themes,
			vec![
				Location {
					name: "test2".to_string(),
					path: xdg_dirs.config_home.join("niji/modules/test2")
				},
				Location {
					name: "test".to_string(),
					path: xdg_dirs.data_dirs[0].join("niji/modules/test")
				}
			]
		);
	}
}
