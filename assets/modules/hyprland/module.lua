local M = {}

local template = niji.Template:load("theme.conf.mustache")
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

	niji.fs.output_artifact(config, {
		out = "theme.conf",
		content = theme_conf,
		sourced_by_config = "hypr/hyprland.conf",
		line_pattern = "^%s*source%s+=.*niji/hyprland/theme%.conf",
		hint = "source = ~/.local/share/niji/hyprland/theme.conf"
	})
end

function M.reload()
	os.execute("hyprctl reload > /dev/null")
end

return M;
