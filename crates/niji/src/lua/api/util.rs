use mlua::{IntoLua, Lua};

use super::ApiModule;

pub struct UtilApi;

impl UtilApi {
	fn by_theme(_: &Lua, (theme, value): (mlua::Table, mlua::Value)) -> mlua::Result<mlua::Value> {
		let table = match value {
			mlua::Value::Table(table) => table,
			_ => return Ok(value),
		};

		let default: mlua::Value = table.get("default")?;

		let Some(name) = theme.get::<Option<String>>("name")? else {
			return Ok(default);
		};

		Ok(table.get::<Option<mlua::Value>>(name)?.unwrap_or(default))
	}

	fn font_size(_: &Lua, (config, default): (mlua::Table, u32)) -> mlua::Result<u32> {
		let font_size = config.get::<Option<u32>>("font_size")?.unwrap_or(default);

		let font_scale = config.get::<Option<f32>>("font_scale")?.unwrap_or(1.0);

		Ok((font_size as f32 * font_scale).round() as u32)
	}
}

impl ApiModule for UtilApi {
	const NAMESPACE: &'static str = "util";

	fn build(lua: &'_ Lua) -> mlua::Result<mlua::Value> {
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
		files::Files,
		lua::runtime::{LuaRuntime, LuaRuntimeInit},
		utils::xdg::XdgDirs,
	};

	#[test]
	fn is_accessible() {
		let tempdir = tempdir().unwrap();
		let xdg = Rc::new(XdgDirs::in_tempdir(&tempdir));
		let files = Rc::new(Files::new(&xdg).unwrap());
		let runtime = LuaRuntime::new(LuaRuntimeInit { xdg, files }).unwrap();

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
