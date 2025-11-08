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
					_ => Ok(mlua::Value::Nil),
				}
			})?,
		)?;

		let module = lua.create_table()?;
		module.set_metatable(Some(meta));
		module.into_lua(lua)
	}
}

#[cfg(test)]
mod tests {
	use std::{fs, rc::Rc};

	use mlua::AsChunk;
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
                assert(niji.mod, "niji.mod not defined!")
                assert(niji.mod.name, "niji.mod.name not defined!")
                assert(niji.mod.path, "niji.mod.path not defined!")

                return {}
            "#,
		)
		.unwrap();

		runtime.load_lua_module(tempdir.path()).unwrap();
	}

	#[test]
	fn is_correct() {
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
			format!(
				r#"
                    assert(niji.mod.name == "{}", "niji.mod.name has the wrong value " .. niji.mod.name)
                    assert(niji.mod.path == "{}", "niji.mod.path has the wrong value " .. niji.mod.path)

                    return {{}}
                "#,
                tempdir.path().file_name().unwrap().display(),
				tempdir.path().display()
			),
		)
		.unwrap();

		runtime.load_lua_module(tempdir.path()).unwrap();
	}
}
