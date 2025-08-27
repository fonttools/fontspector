# Using fontspector

## Command line options

## Profiles

## Configuration file

Some command-line parameters can be configured in a configuration file.
This may be in [TOML](https://toml.io/en/) format, and the name of this file is passed in
on the fontspector command line with the `--configuration` parameter.

## Configuring checks to run

- Instead of using `-c` to specify checks, a list of checks can be provided using the `explicit_checks` key:

```toml
explicit_checks = [
    'opentype/family/underline_thickness',
    'opentype/family/panose_familytype',
]
```

- Instead of using `-x` to exclude checks, a list of checks to exclude can be provided using the `exclude_checks` key.

- Individual checks can be skipped _for particular files_ by providing an option called `exclude_files` to the [check options](#providing-options-to-checks). The value of the `exclude_files` option must be a _list_ of _basenames_ of the files to skip. For example:

```toml
[has_HVAR]
exclude_files = [
    "MyFont-VF.ttf"
]
```

- Likewise, providing an option called `explicit_files` to the check options for a check will _only_ run the check for files mentioned in the list.

## Overriding check status

Additionally, the configuration file can be used to replace the status of
particular checks. To do this, you will need to know the _message ID_,
which is reported with the result. For example, when the
`mandatory_glyphs` check reports that the `.notdef`
glyph does not contain any outlines, it reports the message ID `empty` and
a `WARN` status. To replace this status and have it return a `FAIL` instead,
place this in the configuration file (if you are using YAML format):

```
overrides:
  mandatory_glyphs:
    empty: FAIL
```

## Providing options to checks

Individual checks and profiles may give semantics to additional configuration values;
the whole configuration file is passed to checks which request access to it.
Currently supported values include:

- `is_icon_font`: A boolean value stating whether the fonts provided are icon fonts.
  (overriding a check on the PANOSE values of those fonts) Certain checks will be
  skipped for icon fonts.
