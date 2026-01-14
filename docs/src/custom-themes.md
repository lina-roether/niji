# Custom Themes

If you build a custom theme, consider contributing it! Just make sure you have
the proper license for the color scheme you're using, as color schemes may be
subject to copyright.

Custom niji themes are defined using [TOML](https://toml.io) files placed into
the `~/.config/niji/themes` directory, with the filename (without the extension)
matching the theme name.

## Basics

The minimal configuration required for a theme is to set `kind` to either
`"light"` or `"dark"`, based on whether it should use dark text on a light
background, or light text on a dark background respectively, and to specify all
the required palette colors under `[palette]`, as per the following example. All
colors are specified using `#RRGGBB` or `#RRGGBBAA` syntax.

```toml
kind = "dark"

[palette]
blue = "#7aa2f7"
yellow = "#e0af68"
orange = "#ff9e64"
red = "#f7768e"
pink = "#f293a5"
green = "#9ece6a"
teal = "#73daca"
purple = "#bb9af7"
white = "#c0caf5"
black = "#1a1b26"
```

niji will then derive all required colors for all theming modules from the
specified color palette.

## Overrides

In cases where you want more direct control over colors used in specific
circumstances, you can override niji's automatic color derivation. The `[ui]`
section can be used to override colors used for graphical user interfaces, and
the `[terminal]` section to override ANSI terminal colors.

Color overrides can be set either by setting an explicit color like under
`[palette]`, by referencing a palette color by name, or by using inline color
derivation syntax to `lighten` or `darken` a palette color by some amount,
choose a specific `shade` with the same hue and chroma, and/or to modify the
color's `alpha` value.

```toml
[ui]
border = "#333333" # Set an explicit color
surface = "black" # Reference a palette color

# Inline color derivations
warning = { color = "orange", darken = 0.1 } # Darken palette orange by 10%
error = { color = "red", lighten = 0.2 } # Lightn palette red by 20%
success = { color = "teal", shade = 0.9 } # Pick a shade wtih 90% lightness based on palette teal
shadow = { color = "black", alpha = 0.2 } # Set the alpha channel of palette black to 0.2
```

### `[ui]`

The `[ui]` section is used for color overrides for graphical interfaces. It
contains the following options:

| Option       | Description                                                                                                       |
| ------------ | ----------------------------------------------------------------------------------------------------------------- |
| `background` | The main background color                                                                                         |
| `surface`    | The background color of surfaces that appear on top of `background` (such as panels or cards)                     |
| `border`     | The color of borders around certain elements. May be set to transparent (`#00000000`) to remove borders.          |
| `shadow`     | The color of drop shadow around certain elements. May be set to transparent (`#00000000`) to remove drop shadows. |
| `text_light` | The light text color to use on dark backgrounds.                                                                  |
| `text_dark`  | The dark text color to use on light backgrounds.                                                                  |
| `success`    | The color indicating a successful action. Usually a shade of green.                                               |
| `warning`    | The color used for warning messages. Usually a shade of yellow or orange.                                         |
| `error`      | The color used for error messages and states. Usually a shade of red.                                             |

### `[terminal]`

The `[terminal]` section contains color overrides corresponding to the standard
16 ANSI colors:

- `dark_black`
- `dark_red`
- `dark_green`
- `dark_yellow`
- `dark_blue`
- `dark_magenta`
- `dark_cyan`
- `dark_white`
- `bright_black`
- `bright_red`
- `bright_green`
- `bright_yellow`
- `bright_blue`
- `bright_magenta`
- `bright_cyan`
- `bright_white`

As well as the following additional options:

| Option             | Description                                                                                      |
| ------------------ | ------------------------------------------------------------------------------------------------ |
| `default`          | The default color to use for non-colored terminal text. Usually a shade of white or black.       |
| `shade_difference` | The difference in lightness to use when deriving dark and bright terminal colors from each other |
