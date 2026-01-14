local M = {}

local wob_ini = niji.Template:load("wob.ini.mustache")

function M.apply(config, theme)
	local custom_config = config.custom_config_file and niji.fs.read_config_asset(config.custom_config_file)
	-- TODO: accent color mechanism
	local accent = theme.palette.teal;

	local config = wob_ini:render {
		background_color = theme.ui.background,
		border_color = theme.ui.border,
		bar_color = accent,
		overflow_background_color = theme.ui.background,
		overflow_border_color = theme.ui.border,
		overflow_bar_color = theme.ui.warning,
		custom_config = custom_config
	}

	niji.fs.write_config("wob/wob.ini", config)
end

function M.reload(config)
	niji.console.info("Restarting wob...")

	local wob_command = config.wob_command or ("tail -f " .. niji.xdg.runtime_dir .. "/wob.sock | wob")
	os.execute("killall wob")
	niji.os.exec_detached(wob_command)
end

return M
