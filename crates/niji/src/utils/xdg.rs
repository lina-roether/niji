use std::{
	borrow::Cow,
	env::{self, split_paths},
	path::{Path, PathBuf},
};

use niji_macros::IntoLua;
use thiserror::Error;

fn map_path_vec(path_vec: &[PathBuf]) -> Vec<Cow<'_, str>> {
	path_vec
		.iter()
		.map(|p| p.to_string_lossy())
		.collect::<Vec<_>>()
}

fn map_path_option(path_option: &Option<PathBuf>) -> Option<Cow<'_, str>> {
	path_option.as_ref().map(|p| p.to_string_lossy())
}

#[derive(Debug, Error)]
pub enum Error {
	#[error("The HOME environment variable is not set")]
	NoHome,
}

#[derive(Debug, Clone, IntoLua)]
pub struct XdgDirs {
	#[lua_with("Path::to_string_lossy")]
	pub config_home: PathBuf,

	#[lua_with("Path::to_string_lossy")]
	pub data_home: PathBuf,

	#[lua_with("Path::to_string_lossy")]
	pub state_home: PathBuf,

	#[lua_with("Path::to_string_lossy")]
	pub cache_home: PathBuf,

	#[lua_with("map_path_option")]
	pub runtime_dir: Option<PathBuf>,

	#[lua_with("map_path_vec")]
	pub data_dirs: Vec<PathBuf>,

	#[lua_with("map_path_vec")]
	pub config_dirs: Vec<PathBuf>,
}

impl XdgDirs {
	pub fn new() -> Result<Self, Error> {
		let Some(home) = env::var_os("HOME").map(PathBuf::from) else {
			return Err(Error::NoHome);
		};

		Ok(Self {
			config_home: env::var_os("XDG_CONFIG_HOME")
				.map(PathBuf::from)
				.unwrap_or_else(|| home.join(".config")),
			data_home: env::var_os("XDG_DATA_HOME")
				.map(PathBuf::from)
				.unwrap_or_else(|| home.join(".local/share")),
			state_home: env::var_os("XDG_STATE_HOME")
				.map(PathBuf::from)
				.unwrap_or_else(|| home.join(".local/state")),
			cache_home: env::var_os("XDG_CACHE_HOME")
				.map(PathBuf::from)
				.unwrap_or_else(|| home.join(".cache")),
			runtime_dir: env::var_os("XDG_RUNTIME_DIR").map(PathBuf::from),
			data_dirs: env::var_os("XDG_DATA_DIRS")
				.map(|a| split_paths(&a).collect::<Vec<_>>())
				.unwrap_or_else(|| vec!["/usr/local/share".into(), "/usr/share".into()]),
			config_dirs: env::var_os("XDG_CONFIG_DIRS")
				.map(|a| split_paths(&a).collect::<Vec<_>>())
				.unwrap_or_else(|| vec!["/etc/xdg".into()]),
		})
	}
}
