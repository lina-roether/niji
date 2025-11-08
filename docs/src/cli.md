# Command Line Interface

To get help with the CLI you can always use `niji help` or
`niji help <command>`.

**Usage**: niji [OPTIONS] <COMMAND>

## Global Options

These options are available for all commands.

| Name               | Description            |
| ------------------ | ---------------------- |
| `-q`, `--quiet`    | Disable log output     |
| `-v`, `--verbose`  | Print debug messages   |
| `-b`, `--no-color` | Disable colored output |
| `-h`, `--help`     | Print help             |
| `-V`, `--version`  | Print version number   |

## Commands

### `niji apply [OPTIONS]`

Applies (or re-applies) the current theme and configuration to all active
modules, or to the selected modules if `--module` is used.

#### Options

| Name                       | Description                                                                         |
| -------------------------- | ----------------------------------------------------------------------------------- |
| `-M`, `--module <modules>` | Apply the specified modules rather than the active ones. Can be set multiple times. |
| `-k`, `--no-reload`        | Don't reload the affected modules                                                   |

### `niji theme get`

Return the name of the currently active theme

### `niji theme show [name]`

Print a preview of the theme with name `[name]`, or the active theme if `[name]`
is omitted.

### `niji theme set [name]`

Set the active theme to `[name]`. By default, this command also applies the
theme to all active modules (can be disabled using `--no-apply`), and reloads
them (can be disabled using `--no-reload`).

#### Options

| Name               | Description                            |
| ------------------ | -------------------------------------- |
| `-n`, `--no-apply` | Don't apply the theme after setting it |
| `-k` `--no-reload` | Don't reload the affected modules      |

### `niji theme list`

List the names of all installed themes

### `niji theme unset`

Unsets the currently set theme. Does not apply or reload any modules.
