use std::{fs, path::PathBuf, rc::Rc};

use log::info;
use mlua::{IntoLua, Lua};

use crate::{
	file_manager::FileManager, files::Files, lua::api::ModuleContext, utils::xdg::XdgDirs,
};

use super::ApiModule;

pub struct FilesystemApi;

fn expand_path(path: &str) -> PathBuf {
	let expanded = shellexpand::tilde(path);
	PathBuf::from(&*expanded)
}

impl FilesystemApi {
	fn write(lua: &Lua, (path, content): (String, String)) -> mlua::Result<String> {
		let file_mgr = lua.app_data_ref::<Rc<FileManager>>().unwrap();
		let path = expand_path(&path);

		fs::create_dir_all(path.parent().unwrap()).map_err(mlua::Error::runtime)?;

		file_mgr
			.write_managed(&path, &content)
			.map_err(mlua::Error::runtime)?;

		Ok(path.to_string_lossy().into_owned())
	}

	fn write_config(lua: &Lua, (path, content): (String, String)) -> mlua::Result<String> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		Self::write(
			lua,
			(
				xdg.config_home.join(path).to_string_lossy().into_owned(),
				content,
			),
		)
	}

	fn write_state(lua: &Lua, (path, content): (String, String)) -> mlua::Result<String> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		Self::write(
			lua,
			(
				xdg.state_home.join(path).to_string_lossy().into_owned(),
				content,
			),
		)
	}

	fn write_data(lua: &Lua, (path, content): (String, String)) -> mlua::Result<String> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		Self::write(
			lua,
			(
				xdg.data_home.join(path).to_string_lossy().into_owned(),
				content,
			),
		)
	}

	fn read_config(lua: &'_ Lua, path: String) -> mlua::Result<mlua::Value<'_>> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		fs::read_to_string(xdg.config_home.join(expand_path(&path)))
			.map_err(mlua::Error::runtime)?
			.into_lua(lua)
	}

	fn read_state(lua: &'_ Lua, path: String) -> mlua::Result<mlua::Value<'_>> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		fs::read_to_string(xdg.state_home.join(expand_path(&path)))
			.map_err(mlua::Error::runtime)?
			.into_lua(lua)
	}

	fn read_data(lua: &'_ Lua, path: String) -> mlua::Result<mlua::Value<'_>> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		fs::read_to_string(xdg.data_home.join(path))
			.map_err(mlua::Error::runtime)?
			.into_lua(lua)
	}

	fn output(lua: &Lua, (path, content): (String, String)) -> mlua::Result<String> {
		let mod_ctx = lua.app_data_ref::<ModuleContext>().unwrap();
		let files = lua.app_data_ref::<Rc<Files>>().unwrap();
		let path = files
			.output_dir()
			.join(&mod_ctx.name)
			.join(expand_path(&path));

		info!("Outputting to {}", path.display());
		fs::create_dir_all(path.parent().unwrap()).map_err(mlua::Error::runtime)?;
		fs::write(&path, content).map_err(mlua::Error::runtime)?;
		Ok(path.to_string_lossy().into_owned())
	}

	fn get_output_dir(lua: &Lua, (): ()) -> mlua::Result<String> {
		let mod_ctx = lua.app_data_ref::<ModuleContext>().unwrap();
		let files = lua.app_data_ref::<Rc<Files>>().unwrap();
		let path = files.output_dir().join(&mod_ctx.name);
		Ok(path.to_string_lossy().into_owned())
	}

	fn read_config_asset(lua: &'_ Lua, path: String) -> mlua::Result<mlua::Value<'_>> {
		let files = lua.app_data_ref::<Rc<Files>>().unwrap();
		let path = files
			.config_file()
			.parent()
			.unwrap()
			.join(expand_path(&path));

		fs::read_to_string(path)
			.map_err(mlua::Error::runtime)?
			.into_lua(lua)
	}
}

impl ApiModule for FilesystemApi {
	const NAMESPACE: &'static str = "fs";

	fn build(lua: &'_ Lua) -> mlua::Result<mlua::Value<'_>> {
		let module = lua.create_table()?;

		module.raw_set("write", lua.create_function(Self::write)?)?;
		module.raw_set("write_config", lua.create_function(Self::write_config)?)?;
		module.raw_set("write_state", lua.create_function(Self::write_state)?)?;
		module.raw_set("write_data", lua.create_function(Self::write_data)?)?;
		module.raw_set("output", lua.create_function(Self::output)?)?;
		module.raw_set("get_output_dir", lua.create_function(Self::get_output_dir)?)?;
		module.raw_set("read_config", lua.create_function(Self::read_config)?)?;
		module.raw_set("read_state", lua.create_function(Self::read_state)?)?;
		module.raw_set("read_data", lua.create_function(Self::read_data)?)?;
		module.raw_set(
			"read_config_asset",
			lua.create_function(Self::read_config_asset)?,
		)?;

		module.into_lua(lua)
	}
}
