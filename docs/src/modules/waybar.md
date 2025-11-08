# Module `waybar`

The `waybar` module provides a fully managed waybar theme in line with your
system's niji theme.

## Activating

To activate the module, add it to your `config.toml`:

```toml
modules = ["waybar"]
```

This will cause niji to take control of your `.config/waybar/style.css` file.

## Configuration

The following global configuration options are relevant for this module:

- `font_family`
- `font_scale`

See [Configuration](../configuration.md#global-options) for a detailed
explanation.

Additionally, these module-specific configuration options can be added to
`config.toml` (shown here with their default values):

```toml
[waybar]

# Set to the ids of custom modules that you use (e.g. "custom-gpu"),
# in order for them to be styled properly.
custom_modules = []

# Set to a string to specify a font to use for icons,
# such as FontAwesome or Material Desing icons
icon_font = false

# Set to false to disable shadows behind waybar elements
show_shadow = true

# The opacity of waybar when in a hidden state
hidden_opacity = 0.0

# The padding in pixels of waybar elements in the x direction.
padding_x = 12

# The padding in pixels of waybar elements in the y direction.
padding_y = 4

# The margin between workspace buttons in pixels
workspace_button_margin = 6

# Set to the path to a css file to include arbitrary custom styles
custom_style_file = false
```

Note, in particular, the `custom_modules` option. If you use custom modules, you
_have to_ add them to the list, otherwise they won't be styled properly.

## More Customization

Since waybar is, by its nature, highly customizable, this module is not going to
fit many people's use cases. You can try to fiddle around with the
`custom_style_file` configuration option, but if you already have a highly
customized waybar theme, I recommend you check out
[Creating Custom Modules](../custom-modules/).

