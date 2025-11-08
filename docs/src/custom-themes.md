# Custom Themes

If you build a custom theme, consider contributing it! Just make sure you have
the proper license for the color scheme you're using, as color schemes may be
subject to copyright.

Custom niji themes are defined using [TOML](https://toml.io) files placed into
the `~/.config/niji/themes` directory, with the filename (without the extension)
matching the theme name.

The file is split into two sections: `[ui]` for GUI colors, and `[terminal]` for
terminal colors. All colors are defined using `#RRGGBB` or `#RRGGBBAA` syntax.

## `[ui]`

The `[ui]` section defines colors for graphical interfaces. It also contains the
option `color_scheme`, which should be either `"light"` or `"dark"` to tell GUI
toolkits whether the theme should be considered a light or a dark theme.

The section contains the following options:

| Option            | Description                                                                                                       |
| ----------------- | ----------------------------------------------------------------------------------------------------------------- |
| `color_scheme`    | Whether the theme should be considered light or dark. Takes only the values `"light"` and `"dark"`.               |
| `background`      | The main background color                                                                                         |
| `text_background` | The color of text appearing on `background`                                                                       |
| `surface`         | The background color of surfaces that appear on top of `background` (such as panels or cards)                     |
| `text_surface`    | The color of text appearing on `surface`                                                                          |
| `primary`         | The primary accent color of the UI                                                                                |
| `text_primary`    | The color of text appearing on `primary`                                                                          |
| `secondary`       | The secondary accent color of the UI                                                                              |
| `border`          | The color of borders around certain elements. May be set to transparent (`#00000000`) to remove borders.          |
| `shadow`          | The color of drop shadow around certain elements. May be set to transparent (`#00000000`) to remove drop shadows. |
| `success`         | The color indicating a successful action. Usually a shade of green.                                               |
| `text_success`    | The color of text appearing on `success`                                                                          |
| `info`            | The color used for informative user feedback. May be set to the same as `surface` to remove the distinction.      |
| `text_info`       | The color of text appearing on `info`. May be set to the same as `text_surface` to remove the distinction.        |
| `warning`         | The color used for warning messages. Usually a shade of yellow or orange.                                         |
| `text_warning`    | The color of text appearing on `warning`                                                                          |
| `error`           | The color used for error messages and states. Usually a shade of red.                                             |
| `text_error`      | The color of text appearing on `error`                                                                            |

## `[terminal]`

The `[terminal]` section contains color definitions corresponding to the
standard 16 ANSI colors:

- `black`
- `red`
- `green`
- `yellow`
- `blue`
- `magenta`
- `cyan`
- `white`
- `bright_black`
- `bright_red`
- `bright_green`
- `bright_yellow`
- `bright_blue`
- `bright_magenta`
- `bright_cyan`
- `bright_white`

## Example

The following is an example for a theme definition for the built-in `tokyonight`
theme:

```toml
[ui]
color_scheme = "dark"
background = "#1a1b26"
text_background = "#c0caf5"
surface = "#414868"
text_surface = "#c0caf5"
primary = "#a9b1d6"
text_primary = "#1a1b26"
secondary = "#73daca"
border = "#1a1b26"
shadow = "#10101080"

success = "#73daca"
text_success = "#e0af68"
info = "#7aa2f7"
text_info = "#1a1b26"
warning = "#e0af68"
text_warning = "#1a1b26"
error = "#f7768e"
text_error = "#1a1b26"

[terminal]
black = "#15161e"
red = "#f7768e"
green = "#9ece6a"
yellow = "#e0af68"
blue = "#7aa2f7"
magenta = "#bb9af7"
cyan = "#7dcfff"
white = "#a9b1d6"
bright_black = "#414868"
bright_red = "#f7768e"
bright_green = "#73daca"
bright_yellow = "#e0af68"
bright_blue = "#7aa2f7"
bright_magenta = "#bb9af7"
bright_cyan = "#7dcfff"
bright_white = "#c0caf5"
```
