use std::{
	fs::File,
	io::{self, BufRead, BufReader},
	path::Path,
	process::{Command, Stdio}
};

use log::debug;
use thiserror::Error;

use crate::{
	config::{ModuleConfig, Theme},
	lua::runtime::{LuaModule, LuaRuntime}
};

#[derive(Debug, Error)]
pub enum LoadError {
	#[error("Failed to read module files: {0}")]
	FileReadErr(#[from] io::Error),

	#[error("Missing dependency: {0}")]
	MissingDependency(String),

	#[error("{0}")]
	LuaErr(#[from] mlua::Error)
}

#[derive(Debug, Error)]
pub enum ExecError {
	#[error("Module is missing an apply function")]
	NoApply,

	#[error(transparent)]
	LuaErr(#[from] mlua::Error)
}

pub struct Module<'lua>(LuaModule<'lua>);

impl<'lua> Module<'lua> {
	const DEPS_FILE: &'static str = "deps.txt";

	pub fn load(runtime: &'lua LuaRuntime, path: &Path) -> Result<Self, LoadError> {
		Self::check_dependencies(path)?;
		let module = runtime.load_lua_module(path)?;
		Ok(Self(module))
	}

	pub fn can_reload(&self) -> bool {
		self.0.has_function("reload").unwrap_or(false)
	}

	pub fn apply(&self, config: ModuleConfig, theme: Theme) -> Result<(), ExecError> {
		if !self.0.has_function("apply")? {
			return Err(ExecError::NoApply);
		}

		Ok(self.0.call("apply", (config, theme))?)
	}

	pub fn reload(&self, config: ModuleConfig) -> Result<(), ExecError> {
		Ok(self.0.call("reload", config)?)
	}

	fn check_dependencies(path: &Path) -> Result<(), LoadError> {
		let deps_file = path.join(Self::DEPS_FILE);
		if !deps_file.exists() {
			return Ok(());
		}

		let reader = BufReader::new(File::open(deps_file)?).lines();
		for dependency in reader {
			Self::check_dependency(&dependency?)?;
		}

		Ok(())
	}

	fn check_dependency(program: &str) -> Result<(), LoadError> {
		debug!("Checking for module dependency {program}...");

		let output = Command::new("/bin/which")
			.arg(program)
			.stdout(Stdio::piped())
			.output()
			.expect("Failed to run /bin/which");

		if !output.status.success() {
			return Err(LoadError::MissingDependency(program.to_string()));
		}

		debug!(
			"Found {program} at {}",
			String::from_utf8_lossy(&output.stdout).trim()
		);

		Ok(())
	}
}
