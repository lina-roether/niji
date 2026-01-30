local M = {}

local template = niji.Template:load("niji.conf.mustache")

function M.apply(config, theme, accent)
	local base_color = niji.Color:blend(theme.ui.background, theme.ui.surface, 0.5)
	local base_text_color = theme.ui:text_on(base_color)
	local bright_text_color = theme.ui.text_light
	if theme.kind == "dark" then
		bright_text_color = theme.ui.text_dark
	end

	local colors = {
		window = theme.ui.background,
		window_text = theme.ui:text_on(theme.ui.background),
		base = base_color,
		alternate_base = theme.ui.background,
		tool_tip_base = theme.ui.surface,
		tool_tip_text = theme.ui:text_on(theme.ui.surface),
		placeholder_text = niji.Color:blend(base_color, base_text_color, 0.7),
		text = base_text_color,
		button = theme.ui.surface,
		button_text = theme.ui:text_on(theme.ui.surface),
		bright_text = bright_text_color,
		light = theme.ui.surface:lighten(0.2),
		midlight = theme.ui.surface:darken(0.1),
		dark = theme.ui.surface:darken(0.2),
		mid = theme.ui.surface:darken(0.1),
		shadow = theme.ui.shadow,
		highlight = accent,
		accent = accent,
		highlighted_text = theme.ui:text_on(accent),
		link = theme.palette.blue,
		link_visited = theme.palette.purple,
		no_role = theme.palette.black
	}

	local disabled_color = niji.Color:blend(colors.base, colors.button, 0.7)

	local colors_disabled = {
		window = colors.window,
		window_text = niji.Color:blend(colors.window, colors.window_text, 0.7),
		base = colors.base,
		alternate_base = colors.alternate_base,
		tool_tip_base = colors.tool_tip_base,
		tool_tip_text = colors.tool_tip_text,
		placeholder_text = colors.placeholder_text,
		text = niji.Color:blend(colors.base, colors.text, 0.7),
		button = disabled_color,
		button_text = niji.Color:blend(disabled_color, colors.button_text, 0.7),
		bright_text = colors.bright_text,
		light = colors.light,
		midlight = colors.midlight,
		dark = colors.dark,
		mid = colors.mid,
		shadow = colors.shadow,
		highlight = colors.highlight,
		accent = colors.accent,
		highlighted_text = colors.highlighted_text,
		link = disabled_color,
		link_visited = disabled_color,
		no_role = colors.no_role
	}

	local colors = template:render {
		active = colors,
		inactive = colors,
		disabled = colors_disabled
	}

	niji.fs.write_config("qt6ct/colors/niji.conf", colors)
end

return M
