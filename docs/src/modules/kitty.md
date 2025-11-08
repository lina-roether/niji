# Module `kitty`

The `kitty` module allows you to set the window and terminal colors for the
[kitty terminal emulator](https://sw.kovidgoyal.net/kitty/).

## Activating

Th activate the module, add it to your `config.toml`:

```toml
modules = ["kitty"]
```

This will create a kitty theme called "niji". If you have reloads enabled for
this module (which they are by default), niji will also automatically apply this
theme. If you do not want this behaviour, you can disable it by adding `"kitty"`
to your `disable_reloads` list (see
[Configuration](../configuration.md#global-options)).

## Configuration

These module-specific configuration options can be added to `config.toml` (shown
here with their default values):

```toml
[kitty]

# The opacity of the terminal background
background_opacity = 1.0

# The foreground color. Can be any key from the [terminal] section of the theme,
# or a custom color, such as "#ff0000".
foreground = "bright_white"
```
