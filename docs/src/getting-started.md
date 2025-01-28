# Getting Started

## Installation

Currently, the only way to install niji is from source. To do this:

1. Clone the git repository and enter the folder
2. Install using cargo: `cargo install --path ./crates/main`
3. Create the config directory: `mkdir ~/.config/niji`
4. Install the builtin modules: `cp ./assets/modules ~/.config/niji/modules`
5. Install the builtin themes: `cp ./assets/themes ~/.config/niji/themes`

## Initial Configuration

Create the configuration file at `~/.config/niji/config.toml`. The first step is to choose which modules
to use. Take a look at [Built-in Modules](./modules/README.md) for a list of available modules. Simply
set your desired modules using this syntax:

```toml
modules = ["hyprland", "waybar"]
```

Afterwards, you should set `font_family`, `cursor_theme` and `cursor_size` as basic preferences.
Make sure you have the cursor theme installed that you select.

```toml
modules = ["hyprland", "waybar"]

[global]
font_family = "Fira Sans"
cursor_theme = "Adwaita"
cursor_size = 22
```

You can now list avaliable themes using `niji theme list`, and preview them using `niji theme show <name>`.
If you've picked one, apply it using `niji theme set <name>`.

## Next Steps

After the initial setup, you may want to consider taking a look at [Configuration](./configuration.md)
for some advanced configuration options.

If you want to use a custom theme, refer to [Custom Themes](./custom-themes.md).

If you want to apply your theme to an application that isn't supported out of the box, you can
take a look at [Custom Modules](./custom-modules.md).
