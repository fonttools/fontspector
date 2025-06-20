# Fontspector

![](fontspector-web/www/Fontspector.svg)

Fontspector is a command-line tool for checking the quality of font projects.
It is a Rust port of [fontbakery](http://github.com/fonttools/fontbakery).

## Components

Fontspector is made up of multiple crates:

- `fontbakery-bridge`: Allows Python fontbakery checks to run inside fontspector
- `fontspector-checkapi`: Defines the API and utility functions for check implementations
- `fontspector-checkhelper`: Procedural macros to facilitate check implementations
- `fontspector-cli`: The main fontspector executable
- `fontspector-py`: A Python module exposing fontspector (for which see below)
- `fontspector-web`: A WASM implementation of fontspector (for which see below)
- `profile-testplugin`: An example of a runtime-loadable test profile
- `profile-googlefonts`, `profile-opentype`, `profile-universal`: Built in profiles and their check implementations
- `profile-microsoft`, `profile-adobe`, ...: Additional profiles which are loaded at runtime (see below)

## Running the CLI tool

You have two options for running the CLI tool - you can download our binaries, or you can compile your own.

- Official release binaries are available from the [GitHub releases page](https://github.com/fonttools/fontspector/releases).
- You can build the latest release from source with `cargo install fontspector`. (If you don't have a Rust compiler installed, you can use `brew install rustup` on macOS Homebrew or [rustup](https://rustup.rs) to install one.)
- You can build the bleeding-edge version with `cargo install git+https://github.com/fonttools/fontspector/`

> On macOS you'll need to pre-install the [protobuf.dev](https://protobuf.dev) package, such as with Homebrew like this:
>
> brew install protobuf
> cargo build --release;
> ./target/release/fontspector ~/font.ttf;

The single `fontspector` binary contains all the built-in checks, profiles, and HTML/Markdown templates. As mentioned below, some profiles require additional plugin binaries.

By default, fontspector CLI is built without Python support. If you want to run
fontbakery checks inside fontspector, build with `cargo build --release --features python`. You can then use the `--use-python` flag at runtime to cause checks registered with Fontbakery to be run in Fontspector if no Rust implementation is available.

## Building the web version

Fontspector also has a WASM-based web version at
https://fonttools.github.io/fontspector/

It is built and deployed from Github Actions, but should you need to
rebuild this manually for development, run:

```
cd fontspector-web
wasm-pack build
cd www; npm install; npm run build
```

The results appear in `../docs/`.

## Running the test suite

We export the Fontspector check runner to a Python module, and then use
`pytest` to run (a modified version of) the fontbakery test suite. To
do this:

```
pip3 install -U maturin
cd fontspector-py
python3 -m venv venv ; . venv/bin/activate
pip install maturin
maturin develop
pytest
```

## Building plugin profiles

Some profiles such as the Microsoft profile require additional tests in Rust
to be registered with Fontspector. This is done through plugins, which are
dynamic libraries containing Rust code which get loaded at runtime. The easiest
way to build these profiles is to use `cargo-cp-artifact`, a Javascript utility.
To do this:

```
npm install
npm run build-microsoft # build-adobe, build-test...
```

This will produce a file called `microsoft.fontspectorplugin`; to use this, run

```
fontspector \
    --plugins microsoft.fontspectorplugin \ # This loads the code
    --profile microsoft \                   # This uses the profile defined in the plugin
    MyFont.ttf
```
