# Templating reference

niji has builtin support for templating using its [`niji.Template` API](./lua-api-reference.md#class-nijitemplate).

The templating language used for this is niji's own dialect of [mustache](https://mustache.github.io/).
You can look at the [mustache documentation](https://mustache.github.io/mustache.5.html) for general information
on how it works, it mostly applies to niji's dialect as well.

One major difference from the mustache specification is that triple mustaches (`{{{name}}}`), which are normally
used to disable escaping of HTML characters, are not supported. Instead, niji templates just never escape HTML characters.

## Custom Formats

The one extension that niji makes to the base mustache specification is custom formats for complex types.
niji's mustache dialect provides special syntax for displaying formattable types using format strings.
This feature is necessary because the vast array of targets niji supports may expect vastly different
formats for different data types, and it is impractial to split those datatypes up into more atomic parts.

For example, you can render a color using a custom format like this:

```mustache
{{my_color : "🔴{r}🟢{g}🔵{b}"}}
```

The result of rendering this with `my_color = "#abcdefff"` would be `🔴171🟢205🔵239`.

Often times, you will want to use a specific format for the entirety of a template.
You can do this by adding a format specification for the type name at the top of your
file like this:

```mustache
{{% "color" : "rgba({r}, {g}, {b}, {a})" %}}

.some-class {
    background-color: {{my_color}}
}
```

The general format for format strings is exactly the same as that which is used by [Rust's standard library](https://doc.rust-lang.org/std/fmt/),
and you can insert any of the properties defined for the type you're working with. A list can be found
in the following section.

## Formattable Types

For now, the only formattable type is `niji.Color`. It exposes the following properties:

| Name | Description                                         |
| ---- | --------------------------------------------------- |
| `r`  | The red component as an integer between 0 and 255   |
| `g`  | The green component as an integer between 0 and 255 |
| `b`  | The blue component as an integer between 0 and 255  |
| `a`  | The alpha component as an integer between 0 and 255 |
| `rx` | The red component as two hexadecimal digits         |
| `gx` | The green component as two hexadecimal digits       |
| `bx` | The blue component as two hexadecimal digits        |
| `ax` | The alpha component as two hexadecimal digits       |
| `rf` | The red component as a float between 0 and 1        |
| `gf` | The green component as a float between 0 and 1      |
| `bf` | The blue component as a float between 0 and 1       |
| `af` | The alpha component as a float between 0 and 1      |
