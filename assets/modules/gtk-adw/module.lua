local M = {}

local template_gtk3 = niji.Template:load("./assets/gtk3.css.mustache");
local template_gtk4 = niji.Template:load("./assets/gtk4.css.mustache");

local function check_adw_gtk3()
	if not niji.fs.data_exists("themes/adw-gtk3") then
		niji.console.warning("adw-gtk3 isn't installed. Without it, theming GTK3 applications won't work!")
	end
end

function M.apply(config, theme, accent)
	if not config.skip_adw_gtk3_check then
		check_adw_gtk3()
	end

	local colors = {
		window_bg_color = theme.ui.background,
		window_fg_color = theme.ui:text_on(theme.ui.background),
		view_bg_color = theme.ui.background,
		view_fg_color = theme.ui:text_on(theme.ui.background),
		accent_bg_color = accent,
		accent_fg_color = theme.ui:text_on(accent),
		headerbar_bg_color = theme.ui.background,
		headerbar_fg_color = theme.ui:text_on(theme.ui.background),
		popover_bg_color = theme.ui.surface,
		popover_fg_color = theme.ui:text_on(theme.ui.surface),
		dialog_bg_color = theme.ui.surface,
		dialog_fg_color = theme.ui:text_on(theme.ui.surface),
		card_bg_color = theme.ui.surface,
		card_fg_color = theme.ui:text_on(theme.ui.surface),
		sidebar_bg_color = theme.ui.surface,
		sidebar_fg_color = theme.ui:text_on(theme.ui.surface),
		sidebar_shade_color = theme.ui.surface,
		sidebar_border_color = theme.ui.border,
		destructive_bg_color = theme.ui.error,
		success_bg_color = theme.ui.success,
		warning_bg_color = theme.ui.warning,
		error_bg_color = theme.ui.error,
		accent_blue = theme.palette.blue,
		accent_teal = theme.palette.teal,
		accent_green = theme.palette.green,
		accent_yellow = theme.palette.yellow,
		accent_orange = theme.palette.orange,
		accent_red = theme.palette.red,
		accent_pink = theme.palette.pink,
		accent_purple = theme.palette.purple,
		accent_slate = niji.Color:blend(theme.palette.teal, theme.palette.black, 0.4)
	}

	local gtk3 = template_gtk3:render(colors);
	local gtk4 = template_gtk4:render(colors);

	niji.fs.write_config("gtk-3.0/gtk.css", gtk3);
	niji.fs.write_config("gtk-3.0/gtk-dark.css", gtk3);
	niji.fs.write_config("gtk-4.0/gtk.css", gtk4);
	niji.fs.write_config("gtk-4.0/gtk-dark.css", gtk4);
end

function M.reload(config)

end

return M;
