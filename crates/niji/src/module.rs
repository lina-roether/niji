use std::{
	fs::File,
	io::{BufRead, BufReader},
	path::Path,
	process::{Command, Stdio},
};

use anyhow::anyhow;
use log::debug;

use crate::{
	config::{ModuleConfig, Theme},
	lua::runtime::{LuaModule, LuaRuntime},
};

pub struct Module<'lua>(LuaModule<'lua>);

impl<'lua> Module<'lua> {
	const DEPS_FILE: &'static str = "deps.txt";

	pub fn load(runtime: &'lua LuaRuntime, path: &Path) -> anyhow::Result<Self> {
		Self::check_dependencies(path)?;
		let module = runtime.load_lua_module(path)?;
		Ok(Self(module))
	}

	pub fn can_reload(&self) -> bool {
		self.0.has_function("reload").unwrap_or(false)
	}

	pub fn apply(&self, config: ModuleConfig, theme: Theme) -> anyhow::Result<()> {
		if !self.0.has_function("apply")? {
			return Err(anyhow!("Module is missing an apply function"));
		}

		Ok(self.0.call("apply", (config, theme))?)
	}

	pub fn reload(&self, config: ModuleConfig) -> anyhow::Result<()> {
		Ok(self.0.call("reload", config)?)
	}

	fn check_dependencies(path: &Path) -> anyhow::Result<()> {
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

	fn check_dependency(program: &str) -> anyhow::Result<()> {
		debug!("Checking for module dependency {program}...");

		let output = Command::new("/bin/which")
			.arg(program)
			.stdout(Stdio::piped())
			.output()
			.expect("Failed to run /bin/which");

		if !output.status.success() {
			return Err(anyhow!("Missing dependency: {program}"));
		}

		debug!(
			"Found {program} at {}",
			String::from_utf8_lossy(&output.stdout).trim()
		);

		Ok(())
	}
}
