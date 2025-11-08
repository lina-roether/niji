# Module `mako`

The `mako` module allows you to theme notifications produced by the
[mako notification daemon](https://github.com/emersion/mako).

## Activating

To activate the module, add it to your `config.toml`:

```toml
modules = ["mako"]
```

This will cause niji to take control of your `.config/mako/config` file.

## Configuration

The following global configuration options are relevant to this module:

- `font_family`
- `font_scale`

See [Configuration](../configuration.md#global-options) for a detailed
explanation.

Additionally, these module-specific configuration options can be added to
`config.toml` (shown here with their default values):

```toml
[mako]

# The border width around notifications
border_width = 2

# The border radius of notifications
border_radius = 10

# The background transparency of the popup
popup_alpha = 1.0

# Set to a path string to set more configuration options
custom_config_file = false
```

Since niji needs to take control of `.config/mako/config`, if you want to set
any of mako's numerous additional configuration options that have nothing to do
with theming, you'll have to create a separate configuration file in your
`.config/niji` directory, and link to it in `config.toml`. For example, if you
wanted to set the default timeout of notifications, you might do something like
this:

`~/.config/niji/config.toml`

```toml
# ...

[mako]
custom_config_file = "./custom/mako_config"
```

`~/.config/niji/custom/mako_config`

```
default-timeout=10000
```
