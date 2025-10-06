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
