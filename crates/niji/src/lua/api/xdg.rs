use std::rc::Rc;

use mlua::IntoLua;

use crate::utils::xdg::XdgDirs;

use super::ApiModule;

pub struct XdgApi;

impl ApiModule for XdgApi {
	const NAMESPACE: &'static str = "xdg";

	fn build(lua: &'_ mlua::Lua) -> mlua::Result<mlua::Value<'_>> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		xdg.as_ref().clone().into_lua(lua)
	}
}
