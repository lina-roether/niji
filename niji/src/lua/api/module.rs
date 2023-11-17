use mlua::{IntoLua, Lua};

use super::{Module, ModuleContext};

pub struct ModuleApi;

impl Module for ModuleApi {
	const NAMESPACE: &'static str = "mod";

	fn build(lua: &Lua) -> mlua::Result<mlua::Value> {
		let meta = lua.create_table()?;
		meta.raw_set(
			"__index",
			lua.create_function(|lua, index: String| {
				let module_ctx = lua.app_data_ref::<ModuleContext>().unwrap();
				match index.as_str() {
					"name" => Ok(Some(module_ctx.name.clone())),
					_ => Ok(None)
				}
			})?
		)?;

		let module = lua.create_table()?;
		module.set_metatable(Some(meta));
		module.into_lua(lua)
	}
}
