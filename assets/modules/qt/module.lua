local M = {}

local template = niji.Template:load("niji.conf.mustache")

local function check_platform_theme()
	local platform_theme = os.getenv("QT_QPA_PLATFORMTHEME")
	if platform_theme ~= "qt5ct" and platform_theme ~= "qt6ct" then
		niji.console.error("In order for niji's QT theme to apply, you must set `QT_QPA_PLATFORMTHEME` to \"qt6ct\"!")
		error("Platform theme check failed")
	end
end

local function check_color_scheme()
	local qt6ct_conf = niji.fs.read_config("qt6ct/qt6ct.conf")
	local qt5ct_conf = niji.fs.read_config("qt5ct/qt5ct.conf")
	local pattern = "\ncolor_scheme_path%s*=.*niji.conf%s*\n"

	if not qt6ct_conf:match(pattern) then
		niji.console.warn(
			"In order for niji's styles to apply to QT6 applications, you have to select 'niji' as the color scheme in qt6ct!");
	end

	if not qt5ct_conf:match(pattern) then
		niji.console.warn(
			"In order for niji's styles to apply to QT5 applications, you have to select 'niji' as the color scheme in qt5ct!");
	end
end

function M.apply(config, theme, accent)
	if config.skip_platform_theme_check ~= true then
		check_platform_theme()
	end

	local base_color = niji.Color:blend(theme.ui.background, theme.ui.surface, 0.5)
	local base_text_color = theme.ui:text_on(base_color)
	local bright_text_color = theme.ui.text_light
	if theme.kind == "dark" then
		bright_text_color = theme.ui.text_dark
	end

	local colors = {
		window = theme.ui.background,
		window_text = theme.ui:text_on(theme.ui.background),
		base = base_color,
		alternate_base = theme.ui.background,
		tool_tip_base = theme.ui.surface,
		tool_tip_text = theme.ui:text_on(theme.ui.surface),
		placeholder_text = niji.Color:blend(base_color, base_text_color, 0.7),
		text = base_text_color,
		button = theme.ui.surface,
		button_text = theme.ui:text_on(theme.ui.surface),
		bright_text = bright_text_color,
		light = theme.ui.surface:lighten(0.2),
		midlight = theme.ui.surface:darken(0.1),
		dark = theme.ui.surface:darken(0.2),
		mid = theme.ui.surface:darken(0.1),
		shadow = theme.ui.shadow,
		highlight = accent,
		accent = accent,
		highlighted_text = theme.ui:text_on(accent),
		link = theme.palette.blue,
		link_visited = theme.palette.purple,
		no_role = theme.palette.black
	}

	local DISABLED_OPACITY = 0.6;

	local disabled_color = niji.Color:blend(colors.base, colors.button, DISABLED_OPACITY)

	local colors_disabled = {
		window = colors.window,
		window_text = niji.Color:blend(colors.window, colors.window_text, DISABLED_OPACITY),
		base = colors.base,
		alternate_base = colors.alternate_base,
		tool_tip_base = colors.tool_tip_base,
		tool_tip_text = colors.tool_tip_text,
		placeholder_text = colors.placeholder_text,
		text = niji.Color:blend(colors.base, colors.text, DISABLED_OPACITY),
		button = disabled_color,
		button_text = niji.Color:blend(disabled_color, colors.button_text, DISABLED_OPACITY),
		bright_text = colors.bright_text,
		light = colors.light,
		midlight = colors.midlight,
		dark = colors.dark,
		mid = colors.mid,
		shadow = colors.shadow,
		highlight = colors.highlight,
		accent = colors.accent,
		highlighted_text = colors.highlighted_text,
		link = disabled_color,
		link_visited = disabled_color,
		no_role = colors.no_role
	}

	local colors = template:render {
		active = colors,
		inactive = colors,
		disabled = colors_disabled
	}

	niji.fs.write_config("qt6ct/colors/niji.conf", colors)
	niji.fs.write_config("qt5ct/colors/niji.conf", colors)
end

function M.reload(config)
	if config.skip_color_scheme_check ~= true then
		check_color_scheme()
	end
	-- The theme gets automatically reloaded if the config directory is touched
	os.execute("touch \"" .. niji.xdg.config_home .. "/\"{qt6ct,qt5ct}");
end

return M
