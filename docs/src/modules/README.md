# Built-in Modules

Niji includes a number of modules already built-in. You can simply activate them
by adding their names to the `"modules"` list in your `config.toml`. For more
details, see [Configuration](../configuration.md), and the documentation of the
respective module.

The built-in modules currently included with niji are:

- [`gtk`](./gtk.md): Theming GTK3 and GTK4 applications
- [`qt`](./qt.md): Theming QT5 and QT6 applications
- [`hyprland`](./hyprland.md): Theming hyprland window decorations
- [`hyprpaper`](./hyprpaper.md): Wallpaper setting support for hyprpaper
- [`kitty`](./kitty.md): Theming kitty window and terminal colors
- [`mako`](./mako.md): Theming mako notifications
- [`sway`](./sway.md): Theming sway window decorations and setting swaybg
  wallpapers
- [`swaylock`](./swaylock.md): Theming your swaylock lock screen
- [`waybar`](./waybar.md): A fully managed waybar theme
- [`wob`](./wob.md): Theming wob indicators

If there is something missing from this list that you'd like to have, consider
writing a [Custom Module](../custom-modules).

