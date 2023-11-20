use std::{
	env,
	path::{Path, PathBuf},
	rc::Rc
};

use mlua::{FromLuaMulti, IntoLuaMulti, Lua};

use crate::{
	config::ModuleConfig, console, file_manager::FileManager, files::Files, utils::xdg::XdgDirs
};

use super::api;

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
	config: ModuleConfig,
	table: Option<mlua::Table<'lua>>
}

impl<'lua> LuaModule<'lua> {
	const ENTRY_POINT: &'static str = "module.lua";

	fn new(lua: &'lua Lua, directory: PathBuf, config: ModuleConfig) -> Self {
		Self {
			lua,
			name: directory
				.file_name()
				.unwrap()
				.to_string_lossy()
				.into_owned(),
			directory,
			config,
			table: None
		}
	}

	fn load(&mut self) -> mlua::Result<()> {
		let chunk = self.lua.load(self.directory.join(Self::ENTRY_POINT));
		let table: mlua::Table = self.in_context(self.lua, || chunk.call(()))?;
		self.table = Some(table);

		console::debug!("Loaded lua module {}", self.directory.display());
		Ok(())
	}

	pub fn call<A, R>(&self, key: &str, args: A) -> mlua::Result<R>
	where
		A: IntoLuaMulti<'lua>,
		R: FromLuaMulti<'lua>
	{
		let Some(table) = &self.table else {
			return Err(mlua::Error::runtime(format!(
				"Module {} is not loaded yet!",
				self.name
			)));
		};

		let function: mlua::Function = table.get(key)?;
		self.in_context(self.lua, || function.call(args))
	}

	fn in_context<R>(
		&self,
		lua: &'lua Lua,
		cb: impl FnOnce() -> mlua::Result<R>
	) -> mlua::Result<R> {
		let prev_dir = env::current_dir().unwrap();
		env::set_current_dir(&self.directory).unwrap();
		api::set_module_context(lua, self.name.clone(), self.config.clone());

		let result: R = cb()?;

		env::set_current_dir(prev_dir).unwrap();
		api::reset_module_context(lua);

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

	pub fn load_lua_module<'lua>(
		&'lua self,
		path: &Path,
		config: ModuleConfig
	) -> mlua::Result<LuaModule<'lua>> {
		let mut module = LuaModule::new(&self.lua, path.to_path_buf(), config);
		module.load()?;
		Ok(module)
	}
}
