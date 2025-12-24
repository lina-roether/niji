use mlua::{UserData, UserDataFields, UserDataMethods};

use crate::{
	theme::{Palette, TerminalTheme, Theme, UiTheme},
	types::color::Color,
};

impl UserData for Palette {
	fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
		methods.add_meta_method("__index", |_, this, index: String| {
			this.get(&index).map_err(mlua::Error::runtime)
		});
	}
}

impl UserData for UiTheme {
	fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
		fields.add_field_method_get("background", |_, this| Ok(this.background));
		fields.add_field_method_get("surface", |_, this| Ok(this.surface));
		fields.add_field_method_get("border", |_, this| Ok(this.border));
		fields.add_field_method_get("shadow", |_, this| Ok(this.shadow));
		fields.add_field_method_get("text_light", |_, this| Ok(this.text_light));
		fields.add_field_method_get("text_dark", |_, this| Ok(this.text_dark));
		fields.add_field_method_get("success", |_, this| Ok(this.success));
		fields.add_field_method_get("warning", |_, this| Ok(this.warning));
		fields.add_field_method_get("error", |_, this| Ok(this.error));
	}

	fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
		methods.add_method("text_color_on", |_, this, background: Color| {
			Ok(this.text_color_on(background))
		});
	}
}

impl UserData for TerminalTheme {
	fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
		fields.add_field_method_get("default", |_, this| Ok(this.default));
		fields.add_field_method_get("dark_black", |_, this| Ok(this.dark_black));
		fields.add_field_method_get("dark_red", |_, this| Ok(this.dark_red));
		fields.add_field_method_get("dark_green", |_, this| Ok(this.dark_green));
		fields.add_field_method_get("dark_yellow", |_, this| Ok(this.dark_yellow));
		fields.add_field_method_get("dark_blue", |_, this| Ok(this.dark_blue));
		fields.add_field_method_get("dark_magenta", |_, this| Ok(this.dark_magenta));
		fields.add_field_method_get("dark_cyan", |_, this| Ok(this.dark_cyan));
		fields.add_field_method_get("dark_white", |_, this| Ok(this.dark_white));
		fields.add_field_method_get("bright_black", |_, this| Ok(this.bright_black));
		fields.add_field_method_get("bright_red", |_, this| Ok(this.bright_red));
		fields.add_field_method_get("bright_green", |_, this| Ok(this.bright_green));
		fields.add_field_method_get("bright_yellow", |_, this| Ok(this.bright_yellow));
		fields.add_field_method_get("bright_blue", |_, this| Ok(this.bright_blue));
		fields.add_field_method_get("bright_magenta", |_, this| Ok(this.bright_magenta));
		fields.add_field_method_get("bright_cyan", |_, this| Ok(this.bright_cyan));
		fields.add_field_method_get("bright_white", |_, this| Ok(this.bright_white));
	}
}

impl UserData for Theme {
	fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
		fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));
		fields.add_field_method_get("palette", |_, this| Ok(this.palette.clone()));
		fields.add_field_method_get("ui", |_, this| Ok(this.ui.clone()));
		fields.add_field_method_get("terminal", |_, this| Ok(this.terminal.clone()));
	}
}
