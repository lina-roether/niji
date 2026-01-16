local M = {}

local install = require("./install")

function M.apply(config, theme, accent)
	install.install(config, theme, accent)
end

function M.reload(config)
	niji.console.info("Setting gsettings values...")
	os.execute("gsettings set org.gnome.desktop.interface gtk-theme \"\"")
	os.execute("gsettings set org.gnome.desktop.interface gtk-theme \"niji\"")

	if config.cursor_theme ~= nil then
		niji.console.debug("Applying cursor theme " .. config.cursor_theme)
		os.execute("gsettings set org.gnome.desktop.interface cursor-theme \"" .. config.cursor_theme .. "\"")
	end
	if config.cursor_size ~= nil then
		niji.console.debug("Setting cursor size " .. config.cursor_size)
		os.execute("gsettings set org.gnome.desktop.interface cursor-size \"" .. config.cursor_size .. "\"")
	end
	if config.font_family ~= nil then
		niji.console.debug("Setting UI font family " .. config.font_family)
		os.execute("gsettings set org.gnome.desktop.interface font-name \"" .. config.font_family .. "\"")
	end
	if config.font_scale ~= nil then
		niji.console.debug("Setting text scaling factor " .. config.font_scale)
		os.execute("gsettings set org.gnome.desktop.interface text-scaling-factor \"" .. config.font_scale .. "\"")
	end
end

return M;
