local M = {}

local template = niji.Template:load("theme.mustache")

function M.apply(config, theme)
	-- TODO: accent color mechanism
	local accent = theme.palette.teal;

	local focused_color = accent;
	if config.muted_border then
		focused_color = theme.ui.surface;
	end
	local focused_text_color = theme.ui:text_on(focused_color);

	local indicator_color = theme.ui.background;
	if config.show_indicator then
		indicator_color = theme.ui.surface;
	end

	local wallpaper = nil
	if config.disable_wallpaper ~= true then
		wallpaper = niji.util.by_theme(theme, config.wallpaper)
	end

	local sway_cfg = template:render {
		unfocused = theme.ui.background,
		text_unfocused = theme.ui.text_default,
		focused = focused_color,
		text_focused = focused_text_color,
		font = config.font_family,
		font_size = niji.util.font_size(config, 12),
		notify = theme.ui.warning,
		text_notify = theme.ui:text_on(theme.ui.warning),
		indicator = theme.ui[indicator_color],
		cursor = config.cursor_theme,
		cursor_size = config.cursor_size,
		wallpaper = wallpaper,
		swaybar = not config.disable_swaybar,
		bar_background = theme.ui.background,
		text_bar_background = theme.ui:text_on(theme.ui.background),
		bar_statusline = theme.ui.background,
		bar_separator = theme.ui.border,
		accent = accent,
		text_accent = theme.ui:text_on(accent),
		active = theme.ui.surface,
		text_active = theme.ui:text_on(theme.ui.surface),
	}

	niji.fs.output_artifact(config, {
		out = "theme",
		content = sway_cfg,
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
