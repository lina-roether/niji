use mlua::{IntoLua, Lua};
use niji_console::prompt;

use super::{Module, ModuleContext};

pub struct ConsoleApi;

macro_rules! define_log_function {
	($name:ident) => {
		fn $name(lua: &Lua, message: mlua::Value) -> mlua::Result<()> {
			let source = Self::get_source(lua)?;
			log::$name!(target: &source, "{}", message.to_string()?);
			Ok(())
		}
	};
}

impl ConsoleApi {
	fn get_source(lua: &Lua) -> mlua::Result<String> {
		let module_ctx = lua.app_data_ref::<ModuleContext>().unwrap();
		Ok(format!("module:{}", module_ctx.name))
	}

	define_log_function!(debug);
	define_log_function!(info);
	define_log_function!(warn);
	define_log_function!(error);

	fn prompt(_: &Lua, (message, default): (mlua::Value, Option<bool>)) -> mlua::Result<bool> {
		let message = message.to_string()?;
		let result = if let Some(default) = default {
			prompt!(default: default, "{message}")
		} else {
			prompt!("{message}")
		};

		Ok(result)
	}
}

impl Module for ConsoleApi {
	const NAMESPACE: &'static str = "console";

	fn build(lua: &Lua) -> mlua::Result<mlua::Value> {
		let module = lua.create_table()?;

		module.raw_set("debug", lua.create_function(Self::debug)?)?;
		module.raw_set("info", lua.create_function(Self::info)?)?;
		module.raw_set("warn", lua.create_function(Self::warn)?)?;
		module.raw_set("error", lua.create_function(Self::error)?)?;
		module.raw_set("prompt", lua.create_function(Self::prompt)?)?;

		module.into_lua(lua)
	}
}
