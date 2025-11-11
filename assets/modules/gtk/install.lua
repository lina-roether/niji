local M = {}

local colors = require("./colors")

local THEME_NAME = "niji"
local THEMES_DIR = os.getenv("HOME") .. "/.themes"
local THEME_LOCATION = THEMES_DIR .. "/" .. THEME_NAME
local SRC_LOCATION = "./colloid";
local TMP_DIR = "/tmp/niji-gtk";
local THEME_LOCATION_TMP = TMP_DIR .. "/colloid/src"
local SASSC_OPT = "-m -t expanded"

local colors_scss_template = niji.Template:load("./assets/_colors.scss.mustache")
local tweaks_temp_template = niji.Template:load("./assets/_tweaks-temp.scss.mustache")

local function tmp_copy_theme()
	niji.console.debug("Copying colloid theme to " .. THEME_LOCATION_TMP)

	os.execute("mkdir -p " .. TMP_DIR)
	os.execute("rm -rf " .. THEME_LOCATION_TMP)
	os.execute("cp -rf " .. SRC_LOCATION .. " " .. TMP_DIR)
end

local function cleanup_tmp()
	niji.console.debug("Clearing temporary files")
	os.execute("rm -rf " .. THEME_LOCATION_TMP)
end

local function apply_tweaks(config, theme)
	niji.console.debug("Injecting tweaks")

	local tweaks = tweaks_temp_template:render {
		variant = theme.ui.color_scheme,
		opacity = config.opacity or "default",
		compact = config.compact or "false",
		translucent = config.translucent or "false",
		rimless = config.rimless or "false",
		window_button = config.window_button or "mac",
		float = config.float or "false"
	}

	local file = io.open(THEME_LOCATION_TMP .. "/sass/_tweaks-temp.scss", "w")
	if file == nil then
		error("Failed to open tweaks file")
	end
	file:write(tweaks)
	file:close()
end

local function inject_colors(theme)
	niji.console.debug("Injecting colors")

	local colors_scss = colors_scss_template:render(colors.make_colors(theme))

	local file = io.open(THEME_LOCATION_TMP .. "/sass/_colors.scss", "w")
	if file == nil then
		error("Failed to open colors file")
	end
	file:write(colors_scss)
	file:close()
end

local function prepare_output_theme()
	niji.console.debug("Clearing install directory")
	os.execute("rm -rf " .. THEME_LOCATION)
	os.execute("mkdir -p " .. THEME_LOCATION)

	os.execute("cp -f ./assets/index.theme " .. THEME_LOCATION)
end

local function sass_compile(src, dest)
	niji.console.debug("Compiling SASS at " .. src)
	niji.console.debug("sassc " .. SASSC_OPT .. " " .. src .. " " .. dest)
	os.execute("sassc " .. SASSC_OPT .. " " .. src .. " " .. dest)
end

local function install_gnome_shell(color_scheme)
	niji.console.debug("Installing gnome shell theme")
	local dest = THEME_LOCATION .. "/gnome-shell"
	local color = "Light"
	local assets_folder = "assets"
	if color_scheme == "dark" then
		color = "Dark"
		assets_folder = "assets-Dark"
	end

	os.execute("mkdir -p " .. dest)
	os.execute("cp -r " .. THEME_LOCATION_TMP .. "/main/gnome-shell/pad-osd.css " .. dest)
	sass_compile(THEME_LOCATION_TMP .. "/main/gnome-shell/gnome-shell-" .. color .. ".scss", dest .. "/gnome-shell.css")

	local src_assets = THEME_LOCATION_TMP .. "/assets/gnome-shell"
	local dest_assets = dest .. "/assets"
	os.execute("cp -rf " .. src_assets .. "/common-assets " .. dest_assets)
	os.execute("cp -rf " .. src_assets .. "/" .. assets_folder .. "/*.svg " .. dest_assets)
	os.execute("cp -rf " .. src_assets .. "/theme " .. dest_assets)

	os.execute("ln -sf " .. dest_assets .. "/no-events.svg " .. dest .. "/no-events.svg")
	os.execute("ln -sf " .. dest_assets .. "/process-working.svg " .. dest .. "/process-working.svg")
	os.execute("ln -sf " .. dest_assets .. "/no-notifications.svg " .. dest .. "/no-notifications.svg")
end

local function install_gtk3(color_scheme)
	niji.console.debug("Installing gtk3 theme")

	local color = "Light"
	if color_scheme == "dark" then color = "Dark" end
	local dest = THEME_LOCATION .. "/gtk-3.0"

	os.execute("mkdir -p " .. dest)

	local src_assets = THEME_LOCATION_TMP .. "/assets/gtk"
	local dest_assets = dest .. "/assets"
	os.execute("cp -rf " .. src_assets .. "/assets " .. dest_assets)
	sass_compile(THEME_LOCATION_TMP .. "/main/gtk-3.0/gtk-" .. color .. ".scss", dest .. "/gtk.css")
	sass_compile(THEME_LOCATION_TMP .. "/main/gtk-3.0/gtk-Dark.scss", dest .. "/gtk-dark.css")
end

local function install_gtk4(color_scheme)
	niji.console.debug("Installing gtk4 theme")

	local color = "Light"
	if color_scheme == "dark" then color = "Dark" end
	local dest = THEME_LOCATION .. "/gtk-4.0"

	os.execute("mkdir -p " .. dest)

	local src_assets = THEME_LOCATION_TMP .. "/assets/gtk"
	local dest_assets = dest .. "/assets"
	os.execute("cp -rf " .. src_assets .. "/assets " .. dest_assets)
	sass_compile(THEME_LOCATION_TMP .. "/main/gtk-4.0/gtk-" .. color .. ".scss", dest .. "/gtk.css")
	sass_compile(THEME_LOCATION_TMP .. "/main/gtk-4.0/gtk-Dark.scss", dest .. "/gtk-dark.css")
end

function M.install(config, theme)
	niji.console.info("Installing " .. THEME_NAME .. " gtk theme...")
	niji.console.debug("Install location is " .. THEME_LOCATION)

	tmp_copy_theme()
	apply_tweaks(config, theme)
	inject_colors(theme)
	prepare_output_theme()
	-- install_gnome_shell(theme.color_scheme)
	install_gtk3(theme.color_scheme)
	install_gtk4(theme.color_scheme)
	cleanup_tmp()
end

return M
