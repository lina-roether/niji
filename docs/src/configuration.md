# Configuration

Niji is configured via its config file, which lies at
`~/.config/niji/config.toml` (Assuming you don't have a custom
`$XDG_CONFIG_HOME` set). The config file uses [TOML](https://toml.io) syntax.

## Base Configuration

The base configuration configures the behavior of the niji framework itself.
These options go at the top level of `config.toml`. Currently, the following
options are available:

```toml
# A list of module names to activate.
# This value is required.
modules = []

# A list of module names that shouldn't be automatically reloaded.
# This is useful if the reloading behavior of that module interferes with your
# specific configuration.
disable_reloads = []
```

## Module Configuration

Configuration options for modules appear after a header containing their name.
The one exception is the special `[global]` header, which applies to all
modules.

All module configuration options are optional, but if you want consistency
across different theming targets, it is a good idea to set the available global
options that make sense for your setup, since default behaviors may differ from
module to module.

### Global Options

The available global module configuration options are as follows, shown here
with example values:

```toml
[global]

# The font family to use for UI
font_family = "Fira Sans"

# A scaling factor for text.
# Use this if you want larger text for better visibility, or smaller text for
# a more compact UI.
font_scale = 1.0

# The cursor theme to use
cursor_theme = "Adwaita"

# The cursor size to use
cursor_size = 22
```

You can also override any of these options individually for each module, simply
by adding to the corresponding section. For example, you could configure the
waybar module to use a different font like this:

```toml
[waybar]
font_family = "Fira Code"
```

#### Setting Wallpapers per Theme

If you have a module that supports setting wallpapers, such as
[`hyprpaper`](./modules/hyprpaper.md), you can set a global wallpaper map, that
specifies which wallpaper to use for each theme. You can do this by adding a
`[global.wallpaper]` heading to your config, with keys corresponding to the
theme names, and values corresponding to the path to the wallpaper. You can also
add a `default` key as a fallback.

An example configuration might look like this:

```toml
[global.wallpaper]
default = "./wallpapers/wp1.png"
tokyonight = "./wallpapers/wp2.png"
dracula = "./wallpapers/wp3.png"
```

If you just want to use a single wallpaper for every theme, you can also just
set the wallpaper option like this:

```toml
[global]
wallpaper = "./wallpaper/my-wallpaper.png"
```

### Module-Specific Options

Module-specific options come after a header with the name of the corresponding
module. An example configuration for the [`waybar` module](./modules/waybar.md)
might look like this:

```toml
[waybar]
icon_font = "Material Design Icons"
show_shadow = false
module_margin = 8
```

What specific options are available differs from module to module. If you are
using a builtin module, you can find their respective documentation in
[Built-In Modules](./modules/index.md).
