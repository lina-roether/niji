use std::{path::PathBuf, rc::Rc};

use mlua::Lua;

use crate::{file_manager::FileManager, files::Files, types::color::Color, utils::xdg::XdgDirs};

use self::{
	console::ConsoleApi, filesystem::FilesystemApi, module_meta::ModuleMetaApi, os::OsApi,
	template::LuaTemplate, util::UtilApi, xdg::XdgApi,
};

mod color;
mod console;
mod filesystem;
mod module_meta;
mod os;
mod template;
mod util;
mod xdg;

pub struct ModuleContext {
	pub name: String,
	pub path: PathBuf,
}

trait ApiModule: Sized {
	const NAMESPACE: &'static str;

	fn build(lua: &'_ Lua) -> mlua::Result<mlua::Value<'_>>;
}

fn insert_module<M: ApiModule>(lua: &Lua, api: &mlua::Table) -> mlua::Result<()> {
	api.raw_set(M::NAMESPACE, M::build(lua)?)
}

const API_GLOBAL: &str = "niji";

pub struct Init {
	pub xdg: Rc<XdgDirs>,
	pub files: Rc<Files>,
	pub file_manager: Rc<FileManager>,
}

pub fn init(lua: &Lua, init: Init) -> mlua::Result<()> {
	reset_module_context(lua);
	lua.set_app_data(init.xdg);
	lua.set_app_data(init.file_manager);
	lua.set_app_data(init.files);

	let api = lua.create_table()?;

	insert_module::<Color>(lua, &api)?;
	insert_module::<FilesystemApi>(lua, &api)?;
	insert_module::<ModuleMetaApi>(lua, &api)?;
	insert_module::<ConsoleApi>(lua, &api)?;
	insert_module::<XdgApi>(lua, &api)?;
	insert_module::<LuaTemplate>(lua, &api)?;
	insert_module::<OsApi>(lua, &api)?;
	insert_module::<UtilApi>(lua, &api)?;

	lua.globals().set(API_GLOBAL, api)?;

	Ok(())
}

pub fn set_module_context(lua: &Lua, ctx: ModuleContext) {
	lua.set_app_data(ctx);
}

pub fn reset_module_context(lua: &Lua) {
	lua.remove_app_data::<ModuleContext>();
}
