use mlua::{IntoLua, Lua};
use niji_console::prompt;

use super::ApiModule;

pub struct ConsoleApi;

macro_rules! define_log_function {
	($name:ident) => {
		fn $name(_: &Lua, message: mlua::Value) -> mlua::Result<()> {
			log::$name!("{}", message.to_string()?);
			Ok(())
		}
	};
}

impl ConsoleApi {
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

impl ApiModule for ConsoleApi {
	const NAMESPACE: &'static str = "console";

	fn build(lua: &'_ Lua) -> mlua::Result<mlua::Value> {
		let module = lua.create_table()?;

		module.raw_set("debug", lua.create_function(Self::debug)?)?;
		module.raw_set("info", lua.create_function(Self::info)?)?;
		module.raw_set("warn", lua.create_function(Self::warn)?)?;
		module.raw_set("error", lua.create_function(Self::error)?)?;
		module.raw_set("prompt", lua.create_function(Self::prompt)?)?;

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
                assert(niji.console, "niji.console not defined!")
                assert(niji.console.debug, "niji.console.debug not defined!")
                assert(niji.console.info, "niji.console.info not defined!")
                assert(niji.console.warn, "niji.console.warn not defined!")
                assert(niji.console.error, "niji.console.error not defined!")
                assert(niji.console.prompt, "niji.console.prompt not defined!")

                return {}
            "#,
		)
		.unwrap();

		runtime.load_lua_module(tempdir.path()).unwrap();
	}
}
