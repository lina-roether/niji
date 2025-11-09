local M = {}

local template = niji.Template:load("niji.conf.mustache")

function srgb_to_linear(alpha)
	if alpha <= 0.04045 then
		return alpha / 12.92
	else
		return ((alpha + 0.055) / 1.055) ^ 2.4
	end
end

function M.apply(config, theme)
	local foreground_color = theme.terminal.bright_white;
	if config.foreground then
		foreground_color = theme.terminal[config.foreground] or config.foreground
	end

	local theme = template:render {
		background = theme.ui.background,
		background_opacity = config.background_opacity or srgb_to_linear(theme.ui.background.a / 256),
		foreground = foreground_color,
		url = theme.ui.secondary,
		alert = theme.ui.warning,
		primary = theme.ui.primary,
		text_primary = theme.ui.text_primary,
		surface = theme.ui.surface,
		text_surface = theme.ui.text_surface,
		black = theme.terminal.black,
		red = theme.terminal.red,
		green = theme.terminal.green,
		yellow = theme.terminal.yellow,
		blue = theme.terminal.blue,
		magenta = theme.terminal.magenta,
		cyan = theme.terminal.cyan,
		white = theme.terminal.white,
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
