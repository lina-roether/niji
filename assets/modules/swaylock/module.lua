local M = {}

local template = niji.template.load("config.mustache")
template:set_format("color", "{rx}{gx}{bx}{ax}")

function M.apply(config, theme)
	local custom_config = config.custom_config_file and niji.fs.read_config_asset(config.custom_config_file)

	local config = template:render {
		inside_color = theme.ui.background,
		inside_clear_color = theme.ui.background,
		inside_caps_lock_color = theme.ui.background,
		inside_ver_color = theme.ui.background,
		inside_wrong_color = theme.ui.background,
		key_hl_color = theme.ui.info,
		caps_lock_key_hl_color = theme.ui.warning,
		bs_hl_color = theme.ui.error,
		caps_lock_bs_hl_color = theme.ui.error,
		layout_bg_color = theme.ui.surface,
		layout_border_color = theme.ui.border,
		layout_text_color = theme.ui.text_surface,
		ring_color = theme.ui.primary,
		ring_clear_color = theme.ui.info,
		ring_caps_lock_color = theme.ui.primary,
		ring_ver_color = theme.ui.warning,
		ring_wrong_color = theme.ui.error,
		separator_color = niji.col.new("#00000000"),
		text_color = theme.ui.text_background,
		text_clear_color = theme.ui.text_background,
		text_caps_lock_color = theme.ui.text_background,
		text_ver_color = theme.ui.text_background,
		text_wrong_color = theme.ui.text_background,
		font_family = config.font_family,
		custom_config = custom_config
	}

	niji.fs.write_config("swaylock/config", config)
end

return M
