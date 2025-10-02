use mlua::{IntoLua, Lua};

use super::{ApiModule, ModuleContext};

pub struct ModuleMetaApi;

impl ApiModule for ModuleMetaApi {
	const NAMESPACE: &'static str = "mod";

	fn build(lua: &'_ Lua) -> mlua::Result<mlua::Value<'_>> {
		let meta = lua.create_table()?;
		meta.raw_set(
			"__index",
			lua.create_function(|lua, (_, index): (mlua::Value, String)| {
				let module_ctx = lua.app_data_ref::<ModuleContext>().unwrap();
				match index.as_str() {
					"name" => Ok(module_ctx.name.clone().into_lua(lua)?),
					"path" => Ok(module_ctx.path.to_string_lossy().into_lua(lua)?),
					_ => Ok(mlua::Value::Nil)
				}
			})?
		)?;

		let module = lua.create_table()?;
		module.set_metatable(Some(meta));
		module.into_lua(lua)
	}
}
