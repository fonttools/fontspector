#!/usr/bin/env python3
"""Example Python fontspector subprocess plugin."""

from __future__ import annotations

import pathlib

from fontspectorapi import FAIL, PASS, Message, ProfileDefinition, check, plugin_main


@check(
    id="python/say_hello",
    title="Check that the Python plugin protocol is working",
    rationale="Demonstrates a simple single-file check in a Python plugin.",
    proposal="https://github.com/fonttools/fontspector",
    applies_to="TTF",
)
def say_hello(font_file: str, context):
    """Return PASS and include a small context-dependent message."""
    basename = pathlib.Path(font_file).name
    context.cache["last_file"] = basename
    yield PASS, Message("hello", f"Python plugin ran for {basename}")


@check(
    id="python/filename_has_ttf_suffix",
    title="Check that the filename has a .ttf suffix",
    rationale="Demonstrates FAIL output and status code/message ergonomics.",
    proposal="https://github.com/fonttools/fontspector",
    applies_to="TTF",
)
def filename_has_ttf_suffix(font_file: str, _context):
    """Fail when the file name does not end in .ttf."""
    if font_file.lower().endswith(".ttf"):
        yield PASS, "File looks like a TTF"
    else:
        yield (
            FAIL,
            Message(
                "filename-suffix",
                "Input file does not end with .ttf",
            ),
        )


@check(
    id="python/collection_file_extensions_match",
    title="Check that all files in a collection have the same extension",
    rationale="Demonstrates a collection check with runs_on_collection enabled.",
    proposal="https://github.com/fonttools/fontspector",
    runs_on_collection=True,
)
def collection_file_extensions_match(files, _context):
    """Fail when collection members use mixed filename extensions."""
    extensions = sorted({pathlib.Path(f).suffix.lower() or "<none>" for f in files})
    if len(extensions) <= 1:
        yield PASS, "Collection uses a consistent file extension"
    else:
        yield (
            FAIL,
            Message(
                "mixed-extensions",
                f"Collection has mixed extensions: {extensions}",
            ),
        )


@check(
    id="python/glyphspackage_parses",
    title="Check that a .glyphspackage file is valid",
    rationale="Demonstrates that custom directory-based filetypes can be registered and used.",
    proposal="https://github.com/fonttools/fontspector",
)
def glyphspackage_parses(file, _context):
    """Fail when the .glyphspackage file is not valid."""
    import glyphsLib

    try:
        glyphsLib.load(file)
        yield PASS, "Glyphs package is valid"
    except Exception as e:
        yield FAIL, Message("invalid-glyphspackage", str(e))


def register(plugin):
    plugin.register_check(say_hello)
    plugin.register_check(filename_has_ttf_suffix)
    plugin.register_check(collection_file_extensions_match)
    plugin.register_check(glyphspackage_parses)
    plugin.register_filetype("GLYPHSPACKAGE", "*.glyphspackage")

    plugin.register_profile(
        "python-example-base",
        ProfileDefinition(
            sections={
                "Python Example Checks": [
                    "python/say_hello",
                    "python/filename_has_ttf_suffix",
                    "python/collection_file_extensions_match",
                    "python/glyphspackage_parses",
                ]
            }
        ),
    )

    plugin.register_profile(
        "python-example",
        ProfileDefinition(
            sections={
                "Python Example Overrides": [
                    "python/say_hello",
                ]
            },
            include_profiles=["python-example-base"],
            exclude_checks=["python/filename_has_ttf_suffix"],
        ),
    )


if __name__ == "__main__":
    raise SystemExit(plugin_main(register, plugin_name="python-example-plugin"))
