# Custom Modules

If you build a custom module, consider contributing it! PRs are always welcome
:)

Custom modules are located in the directory `~/.config/niji/modules`. Each
module is a folder in that directory; the name of the folder is the module name.

The heart of a niji module is a lua module in the module folder called
`module.lua`. It has this general structure:

```lua
local M = {}

function M.apply(config, theme)
    -- Apply the theme here
end

function M.reload(config)
    -- Reload the application here.
end

return M
```

The module defines two handlers, `apply` and `reload`.

The `apply` handler receives the [module config](#module-config) and the
[theme](#theme) as parameters. It is responsible for taking the theme,
transforming its contents and writing them to where they need to go for the
theming target to use them.

The `reload` handler is optional. It is responsible for reloading the theming
target to apply the new config. The reason why these two are separate is so that
niji can more easily tell which modules support live reloading, and so that
users can selectively disable live reloading for certain modules.

The exact semantics of the two handlers are different depending on the nature of
the theming target, but in general, `apply` should apply the theme and config in
the least invasive way possible, while `reload` does whatever is necessary to
live-reload the theming target.

## Module Config

The module config, which is passed as the first parameter to both the `apply`
and the `reload` handlers, is a table with string keys and arbitrary values. It
comes from combining the module-specific configuration for your module with the
global module configuration, both of which are defined in `config.toml`. See
[Configuration](../configuration.md) for more information.

Simple modules for personal use probably won't use this feature much, but it is
recommended that modules which are used by multiple users and may be merged to
be builtin modules use the config feature to provide options to users, and
conform to certain global configuration options like `font_scale`.

## Theme

The theme is a table that corresponds directly to the theme format documented in
[Custom Themes](../custom-themes.md). All color values in the theme table are
passed as a [`niji.Color`](./lua-api-reference.md#class-nijicolor) instance.

## Lua API

Niji provides its own Lua API for building modules. It is fully documented in
the section [Lua API Reference](./lua-api-reference.md).

## Templates

A very common thing that modules need to do is inserting some values from the
theme into a pre-made config file template. niji provides a builtin system to do
this, which is documented in the section
[Templating Reference](./templating-reference.md), and can be used via the
[`niji.Template`](./lua-api-reference.md#class-nijitemplate) class from the Lua
API.
