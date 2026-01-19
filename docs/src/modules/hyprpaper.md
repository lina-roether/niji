# Module `hyprpaper`

The `hyprpaper` module allows you to automatically reconfigure
[hyprpaper](https://github.com/hyprwm/hyprpaper) to use a specific wallpaper for
each theme.

See also the [`hyprland` module](./hyprland.md).

## Activating

To activate the module, add it to your `config.toml`:

```toml
modules = ["hyprpaper"]
```

This will cause niji to take control of your `.config/hypr/hyprpaper.conf` file.

## Configuration

In order for this module to do anything, you have to have a wallpaper map
configured. See
[Setting Wallpapers per Theme](../configuration.md#setting-wallpapers-per-theme)
for information on how to do that.

Beyond that, these module-specific configuration options can be added to
`config.toml` (shown here with their default values):

```toml
[hyprpaper]

# Set to true to show the hyprland splash text on the wallpaper
splash = false

# Set the fit mode. See the hyprland docs for details:
# <https://wiki.hypr.land/Hypr-Ecosystem/hyprpaper/>
fit_mode = "cover"

# The command to be used to start hyprpaper when restarting it
hyprpaper_command = "hyprpaper > /dev/null"
```
