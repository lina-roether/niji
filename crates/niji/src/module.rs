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

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
	use std::{collections::HashMap, fs, rc::Rc};

	use tempfile::tempdir;

	use crate::{
		config::test_utils::test_theme, files::Files, lua::runtime::LuaRuntimeInit,
		utils::xdg::XdgDirs,
	};

	use super::*;

	#[test]
	fn load() {
		let tempdir = tempdir().unwrap();
		let xdg = Rc::new(XdgDirs::in_tempdir(&tempdir));
		let files = Rc::new(Files::new(&xdg).unwrap());
		let runtime = LuaRuntime::new(LuaRuntimeInit {
			xdg: xdg.clone(),
			files,
		})
		.unwrap();

		fs::create_dir_all(xdg.config_home.join("niji/modules/test")).unwrap();
		fs::write(
			xdg.config_home.join("niji/modules/test/module.lua"),
			"return { apply = function(config, theme) end }",
		)
		.unwrap();

		Module::load(&runtime, &xdg.config_home.join("niji/modules/test")).unwrap();
	}

	#[test]
	fn load_syntax_error() {
		let tempdir = tempdir().unwrap();
		let xdg = Rc::new(XdgDirs::in_tempdir(&tempdir));
		let files = Rc::new(Files::new(&xdg).unwrap());
		let runtime = LuaRuntime::new(LuaRuntimeInit {
			xdg: xdg.clone(),
			files,
		})
		.unwrap();

		fs::create_dir_all(xdg.config_home.join("niji/modules/test")).unwrap();
		fs::write(
			xdg.config_home.join("niji/modules/test/module.lua"),
			"this is a syntax error! yay :3",
		)
		.unwrap();

		Module::load(&runtime, &xdg.config_home.join("niji/modules/test")).unwrap_err();
	}

	#[test]
	fn load_not_a_module_error() {
		let tempdir = tempdir().unwrap();
		let xdg = Rc::new(XdgDirs::in_tempdir(&tempdir));
		let files = Rc::new(Files::new(&xdg).unwrap());
		let runtime = LuaRuntime::new(LuaRuntimeInit {
			xdg: xdg.clone(),
			files,
		})
		.unwrap();

		fs::create_dir_all(xdg.config_home.join("niji/modules/test")).unwrap();
		fs::write(
			xdg.config_home.join("niji/modules/test/module.lua"),
			"function apply(config, theme) end",
		)
		.unwrap();

		Module::load(&runtime, &xdg.config_home.join("niji/modules/test")).unwrap_err();
	}

	#[test]
	fn check_can_reload_false() {
		let tempdir = tempdir().unwrap();
		let xdg = Rc::new(XdgDirs::in_tempdir(&tempdir));
		let files = Rc::new(Files::new(&xdg).unwrap());
		let runtime = LuaRuntime::new(LuaRuntimeInit {
			xdg: xdg.clone(),
			files,
		})
		.unwrap();

		fs::create_dir_all(xdg.config_home.join("niji/modules/test")).unwrap();
		fs::write(
			xdg.config_home.join("niji/modules/test/module.lua"),
			"return { apply = function(config, theme) end }",
		)
		.unwrap();

		let module = Module::load(&runtime, &xdg.config_home.join("niji/modules/test")).unwrap();
		assert!(!module.can_reload());
	}

	#[test]
	fn check_can_reload_true() {
		let tempdir = tempdir().unwrap();
		let xdg = Rc::new(XdgDirs::in_tempdir(&tempdir));
		let files = Rc::new(Files::new(&xdg).unwrap());
		let runtime = LuaRuntime::new(LuaRuntimeInit {
			xdg: xdg.clone(),
			files,
		})
		.unwrap();

		fs::create_dir_all(xdg.config_home.join("niji/modules/test")).unwrap();
		fs::write(
			xdg.config_home.join("niji/modules/test/module.lua"),
			"return { apply = function(config, theme) end, reload = function() end }",
		)
		.unwrap();

		let module = Module::load(&runtime, &xdg.config_home.join("niji/modules/test")).unwrap();
		assert!(module.can_reload());
	}

	#[test]
	fn apply() {
		let tempdir = tempdir().unwrap();
		let xdg = Rc::new(XdgDirs::in_tempdir(&tempdir));
		let files = Rc::new(Files::new(&xdg).unwrap());
		let runtime = LuaRuntime::new(LuaRuntimeInit {
			xdg: xdg.clone(),
			files,
		})
		.unwrap();

		fs::create_dir_all(xdg.config_home.join("niji/modules/test")).unwrap();
		fs::write(
			xdg.config_home.join("niji/modules/test/module.lua"),
			"return { apply = function(config, theme) end }",
		)
		.unwrap();

		let module = Module::load(&runtime, &xdg.config_home.join("niji/modules/test")).unwrap();
		module.apply(HashMap::new(), test_theme()).unwrap();
	}

	#[test]
	fn apply_error() {
		let tempdir = tempdir().unwrap();
		let xdg = Rc::new(XdgDirs::in_tempdir(&tempdir));
		let files = Rc::new(Files::new(&xdg).unwrap());
		let runtime = LuaRuntime::new(LuaRuntimeInit {
			xdg: xdg.clone(),
			files,
		})
		.unwrap();

		fs::create_dir_all(xdg.config_home.join("niji/modules/test")).unwrap();
		fs::write(
			xdg.config_home.join("niji/modules/test/module.lua"),
			"return { apply = function(config, theme) error('oops') end }",
		)
		.unwrap();

		let module = Module::load(&runtime, &xdg.config_home.join("niji/modules/test")).unwrap();
		module.apply(HashMap::new(), test_theme()).unwrap_err();
	}

	#[test]
	fn reload() {
		let tempdir = tempdir().unwrap();
		let xdg = Rc::new(XdgDirs::in_tempdir(&tempdir));
		let files = Rc::new(Files::new(&xdg).unwrap());
		let runtime = LuaRuntime::new(LuaRuntimeInit {
			xdg: xdg.clone(),
			files,
		})
		.unwrap();

		fs::create_dir_all(xdg.config_home.join("niji/modules/test")).unwrap();
		fs::write(
			xdg.config_home.join("niji/modules/test/module.lua"),
			"return { reload = function() end }",
		)
		.unwrap();

		let module = Module::load(&runtime, &xdg.config_home.join("niji/modules/test")).unwrap();
		module.reload(ModuleConfig::new()).unwrap()
	}

	#[test]
	fn reload_error() {
		let tempdir = tempdir().unwrap();
		let xdg = Rc::new(XdgDirs::in_tempdir(&tempdir));
		let files = Rc::new(Files::new(&xdg).unwrap());
		let runtime = LuaRuntime::new(LuaRuntimeInit {
			xdg: xdg.clone(),
			files,
		})
		.unwrap();

		fs::create_dir_all(xdg.config_home.join("niji/modules/test")).unwrap();
		fs::write(
			xdg.config_home.join("niji/modules/test/module.lua"),
			"return { reload = function() error('oops') end }",
		)
		.unwrap();

		let module = Module::load(&runtime, &xdg.config_home.join("niji/modules/test")).unwrap();
		module.reload(ModuleConfig::new()).unwrap_err();
	}
}
