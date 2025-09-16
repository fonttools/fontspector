# Using Fontspector

Fontspector is a tool for quality control of OpenType fonts. It is a font testing framework written in Rust that helps you to find and fix problems in your fonts. This guide explains how to use the `fontspector` command-line tool.

## Command line options

*   `--plugins <PLUGINS>`: Plugins to load.
*   `-p, --profile <PROFILE>`: Profile to check (default: universal).
*   `-L, --list-checks`: List the checks available in the selected profile.
*   `--list-checks-json`: List checks in JSON format.
*   `--configuration <CONFIGURATION>`: Read configuration file (TOML/YAML).
*   `-c, --checkid <CHECKID>`: Explicit check-ids (or parts of their name) to be executed.
*   `-x, --exclude-checkid <EXCLUDE_CHECKID>`: Exclude check-ids (or parts of their name) from execution.
*   `--full-lists`: Report full lists of items instead of abbreviated lists.
*   `-e, --error-code-on <STATUS>`: Threshold for emitting process error code 1. (default: fail). Possible values: `skip`, `pass`, `info`, `warn`, `fail`, `error`.
*   `-v, --verbose...`: Increase logging.
*   `-l, --loglevel <LOGLEVEL>`: Log level (default: warn). Possible values: `skip`, `pass`, `info`, `warn`, `fail`, `error`.
*   `-q, --quiet`: Be quiet, don’t report anything on the terminal.
*   `--succinct`: This is a slightly more compact and succinct output layout.
*   `--timeout <TIMEOUT>`: Timeout (in seconds) for network operations.
*   `--skip-network`: Skip network checks.
*   `--json <JSON>`: Write a JSON formatted report to the given filename.
*   `--csv <CSV>`: Write a CSV formatted report to the given filename.
*   `--ghmarkdown <GHMARKDOWN>`: Write a GitHub-Markdown formatted report to the given filename.
*   `--html <HTML>`: Write a HTML formatted report to the given filename.
*   `--update-templates`: Copy bundled templates to user template directory.
*   `--badges <BADGES>`: Write JSON badges to the given directory.
*   `--hotfix`: Hotfix found problems in the binaries.
*   `--fix-sources`: Fix sources.
*   `<INPUTS>...`: Input fonts to test.

For example:

```
fontspector --html report.html --profile googlefonts *.ttf
```

will check all the `.ttf` files in the current directory against the `googlefonts` profile (explained below) and write a report in HTML report to `report.html`.

## Profiles

A profile is a collection of checks, in a particular order and organised into sections, which you might want to use to perform QA on your fonts. For example, the `opentype` profile checks that your font is compliant with the OpenType Specification; other profiles contain more checks. Profiles can be built in to Fontspector, loaded at runtime, or custom defined.

You can select a profile using the `--profile` command line argument to Fontspector. You are not limited to the sets of checks which come with Fontspector; you can use the profiles as a starting point and additionally customize the selection of checks by either specifying or excluding checks, as detailed under [Configuration file](#configuration-file) below, or you can determine your own set of checks altogether by writing your own [TOML profile](#toml-profiles).

### Built-in profiles

The fontspector binary currently ships with the following profiles built in:

* `--profile opentype`: Runs checks which verify compliance with the OpenType Specification.
* `--profile universal`: Runs additional checks which Fontbakery and Fontspector community members have found useful "best practices", but which do not enforce any foundry-specific behaviour.
* `--profile googlefonts`: Runs checks which ensure that fonts comply with the [Google Fonts Guide](https://googlefonts.github.io/gf-guide/). (Note: This profile embeds expectations which may not be universally applicable. For example, fonts will fail if they contain fvar instances which do not match those served by the GF font servers. This doesn't mean your font is *wrong*, it just means it isn't what Google Fonts expects. So only use this profile if you are planning on submitting a font to Google Fonts.)
* `--profile iso15008`: Runs checks which ensure fonts comply with the ISO15008 standard for in-car displays.
* `--profile fontwerk`: Runs checks to verify compliance with the expectations of the Fontwerk foundry.

### External profiles

Some profiles such as the Microsoft profile require additional tests in Rust
to be registered with Fontspector. This is done through plugins, which are
dynamic libraries containing Rust code which get loaded at runtime.

These profiles need to be build from Rust sources before being used. See the [installation guide](INSTALLATION.md#build-from-source) for preliminary requirements for building from Rust sources.

The easiest way to build these profiles is to use `cargo-cp-artifact`, a Javascript utility.
To do this:

```
npm install
rpm run build-microsoft # build-adobe, build-test...
```

This will produce a file called `microsoft.fontspectorplugin`; to use this, run

```
fontspector \
    --plugins microsoft.fontspectorplugin \ # This loads the code
    --profile microsoft \                   # This uses the profile defined in the plugin
    MyFont.ttf
```

### TOML profiles

Additionally, sets of checks can be defined in a [TOML](https://toml.io/en/) file. For example, checks to be used by the Fontbureau foundry can be found in [`fontbureau.toml`](https://github.com/fonttools/fontspector/blob/main/profiles/fontbureau.toml) in the Fontspector source repository.

These TOML files can be passed in on the `--profile` argument just like built-in profile names:

```
fontspector --profile /path/to/fontbureau.toml ... files ...
```

### Writing your own profiles



## Configuration file

Some command-line parameters can be configured in a configuration file.
This may be in [TOML](https://toml.io/en/) format, and the name of this file is passed in
on the fontspector command line with the `--configuration` argument.

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
