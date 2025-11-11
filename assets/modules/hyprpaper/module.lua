local M = {}

local template = niji.Template:load("hyprpaper.conf.mustache")

function M.apply(config, theme)
	local hyprpaper_conf = template:render {
		wallpaper = niji.util.by_theme(theme, config.wallpaper),
		splash = config.splash,
		splash_offset = config.splash_offset
	}

	niji.fs.write_config("hypr/hyprpaper.conf", hyprpaper_conf)
end

function M.reload(config)
	niji.console.info("Restarting hyprpaper...")
	os.execute("pkill hyprpaper")
	niji.os.exec_detached(config.hyprpaper_command or "hyprpaper > /dev/null")
end

return M
