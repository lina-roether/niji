use std::process::Command;

use mlua::{IntoLua, Lua};

use super::ApiModule;

pub struct OsApi;

impl OsApi {
	fn exec_detached(_: &Lua, command: String) -> mlua::Result<()> {
		Command::new("sh")
			.args(["-c", &command])
			.spawn()
			.map_err(mlua::Error::runtime)?;

		Ok(())
	}
}

impl ApiModule for OsApi {
	const NAMESPACE: &'static str = "os";

	fn build(lua: &'_ Lua) -> mlua::Result<mlua::Value<'_>> {
		let table = lua.create_table()?;

		table.raw_set("exec_detached", lua.create_function(Self::exec_detached)?)?;

		table.into_lua(lua)
	}
}
