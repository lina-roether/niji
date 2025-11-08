use std::{fs, path::PathBuf, rc::Rc};

use log::info;
use mlua::{FromLua, IntoLua, Lua};
use regex::Regex;

use crate::{
	file_manager::FileManager, files::Files, lua::api::ModuleContext, utils::xdg::XdgDirs,
};

use super::ApiModule;

pub struct FilesystemApi;

fn expand_path(path: &str) -> PathBuf {
	let expanded = shellexpand::tilde(path);
	PathBuf::from(&*expanded)
}

fn parse_lua_pattern(pattern: String) -> mlua::Result<Regex> {
	let pattern_ast = lua_pattern::parse(pattern).map_err(mlua::Error::runtime)?;
	let regex_str =
		lua_pattern::try_to_regex(&pattern_ast, false, false).map_err(mlua::Error::runtime)?;
	Regex::new(&regex_str).map_err(mlua::Error::runtime)
}

fn get_value_or_list<V: FromLua>(lua: &Lua, value: mlua::Value) -> mlua::Result<Vec<V>> {
	match value {
		mlua::Value::Table(table) => {
			let mut values = Vec::with_capacity(table.len()? as usize);
			for i in 0..table.len()? {
				values.push(table.get(i)?);
			}
			Ok(values)
		}
		_ => Ok(vec![V::from_lua(value, lua)?]),
	}
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

	fn read_config(lua: &'_ Lua, path: String) -> mlua::Result<String> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		fs::read_to_string(xdg.config_home.join(expand_path(&path))).map_err(mlua::Error::runtime)
	}

	fn read_state(lua: &'_ Lua, path: String) -> mlua::Result<mlua::Value> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		fs::read_to_string(xdg.state_home.join(expand_path(&path)))
			.map_err(mlua::Error::runtime)?
			.into_lua(lua)
	}

	fn read_data(lua: &'_ Lua, path: String) -> mlua::Result<mlua::Value> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		fs::read_to_string(xdg.data_home.join(path))
			.map_err(mlua::Error::runtime)?
			.into_lua(lua)
	}

	fn output_unchecked(lua: &Lua, (path, content): (String, String)) -> mlua::Result<String> {
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

	fn output_source(
		lua: &Lua,
		(config, options): (mlua::Table, mlua::Table),
	) -> mlua::Result<String> {
		let mod_ctx = lua.app_data_ref::<ModuleContext>().unwrap();
		let path = Self::output_unchecked(lua, (options.get("out")?, options.get("content")?))?;

		if config
			.get::<Option<bool>>("suppress_not_sourced_warning")?
			.unwrap_or(false)
		{
			return Ok(path);
		}

		let mut check_files: Vec<String> = Vec::new();
		let mut config_paths: Vec<String> = Vec::new();

		if let Some(cfg_paths) = options.get::<Option<mlua::Value>>("sourced_from_config")? {
			for cfg_path in get_value_or_list::<String>(lua, cfg_paths)? {
				check_files.push(Self::read_config(lua, cfg_path.clone())?);
				config_paths.push(cfg_path);
			}
		} else {
			let paths = options.get::<mlua::Value>("sourced_from_path")?;
			for path in get_value_or_list(lua, paths)? {
				check_files.push(fs::read_to_string(&path).map_err(mlua::Error::runtime)?);
				config_paths.push(path);
			}
		};

		let hint_text = if let Some(hint) = options.get::<Option<String>>("hint")?
			&& !config_paths.is_empty()
		{
			if config_paths.len() == 1 {
				format!(
					"\nTo do this, add the following line to {}:\n{hint}\n",
					config_paths[0]
				)
			} else {
				format!(
					"\nTo do this, add the following line to one of {}:\n{hint}\n",
					config_paths.join(", ")
				)
			}
		} else {
			String::new()
		};

		let pattern: String = options.get("pattern")?;
		let regex = parse_lua_pattern(pattern)?;
		if check_files.iter().all(|file| !regex.is_match(file)) {
			log::warn!(
				"You don't seem to have included niji's generated config for {}!\n{hint_text}\nTo \
				 suppress this warning instead, set suppress_not_sourced_warning in the module \
				 options.",
				mod_ctx.name
			)
		}

		Ok(path)
	}

	fn get_output_dir(lua: &Lua, (): ()) -> mlua::Result<String> {
		let mod_ctx = lua.app_data_ref::<ModuleContext>().unwrap();
		let files = lua.app_data_ref::<Rc<Files>>().unwrap();
		let path = files.output_dir().join(&mod_ctx.name);
		Ok(path.to_string_lossy().into_owned())
	}

	fn read_config_asset(lua: &'_ Lua, path: String) -> mlua::Result<mlua::Value> {
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

	fn build(lua: &'_ Lua) -> mlua::Result<mlua::Value> {
		let module = lua.create_table()?;

		module.raw_set("write", lua.create_function(Self::write)?)?;
		module.raw_set("write_config", lua.create_function(Self::write_config)?)?;
		module.raw_set("write_state", lua.create_function(Self::write_state)?)?;
		module.raw_set("write_data", lua.create_function(Self::write_data)?)?;
		module.raw_set(
			"output_unchecked",
			lua.create_function(Self::output_unchecked)?,
		)?;
		module.raw_set("output_source", lua.create_function(Self::output_source)?)?;
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

#[cfg(test)]
mod tests {
	use std::fs;

	use tempfile::tempdir;

	use crate::{
		file_manager::FileManager,
		files::Files,
		lua::runtime::{LuaRuntime, LuaRuntimeInit},
	};

	use super::*;

	#[test]
	fn is_accessible() {
		let tempdir = tempdir().unwrap();
		let xdg = Rc::new(XdgDirs::in_tempdir(&tempdir));
		let files = Rc::new(Files::new(&xdg).unwrap());
		let file_manager = Rc::new(FileManager::new(files.clone()).unwrap());
		let runtime = LuaRuntime::new(LuaRuntimeInit {
			xdg,
			files,
			file_manager,
		})
		.unwrap();

		fs::write(
			tempdir.path().join("module.lua"),
			r#"
                assert(niji.fs, "niji.fs not defined!")
                assert(niji.fs.write, "niji.fs.write not defined!")
                assert(niji.fs.write_config, "niji.fs.write_config not defined!")
                assert(niji.fs.write_state, "niji.fs.write_state not defined!")
                assert(niji.fs.write_data, "niji.fs.write_data not defined!")
                assert(niji.fs.output, "niji.fs.output not defined!")
                assert(niji.fs.get_output_dir, "niji.fs.get_output_dir not defined!")
                assert(niji.fs.read_config, "niji.fs.read_config not defined!")
                assert(niji.fs.read_state, "niji.fs.read_state not defined!")
                assert(niji.fs.read_data, "niji.fs.read_data not defined!")
                assert(niji.fs.read_config_asset, "niji.fs.read_config_asset not defined!")

                return {}
            "#,
		)
		.unwrap();

		runtime.load_lua_module(tempdir.path()).unwrap();
	}
}
