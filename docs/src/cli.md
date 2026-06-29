# Command Line Interface

To get a full list of CLI options, use `niji help` or `niji help <command>`.

## Setting the theme and accent color

Get a list of available themes using:

```sh
niji theme list
```

To preview a theme from the list, use:

```sh
niji theme preview <theme>
```

Optionally, specify an explicit accent color to use:

```sh
niji theme preview <theme> --accent <accent>
```

`<accent>` can be one of `pink`, `red`, `orange`, `yellow`, `green`, `teal`,
`blue`, `purple`, `black` or `white`.

To set a theme, use:

```sh
niji theme set <theme>
```

If you want to use an accent color other than the default, you may specify it as
follows:

```sh
niji theme set <theme> --accent <accent>
```

To change the accent color and keep the current theme, use:

```sh
niji accent set <accent>
```

## Applying re-applying modules

If you want to manually re-apply the current theme and accent color, use:

```sh
niji apply
```

To (re-)apply a specific module, use:

```sh
niji apply --module <name>
```
