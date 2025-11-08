# Module `swaylock`

The `swaylock` module allows you to theme your
[swaylock](https://github.com/swaywm/swaylock) lock screen.

## Activating

To activate the module, add it to your `config.toml`:

```toml
modules = ["swaylock"]
```

This will cause niji to take control of your `.config/swaylock.config` file.

## Configuration

The following global configuration options are relevant for this module:

- `font_family`
- `font_scale`

See [Configuration](../configuration.md#global-options) for a detailed
explanation.

Additionally, these module-specific configuration options can be added to
`config.toml` (shown here with their default values):

```toml
[swaylock]

# Set to a path string to set additional configuration options
custom_config_file = false
```

This module is only concerned with setting colors. Any additional configuration
of swaylock, particularly if you are using something like _swaylock-effects_,
needs to be done in a separate custom configuration file. An example
configuration might look like this:

`~/.config/niji/config.toml`

```toml
# ...

[swaylock]
custom_config_file = "./custom/swaylock_config"
```

`~/.config/niji/custom/swaylock_config`

```
clock
indicator
grace=3
fade-in=1
```
