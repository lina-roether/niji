use std::{
	env,
	path::{Path, PathBuf},
	rc::Rc
};

use log::debug;
use mlua::{FromLuaMulti, IntoLuaMulti, Lua};

use crate::{file_manager::FileManager, files::Files, utils::xdg::XdgDirs};

use super::api::{self, ModuleContext};

pub struct LuaRuntimeInit {
	pub xdg: Rc<XdgDirs>,
	pub files: Rc<Files>,
	pub file_manager: Rc<FileManager>
}

pub struct LuaRuntime {
	lua: Lua
}

#[derive(Debug)]
pub struct LuaModule<'lua> {
	lua: &'lua Lua,
	name: String,
	directory: PathBuf,
	table: Option<mlua::Table<'lua>>
}

impl<'lua> LuaModule<'lua> {
	const ENTRY_POINT: &'static str = "module.lua";

	fn new(lua: &'lua Lua, directory: PathBuf) -> Self {
		Self {
			lua,
			name: directory
				.file_name()
				.unwrap()
				.to_string_lossy()
				.into_owned(),
			directory,
			table: None
		}
	}

	fn load(&mut self) -> mlua::Result<()> {
		let chunk = self.lua.load(self.directory.join(Self::ENTRY_POINT));
		let table: mlua::Table = self.in_context(self.lua, || chunk.call(()))?;
		self.table = Some(table);

		debug!("Loaded lua module {}", self.directory.display());
		Ok(())
	}

	pub fn has_function(&'lua self, key: &str) -> mlua::Result<bool> {
		let table = self.get_table()?;

		let Some(value) = table.get::<_, Option<mlua::Value>>(key)? else {
			return Ok(false);
		};

		Ok(matches!(value, mlua::Value::Function(..)))
	}

	pub fn call<A, R>(&'lua self, key: &str, args: A) -> mlua::Result<R>
	where
		A: IntoLuaMulti<'lua>,
		R: FromLuaMulti<'lua>
	{
		let table = self.get_table()?;

		let function: mlua::Function = table.get(key)?;
		self.in_context(self.lua, move || function.call(args))
	}

	fn get_table(&'lua self) -> mlua::Result<&mlua::Table> {
		let Some(table) = &self.table else {
			return Err(mlua::Error::runtime(format!(
				"Module {} is not loaded yet!",
				self.name
			)));
		};
		Ok(table)
	}

	fn in_context<R>(
		&self,
		lua: &'lua Lua,
		cb: impl FnOnce() -> mlua::Result<R>
	) -> mlua::Result<R> {
		let prev_dir = env::current_dir().unwrap();
		env::set_current_dir(&self.directory).unwrap();
		api::set_module_context(
			lua,
			ModuleContext {
				name: self.name.clone(),
				path: self.directory.clone()
			}
		);

		let result: R = cb()?;

		api::reset_module_context(lua);
		env::set_current_dir(prev_dir).unwrap();
		Ok(result)
	}
}

impl LuaRuntime {
	pub fn new(init: LuaRuntimeInit) -> mlua::Result<Self> {
		let lua = Lua::new();

		lua.load_from_std_lib(mlua::StdLib::ALL_SAFE)?;
		api::init(
			&lua,
			api::Init {
				xdg: init.xdg,
				files: init.files,
				file_manager: init.file_manager
			}
		)?;

		Ok(Self { lua })
	}

	pub fn load_lua_module<'lua>(&'lua self, path: &Path) -> mlua::Result<LuaModule<'lua>> {
		let mut module = LuaModule::new(&self.lua, path.to_path_buf());
		module.load()?;
		Ok(module)
	}
}
