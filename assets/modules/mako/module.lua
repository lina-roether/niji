local M = {}

local template = niji.Template:load("config.mustache")

function M.apply(config, theme)
	local custom_config = config.custom_config_file and niji.fs.read_config_asset(config.custom_config_file)
	local alpha = config.popup_alpha or 1.0

	local cfg = template:render {
		font_family = config.font_family or "sans-serif",
		font_size = niji.util.font_size(config, 11),
		background_color = theme.ui.background:with_alpha(alpha),
		text_color = theme.ui.text_background,
		border_size = config.border_width or 2,
		low_border_color = theme.ui.border,
		normal_border_color = theme.ui.info,
		high_border_color = theme.ui.warning,
		border_radius = config.border_radius or 10,
		custom_config = custom_config
	}

	niji.fs.output_source(config, {
		out = "config",
		content = cfg,
		sourced_by_config = "mako/config",
		line_pattern = "^%s*include%s*=.*niji/mako/config",
		hint = "[]\ninclude=~/.local/share/niji/mako/config"
	})
end

function M.reload()
	os.execute("makoctl reload")
end

return M
