# Getting Started

## Installation

### AUR

Arch Linux users can install niji from the AUR using the `niji-git` package.

### Manually

To install niji manually from source, do the following steps:

1. Make sure you have the rust toolchain and _just_ installed
2. Clone the git repository and enter the folder
3. Build the project using `just build`
4. Install niji using `sudo just install`

## Initial Configuration

Create the configuration file at `~/.config/niji/config.toml`. The first step is
to choose which modules to use. Take a look at [Built-in Modules](./modules/)
for a list of available modules. Simply set your desired modules using this
syntax:

```toml
modules = ["hyprland", "waybar"]
```

Afterwards, you should set `font_family`, `cursor_theme` and `cursor_size` as
basic preferences. Make sure you have the cursor theme installed that you
select.

```toml
modules = ["hyprland", "waybar"]

[global]
font_family = "Fira Sans"
cursor_theme = "Adwaita"
cursor_size = 22
```

Lastly, be sure to refer to the
[documentation of each of your selected modules](./modules/) and check for
available configuration options and additional necessary steps for activation.

You can now list available themes using `niji theme list`, and preview them
using `niji theme preview <name>`. You can also choose an accent color out of
`pink`, `red`, `orange`, `yellow`, `green`, `teal`, `blue`, `purple`, `black`
and `white`.

If you've picked a theme and accent color, apply it using:

```sh
niji theme set <theme> --accent <accent>
```

## Next Steps

After the initial setup, you may want to consider taking a look at
[Configuration](./configuration.md) for some advanced configuration options.

If you want to use a custom theme, refer to [Custom Themes](./custom-themes.md).

If you want to apply your theme to an application that isn't supported out of
the box, you can take a look at [Custom Modules](./custom-modules.md).
