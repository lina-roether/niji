local M = {}

local template = niji.template.load("config.mustache")

function M.apply(config, theme)
	local custom_config = config.custom_config_file and niji.fs.read_config_asset(config.custom_config_file)
	local alpha = config.popup_alpha or 0.8

	local config = template:render {
		font_family = config.font_family or "sans-serif",
		font_size = niji.util.font_size(config, 11),
		background_color = theme.ui.surface:with_alpha(alpha),
		text_color = theme.ui.text_background,
		border_size = config.border_width or 2,
		border_color = theme.ui.border,
		border_radius = config.border_radius or 10,
		custom_config = custom_config
	}

	niji.fs.write_config("mako/config", config)
end

function M.reload()
	os.execute("makoctl reload")
end

return M
