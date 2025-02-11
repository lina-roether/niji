# Module `gtk`

The `gtk` module allows you to theme GTK3 and GTK4 applications. Note that other
targets for GTK themes, such as GTK2, qt6gtk2, gnome-shell, etc. are currently
not supported.

The niji theme is a modified version of the amazing [Colloid theme](https://github.com/vinceliuice/Colloid-gtk-theme)
by vinceliuice.

## Activating

To activate the module, add it to your `config.toml`:

```toml
modules = ["gtk"]
```

This will export a GTK theme called "niji" to your system. If you have reloads enabled
for this module (which they are by default), niji will also automatically set the
system GTK theme when applying. If you don't want this behavior, you can disable it
by adding `"gtk"` to your `disable_reloads` list (see [Configuration](../configuration.md)).

### libadwaita

Apps that use libadwaita, such as nautilus, are quite stubborn when it comes to convincing
them to use a gtk theme other than Adwaita. The easiest way to fix this is to globally set
the environent variable `GTK_THEME` to `niji`, which works.

If you are using Arch Linux, you can also use [libadwaita-without-adwaita-git](https://aur.archlinux.org/packages/libadwaita-without-adwaita-git)
for an arguably cleaner solution; this patched version of libadwaita properly respects
your system gtk theme, and makes apps like nautilus work properly with niji, nwg-look, and other tools,
out of the box.

## Configuration

The following global configuration options are relevant to this module:

- `cursor_theme`
- `cursor_size`
- `font_family`
- `font_scale`

See [Configuration](../configuration.md#global-options) for a detailed explanation. Note that these options
do not work if reloads are disabled for this module.

Additionaly, these module-specific configuration options can be added to `config.toml`
(shown here with their default values):

```toml
[gtk]

# Set to true to use a more compact layout in gtk apps
compact = false

# Set to "normal" for less flashy window buttons
window_button = "mac"
```
