# Fontspector

![](fontspector-web/www/Fontspector.svg)

Fontspector is a command-line tool for checking the quality of font projects.
It is a Rust port of [fontbakery](http://github.com/fonttools/fontbakery).

## Installation

Please see the [installation guide](INSTALLATION.md) for downloading and installing Fontspector.

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

## Contributing

If you wish to contribute to the development of fontspector, you are
very welcome! Please read the [contributors guide](CONTRIBUTING.md) for
more details.

Members of the fontspector community are expected to agree with the
[contributor code of conduct](CODE_OF_CONDUCT.md). If you come across
any behaviour which does not meet our standards, please see the
reporting procedures in the code of conduct.
