# Installing Fontspector

* First, do you need to install Fontspector? We provide a [web-based version](https://fonttools.github.io/fontspector/) with 99% of the functionality of the command line version? It runs entirely in your browser and no fonts are uploaded anywhere.

If you want to test *lots* of fonts, you have two options for installing the CLI tool - you can download our binaries, or you can compile your own.

## Download our binaries

- Official release binaries are available from the [GitHub releases page](https://github.com/fonttools/fontspector/releases/latest). Look under the "Assets" tab at the bottom and download the appropriate file for your architecture and computer:

  - Choose `aarch64-apple-darwin` for Apple Silicon users.
  - Choose `x86_64-apple-darwin` for Intel Mac users.
  - Choose `x86_64-pc-windows-gnu` if you're on Windows.
  - Choose `x86_64-unknown-linux-gnu` on Linux.

Once you have downloaded the archive, unpack it. It should contain a single file. Place this anywhere in your path. 
- On Mac and Linux you might want to put it into `/usr/local/bin/`, or `~/bin` if you have that in your path. You can find what locations are in your path with the terminal command `echo $PATH`. This is a list of locations separated by colons.
- On Mac, if you see `/usr/local/bin/` in the path list, you can use `open /usr/local/bin/` to open this folder in Finder, and then you can drag and drop the executable file into it.
- On Windows, you could put it in `C:\Users\Yourname\AppData\Local\Programs`.

- Altenatively, you can download _and_ install with `cargo-binstall fontspector`. If you don't have `cargo-binstall`, you need to install it first:
  - `brew install cargo-binstall` on macOS Homebrew
  - On Linux, or if you don't like Homebrew, using the bash script: `curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash`
  - On Windows, run `Set-ExecutionPolicy Unrestricted -Scope Process; iex (iwr "https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.ps1").Content`

Once you have downloaded and installed Fontspector by either of these methods, go to [First Run](#first-run) below.

## Build from source

You shouldn't need to build Fontspector from source, but here are some reasons why you might:

* You want to work on developing Fontspector itself.
* You want to turn on some of Fontspector's non-default features: the ability to run Fontbakery checks, or the ability to dump font reports into a SQL database.

If you want to get the very latest Fontspector builds, perhaps to pick up some new checks that haven't made it into a release yet, you still don't need to build from source! In the same way that GitHub releases provide pre-compiled binaries, every time new code is added to Fontspector, it's tested, built and packaged by GitHub's continuous integration system. You can get hold of these packages by clicking on the latest entry in [this list](https://github.com/fonttools/fontspector/actions/workflows/rust.yml?query=branch%3Amain) and downloading the package from the "Artifacts" tab at the bottom of the page you want to.

If I haven't put you off so far and you're determined to build from source, you'll need a Rust compiler installed. If you don't have one, you can use `brew install rustup` on macOS Homebrew; on Linux, Windows, or if you don't like Homebrew, go to [rustup](https://rustup.rs) to install one. 

You'll also need to pre-install the [protobuf.dev](https://protobuf.dev) package. Instructions for how to do this are on the [protobuf GitHub project](https://github.com/protocolbuffers/protobuf#protobuf-compiler-installation), but Homebrew or other package managers should have a "protobuf" or "protoc" package you can install. Hey, you're building from source, you're a hacker now, so you can work it out.

* You can build the latest release from source with `cargo install --release fontspector`. (The `--release` flag makes a "release" - i.e. fast - binary, as opposed to a development binary which is quicker to compile but runs more slowly.)
* You can build the very latest source with `cargo install --release git+https://github.com/fonttools/fontspector`.

* To turn on the Python feature, add `--features python`. You can then use the `--use-python` flag at runtime to cause checks registered with Fontbakery to be run in Fontspector if no Rust implementation is available.
* To turn on the database feature, add `--features duckdb`. You can then use `--duckdb file.db` to log reports to a DuckDB database.

## First run

Now you have a `fontspector` binary installed somewhere in your system, ready to go. The single `fontspector` binary contains all the built-in checks, profiles, and HTML/Markdown templates.

Please note that this is a command-line binary, meant to be used inside a terminal. In particular, on Windows, if you double-click on the executable file, nothing will happen! Open a terminal window and run `fontspector` frm there.

To check everything is working, run `fontspector --list-checks`. This should output a list of registered checks.

**Note for macOS users:** If you are using a downloaded binary and the command fails, it may be because the binary is not code-signed. You may see a warning that "fontspector can't be opened because it is from an unidentified developer." To fix this, open your `System Settings` application, go to the `Privacy & Security` section, and you should see a message about `fontspector` being blocked under the "Security" heading. Click the "Open Anyway" button to grant permission for it to run. You only need to do this once. (Alternatively, you can right-click the executable and select “Open,” then select “Open” again, in the warning popup.)

If the command still does not work, check that fontspector is installed and in your path; if that doesn't work, please [open an issue](https://github.com/fonttools/fontspector/issues), making sure to state how you obtained your Fontspector binary, where you put it, how you ran it, what output you got from the computer, what operating system you are using, and any other details which might be helpful to debug the problem.

Once everything is working, you can now go to the [user's guide](USING.md).

## Special case: Running from a GitHub Actions CI

We provide a GitHub action called [fonttools/setup-fontspector](https://github.com/fonttools/setup-fontspector/) which can be used to automate the above inside a GH Actions pipeline. See the README of setup-fontspector for more details.
