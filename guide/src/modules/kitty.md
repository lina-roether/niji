# Module `kitty`

The `kitty` module allows you to set the window and terminal colors for the
[kitty terminal emulator](https://sw.kovidgoyal.net/kitty/).

## Activating

Th activate the module, add it to your `config.toml`:

```toml
modules = ["kitty"]
```

This will create a kitty theme called "niji". If you have reloads enabled for this module
(which they are by default), niji will also automatically apply this theme. If you do not want
this behaviour, you can disable it by adding `"kitty"` to your `disable_reloads` list
(see [Configuration](../configuration.md)).
