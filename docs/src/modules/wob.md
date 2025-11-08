# Module `wob`

The `mako` module allows you to theme
[wob bars](https://github.com/francma/wob).

## Activating

To activate the module, add it to your `config.toml`:

```toml
modules = ["wob"]
```

This will cause niji to take control of your `.config/wob/wob.ini` file.

Note that if you want live reloading to work, you will likely need to configure
a `wob_command`. See the following for details.

## Configuration

These module-specific configuration options can be added to `config.toml` (shown
here with their default values):

```toml
[mako]
# The command used to start wob when reloading
wob_command = "tail -f $XDG_RUNTIME_DIR/wob.sock | wob"

# Set to a path string to specify custom values for wob.ini
custom_config_file = false
```

Due to the nature of how wob functions, it is likely necessary to customize the
`wob_command` option to get live reloading to function correctly. Simply set it
to the same command that you use to start wob initially.

Since niji needs to take control of `.config/wob/wob.ini`, if you want to set
any of wobs's additional configuration options, or override a value set by niji,
you'll have to create a separate configuration file in your `.config/niji`
directory, and link to it in `config.toml`. For example, if you wanted to show
the bar at the top of the screen, you might do something like this:

`~/.config/niji/config.toml`

```toml
# ...

[wob]
custom_config_file = "./custom/wob.ini"
```

`~/.config/niji/custom/wob.ini`

```
anchor = top
```
