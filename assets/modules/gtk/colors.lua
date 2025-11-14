local Color = niji.Color;

local M = {}

local function by_scheme(theme, for_light, for_dark)
	if theme.ui.color_scheme == "light" then
		return for_light
	else
		return for_dark
	end
end

local function background_colors(theme)
	return {
		theme.ui.background,
		by_scheme(theme, theme.ui.background:darken(0.1), theme.ui.background:lighten(0.1)),
		by_scheme(theme, theme.ui.background:darken(0.2), theme.ui.background:lighten(0.2)),
		by_scheme(theme, theme.ui.background:darken(0.3), theme.ui.background:lighten(0.3)),

		surface = by_scheme(theme, theme.ui.background:darken(0.06), theme.ui.background:lighten(0.06)),
		black = {
			theme.ui.background:shade(0.02),
			theme.ui.background:shade(0.1),
			theme.ui.background:shade(0.2)
		},
		white = {
			theme.ui.background:shade(0.98),
			theme.ui.background:shade(0.9),
			theme.ui.background:shade(0.8)
		}
	}
end

local function fill_color(theme)
	return by_scheme(theme, theme.ui.background:darken(0.1), theme.ui.background:lighten(0.1))
end

local function text_colors(theme)
	return {
		theme.ui.text_background,
		theme.ui.text_background:with_alpha(0.8),
		theme.ui.text_background:with_alpha(0.6),
		theme.ui.text_background:with_alpha(0.4),
		theme.ui.text_background:with_alpha(0.3),
		theme.ui.text_background:with_alpha(0.2)
	}
end

local function overlay_colors(theme)
	local col = Color:new("#000")
	if theme.ui.color_scheme == "dark" then col = Color:new("#fff") end
	return {
		col:with_alpha(0.04),
		col:with_alpha(0.08),
		col:with_alpha(0.12),
		col:with_alpha(0.16)
	}
end

function M.make_colors(theme)
	local tooltip_opacity = 0.9
	local panel_opacity = 0.9
	local bg = background_colors(theme)
	local fill = fill_color(theme)
	local txt = text_colors(theme)
	local overlay = overlay_colors(theme)

	return {
		primary = theme.ui.primary,
		drop_target_color = theme.ui.warning,
		indicator = theme.ui.primary,
		titlebar_indicator = by_scheme(theme, theme.ui.primary, "currentColor"),
		inverse_indicator = theme.ui.primary,
		applet_primary = theme.ui.primary,

		background = bg[1],
		surface = bg.surface,
		base = bg[1],
		base_alt = bg[2],

		tooltip = bg[2]:with_alpha(tooltip_opacity),
		osd = bg.black[1],
		scrim = bg.black[2],
		scrim_alt = bg.black[3],
		scrim_inverse = bg.white[2],
		titlebar = by_scheme(theme, bg[2], bg[1]),
		titlebar_backdrop = by_scheme(theme, bg[2], bg[1]),
		titlebar_primary = theme.ui.primary,
		sidebar = by_scheme(theme, bg[2], bg[1]),
		sidebar_backdrop = by_scheme(theme, bg[2], bg[1]),
		sidebar_primary = theme.ui.primary,
		popover = bg[1],
		panel_solid = bg[3],
		panel = bg[3]:with_alpha(panel_opacity),

		button = fill,
		entry = fill,

		link = theme.ui.secondary,
		link_visited = theme.ui.secondary:darken(0.3),

		warning = theme.ui.warning,
		error = theme.ui.error,
		success = theme.ui.success,

		suggested = theme.ui.primary,
		destructive = theme.ui.error,

		assets_color = theme.ui.secondary,
		frame = theme.ui.border,
		border = theme.ui.border,
		shade = theme.ui.shadow,
		window_border = theme.ui.border,
		solid_border = theme.ui.border,
		border_alt = theme.ui.border:darken(0.1),

		overlay_normal = "transparent",
		overlay_hover = overlay[2],
		overlay_focus = overlay[1],
		overlay_focus_hover = overlay[3],
		overlay_active = overlay[4],
		overlay_checked = overlay[4],
		overlay_selected = overlay[1],

		text = txt[1],
		text_secondary = txt[2],
		text_disabled = txt[3],
		text_secondary_disabled = txt[4],
		track = txt[5],
		track_disabled = txt[6],
		divider = txt[6],
		fill = fill,

		titlebar_text = txt[1],
		titlebar_text_secondary = txt[2],
		titlebar_text_disabled = txt[3],
		titlebar_text_secondary_disabled = txt[4],
		titlebar_track = txt[5],
		titlebar_track_disabled = txt[6],
		titlebar_divider = txt[6],
		titlebar_fill = fill,

		panel_text = txt[1],
		panel_text_secondary = txt[2],
		panel_text_disabled = txt[3],
		panel_text_secondary_disabled = txt[4],
		panel_track = txt[5],
		panel_track_disabled = txt[6],
		panel_divider = txt[6],
		panel_fill = fill,

		titlebutton_close = theme.ui.error,
		titlebutton_max = theme.terminal.bright_green,
		titlebutton_min = theme.terminal.bright_yellow,

		button_close = theme.ui.error,
		button_max = theme.terminal.bright_green,
		button_min = theme.terminal.bright_yellow,

		links = theme.ui.secondary,

		placeholder_text_color = by_scheme(theme, "mix($black, $base, percentage(0.6))",
			"mix($white, $base, percentage(0.6))"),

		on_primary = theme.ui.text_primary,
		on_background = theme.ui.text_background,
		on_surface = theme.ui.text_surface,
		on_warning = theme.ui.text_warning,
		on_error = theme.ui.text_error,
		on_success = theme.ui.text_success,
		on_assets = theme.ui.background,

		red_light = theme.ui.palette.red_light,
		red_dark = theme.ui.palette.red_dark,
		pink_light = theme.ui.palette.pink_light,
		pink_dark = theme.ui.palette.pink_dark,
		purple_light = theme.ui.palette.purple_light,
		purple_dark = theme.ui.palette.purple_dark,
		blue_light = theme.ui.palette.blue_light,
		blue_dark = theme.ui.palette.blue_dark,
		teal_light = theme.ui.palette.teal_light,
		teal_dark = theme.ui.palette.teal_dark,
		green_light = theme.ui.palette.green_light,
		green_dark = theme.ui.palette.green_dark,
		yellow_light = theme.ui.palette.yellow_light,
		yellow_dark = theme.ui.palette.yellow_dark,
		orange_light = theme.ui.palette.orange_light,
		orange_dark = theme.ui.palette.orange_dark,

		grey_050 = theme.ui.background:shade(0.95),
		grey_100 = theme.ui.background:shade(0.9),
		grey_150 = theme.ui.background:shade(0.85),
		grey_200 = theme.ui.background:shade(0.8),
		grey_250 = theme.ui.background:shade(0.75),
		grey_300 = theme.ui.background:shade(0.7),
		grey_350 = theme.ui.background:shade(0.65),
		grey_400 = theme.ui.background:shade(0.6),
		grey_450 = theme.ui.background:shade(0.55),
		grey_500 = theme.ui.background:shade(0.5),
		grey_550 = theme.ui.background:shade(0.45),
		grey_600 = theme.ui.background:shade(0.4),
		grey_650 = theme.ui.background:shade(0.35),
		grey_700 = theme.ui.background:shade(0.3),
		grey_750 = theme.ui.background:shade(0.25),
		grey_800 = theme.ui.background:shade(0.2),
		grey_850 = theme.ui.background:shade(0.15),
		grey_900 = theme.ui.background:shade(0.1),
		grey_950 = theme.ui.background:shade(0.05),

		black = Color:new("#000"),
		white = Color:new("#fff")
	}
end

return M;
