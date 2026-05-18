# Plugins

Fontspector plugins are standalone executables. Fontspector discovers them by running a small subprocess protocol.

## Using plugins

1. Make sure your plugin executable can be launched.
2. Either:
   - Put it on your PATH and pass its command name, or
   - Pass an absolute/relative executable path.
3. Run fontspector with one or more --plugin values.
4. Choose a profile with --profile.

Important: loading a plugin does not automatically switch profile. If your plugin defines profile myprofile, you must pass --profile myprofile.

Examples:

```bash
# Single plugin path
fontspector --plugin ./myplugin --profile myprofile -L

# Plugin found in PATH
fontspector --plugin myplugin --profile myprofile -L

# Multiple plugins (comma-separated)
fontspector --plugin ./plugin_a,./plugin_b --profile myprofile -L
```

## Writing plugins in Rust

See the working example plugin in profile-testplugin.

Recommended crate layout is a mixed lib/bin crate:

- src/lib.rs: check definitions and registration logic
- src/main.rs: tiny executable entrypoint

Cargo.toml shape:

```toml
[lib]
name = "my_plugin"

[[bin]]
name = "myplugin-fontspectorplugin"
path = "src/main.rs"

[dependencies]
fontspector-checkapi = { path = "../fontspector-checkapi" }
```

In src/lib.rs:

1. Define checks using the check helper macro from the prelude.
2. Implement fontspector_checkapi::ProfileProvider for your plugin type.
3. Register filetypes/checks/profiles in register().

```rust
use fontspector_checkapi::prelude::*;

pub struct MyPlugin;

#[check(
    id = "myplugin/example",
    title = "Example check",
    rationale = "Shows plugin checks",
    proposal = "https://example.com"
)]
fn example_check(_t: &Testable, _ctx: &Context) -> CheckFnResult {
    Ok(Status::just_one_pass())
}

impl fontspector_checkapi::ProfileProvider for MyPlugin {
    fn register(&self, registry: &mut Registry) -> Result<(), FontspectorError> {
        registry.register_simple_profile("myprofile", vec![example_check])
    }
}
```

In src/main.rs:

```rust
use fontspector_checkapi::plugin::plugin_main;
use my_plugin::MyPlugin;

fn main() {
    plugin_main(MyPlugin);
}
```

Build and run:

```bash
cargo build -p your-plugin-crate
fontspector --plugin /path/to/myplugin-fontspectorplugin --profile myprofile -L
```

## Writing plugins in Python

This repository includes a helper runtime in python/fontspectorapi.py and an example in python/exampleplugin.py.

Authoring model:

1. Import status constants, Message, check decorator, and plugin_main.
2. Decorate check functions with @check(...).
3. Yield statuses in a FontBakery-like style:
   - yield PASS
   - yield PASS, "message"
   - yield FAIL, Message("code", "message")
4. Register checks and profiles in register(plugin).
5. Call plugin_main(register, plugin_name="...") from __main__.

Minimal example:

```python
#!/usr/bin/env python3
from fontspectorapi import PASS, Message, check, plugin_main

@check(
    id="python/example",
    title="Example",
    rationale="Example check",
    proposal="https://example.com",
)
def example(font_file, context):
    yield PASS, Message("ok", f"Checked {font_file}")


def register(plugin):
    plugin.register_simple_profile("python-example", [example])


if __name__ == "__main__":
    raise SystemExit(plugin_main(register, plugin_name="python-example-plugin"))
```

Collection checks are supported by setting runs_on_collection=True in @check and accepting a list of files.

Explicit profiles are also supported via ProfileDefinition, including include_profiles and exclude_checks.

```python
from fontspectorapi import ProfileDefinition

plugin.register_profile(
    "python-example",
    ProfileDefinition(
        sections={"My Section": ["python/example"]},
        include_profiles=["base-profile"],
        exclude_checks=["python/skip-me"],
    ),
)
```

## Plugin API protocol (language-agnostic)

Any language can implement this protocol as long as the executable behavior and JSON payloads match.

### Commands

A plugin executable must support:

- --metadata
- --check CHECK_ID FILE [FILE ...]

Compatibility note: the host also accepts legacy subcommands metadata and check CHECK_ID FILE....

### Metadata response

--metadata must print JSON to stdout with this shape:

```json
{
  "api_version": 1,
  "plugin_name": "my-plugin",
  "profiles": {
    "myprofile": {
      "sections": {
        "Section Name": ["check/id"]
      },
      "include_profiles": [],
      "exclude_checks": [],
      "overrides": {},
      "configuration_defaults": {}
    }
  },
  "checks": [
    {
      "id": "check/id",
      "title": "Human title",
      "rationale": "Why this exists",
      "proposal": ["https://link"],
      "applies_to": "TTF",
      "runs_on_collection": false,
      "metadata": {},
      "hotfix_available": false,
      "sourcefix_available": false
    }
  ],
  "filetypes": {
    "TOML": "*.toml"
  }
}
```

### Check response

--check must print JSON to stdout with this shape:

```json
{
  "check_id": "check/id",
  "check_name": "Human title",
  "check_rationale": "Why this exists",
  "filename": "path/to/file.ttf",
  "section": "plugin",
  "subresults": [
    {
      "severity": "PASS",
      "code": "optional-code",
      "message": "optional-message",
      "metadata": []
    }
  ],
  "worst_status": "PASS",
  "hotfix_available": false,
  "sourcefix_available": false
}
```

For collection checks, set filename to null.

Valid severity values:

- SKIP
- INFO
- PASS
- WARN
- FAIL
- FATAL
- ERROR

### Protocol rules

1. Write only JSON to stdout for successful protocol responses.
2. Write diagnostics/errors to stderr.
3. Exit code 0 for success; non-zero for failures.
4. For single-file checks, require exactly one FILE.
5. For collection checks, accept one or more FILE values.

## Reference implementations in this repository

- Rust plugin example: profile-testplugin
- Python helper API: python/fontspectorapi.py
- Python plugin example: python/exampleplugin.py
