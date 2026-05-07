# Module `sway`

The `sway` module allows you to theme [sway](https://github.com/swaywm/sway)
window decorations, as well as setting your swaybg wallpaper per theme.

## Activating

To activate the module, add it to your `config.toml`:

```toml
modules = ["sway"]
```

Niji will now output a sway configuration file to
`~/.local/share/niji/sway/theme`. To enable it, add the following line to the
bottom of your sway config:

```
include ~/.local/share/niji/sway/theme
```

If you want to override any of the settings exported by niji, you can simply add
more configuration after the include statement.

## Using bars

### Swaybar

When using **swaybar**, make sure to explicitly reference the bar id that niji
applies its styles to (0 by default) when configuring, otherwise a second bar
will be created.

Additionally, the default sway config sets colors for swaybar, so make sure to
remove those to not override niji's styling.

### Waybar and others

When using **waybar** or otherwise not using swaybar, make sure to disable the
swaybar styles:

```toml
[sway.swaybar]
disabled = true
```

## Configuration

The following global configuration options are relevant to this module:

- `font_family`
- `font_scale`
- `cursor_theme`
- `cursor_size`
- `wallpaper`

See [Configuration](../configuration.md#global-options) for a detailed
explanation. In particular, see
[Setting Wallpapers per Theme](../configuration.md#setting-wallpapers-per-theme)
for information on the `wallpaper` setting.

Additionally, these module-specific configuration options can be added to
`config.toml` (shown here with their default values):

```toml
[sway]

# Set to true to use a muted border color for focused windows instead of
# the accent color
muted_border = false

# Set to true to display sway's indicator bar to show where the next window
# will open
show_indicator = false

# Set to true to prevent niji from managing the wallpaper via swaybg
disable_wallpaper = false


[sway.swaybar]
# Set to true to prevent niji from setting swaybar styles
disable = false

# Use to specify the bar id niji will apply its styles to
bar_id = 0
```
