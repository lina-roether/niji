local M = {}

local style_css = niji.Template:load("style.css.mustache")

function M.apply(config, theme)
	local custom_style = config.custom_style_file and niji.fs.read_config_asset(config.custom_style_file)
	local show_shadow = true
	if config.show_shadow ~= nil then show_shadow = config.show_shadow end

	-- TODO: accent color mechanism
	local accent = theme.palette.teal;

	local style = style_css:render {
		icon_font = config.icon_font,
		font = config.font_family,
		font_size = niji.util.font_size(config, 18),
		transition_duration = config.transition_duration or "200ms",
		hidden_opacity = config.hidden_opacity or 0.0,
		bar_background = "transparent",
		background = theme.ui.background,
		text_background = theme.ui.text_default,
		surface = theme.ui.surface,
		text_surface = theme.ui:text_on(theme.ui.surface),
		primary = accent,
		text_primary = theme.ui:text_on(accent),
		warning = theme.ui.warning,
		text_warning = theme.ui:text_on(theme.ui.warning),
		info = accent,
		text_info = theme.ui:text_on(accent),
		padding_x = config.padding_x or 12,
		padding_y = config.padding_y or 4,
		workspace_button_margin = config.workspace_button_margin or 6,
		border_radius = config.border_radius or 4,
		border_width = config.border_width or 2,
		custom_modules = config.custom_modules or {},
		show_shadow = show_shadow,
		shadow = theme.ui.shadow,
		custom_style = custom_style
	}

	niji.fs.write_config("waybar/style.css", style)
end

function M.reload(config)
	niji.console.info("Restarting waybar...")
	os.execute("killall waybar")
	niji.os.exec_detached(config.waybar_command or "waybar &> /dev/null")
end

return M
