use std::{
	env,
	path::{Path, PathBuf},
	rc::Rc,
};

use anyhow::anyhow;
use log::debug;
use mlua::{FromLuaMulti, IntoLuaMulti, Lua};

use crate::{files::Files, utils::xdg::XdgDirs};

use super::api::{self, ModuleContext};

pub struct LuaRuntimeInit {
	pub xdg: Rc<XdgDirs>,
	pub files: Rc<Files>,
}

pub struct LuaRuntime {
	lua: Lua,
}

#[derive(Debug)]
pub struct LuaModule<'a> {
	lua: &'a Lua,
	name: String,
	directory: PathBuf,
	table: Option<mlua::Table>,
}

impl<'a> LuaModule<'a> {
	const ENTRY_POINT: &'static str = "module.lua";

	fn new(lua: &'a Lua, directory: PathBuf) -> Self {
		Self {
			lua,
			name: directory
				.file_name()
				.unwrap()
				.to_string_lossy()
				.into_owned(),
			directory,
			table: None,
		}
	}

	fn load(&mut self) -> anyhow::Result<()> {
		let chunk = self.lua.load(self.directory.join(Self::ENTRY_POINT));
		let value: mlua::Value = self.in_context(self.lua, || chunk.call(()))?;
		let mlua::Value::Table(table) = value else {
			return Err(anyhow!(
				"Expected lua module `{}` to return a table, but received {} instead",
				self.name,
				value.type_name()
			));
		};

		self.table = Some(table);

		debug!("Loaded lua module {}", self.directory.display());
		Ok(())
	}

	pub fn has_function(&'a self, key: &str) -> mlua::Result<bool> {
		let table = self.get_table()?;

		let Some(value) = table.get::<Option<mlua::Value>>(key)? else {
			return Ok(false);
		};

		Ok(matches!(value, mlua::Value::Function(..)))
	}

	pub fn call<A, R>(&self, key: &str, args: A) -> mlua::Result<R>
	where
		A: IntoLuaMulti,
		R: FromLuaMulti,
	{
		let table = self.get_table()?;

		let function: mlua::Function = table.get(key)?;
		self.in_context(self.lua, move || function.call(args))
	}

	fn get_table(&self) -> mlua::Result<&mlua::Table> {
		let Some(table) = &self.table else {
			return Err(mlua::Error::runtime(format!(
				"Module {} is not loaded yet!",
				self.name
			)));
		};
		Ok(table)
	}

	fn in_context<R>(&self, lua: &'a Lua, cb: impl FnOnce() -> mlua::Result<R>) -> mlua::Result<R> {
		let prev_dir = env::current_dir().unwrap_or_else(|_| {
			log::error!("Current working directory is inaccessible! defaulting to home directory");
			env::home_dir().unwrap()
		});
		env::set_current_dir(&self.directory).unwrap();
		api::set_module_context(
			lua,
			ModuleContext {
				name: self.name.clone(),
				path: self.directory.clone(),
			},
		);

		let result: R = cb()?;

		api::reset_module_context(lua);
		if let Err(err) = env::set_current_dir(prev_dir) {
			log::error!("Cannot reset working directory: {err}\n defaulting to home directory");
			env::set_current_dir(env::home_dir().unwrap()).unwrap();
		}
		Ok(result)
	}
}

impl LuaRuntime {
	pub fn new(init: LuaRuntimeInit) -> mlua::Result<Self> {
		let lua = Lua::new();

		lua.load_std_libs(mlua::StdLib::ALL_SAFE)?;
		api::init(
			&lua,
			api::Init {
				xdg: init.xdg,
				files: init.files,
			},
		)?;

		Ok(Self { lua })
	}

	pub fn load_lua_module(&self, path: &Path) -> anyhow::Result<LuaModule<'_>> {
		let mut module = LuaModule::new(&self.lua, path.to_path_buf());
		module.load()?;
		Ok(module)
	}
}

#[cfg(test)]
mod tests {
	use std::fs;

	use tempfile::tempdir;

	use super::*;

	#[test]
	fn init() {
		let tempdir = tempdir().unwrap();
		let xdg = Rc::new(XdgDirs::in_tempdir(&tempdir));
		let files = Rc::new(Files::new(&xdg).unwrap());
		LuaRuntime::new(LuaRuntimeInit {
			xdg: xdg.clone(),
			files,
		})
		.unwrap();
	}

	#[test]
	fn load_module() {
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
			"return {}",
		)
		.unwrap();

		runtime
			.load_lua_module(&xdg.config_home.join("niji/modules/test"))
			.unwrap();
	}

	#[test]
	fn load_module_error_not_a_table() {
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
			"return 69",
		)
		.unwrap();

		runtime
			.load_lua_module(&xdg.config_home.join("niji/modules/test"))
			.unwrap_err();
	}

	#[test]
	fn load_module_syntax_error() {
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
			"oh look, a sytnax error!",
		)
		.unwrap();

		runtime
			.load_lua_module(&xdg.config_home.join("niji/modules/test"))
			.unwrap_err();
	}

	#[test]
	fn has_function() {
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
			"return { foo = function() end, bar = 3 }",
		)
		.unwrap();

		let module = runtime
			.load_lua_module(&xdg.config_home.join("niji/modules/test"))
			.unwrap();

		assert!(module.has_function("foo").unwrap());
		assert!(!module.has_function("bar").unwrap());
		assert!(!module.has_function("baz").unwrap());
	}

	#[test]
	fn call_function() {
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
			"return { foo = function(arg) return 'Argument was `' .. arg .. '`' end }",
		)
		.unwrap();

		let module = runtime
			.load_lua_module(&xdg.config_home.join("niji/modules/test"))
			.unwrap();

		assert_eq!(
			module.call::<_, String>("foo", ":3").unwrap(),
			"Argument was `:3`".to_string()
		);
	}
}
