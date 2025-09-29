use std::{
	fs, io,
	path::{Path, PathBuf},
};

use thiserror::Error;

use crate::utils::{
	fs::{find_dirs, find_files},
	xdg::XdgDirs,
};

#[derive(Debug, Error)]
pub enum Error {
	#[error("Failed to create {0}: {1}")]
	CreationFailed(String, io::Error),
}

#[derive(Debug)]
pub struct Files {
	config_file: PathBuf,
	current_theme_file: PathBuf,
	managed_files_file: PathBuf,
	output_dir: PathBuf,
	themes_dirs: Vec<PathBuf>,
	modules_dirs: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Location {
	pub name: String,
	pub path: PathBuf,
}

impl Files {
	const PREFIX: &'static str = "niji";
	const CONFIG_FILE: &'static str = "config.toml";
	const CURRENT_THEME_FILE: &'static str = "current_theme.txt";
	const MANAGED_FILES_FILE: &'static str = "managed_files.csv";
	const THEMES_DIR: &'static str = "themes";
	const THEME_MAIN_FILE_NAME: &'static str = "theme.toml";
	const MODULES_DIR: &'static str = "modules";

	pub fn new(xdg: &XdgDirs) -> Result<Self, Error> {
		let config_dir = xdg.config_home.join(Self::PREFIX);
		let data_dir = xdg.data_home.join(Self::PREFIX);
		let state_dir = xdg.state_home.join(Self::PREFIX);

		init_dir(&config_dir)?;
		init_dir(&data_dir)?;
		init_dir(&state_dir)?;

		let config_file = config_dir.join(Self::CONFIG_FILE);
		let current_theme_file = state_dir.join(Self::CURRENT_THEME_FILE);
		let managed_files_file = state_dir.join(Self::MANAGED_FILES_FILE);
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

		Ok(Self {
			config_file,
			output_dir: data_dir,
			current_theme_file,
			managed_files_file,
			themes_dirs,
			modules_dirs,
		})
	}

	#[inline]
	pub fn config_file(&self) -> &Path {
		&self.config_file
	}

	#[inline]
	pub fn current_theme_file(&self) -> &Path {
		&self.current_theme_file
	}

	#[inline]
	pub fn managed_files_file(&self) -> &Path {
		&self.managed_files_file
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

fn init_dir(dir: &Path) -> Result<(), Error> {
	fs::create_dir_all(dir).map_err(|err| Error::CreationFailed(dir.display().to_string(), err))
}
