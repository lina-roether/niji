local M = {}

local template = niji.Template:load("niji.conf.mustache")

local function srgb_to_linear(alpha)
	if alpha <= 0.04045 then
		return alpha / 12.92
	else
		return ((alpha + 0.055) / 1.055) ^ 2.4
	end
end

function M.apply(config, theme, accent)
	local foreground_color = theme.terminal.default;
	if config.foreground then
		foreground_color = theme.terminal[config.foreground] or config.foreground
	end

	local theme = template:render {
		background = theme.ui.background,
		background_opacity = config.background_opacity or srgb_to_linear(theme.ui.background.a / 256),
		foreground = foreground_color,
		url = theme.palette.blue,
		alert = theme.ui.warning,
		primary = accent,
		text_primary = theme.ui:text_on(accent),
		surface = theme.ui.surface,
		text_surface = theme.ui:text_on(theme.ui.surface),
		black = theme.terminal.dark_black,
		red = theme.terminal.dark_red,
		green = theme.terminal.dark_green,
		yellow = theme.terminal.dark_yellow,
		blue = theme.terminal.dark_blue,
		magenta = theme.terminal.dark_magenta,
		cyan = theme.terminal.dark_cyan,
		white = theme.terminal.dark_white,
		bright_black = theme.terminal.bright_black,
		bright_red = theme.terminal.bright_red,
		bright_green = theme.terminal.bright_green,
		bright_yellow = theme.terminal.bright_yellow,
		bright_blue = theme.terminal.bright_blue,
		bright_magenta = theme.terminal.bright_magenta,
		bright_cyan = theme.terminal.bright_cyan,
		bright_white = theme.terminal.bright_white
	}

	niji.console.info("Installing niji kitty theme...")
	niji.fs.write_config("kitty/themes/niji.conf", theme)
end

function M.reload()
	niji.console.info("Setting kitty theme...")
	os.execute("kitten themes --reload-in=all niji")
end

return M
