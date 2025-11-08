local M = {}

local template = niji.Template:load("theme.mustache")

function M.apply(config, theme)
	local focused_color = config.focused_color or "surface";
	local focused_text_color = "text_" .. focused_color;
	local indicator_color = config.indicator_color or "surface";

	local wallpaper = nil
	if config.disable_wallpaper ~= true then
		wallpaper = niji.util.by_theme(theme, config.wallpaper)
	end

	local theme = template:render {
		unfocused = theme.ui.background,
		text_unfocused = theme.ui.text_background,
		focused = theme.ui[focused_color],
		text_focused = theme.ui[focused_text_color],
		font = config.font_family,
		font_size = niji.util.font_size(config, 12),
		notify = theme.ui.warning,
		text_notify = theme.ui.text_warning,
		indicator = theme.ui[indicator_color],
		cursor = config.cursor_theme,
		cursor_size = config.cursor_size,
		wallpaper = wallpaper
	}

	niji.fs.output_source(config, {
		out = "theme",
		content = theme,
		sourced_by_config = "sway/config",
		line_pattern = "^%s*include%s+.*niji/sway/theme",
		hint = "include = ~/.local/share/niji/sway/theme"
	})
end

function M.reload(config)
	if
		config.cursor_theme ~= nil and config.cursor_theme ~= os.getenv("XCURSOR_THEME") or
		config.cursor_size ~= nil and tostring(config.cursor_size) ~= os.getenv("XCURSOR_SIZE")
	then
		niji.console.warn("Some programs will only reflect cursor theme changes after reopening")
	end

	os.execute("swaymsg reload -q > /dev/null")
end

return M
