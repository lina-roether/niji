use std::rc::Rc;

use mlua::IntoLua;

use crate::utils::xdg::XdgDirs;

use super::ApiModule;

pub struct XdgApi;

impl ApiModule for XdgApi {
	const NAMESPACE: &'static str = "xdg";

	fn build(lua: &'_ mlua::Lua) -> mlua::Result<mlua::Value> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		xdg.as_ref().clone().into_lua(lua)
	}
}

#[cfg(test)]
mod tests {
	use std::fs;

	use tempfile::tempdir;

	use crate::{
		files::Files,
		lua::runtime::{LuaRuntime, LuaRuntimeInit},
	};

	use super::*;

	#[test]
	fn is_accessible() {
		let tempdir = tempdir().unwrap();
		let xdg = Rc::new(XdgDirs::in_tempdir(&tempdir));
		let files = Rc::new(Files::new(&xdg).unwrap());
		let runtime = LuaRuntime::new(LuaRuntimeInit { xdg, files }).unwrap();

		fs::write(
			tempdir.path().join("module.lua"),
			r#"
                assert(niji.xdg, "niji.xdg not defined!")
                assert(niji.xdg.config_home, "niji.xdg.config_home not defined!")
                assert(niji.xdg.data_home, "niji.xdg.data_home not defined!")
                assert(niji.xdg.state_home, "niji.xdg.state_home not defined!")
                assert(niji.xdg.cache_home, "niji.xdg.cache_home not defined!")
                assert(niji.xdg.data_dirs, "niji.xdg.data_dirs not defined!")
                assert(niji.xdg.config_dirs, "niji.xdg.config_dirs not defined!")

                return {}
            "#,
		)
		.unwrap();

		runtime.load_lua_module(tempdir.path()).unwrap();
	}
}
