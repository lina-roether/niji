use mlua::{IntoLua, Lua};

use super::ApiModule;

pub struct UtilApi;

impl UtilApi {
	fn by_theme<'lua>(
		_: &'lua Lua,
		(theme, value): (mlua::Table<'lua>, mlua::Value<'lua>),
	) -> mlua::Result<mlua::Value<'lua>> {
		let table = match value {
			mlua::Value::Table(table) => table,
			_ => return Ok(value),
		};

		let default: mlua::Value = table.get("default")?;

		let Some(name) = theme.get::<_, Option<String>>("name")? else {
			return Ok(default);
		};

		Ok(table
			.get::<_, Option<mlua::Value>>(name)?
			.unwrap_or(default))
	}

	fn font_size<'lua>(
		_: &'lua Lua,
		(config, default): (mlua::Table<'lua>, u32),
	) -> mlua::Result<u32> {
		let font_size = config
			.get::<_, Option<u32>>("font_size")?
			.unwrap_or(default);

		let font_scale = config.get::<_, Option<f32>>("font_scale")?.unwrap_or(1.0);

		Ok((font_size as f32 * font_scale).round() as u32)
	}
}

impl ApiModule for UtilApi {
	const NAMESPACE: &'static str = "util";

	fn build(lua: &'_ Lua) -> mlua::Result<mlua::Value<'_>> {
		let module = lua.create_table()?;

		module.raw_set("by_theme", lua.create_function(Self::by_theme)?)?;
		module.raw_set("font_size", lua.create_function(Self::font_size)?)?;

		module.into_lua(lua)
	}
}

#[cfg(test)]
mod tests {
	use std::{fs, rc::Rc};

	use tempfile::tempdir;

	use crate::{
		file_manager::FileManager,
		files::Files,
		lua::runtime::{LuaRuntime, LuaRuntimeInit},
		utils::xdg::XdgDirs,
	};

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
                assert(niji.util, "niji.util not defined!")
                assert(niji.util.by_theme, "niji.util.by_theme not defined!")
                assert(niji.util.font_size, "niji.util.font_size not defined!")

                return {}
            "#,
		)
		.unwrap();

		runtime.load_lua_module(tempdir.path()).unwrap();
	}
}
