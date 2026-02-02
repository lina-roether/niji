# Module `qt`

The `qt` module allows you to theme QT5 and QT6 applications, using `qt5ct` and
`qt6ct` respectively, using niji.

This module does not currently support font configuration. Please use `qt5ct`
and `qt6ct` to set those manually.

> [!IMPORTANT] This module will only work properly if you select 'niji' as the
> color scheme in both `qt5ct` and `qt6ct`!

## Activating

To activate the module, add it to your `config.toml`:

```toml
modules = ["qt"]
```

This will export a color scheme called "niji" to your system that can be used by
`qt5ct` and `qt6ct`. If you have reloads enabled for this module (which they are
by default), niji will also force-reload the styles of all running qt
applications upon application.
