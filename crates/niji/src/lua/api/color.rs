use std::str::FromStr;

use mlua::{FromLua, IntoLua, Lua, UserData, UserDataFields, UserDataMethods};

use crate::types::color::Color;

use super::ApiModule;

impl UserData for Color {
	fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
		fields.add_field_method_get("r", |_, this| Ok(this.r));
		fields.add_field_method_get("g", |_, this| Ok(this.g));
		fields.add_field_method_get("b", |_, this| Ok(this.b));
		fields.add_field_method_get("a", |_, this| Ok(this.a));
	}

	fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
		methods.add_method("new", |_, _, color_str: String| {
			Color::from_str(&color_str).map_err(mlua::Error::runtime)
		});
		methods.add_method("blend", |lua, _, (col1, col2, t): (Color, Color, f32)| {
			Color::blend(col1, col2, t).into_lua(lua)
		});
		methods.add_method("mix", |lua, _, (col1, col2): (Color, Color)| {
			Color::mix(col1, col2).into_lua(lua)
		});

		methods.add_method("lighten", |_, this, amount: f32| Ok(this.lighten(amount)));
		methods.add_method("darken", |_, this, amount: f32| Ok(this.darken(amount)));
		methods.add_method("shade", |_, this, lightness: f32| Ok(this.shade(lightness)));
		methods.add_method("with_alpha", |_, this, alpha: f32| {
			Ok(this.with_alpha(alpha))
		});

		methods.add_meta_method("__tostring", |_, this, ()| Ok(this.to_string()));
	}
}

impl FromLua for Color {
	fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
		match value {
			mlua::Value::String(str) => {
				Color::from_str(&str.to_str()?).map_err(mlua::Error::runtime)
			}
			mlua::Value::UserData(data) => {
				let color_ref = data.borrow::<Color>()?;
				Ok(*color_ref)
			}
			_ => Err(mlua::Error::runtime("Cannot cast this value to a color!")),
		}
	}
}

impl ApiModule for Color {
	const NAMESPACE: &'static str = "Color";

	fn build(lua: &'_ Lua) -> mlua::Result<mlua::Value> {
		Color::new_rgba(0, 0, 0, 0).into_lua(lua)
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
                assert(niji.Color, "niji.Color not defined!")
                assert(niji.Color.new, "niji.Color.new not defined!")
                assert(niji.Color.blend, "niji.Color.blend not defined!")
                assert(niji.Color.mix, "niji.Color.mix not defined!")
                assert(niji.Color.lighten, "niji.Color.lighten not defined!")
                assert(niji.Color.darken, "niji.Color.darken not defined!")
                assert(niji.Color.shade, "niji.Color.darken not defined!")
                assert(niji.Color.with_alpha, "niji.Color.with_alpha not defined!")
                assert(niji.Color.r, "niji.Color.r not defined!")
                assert(niji.Color.g, "niji.Color.g not defined!")
                assert(niji.Color.b, "niji.Color.b not defined!")
                assert(niji.Color.a, "niji.Color.a not defined!")

                return {}
            "#,
		)
		.unwrap();

		runtime.load_lua_module(tempdir.path()).unwrap();
	}
}
