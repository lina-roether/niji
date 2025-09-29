use std::{collections::HashMap, str::FromStr};

use crate::{template::load_template, types::color::Color};
use mlua::{IntoLua, UserData, UserDataMethods};
use niji_templates::Template;

use super::ApiModule;

fn is_array(table: &mlua::Table) -> bool {
	for i in 1..=table.clone().pairs::<mlua::Value, mlua::Value>().count() {
		let Ok(value) = table.raw_get::<_, mlua::Value>(i) else {
			return false;
		};
		if value == mlua::Value::Nil {
			return false;
		}
	}
	true
}

fn to_template_vec(table: mlua::Table) -> mlua::Result<niji_templates::Value> {
	let len = table.raw_len();

	// Cannot differentiate between empty maps and empty arrays.
	// Just returning nil gives more intuitive behavior.
	if len == 0 {
		return Ok(niji_templates::Value::Nil);
	}

	let mut vec = Vec::with_capacity(len);

	for i in 1..=len {
		vec.push(get_template_value(table.raw_get(i)?)?);
	}

	Ok(vec.into())
}

fn to_template_map(table: mlua::Table) -> mlua::Result<niji_templates::Value> {
	let mut map = HashMap::new();

	for pair in table.pairs::<String, mlua::Value>() {
		let (key, value) = pair?;
		map.insert(key, get_template_value(value)?);
	}

	Ok(map.into())
}

fn get_template_value(value: mlua::Value) -> mlua::Result<niji_templates::Value> {
	let template_val: niji_templates::Value = match value {
		mlua::Value::Number(num) => num.into(),
		mlua::Value::Integer(int) => int.into(),
		mlua::Value::Boolean(bool) => bool.into(),
		mlua::Value::String(string) => string.to_string_lossy().into_owned().into(),
		mlua::Value::UserData(user_data) => {
			if let Ok(color) = user_data.borrow::<Color>() {
				(*color).into()
			} else {
				return Err(mlua::Error::runtime(format!(
					"This userdata type isn't supported in templates: {user_data:?}"
				)));
			}
		}
		mlua::Value::Table(table) => {
			if is_array(&table) {
				to_template_vec(table)?
			} else {
				to_template_map(table)?
			}
		}
		mlua::Value::Nil => niji_templates::Value::Nil,
		value => {
			return Err(mlua::Error::runtime(format!(
				"This type isn't supported in templates: {value:?}"
			)))
		}
	};

	Ok(template_val)
}

pub struct LuaTemplate(Template);

impl From<Template> for LuaTemplate {
	fn from(value: Template) -> Self {
		Self(value)
	}
}

impl UserData for LuaTemplate {
	fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
		methods.add_method("load", |lua, _, path: String| {
			let template = load_template(&path).map_err(|e| {
				mlua::Error::runtime(format!("Failed to load template {path}: {e}"))
			})?;
			LuaTemplate(template).into_lua(lua)
		});

		methods.add_method("parse", |lua, _, template: String| {
			let template: Template = template.parse().map_err(mlua::Error::runtime)?;

			LuaTemplate(template).into_lua(lua)
		});

		methods.add_method_mut(
			"set_format",
			|_, this, (ty, fmtstr): (String, String)| -> mlua::Result<()> {
				this.0.set_format(ty, fmtstr);
				Ok(())
			}
		);

		methods.add_method_mut(
			"render",
			|_, this, value: mlua::Value| -> mlua::Result<String> {
				this.0
					.render(&get_template_value(value)?)
					.map_err(mlua::Error::runtime)
			}
		)
	}
}

impl ApiModule for LuaTemplate {
	const NAMESPACE: &'static str = "Template";

	fn build(lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
		LuaTemplate(Template::from_str("").unwrap()).into_lua(lua)
	}
}
