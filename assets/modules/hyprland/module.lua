local M = {}

local template = niji.Template:load("theme.conf.mustache")

local function warn_if_not_sourced(config)
	if config.suppress_not_sourced_warning then
		return
	end

	local hyprland_config = niji.fs.read_config("hypr/hyprland.conf");

	if not string.match(hyprland_config, "[\\^\n]%s*source%s+=.*niji/hyprland/theme%.conf") then
		niji.console.warn(
			"You don't seem to have sourced niji's generated config for hyprland!\n" ..
			"\n" ..
			"To do this, add the following line to your hyprland.conf:\n" ..
			"source = " .. niji.fs.get_output_dir() .. "/theme.conf\n" ..
			"\n" ..
			"To suppress this warning instead, set suppress_not_sourced_warning in the module options."
		)
	end
end

function M.apply(config, theme)
	local configure_cursor = config.cursor_theme ~= nil and config.cursor_size ~= nil
	if configure_cursor then
		niji.console.debug("Configuring cursor theme \"" .. config.cursor_theme .. "\" " .. config.cursor_size)
	end

	local theme_conf = template:render {
		configure_cursor = configure_cursor,
		cursor_theme = config.cursor_theme,
		cursor_size = config.cursor_size,
		border_color = theme.ui.background,
		active_border_color = theme.ui[config.focused_color or "surface"],
		shadow_color = theme.ui.shadow
	}

	niji.fs.output("theme.conf", theme_conf)

	warn_if_not_sourced(config)
end

function M.reload()
	os.execute("hyprctl reload > /dev/null")
end

return M;
