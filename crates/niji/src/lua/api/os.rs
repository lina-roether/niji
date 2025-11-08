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
                assert(niji.os, "niji.os not defined!")
                assert(niji.os.exec_detached, "niji.os.exec_detached not defined!")

                return {}
            "#,
		)
		.unwrap();

		runtime.load_lua_module(tempdir.path()).unwrap();
	}
}
