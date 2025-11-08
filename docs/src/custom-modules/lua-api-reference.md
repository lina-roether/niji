# Lua API Reference

niji's Lua API resides in the global `niji` namespace. There is no need to
import it. The niji namespace contains several sub-namespaces and classes for
different purposes, which are listed in this document.

In addition, you can always use the
[Lua standard library](https://www.lua.org/manual/5.3/) which is fully
supported. If one of the functions in the niji API fits what you want to do
however, you should always prefer using the niji API, as it provides better
integration and safety features.

Modules are always executed with their working directory inside of their module
folder, so you can easily reference bundled assets like template files using
relative paths.

Contents:

- [Class `niji.Color`](#class-nijicolor)
- [Class `niji.Template`](#class-nijitemplate)
- [Namespace `niji.console`](#namespace-nijiconsole)
- [Namespace `niji.fs`](#namespace-nijifs)
- [Namespace `niji.mod`](#namespace-nijimod)
- [Namespace `niji.os`](#namespace-nijios)
- [Namespace `niji.util`](#namespace-nijiutil)
- [Namespace `niji.xdg`](#namespace-nijixdg)

## Class `niji.Color`

The class `niji.Color` represents an RGBA color. It can be used to perform
certain manipulations on colors. All color manipulations use the Oklab
perceptual color space, so while some results may appear unexpected through an
RGB lens, they should look good.

All color fields in the `theme` parameter of the module application handler are
also instances of `niji.Color`.

All functions that accept colors also accept strings of the format `"#RRGGBB"`
and `"#RRGGBBAA"`.

### Property `niji.Color.r`

The red channel of the color as an integer between 0 and 255

### Property `niji.Color.g`

The green channel of the color as an integer between 0 and 255

### Property `niji.Color.b`

The blue channel of the color as an integer between 0 and 255

### Property `niji.Color.a`

The alpha channel of the color as an integer between 0 and 255

### Static `niji.Color:new(color_string)`

Constructs a new `niji.Color` object.

- `color_string`: A string representing the desired color (`string`)
- returns: The resulting color (`niji.Color`)

```lua
local my_color = niji.Color:new("#ab38a3ff")

-- Prints "#ab3aa3ff"
niji.console.debug(my_color)
```

### Static `niji.Color:blend(color_1, color_2, t)`

Interpolates between two colors.

- `color_1`: The first of the two colors to interpolate between (`string` or
  `niji.Color`)
- `color_2`: The second of the two colors to interpolate between (`string` or
  `niji.Color`)
- `t`: A number between 0 and 1 that controls the interpolation (`float`)
- returns: The resulting color (`niji.Color`)

```lua
local my_color = niji.Color:blend("#ff0000", "#00ff00", 0.3)

-- Prints "#ff6300ff"
niji.console.debug(my_color)

```

### Static `niji.Color:mix(color_1, color_2)`

Mixes two colors together evenly. Equivalent to calling `niji.Color:blend` with
a `t` of 0.5.

- `color_1`: The first of the two colors to mix together (`string` or
  `niji.Color`)
- `color_2`: The second of the two colors to mix together (`string` or
  `niji.Color`)

```lua
local my_color = niji.Color:mix("#ff0000", "#00ff00")

-- Prints "#f99500ff"
niji.console.debug(my_color)

```

### `niji.Color:lighten(amount)`

Lightens the color by the given amount. "Amount" here refers to relative
perceived lightness, which means that the change in lightness for a given amount
parameter should look the same for any base color, unless the resulting color
would fall outside the RGB color gamut.

- `amount`: The desired relative perceived lightness, ranging between -1 and 1
  (`float`)

```lua
local base_color = niji.Color:new("#123faa")
local lightened_color = base_color:lighten(0.2)

-- Prints "#4a7eeeff"
niji.console.debug(lightened_color)
```

### `niji.Color:darken(amount)`

Darkens the color by the given amount. Equivalent to calling
`niji.Color:lighten` with `-amount`.

- `amount`: The desired relative perceived lightness, ranging between -1 and 1
  (`float`)

```lua
local base_color = niji.Color:new("#c670f9")
local lightened_color = base_color:darken(0.2)

-- Prints "#872eb5ff"
niji.console.debug(lightened_color)
```

### `niji.Color:shade(lightness)`

Selects a shade of the color that has the provided absolute perceived lightness.
As with other operations, if that color falls outside the RGB gamut, it gets
gamut-clipped.

- `lightness`: The desired perceived lightness, ranging between 0 and 1
  (`float`)

```lua
local base_color = niji.Color:new("#cb9174")
local shade = base_color:shade(0.4)

-- Prints "#6c3a1fff"
niji.console.debug(shade)
```

### `niji.Color:with_alpha(alpha)`

Returns the same color with the provided alpha value.

- `alpha`: The desired alpha value, ranging between 0 and 1 (`float`)

```lua
local base_color = niji.Color:new("#abcdef")
local transparent_color = base_color:with_alpha(0.5)

-- Prints "#abcdef80"
niji.console.debug(transparent_color)
```

## Class `niji.Template`

The class `niji.Template` is the lua API for niji's builtin
[templating system](./templating-reference.md). It can be used to load, parse
and render templates.

### Static `niji.Template:parse(template)`

Parses the provided string `template` as a template.

- `template`: The template string (`string`)
- returns: The parsed template object (`niji.Template`)

```lua
local my_template = niji.Template:parse("Hello {{name}}!")
```

### Static `niji.Template:load(path)`

A utility method that loads a template from a file.

- `path`: The path to the template file to load (`string`)
- returns: The parsed template object (`niji.Template`)

```lua
local my_template = niji.Template:load("my_template.mustache")
```

### `niji.Template:render(value)`

Render the template to a string using a given input value. This is most commonly
a table of keys and values used in the template.

- `value`: The value to use as an input in the template (any type)
- returns: The rendered string (`string`)

```lua
local my_template = niji.Template:parse("Hello {{name}}!")
local rendered = my_template:render({ name = "World" })

-- prints "Hello World!"
niji.console.debug(rendered)
```

### `niji.Template:set_format(type_name, format_string)`

Set the custom format for the specified type. See
[the template system reference](./templating-reference.md#custom-formats) for
more information.

- `type_name`: The name of the type for which to set the format (`string`)
- `format_string`: The format string to use for the given type

## Namespace `niji.console`

The `niji.console` namespace provides niji-flavored functions for interacting
with the console.

### `niji.console.debug(message)`

Sends a debug message to the console. Note that these messages are only visible
if `--verbose` is passed as an argument to niji.

- `message`: The message to send (any type)

### `niji.console.info(message)`

Sends an info message to the console.

- `message`: The message to send (any type)

### `niji.console.warn(message)`

Sends a warning message to the console.

- `message`: The message to send (any type)

### `niji.console.error(message)`

Sends an error message to the console.

- `message`: The message to send (any type)

### `niji.console.prompt(message, default)`

Sends a confirmation prompt to the user. If `default` is not nil, pressing enter
without entering a response will return that value. If `default` is nil,
pressing enter without entering a response will trigger a reprompt.

- `prompt`: The message to show in the prompt (any type)
- `default`: The default value for the prompt (`bool` or `nil`)
- returns: The response from the user (`bool`)

```lua
if niji.console.prompt("Do the thing?", true) then
    doTheThing()
end
```

## Namespace `niji.fs`

The namespace `niji.fs` contains functions for interacting with the file system.
While it is much more restrictive than the filesystem API built into lua, it is
strongly recommended to use `niji.fs` functions over raw lua functions whenever
possible, because they have a lot of extra safety features, such as
automatically checking for conflicts with preexisting files.

### `niji.fs.write(path, content)`

This function should be used when you have to write to a file that might already
exist on the system, and which you might not want to silently overwrite if it
does; the major example for this is configuration files which don't support
including files from other locations.

If you just want to output a file that can then be included/imported by another
program, consider using `niji.fs.output`. This is often a better approach,
because it is a lot less invasive.

Calling `niji.fs.write` will cause niji to check if the file already exists, and
contains data that wasn't written to it by niji. If that is the case, niji will
inform the user via a prompt, and create a backup of the previous version if
necessary. Ultimately, it writes `content` to file at the given `path`.

- `path`: The path of the file to write to (`string`). You can use "~" to refer
  to the current user's home directory.
- `content`: The string to write to the file (`string`)
- returns: The absolute, canonical path of the file written to (`string`)

### `niji.fs.write_config(path, content)`

A version of `niji.fs.write` that takes paths relative to `~/.config`.

- `path`: The relative path to the config file to write to (`string`)
- `content`: The content to write to the file (`string`)
- returns: The absolute, canonical path of the file written to (`string`)

### `niji.fs.write_state(path, content)`

A version of `niji.fs.write` that takes paths relative to `~/.local/state`.

- `path`: The relative path of the state file to write to (`string`)
- `content`: The content to write to the file (`string`)
- returns: The absolute, canonical path of the file written to (`string`)

### `niji.fs.write_data(path, content)`

A version of `niji.fs.write` that takes path relative to `~/.local/share`.

- `path`: The relative path of the data file to write to (`string`)
- `content`: The content to write to the file (`string`)
- returns: The absolute, canonical path of the file written to (`string`)

### `niji.fs.output_source(config, opts)`

Outputs a file to the module's output directory, and validates that it is
included in a specified config file. Prints a warning if this is not the case.

- `config`: the module config passed to the `apply` function
- `opts`: a table of options:
  - `out`: the file path to output to, relative to your module's output folder,
    by default located at `~/.local/share/niji/<module name>`
  - `content`: the string to write to the output file
  - `sourced_by_config`: the config file (or list of config files) that the
    `out` file is expected to be sourced or included by. The path is relative to
    `~/.config` (or `$XDG_CONFIG_HOME`).
  - `sourced_by_path`: use instead of `sourced_by_config` if you need to specify
    arbitrary paths that aren't relative to the systems config home.
  - `pattern`: lua regex pattern that the config files are matched against to
    check if they're including the output file properly
  - `hint`: example of how to properly include the output file in the config
    file

### `niji.fs.output_unchecked(path, content)`

Outputs a file to the module's output directory without performing further
checks.

If the output file is crucial for this module to function, consider using
`niji.fs.output_source` instead.

The `path` argument for this functions is relative to your module's output
folder, which, by default, is located at `~/.local/share/niji/<module name>`.

- `path`: The relative path of the output file within the output folder
  (`string`)
- `content`: The content to write to the file (`string`)
- returns: The absolute path of the file that was written to (`string`)

### `niji.fs.get_output_dir()`

Returns the directory that `niji.fs.output` places files into.

- returns: the output directory

### `niji.fs.read_config_asset(path)`

Reads a file with a path relative to `.config/niji`. This is often used for
things like supplementary configuration files that get inserted into the
generated config of a module, as used in the `mako` module for example.

- `path`: The relative path of the asset file within the config folder
  (`string`)
- returns: The contents of the file (`string`)

### `niji.fs.read_config(path)`

Reads a file with a path relative to `~/.config`.

- `path`: The relative path of the config file (`string`)
- returns: The contents of the file (`string`)

### `niji.fs.read_state(path)`

Reads a file with a path relative to `~/.local/state`.

- `path`: The relative path of the state file (`string`)
- returns: The contents of the file (`string`)

### `niji.fs.read_data(path)`

Reads a file with a path relative to `~/.local/share`.

- `path`: The relative path of the state file (`string`)
- returns: The contents of the file (`string`)

## Namespace `niji.mod`

The namespace `niji.mod` can be used to obtain metadata about the current
module.

### `niji.mod.name`

The name of the current module (`string`)

### `niji.mod.path`

The absolute path to the module folder of the current module (`string`)

## Namespace `niji.os`

The namespace `niji.os` contains supplementary functions to the "os"
functionality in lua.

### `niji.os.exec_detached(command)`

Behaves like the builtin `os.exec` function in lua, except it detaches the
command from the parent process, allowing it to keep running in the background
after niji has completed execution.

You should use this method if you want to restart a background process with
updated configuration, for example.

- `command`: The command to execute in the background (`string`)

## Namespace `niji.util`

The namespace `niji.util` implements functions for a couple of common, specific
operations.

### `niji.util.font_size(config, default)`

This function allows modules to easily handle the global `font_scale` option. If
you output a font size to somewhere, simply pass it through this function to
allow the user to scale their system font to their liking.

- `config`: The module configuration passed to the module handlers
- `default`: The font size to use if the scale is 1.0 (`int`)
- returns: The properly scaled font size (`int`)

```lua
function M.apply(config, theme)
    local my_font_size = niji.util.font_size(config, 12)

    -- ...
end
```

### `niji.util.by_theme(theme, value)`

This function makes it easy to allow a config option to optionally be set for
each theme individually. The archetypical use case is modules with wallpaper
support handling the global `wallpaper` config option.

The logic itself is fairly simple; if `value` is a table, return
`value[<theme name>]`, or `value.default` if that is not set. Otherwise, just
return `value` itself.

- `theme`: The theme passed to the apply handler
- `value`: The config value to handle

```lua
function M.apply(config, theme)
    local selected_wallpaper = niji.util.by_theme(theme, config.wallpaper)

    -- ...
end
```

## Namespace `niji.xdg`

The namespace `niji.xdg` provides bindings to the configured locations for the
XDG base directories standard.

### `niji.xdg.config_home`

`$XDG_CONFIG_HOME`, or `$HOME/.config` by default. (`string`)

### `niji.xdg.data_home`

`$XDG_DATA_HOME`, or `$HOME/.local/share` by default. (`string`)

### `niji.xdg.state_home`

`$XDG_STATE_HOME`, or `$HOME/.local/state` by default. (`string`)

### `niji.xdg.cache_home`

`$XDG_CACHE_HOME`, or `$HOME/.cache` by default. (`string`)

### `niji.xdg.runtime_dir`

The value of `$XDG_RUNTIME_DIR`. (`string`)

### `niji.xdg_data_dirs`

The paths contained in `$XDG_DATA_DIRS`, or
`{ "/usr/local/share", "/usr/share" }` by default. (`string[]`)

### `niji.xdg_config.dirs`

The paths contained in `$XDG_CONFIG_DIRS`, `{ "/etc/xdg" }` by default.
(`string[]`)
