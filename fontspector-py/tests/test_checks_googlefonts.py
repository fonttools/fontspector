import glob
import math
import os
from pathlib import Path
import shutil
import tempfile

import pytest
import requests
from conftest import check_id
from fontTools.ttLib import TTFont

from fontbakery.checks.vendorspecific.googlefonts.conditions import (
    expected_font_names,
)
from fontbakery.codetesting import (
    TEST_FILE,
    MockFont,
    assert_PASS,
    assert_results_contain,
    assert_SKIP,
    portable_path,
    MockContext,
)
from fontbakery.constants import (
    OFL_BODY_TEXT,
    MacintoshEncodingID,
    MacintoshLanguageID,
    NameID,
    PlatformID,
    WindowsEncodingID,
    WindowsLanguageID,
)
from fontbakery.status import DEBUG, ERROR, FAIL, FATAL, INFO, PASS, SKIP, WARN
from fontbakery.testable import Font

check_statuses = (ERROR, FAIL, SKIP, PASS, WARN, INFO, DEBUG)

OVERRIDE_SUFFIX = ""

mada_fonts = [
    TEST_FILE("mada/Mada-Black.ttf"),
    TEST_FILE("mada/Mada-ExtraLight.ttf"),
    TEST_FILE("mada/Mada-Medium.ttf"),
    TEST_FILE("mada/Mada-SemiBold.ttf"),
    TEST_FILE("mada/Mada-Bold.ttf"),
    TEST_FILE("mada/Mada-Light.ttf"),
    TEST_FILE("mada/Mada-Regular.ttf"),
]


@pytest.fixture
def mada_ttFonts():
    return [TTFont(path) for path in mada_fonts]


cabin_fonts = [
    TEST_FILE("cabin/Cabin-BoldItalic.ttf"),
    TEST_FILE("cabin/Cabin-Bold.ttf"),
    TEST_FILE("cabin/Cabin-Italic.ttf"),
    TEST_FILE("cabin/Cabin-MediumItalic.ttf"),
    TEST_FILE("cabin/Cabin-Medium.ttf"),
    TEST_FILE("cabin/Cabin-Regular.ttf"),
    TEST_FILE("cabin/Cabin-SemiBoldItalic.ttf"),
    TEST_FILE("cabin/Cabin-SemiBold.ttf"),
]

cabin_condensed_fonts = [
    TEST_FILE("cabincondensed/CabinCondensed-Regular.ttf"),
    TEST_FILE("cabincondensed/CabinCondensed-Medium.ttf"),
    TEST_FILE("cabincondensed/CabinCondensed-Bold.ttf"),
    TEST_FILE("cabincondensed/CabinCondensed-SemiBold.ttf"),
]

rosarivo_fonts = [
    TEST_FILE("rosarivo_metadata/Rosarivo-Italic.ttf"),
    TEST_FILE("rosarivo_metadata/Rosarivo-Regular.ttf"),
]

cjk_font = TEST_FILE("cjk/NotoSansJP[wght].ttf")


@pytest.fixture
def cabin_ttFonts():
    return [TTFont(path) for path in cabin_fonts]


@pytest.fixture
def vf_ttFont():
    path = TEST_FILE("varfont/Oswald-VF.ttf")
    return TTFont(path)


def change_name_table_id(ttFont, nameID, newEntryString, platEncID=0):
    for i, nameRecord in enumerate(ttFont["name"].names):
        if nameRecord.nameID == nameID and nameRecord.platEncID == platEncID:
            ttFont["name"].names[i].string = newEntryString


def delete_name_table_id(ttFont, nameID):
    delete = []
    for i, nameRecord in enumerate(ttFont["name"].names):
        if nameRecord.nameID == nameID:
            delete.append(i)
    for i in sorted(delete, reverse=True):
        del ttFont["name"].names[i]


@pytest.fixture
def cabin_regular_path():
    return portable_path("data/test/cabin/Cabin-Regular.ttf")


def fake_mdpb(tmp_path, md):
    _fake_mdpb = tmp_path / "METADATA.pb"
    from google.protobuf import text_format

    textproto = text_format.MessageToString(md, as_utf8=True)
    _fake_mdpb.write_text(textproto, encoding="utf-8")
    return str(_fake_mdpb)


def read_mdpb(md):
    from fontbakery.checks.vendorspecific.googlefonts.utils import (
        get_FamilyProto_Message,
    )

    return get_FamilyProto_Message(md)


@check_id("googlefonts/description/broken_links")
def test_check_description_broken_links(check, tmp_path):
    """Does DESCRIPTION file contain broken links ?"""

    font = TEST_FILE("cabin/DESCRIPTION.en_us.html")
    assert_PASS(check(font), "with description file that has no links...")

    p = tmp_path / "DESCRIPTION.en_us.html"

    good_desc = open(font).read()
    good_desc += (
        "<a href='http://example.com'>Good Link</a>"
        "<a href='http://fonts.google.com'>Another Good One</a>"
    )
    p.write_text(good_desc, encoding="utf-8")

    assert_PASS(
        check(str(p)),
        "with description file that has good links...",
    )

    bad_desc = (
        good_desc + "<a href='mailto:juca@members.fsf.org'>An example mailto link</a>"
    )
    p.write_text(bad_desc, encoding="utf-8")

    assert_results_contain(
        check(str(p)),
        FAIL,
        "email",
        'with a description file containing "mailto" links...',
    )

    bad_desc = (
        good_desc
        + "<a href='http://thisisanexampleofabrokenurl.com/'>This is a Bad Link</a>"
    )
    p.write_text(bad_desc, encoding="utf-8")

    assert_results_contain(
        check(str(p)),
        FAIL,
        "broken-links",
        "with a description file containing a known-bad URL...",
    )

    # Sadly we can't currently mock a timeout

    # bad_desc = (
    #     good_desc
    #     + "<a href='http://timeout.example.invalid/'>This is a link that times out</a>"
    # )
    # p.write_text(bad_desc, encoding="utf-8")

    # assert_results_contain(
    #     check(str(p)),
    #     WARN,
    #     "timeout",
    #     "with a description file containing a URL that times out...",
    # )


@check_id("googlefonts/metadata/validate")
def test_check_metadata_validate(check):
    """Check METADATA.pb parse correctly."""

    good = TEST_FILE("stixtwomath/METADATA.pb")
    assert_PASS(check(good), "with a good METADATA.pb file...")

    bad = TEST_FILE("broken_metadata/METADATA.pb")
    assert list(check(bad))[0].status == FATAL


@pytest.mark.skip("Check not ported yet.")
@check_id("googlefonts/metadata/designer_values")
def test_check_metadata_designer_values(check):
    """Multiple values in font designer field in
    METADATA.pb must be separated by commas."""

    font = TEST_FILE("merriweather/Merriweather-Regular.ttf")
    assert_PASS(check(font), "with a good METADATA.pb file...")

    md = Font(font).family_metadata
    md.designer = "Pentagram, MCKL"
    assert_PASS(
        check(MockFont(file=font, family_metadata=md)),
        "with a good multiple-designers string...",
    )

    md.designer = "Pentagram / MCKL"  # This actually happened on an
    # early version of the Red Hat Text family
    assert_results_contain(
        check(MockFont(file=font, family_metadata=md)),
        FAIL,
        "slash",
        "with a bad multiple-designers string (names separated by a slash char)...",
    )


@check_id("googlefonts/metadata/subsets_correct")
def test_check_metadata_includes_production_subsets(check, tmp_path):
    """Check METADATA.pb has production subsets."""

    mdpb = TEST_FILE("cabinvf/METADATA.pb")
    fonts = [
        TEST_FILE("cabinvf/Cabin[wdth,wght].ttf"),
        TEST_FILE("cabinvf/Cabin-Italic[wdth,wght].ttf"),
    ]

    assert_PASS(check([mdpb] + fonts), "with a good METADATA.pb for this family...")

    md = read_mdpb(mdpb)
    # Then we induce the problem by removing a subset:
    md.subsets.pop()
    assert_results_contain(
        check([fake_mdpb(tmp_path, md)] + fonts),
        FAIL,
        "missing-subsets",
        "with a bad METADATA.pb (last subset has been removed)...",
    )


# note: The copyright checks do not actually verify that the project name is correct.
#       They only focus on the string format.
GOOD_COPYRIGHT_NOTICE_STRINGS = (
    (
        "Copyright 2017 The Archivo Black Project Authors"
        " (https://github.com/Omnibus-Type/ArchivoBlack)"
    ),
    (
        "Copyright 2017-2018 The YearRange Project Authors"
        " (https://github.com/Looks/Good)"
    ),
    (
        "Copyright 2012-2014, 2016, 2019-2021, 2023 The MultiYear Project Authors"
        " (https://github.com/With/ManyRanges)"
    ),
    # We also ignore case, so these should also PASS:
    (
        "COPYRIGHT 2017 THE ARCHIVO BLACK PROJECT AUTHORS"
        " (HTTPS://GITHUB.COM/OMNIBUS-TYPE/ARCHIVOBLACK)"
    ),
    (
        "copyright 2017 the archivo black project authors"
        " (https://github.com/omnibus-type/archivoblack)"
    ),
)


@check_id("googlefonts/font_copyright")
def test_check_font_copyright(check, tmp_path):
    """Copyright notice on METADATA.pb matches canonical pattern ?"""

    # Our reference Cabin Regular is known to be bad
    # Since it provides an email instead of a git URL.
    # Also the check should work fine without a METADATA.pb file.
    font = TEST_FILE("cabin/Cabin-Regular.ttf")
    assert_results_contain(
        check(font), FAIL, "bad-notice-format", "with a bad copyright notice string..."
    )

    ttFont = TTFont(font)

    # Then, to make the check PASS, we change it into a few good strings:
    for good_string in GOOD_COPYRIGHT_NOTICE_STRINGS:
        for i, entry in enumerate(ttFont["name"].names):
            if entry.nameID == NameID.COPYRIGHT_NOTICE:
                ttFont["name"].names[i].string = good_string.encode(entry.getEncoding())

        md = Font(font).family_metadata
        md.fonts[0].copyright = good_string

        assert_PASS(
            check([ttFont, fake_mdpb(tmp_path, md)]),
            "with a good copyright notice string...",
        )

        too_long = good_string + "x" * (501 - len(good_string))
        md.fonts[0].copyright = too_long
        for i, entry in enumerate(ttFont["name"].names):
            if entry.nameID == NameID.COPYRIGHT_NOTICE:
                ttFont["name"].names[i].string = too_long.encode(entry.getEncoding())

        assert_results_contain(
            check([ttFont, fake_mdpb(tmp_path, md)]),
            FAIL,
            "max-length",
            "with a 501-char copyright notice string...",
        )

    # Now let's make them different
    md.fonts[0].copyright = good_string
    assert_results_contain(
        check([ttFont, fake_mdpb(tmp_path, md)]),
        FAIL,
        "mismatch",
        "with a bad METADATA.pb (with a copyright string not matching this font)...",
    )


# FIXME!
# GFonts hosted Cabin files seem to have changed in ways
# that break some of the assumptions in the code-test below.
# More info at https://github.com/fonttools/fontbakery/issues/2581
@pytest.mark.xfail(strict=True)
@pytest.mark.skip("Check not ported yet.")
@check_id("googlefonts/production_encoded_glyphs")
def test_check_production_encoded_glyphs(check, cabin_ttFonts):
    """Check glyphs are not missing when compared to version on fonts.google.com"""

    for font in cabin_fonts:
        # Cabin font hosted on fonts.google.com contains
        # all the glyphs for the font in data/test/cabin
        assert_PASS(check(font), f"with '{font}'")

        ttFont = TTFont(font)
        # Take A glyph out of font
        ttFont["cmap"].getcmap(3, 1).cmap.pop(ord("A"))
        ttFont["glyf"].glyphs.pop("A")
        assert_results_contain(check(ttFont), FAIL, "lost-glyphs")


@pytest.mark.parametrize(
    """fp,mod,result""",
    [
        # tests from test_check_name_familyname:
        (TEST_FILE("cabin/Cabin-Regular.ttf"), {}, PASS),
        (
            TEST_FILE("cabin/Cabin-Regular.ttf"),
            {NameID.FONT_FAMILY_NAME: "Wrong"},
            FAIL,
        ),
        (TEST_FILE("overpassmono/OverpassMono-Regular.ttf"), {}, PASS),
        (TEST_FILE("overpassmono/OverpassMono-Bold.ttf"), {}, PASS),
        (TEST_FILE("overpassmono/OverpassMono-Regular.ttf"), {1: "Foo"}, FAIL),
        (TEST_FILE("merriweather/Merriweather-Black.ttf"), {}, PASS),
        (TEST_FILE("merriweather/Merriweather-LightItalic.ttf"), {}, PASS),
        (
            TEST_FILE("merriweather/Merriweather-LightItalic.ttf"),
            {NameID.FONT_FAMILY_NAME: "Merriweather Light Italic"},
            FAIL,
        ),
        (TEST_FILE("abeezee/ABeeZee-Regular.ttf"), {}, PASS),
        # tests from test_check_name_subfamilyname
        (TEST_FILE("overpassmono/OverpassMono-Regular.ttf"), {}, PASS),
        (TEST_FILE("overpassmono/OverpassMono-Bold.ttf"), {}, PASS),
        (TEST_FILE("merriweather/Merriweather-Black.ttf"), {}, PASS),
        (TEST_FILE("merriweather/Merriweather-LightItalic.ttf"), {}, PASS),
        (TEST_FILE("montserrat/Montserrat-BlackItalic.ttf"), {}, PASS),
        (TEST_FILE("montserrat/Montserrat-Black.ttf"), {}, PASS),
        (TEST_FILE("montserrat/Montserrat-BoldItalic.ttf"), {}, PASS),
        (TEST_FILE("montserrat/Montserrat-Bold.ttf"), {}, PASS),
        (TEST_FILE("montserrat/Montserrat-ExtraBoldItalic.ttf"), {}, PASS),
        (TEST_FILE("montserrat/Montserrat-ExtraBold.ttf"), {}, PASS),
        (TEST_FILE("montserrat/Montserrat-ExtraLightItalic.ttf"), {}, PASS),
        (TEST_FILE("montserrat/Montserrat-ExtraLight.ttf"), {}, PASS),
        (TEST_FILE("montserrat/Montserrat-Italic.ttf"), {}, PASS),
        (TEST_FILE("montserrat/Montserrat-LightItalic.ttf"), {}, PASS),
        (TEST_FILE("montserrat/Montserrat-Light.ttf"), {}, PASS),
        (TEST_FILE("montserrat/Montserrat-MediumItalic.ttf"), {}, PASS),
        (TEST_FILE("montserrat/Montserrat-Medium.ttf"), {}, PASS),
        (TEST_FILE("montserrat/Montserrat-Regular.ttf"), {}, PASS),
        (TEST_FILE("montserrat/Montserrat-SemiBoldItalic.ttf"), {}, PASS),
        (TEST_FILE("montserrat/Montserrat-SemiBold.ttf"), {}, PASS),
        (TEST_FILE("montserrat/Montserrat-ThinItalic.ttf"), {}, PASS),
        (TEST_FILE("montserrat/Montserrat-Thin.ttf"), {}, PASS),
        (
            TEST_FILE("montserrat/Montserrat-ThinItalic.ttf"),
            {NameID.FONT_SUBFAMILY_NAME: "Not a proper style"},
            FAIL,
        ),
        # tests from test_check_name_fullfontname
        (TEST_FILE("cabin/Cabin-Regular.ttf"), {}, PASS),
        # warn should be raised since full name is missing Regular
        (TEST_FILE("cabin/Cabin-Regular.ttf"), {4: "Cabin"}, WARN),
        (TEST_FILE("cabin/Cabin-BoldItalic.ttf"), {}, PASS),
        (
            TEST_FILE("cabin/Cabin-BoldItalic.ttf"),
            {NameID.FULL_FONT_NAME: "Make it fail"},
            FAIL,
        ),
        (TEST_FILE("abeezee/ABeeZee-Regular.ttf"), {}, PASS),
        # tests from test_check_name_typographicfamilyname
        (TEST_FILE("montserrat/Montserrat-BoldItalic.ttf"), {}, PASS),
        (
            TEST_FILE("montserrat/Montserrat-BoldItalic.ttf"),
            {NameID.TYPOGRAPHIC_FAMILY_NAME: "Arbitrary name"},
            FAIL,
        ),
        (TEST_FILE("montserrat/Montserrat-ExtraLight.ttf"), {}, PASS),
        (
            TEST_FILE("montserrat/Montserrat-ExtraLight.ttf"),
            {NameID.TYPOGRAPHIC_FAMILY_NAME: "Foo"},
            FAIL,
        ),
        (
            TEST_FILE("montserrat/Montserrat-ExtraLight.ttf"),
            {NameID.TYPOGRAPHIC_FAMILY_NAME: None},
            FAIL,
        ),
        # tests from test_check_name_typographicsubfamilyname
        (TEST_FILE("montserrat/Montserrat-BoldItalic.ttf"), {}, PASS),
        (
            TEST_FILE("montserrat/Montserrat-BoldItalic.ttf"),
            {NameID.TYPOGRAPHIC_SUBFAMILY_NAME: "Foo"},
            FAIL,
        ),
        (TEST_FILE("montserrat/Montserrat-ExtraLight.ttf"), {}, PASS),
        (
            TEST_FILE("montserrat/Montserrat-ExtraLight.ttf"),
            {NameID.TYPOGRAPHIC_SUBFAMILY_NAME: None},
            FAIL,
        ),
        (
            TEST_FILE("montserrat/Montserrat-ExtraLight.ttf"),
            {NameID.TYPOGRAPHIC_SUBFAMILY_NAME: "Generic Name"},
            FAIL,
        ),
        # variable font checks
        (TEST_FILE("cabinvf/Cabin[wdth,wght].ttf"), {}, PASS),
        # Open Sans' origin is Light so this should pass
        (
            TEST_FILE("varfont/OpenSans[wdth,wght].ttf"),
            {
                NameID.FONT_SUBFAMILY_NAME: "Regular",
                NameID.TYPOGRAPHIC_SUBFAMILY_NAME: "Light",
            },
            PASS,
        ),
        (
            TEST_FILE("varfont/OpenSans[wdth,wght].ttf"),
            {
                NameID.FONT_SUBFAMILY_NAME: "Regular",
                NameID.TYPOGRAPHIC_SUBFAMILY_NAME: "Condensed Light",
            },
            FAIL,
        ),
        (TEST_FILE("varfont/RobotoSerif[GRAD,opsz,wdth,wght].ttf"), {}, FAIL),
        # Roboto Serif has an opsz axes so this should pass
        (
            TEST_FILE("varfont/RobotoSerif[GRAD,opsz,wdth,wght].ttf"),
            {
                NameID.FONT_FAMILY_NAME: "Roboto Serif",
                NameID.FONT_SUBFAMILY_NAME: "Regular",
                NameID.FULL_FONT_NAME: "Roboto Serif Regular",
                NameID.POSTSCRIPT_NAME: "RobotoSerif-Regular",
                NameID.TYPOGRAPHIC_FAMILY_NAME: None,
                NameID.TYPOGRAPHIC_SUBFAMILY_NAME: None,
            },
            PASS,
        ),
        (TEST_FILE("varfont/Georama[wdth,wght].ttf"), {}, PASS),
        # Georama's default fvar vals are wdth=62.5, wght=100
        # which means ExtraCondensed Thin should appear in the family name
        (
            TEST_FILE("varfont/Georama[wdth,wght].ttf"),
            {
                NameID.FONT_FAMILY_NAME: "Georama ExtraCondensed Thin",
                NameID.FONT_SUBFAMILY_NAME: "Regular",
                NameID.TYPOGRAPHIC_FAMILY_NAME: "Georama",
                NameID.TYPOGRAPHIC_SUBFAMILY_NAME: "ExtraCondensed Thin",
            },
            PASS,
        ),
    ],
)
@check_id("googlefonts/font_names")
def test_check_font_names(check, fp, mod, result):
    """Check font names are correct"""
    # Please note: This check was introduced in
    # https://github.com/fonttools/fontbakery/pull/3800 which has replaced
    # the following checks:
    #   googlefonts/name/familyname
    #   googlefonts/name/subfamilyname
    #   googlefonts/name/typographicfamilyname
    #   googlefonts/name/typographicsubfamilyname
    #
    # It works by simply using the nametable builder which is found in the
    # axis registry,
    # https://github.com/googlefonts/axisregistry/blob/main/Lib/axisregistry/__init__.py#L232
    # this repository already has good unit tests but this check will also include the
    # previous test cases found in fontbakery.
    # https://github.com/googlefonts/axisregistry/blob/main/tests/test_names.py

    ttFont = TTFont(fp)
    # get the expecteed font names now before we modify them
    expected = expected_font_names(ttFont, [])
    if mod:
        for k, v in mod.items():
            if v is None:
                ttFont["name"].removeNames(k)
            else:
                ttFont["name"].setName(v, k, 3, 1, 0x409)

    if result == PASS:
        assert_PASS(
            check(ttFont),
            "with a good font...",
        )
    elif result == WARN:
        assert_results_contain(
            check(ttFont),
            WARN,
            "lacks-regular",
            "with bad names",
        )
    else:
        assert_results_contain(
            check(ttFont),
            FAIL,
            "bad-names",
            "with bad names",
        )


@pytest.mark.skip("Check not ported yet.")
@check_id("googlefonts/varfont/generate_static")
def test_check_varfont_generate_static(check):
    """Check a static ttf can be generated from a variable font."""

    ttFont = TTFont(TEST_FILE("cabinvfbeta/CabinVFBeta.ttf"))
    assert_PASS(check(ttFont))

    # Mangle the coordinates of the first named instance
    # to deliberately break the variable font.
    ttFont["fvar"].instances[0].coordinates = {"fooo": 400.0, "baar": 100.0}
    msg = assert_results_contain(check(ttFont), FAIL, "varlib-mutator")
    assert "fontTools.varLib.mutator failed" in msg

    # Now delete the fvar table to exercise a SKIP result due an unfulfilled condition.
    del ttFont["fvar"]
    msg = assert_results_contain(check(ttFont), SKIP, "unfulfilled-conditions")
    assert "Unfulfilled Conditions: is_variable_font" in msg


@check_id("googlefonts/fvar_instances")
def test_check_fvar_instances__another_test(check):  # TODO: REVIEW THIS.
    """Check variable font instances."""

    ttFont = TTFont(TEST_FILE("cabinvf/Cabin[wdth,wght].ttf"))

    # rename the first fvar instance so the font is broken
    ttFont["name"].setName("foo", 258, 3, 1, 0x409)

    # So it must FAIL the check:
    assert_results_contain(
        check(ttFont), FAIL, "bad-fvar-instances", "with a bad font..."
    )

    # rename the first fvar instance so it is correct
    ttFont["name"].setName("Regular", 258, 3, 1, 0x409)

    assert_PASS(check(ttFont), "with a good font...")


@check_id("googlefonts/fvar_instances")
def test_check_fvar_instances__yet_another_test(check):  # TODO: REVIEW THIS.
    """A variable font must have named instances."""

    # ExpletusVF does have instances.
    # Note: The "broken" in the path name refers to something else.
    #       (See test_check_fvar_name_entries)
    ttFont = TTFont(TEST_FILE("cabinvf/Cabin[wdth,wght].ttf"))

    # So it must PASS the check:
    assert_PASS(check(ttFont), "with a good font...")

    # If we delete all instances, then it must FAIL:
    while len(ttFont["fvar"].instances):
        del ttFont["fvar"].instances[0]

    assert_results_contain(
        check(ttFont), FAIL, "bad-fvar-instances", "with a bad font..."
    )


@check_id("googlefonts/fvar_instances")
def test_check_fvar_instances__whats_going_on_here(check):  # TODO: REVIEW THIS.
    """Variable font weight coordinates must be multiples of 100."""

    # This copy of Markazi Text has an instance with
    # a 491 'wght' coordinate instead of 500.
    ttFont = TTFont(TEST_FILE("broken_markazitext/MarkaziText-VF.ttf"))

    # So it must FAIL the check:
    assert_results_contain(
        check(ttFont), FAIL, "bad-fvar-instances", "with a bad font..."
    )

    # Let's then change the weight coordinates to make it PASS the check:
    # instances are from 400-700 (Regular-Bold) so set start to 400
    wght_val = 400
    for i, instance in enumerate(ttFont["fvar"].instances):
        ttFont["fvar"].instances[i].coordinates["wght"] = wght_val
        wght_val += 100

    assert_PASS(check(ttFont), "with a good font...")


@check_id("googlefonts/family/italics_have_roman_counterparts")
def test_check_family_italics_have_roman_counterparts(check):
    """Ensure Italic styles have Roman counterparts."""

    fonts = [
        TEST_FILE("merriweather/Merriweather-BlackItalic.ttf"),
        TEST_FILE("merriweather/Merriweather-Black.ttf"),
        TEST_FILE("merriweather/Merriweather-BoldItalic.ttf"),
        TEST_FILE("merriweather/Merriweather-Bold.ttf"),
        TEST_FILE("merriweather/Merriweather-Italic.ttf"),
        TEST_FILE("merriweather/Merriweather-LightItalic.ttf"),
        TEST_FILE("merriweather/Merriweather-Light.ttf"),
        TEST_FILE("merriweather/Merriweather-Regular.ttf"),
    ]

    assert_PASS(check(fonts), "with a good family...")

    fonts.pop(-1)  # remove the last one, which is the Regular
    assert_results_contain(
        check(fonts),
        FAIL,
        "missing-roman",
        "with a family that has an Italic but lacks a Regular.",
    )

    shutil.copy(
        TEST_FILE("merriweather/Merriweather-Italic.ttf"),
        TEST_FILE("merriweather/MerriweatherItalic.ttf"),
    )
    fonts.append(TEST_FILE("merriweather/MerriweatherItalic.ttf"))
    assert_results_contain(
        check(fonts),
        WARN,
        "bad-filename",
        "with a family that has a non-canonical italic filename.",
    )
    os.unlink(TEST_FILE("merriweather/MerriweatherItalic.ttf"))

    # This check must also be able to deal with variable fonts!
    fonts = [
        TEST_FILE("cabinvf/Cabin-Italic[wdth,wght].ttf"),
        TEST_FILE("cabinvf/Cabin[wdth,wght].ttf"),
    ]
    assert_PASS(check(fonts), "with a good set of varfonts...")

    fonts = [TEST_FILE("cabinvf/Cabin-Italic[wdth,wght].ttf")]
    assert_results_contain(
        check(fonts),
        FAIL,
        "missing-roman",
        "with an Italic varfont that lacks a Roman counterpart.",
    )


@pytest.mark.skip("Check not ported yet.")
@check_id("googlefonts/repo/upstream_yaml_has_required_fields")
def test_check_repo_upstream_yaml_has_required_fields(check):
    """Check upstream.yaml has all required fields"""

    upstream_yaml = {
        "branch": "main",
        "files": {"TestFamily-Regular.ttf": "TestFamily-Regular.ttf"},
    }
    # Pass if upstream.yaml file contains all fields
    assert_PASS(
        check(MockFont(upstream_yaml=upstream_yaml)),
        "for an upstream.yaml which contains all fields",
    )

    # Fail if it doesn't
    upstream_yaml.pop("files")
    assert_results_contain(
        check(MockFont(upstream_yaml=upstream_yaml)),
        FAIL,
        "missing-fields",
        "for an upsream.yaml which doesn't contain all fields",
    )


@pytest.mark.skip("Check not ported yet.")
@check_id("googlefonts/repo/fb_report")
def test_check_repo_fb_report(check, tmp_path):
    """A font repository should not include FontBakery report files"""

    family_dir = tmp_path / "repo_fb_report"
    family_dir.mkdir()
    src_family = portable_path("data/test/varfont")

    shutil.copytree(src_family, family_dir, dirs_exist_ok=True)

    assert_PASS(
        check(MockFont(family_directory=family_dir)),
        "for a repo without FontBakery report files.",
    )

    assert_PASS(
        check(MockFont(family_directory=family_dir)),
        "with a json file that is not a FontBakery report.",
    )

    # Add a json file that is not a FB report
    open(os.path.join(family_dir, "something_else.json"), "w+", encoding="utf-8").write(
        "this is not a FB report"
    )

    FB_REPORT_SNIPPET = """
{
    "result": {
        "INFO": 8,
        "PASS": 81,
        "SKIP": 74,
        "WARN": 4
    },
    "sections": [
    """
    # Report files must be detected even if placed on subdirectories and the check code
    # should not rely only on filename (such as "Jura-Regular.fb-report.json")
    # but should instead inspect the contents of the file:
    open(
        os.path.join(family_dir, "jura", "static", "my_fontfamily_name.json"),
        "w+",
        encoding="utf-8",
    ).write(FB_REPORT_SNIPPET)
    assert_results_contain(
        check(MockFont(family_directory=family_dir)),
        WARN,
        "fb-report",
        "with an actual snippet of a report.",
    )


@pytest.mark.skip("Check not ported yet.")
@check_id("googlefonts/repo/zip_files")
def test_check_repo_zip_files(check, tmp_path):
    """A font repository should not include ZIP files"""

    family_dir = tmp_path / "repo_zip_files"
    family_dir.mkdir()
    src_family = portable_path("data/test/varfont")

    shutil.copytree(src_family, family_dir, dirs_exist_ok=True)

    assert_PASS(
        check(MockFont(family_directory=family_dir)), "for a repo without ZIP files."
    )

    for ext in ["zip", "rar", "7z"]:
        # ZIP files must be detected even if placed on subdirectories:
        filepath = os.path.join(family_dir, "jura", "static", f"fonts-release.{ext}")
        # create an empty file. The check won't care about the contents:
        open(filepath, "w+", encoding="utf-8")
        assert_results_contain(
            check(MockFont(family_directory=family_dir)),
            FAIL,
            "zip-files",
            f"when a {ext} file is found.",
        )
        # remove the file before testing the next one ;-)
        os.remove(filepath)


@check_id("googlefonts/vertical_metrics")
def test_check_vertical_metrics(check):
    font = TEST_FILE("akshar/Akshar[wght].ttf")
    assert_results_contain(check(font), SKIP, "already-onboarded")

    # Defeat the 'not listed_on_gfonts_api' condition.
    # linegap is not 0
    assert_results_contain(
        check(font, skip_network=True),
        FAIL,
        "bad-hhea.lineGap",
        'hhea.lineGap is "150" it should be 0',
    )
    ttFont = TTFont(font)

    # hhea sum is above 2000 -> FAIL
    ttFont["hhea"].lineGap = 0
    ttFont["OS/2"].sTypoLineGap = 0
    ttFont["hhea"].descent = -2000
    ttFont["OS/2"].sTypoDescender = -2000
    assert_results_contain(
        check(ttFont, skip_network=True),
        FAIL,
        "bad-hhea-range",
        "hhea sum is above 2000",
    )

    # hhea sum is below 1200 -> FAIL
    ttFont["hhea"].descent = 0
    ttFont["OS/2"].sTypoDescender = 0
    assert_results_contain(
        check(ttFont, skip_network=True),
        FAIL,
        "bad-hhea-range",
        "hhea sum is below 1200",
    )

    # hhea sum is above 1500 -> WARN
    ttFont["hhea"].descent = -700
    ttFont["OS/2"].sTypoDescender = -700
    assert_results_contain(
        check(ttFont, skip_network=True),
        WARN,
        "bad-hhea-range",
        "hhea sum is above 1500",
    )

    # hhea sum is in range
    ttFont["hhea"].descent = -300
    ttFont["OS/2"].sTypoDescender = -300
    assert_PASS(check(ttFont, skip_network=True))

    # reset
    def reset_metrics():
        ttFont["hhea"].ascent = 900
        ttFont["hhea"].descent = -300
        ttFont["OS/2"].sTypoAscender = 900
        ttFont["OS/2"].sTypoDescender = -300
        ttFont["hhea"].lineGap = 0
        ttFont["OS/2"].sTypoLineGap = 0
        ttFont["OS/2"].usWinAscent = 900
        ttFont["OS/2"].usWinDescent = 300

    # ascenders are negative -> FAIL
    reset_metrics()
    ttFont["OS/2"].sTypoAscender = -900
    assert_results_contain(
        check(ttFont, skip_network=True),
        FAIL,
        "typo-ascender",
        "typo ascender is negative",
    )
    reset_metrics()
    ttFont["hhea"].ascent = -900
    assert_results_contain(
        check(ttFont, skip_network=True),
        FAIL,
        "hhea-ascent",
        "hhea ascent is negative",
    )

    # descenders are positive -> FAIL
    reset_metrics()
    ttFont["OS/2"].sTypoDescender = 300
    assert_results_contain(
        check(ttFont, skip_network=True),
        FAIL,
        "typo-descender",
        "typo descender is positive",
    )
    reset_metrics()
    ttFont["hhea"].descent = 300
    assert_results_contain(
        check(ttFont, skip_network=True),
        FAIL,
        "hhea-descent",
        "hhea descent is positive",
    )

    # This can't happen in a font binary, since they're unsigned values.

    # # winascent is negative -> FAIL
    # reset_metrics()
    # ttFont["OS/2"].usWinAscent = -900
    # assert_results_contain(
    #     check(MockFont(file=font, listed_on_gfonts_api=False, ttFont=ttFont)),
    #     FAIL,
    #     "win-ascent",
    #     "OS/2.usWinAscent is negative",
    # )

    # # windescent is negative -> FAIL
    # reset_metrics()
    # ttFont["OS/2"].usWinDescent = -300
    # assert_results_contain(
    #     check(MockFont(file=font, listed_on_gfonts_api=False, ttFont=ttFont)),
    #     FAIL,
    #     "win-descent",
    #     "OS/2.usWinDescent is negative",
    # )


@check_id("googlefonts/vertical_metrics_regressions")
def test_check_vertical_metrics_regressions(check):
    def new_context():
        context = MockContext(
            testables=[Font(x) for x in cabin_fonts], config={"skip_network": False}
        )
        for testable in context.testables:
            testable.context = context
        return context

    # Cabin test family should match by default
    context = new_context()
    assert_PASS(check([TEST_FILE("cabin/Cabin-Regular.ttf")]), "with a good family...")

    # FAIL with changed vertical metric values
    local_regular = TTFont(TEST_FILE("cabin/Cabin-Regular.ttf"))
    local_regular["OS/2"].sTypoAscender = 0
    assert_results_contain(
        check([local_regular]),
        FAIL,
        "bad-typo-ascender",
        "with a family which has an incorrect typoAscender...",
    )

    local_regular["OS/2"].sTypoDescender = 0
    assert_results_contain(
        check([local_regular]),
        FAIL,
        "bad-typo-descender",
        "with a family which has an incorrect typoDescender...",
    )

    local_regular["hhea"].ascent = 0
    assert_results_contain(
        check([local_regular]),
        FAIL,
        "bad-hhea-ascender",
        "with a family which has an incorrect hhea ascender...",
    )

    local_regular["hhea"].descent = 0
    assert_results_contain(
        check([local_regular]),
        FAIL,
        "bad-hhea-descender",
        "with a family which has an incorrect hhea descender...",
    )


@check_id("googlefonts/cjk_vertical_metrics")
def test_check_cjk_vertical_metrics(check):

    # Iansui was built with our new vertical metrics schema, so should work
    ttFont = TTFont(TEST_FILE("cjk/Iansui-Regular.ttf"))
    assert_PASS(check(ttFont, skip_network=True), "for Iansui")

    # Noto Sans was built with our old vertical metrics schema, so won't
    ttFont = TTFont(cjk_font)
    results = check(ttFont, skip_network=True)
    assert_results_contain(
        results,
        FAIL,
        "bad-fselection-bit7",
        "for font where OS/2 fsSelection bit 7 is enabled",
    )

    assert_results_contain(
        results,
        FAIL,
        "bad-OS/2.sTypoAscender",
        "for font with bad OS/2.sTypoAscender",
    )

    assert_results_contain(
        results,
        FAIL,
        "bad-OS/2.sTypoDescender",
        "for font with bad OS/2.sTypoDescender",
    )


@check_id("googlefonts/cjk_vertical_metrics_regressions")
def test_check_cjk_vertical_metrics_regressions(check):
    # TODO: try to remove deepcopy usage
    from copy import deepcopy

    ttFont = TTFont(TEST_FILE("cjk/YujiBoku-Regular.ttf"))
    assert_PASS(
        check(TEST_FILE("cjk/YujiBoku-Regular.ttf")),
        "for Yuji Boku",
    )

    # Change a single metric
    ttFont2 = deepcopy(ttFont)
    ttFont2["hhea"].ascent = 0
    assert_results_contain(
        check(ttFont2),
        FAIL,
        "cjk-metric-regression",
        "hhea ascent is 0 when it should be 880",
    )

    # Change upm of font being checked
    ttFont3 = deepcopy(ttFont)
    ttFont3["head"].unitsPerEm = 2000
    assert_results_contain(
        check(ttFont3),
        FAIL,
        "cjk-metric-regression",
        "upm is 2000 and vert metrics values are not updated",
    )

    # Change upm of checked font and update vert metrics
    ttFont4 = deepcopy(ttFont)
    ttFont4["head"].unitsPerEm = 2000
    for tbl, attrib in [
        ("OS/2", "sTypoAscender"),
        ("OS/2", "sTypoDescender"),
        ("OS/2", "sTypoLineGap"),
        ("OS/2", "usWinAscent"),
        ("OS/2", "usWinDescent"),
        ("hhea", "ascent"),
        ("hhea", "descent"),
        ("hhea", "lineGap"),
    ]:
        current_val = getattr(ttFont4[tbl], attrib)
        setattr(ttFont4[tbl], attrib, current_val * 2)
    assert_PASS(
        check(ttFont4),
        "for Yuji Boku with doubled upm and doubled vert metrics",
    )


@check_id("googlefonts/fvar_instances")
def test_check_varfont_instance_coordinates(check, vf_ttFont):
    # OpenSans-Roman-VF is correct
    assert_PASS(
        check(vf_ttFont), "with a variable font which has correct instance coordinates."
    )

    from copy import copy

    vf_ttFont2 = copy(vf_ttFont)
    for instance in vf_ttFont2["fvar"].instances:
        for axis in instance.coordinates.keys():
            instance.coordinates[axis] = 0
    assert_results_contain(
        check(vf_ttFont2),
        FAIL,
        "bad-fvar-instances",
        "with a variable font which does not have correct instance coordinates.",
    )


@check_id("googlefonts/fvar_instances")
def test_check_varfont_instance_names(check, vf_ttFont):
    assert_PASS(
        check(vf_ttFont), "with a variable font which has correct instance names."
    )

    from copy import copy

    vf_ttFont2 = copy(vf_ttFont)
    for instance in vf_ttFont2["fvar"].instances:
        instance.subfamilyNameID = 300
    broken_name = "ExtraBlack Condensed 300pt"
    vf_ttFont2["name"].setName(
        broken_name,
        300,
        PlatformID.MACINTOSH,
        MacintoshEncodingID.ROMAN,
        MacintoshLanguageID.ENGLISH,
    )
    vf_ttFont2["name"].setName(
        broken_name,
        300,
        PlatformID.WINDOWS,
        WindowsEncodingID.UNICODE_BMP,
        WindowsLanguageID.ENGLISH_USA,
    )
    assert_results_contain(
        check(vf_ttFont2),
        FAIL,
        "bad-fvar-instances",
        "with a variable font which does not have correct instance names.",
    )
    # Let's see if the check is skipped if a font contains a MORF axis.
    # We allow fonts with a MORF axis to have custom fvar instances.
    from fontTools.ttLib.tables._f_v_a_r import Axis

    vf_ttFont3 = copy(vf_ttFont)
    morf_axis = Axis()
    morf_axis.axisTag = "MORF"
    vf_ttFont3["fvar"].axes.append(morf_axis)
    for instance in vf_ttFont3["fvar"].instances:
        instance.coordinates["MORF"] = 0
    assert_SKIP(check(vf_ttFont3))


@check_id("googlefonts/STAT/axisregistry")
def test_check_STAT_gf_axisregistry(check):
    """Validate STAT particle names and values
    match the fallback names in GFAxisRegistry."""
    from fontTools.otlLib.builder import buildStatTable

    # Our reference varfont, CabinVF,
    # has "Regular", instead of "Roman" in its 'ital' axis on the STAT table:
    ttFont = TTFont(TEST_FILE("cabinvf/Cabin[wdth,wght].ttf"))
    assert_results_contain(check(ttFont), FAIL, "invalid-name")

    # LibreCaslonText is good though:
    ttFont = TTFont(TEST_FILE("librecaslontext/LibreCaslonText[wght].ttf"))
    assert_PASS(check(ttFont))

    # Let's break it by setting an invalid coordinate for "Bold":
    assert (
        ttFont["STAT"].table.AxisValueArray.AxisValue[3].ValueNameID
        == ttFont["name"].names[4].nameID
    )
    assert ttFont["name"].names[4].toUnicode() == "Bold"
    # instead of the expected 700
    # Note: I know it is AxisValue[3] and names[4]
    # because I inspected the font using ttx.
    ttFont["STAT"].table.AxisValueArray.AxisValue[3].Value = 800
    assert_results_contain(check(ttFont), FAIL, "bad-coordinate")

    # Let's remove all Axis Values. This will fail since we Google Fonts
    # requires them.
    ttFont["STAT"].table.AxisValueArray = None
    assert_results_contain(check(ttFont), FAIL, "missing-axis-values")

    # Let's add a MORF Axis with custom axisvalues
    stat = [
        {
            "tag": "MORF",
            "name": "Morph",
            "values": [
                {"name": "Foo", "value": 0},
                {"name": "Bar", "value": 100},
            ],
        },
        {
            "tag": "wght",
            "name": "Weight",
            "values": [
                {"name": "Regular", "value": 400, "flags": 0x2},
                {"name": "Bold", "value": 700},
            ],
        },
    ]
    buildStatTable(ttFont, stat)
    assert_PASS(check(ttFont))

    # Let's make a weight axisvalue incorrect.
    stat[1]["values"][1]["value"] = 800
    buildStatTable(ttFont, stat)
    assert_results_contain(check(ttFont), FAIL, "bad-coordinate")


@pytest.mark.skip("This check is ported, but we can't mock the requests.")
@check_id("googlefonts/metadata/designer_profiles")
def test_check_metadata_designer_profiles(check, requests_mock):
    """METADATA.pb: Designer is listed with the correct name on
    the Google Fonts catalog of designers?"""

    requests_mock.get(
        "https://raw.githubusercontent.com/google/fonts/master/"
        "catalog/designers/delvewithrington/info.pb",
        status_code=404,
    )
    sorkintype_info = """
        designer: "Sorkin Type"
        link: ""
        avatar {
          file_name: "sorkin_type.png"
        }
        """
    requests_mock.get(
        "https://raw.githubusercontent.com/google/fonts/master/"
        "catalog/designers/sorkintype/info.pb",
        text=sorkintype_info,
    )
    requests_mock.get(
        "https://raw.githubusercontent.com/google/fonts/master/"
        "catalog/designers/sorkintype/sorkin_type.png",
        content=b"\x89PNG\x0d\x0a\x1a\x0a",
    )

    # Delve Withrington is still not listed on the designers catalog.
    font = TEST_FILE("overpassmono/OverpassMono-Regular.ttf")
    assert_results_contain(check(font), WARN, "profile-not-found")

    # Cousine lists designers: "Multiple Designers"
    font = TEST_FILE("cousine/Cousine-Regular.ttf")
    assert_results_contain(check(font), FAIL, "multiple-designers")

    # This reference Merriweather font family lists "Sorkin Type" in its METADATA.pb
    # file. And this foundry has a good profile on the catalog.
    font = TEST_FILE("merriweather/Merriweather-Regular.ttf")
    assert_PASS(check(font))

    # TODO: FAIL, "mismatch"
    # TODO: FAIL, "link-field"
    # TODO: FAIL, "missing-avatar"
    # TODO: FAIL, "bad-avatar-filename"


@pytest.mark.skip("Check not ported yet.")
@check_id("googlefonts/description/family_update")
def test_check_description_family_update(check, requests_mock):
    """On a family update, the DESCRIPTION.en_us.html
    file should ideally also be updated."""

    font = TEST_FILE("abeezee/ABeeZee-Regular.ttf")
    ABEEZEE_DESC = (
        "https://github.com/google/fonts/raw/main/ofl/abeezee/DESCRIPTION.en_us.html"
    )

    desc = "<html>My fake description.</html>"
    requests_mock.get(ABEEZEE_DESC, text=desc)

    assert_results_contain(
        check(MockFont(file=font, description=desc)), WARN, "description-not-updated"
    )

    assert_PASS(check(MockFont(file=font, description=desc + "\nSomething else...")))


@pytest.mark.skip("Check not ported yet.")
@check_id("googlefonts/repo/sample_image")
def test_check_repo_sample_image(check):
    """Check README.md has a sample image."""

    # That's what we'd like to see:
    # README.md including a sample image and highlighting it in the
    # upper portion of the document (no more than 10 lines from the top).
    readme = TEST_FILE("issue_2898/good/README.md")
    assert_PASS(check(readme))

    # This one is still good, but places the sample image too late in the page:
    readme = TEST_FILE("issue_2898/not-ideal-placement/README.md")
    assert_results_contain(check(readme), WARN, "not-ideal-placement")

    # Here's a README.md in a project completely lacking such sample image.
    # This will likely become a FAIL in the future:
    readme = TEST_FILE("issue_2898/no-sample/README.md")
    assert_results_contain(check(readme), WARN, "no-sample")  # FIXME: Make this a FAIL!

    # This is really broken, as it references an image that is not available:
    readme = TEST_FILE("issue_2898/image-missing/README.md")
    assert_results_contain(check(readme), FAIL, "image-missing")

    # An here a README.md that does not include any sample image,
    # while an image file can be found within the project's directory tree.
    # This image could potentially be a font sample, so we let the user know
    # that it might be the case:
    readme = TEST_FILE("issue_2898/image-not-displayed/README.md")
    assert_results_contain(check(readme), WARN, "image-not-displayed")


@check_id("googlefonts/metadata/can_render_samples")
def test_check_metadata_can_render_samples(check, tmp_path):
    """Check README.md has a sample image."""
    font = TEST_FILE("cabin/Cabin-Regular.ttf")
    mdpb = TEST_FILE("cabin/METADATA.pb")
    assert_results_contain(check([font, mdpb]), SKIP, "no-languages")

    # This will try to render using strings provided by the gflanguages package
    # Available at https://pypi.org/project/gflanguages/
    md = Font(font).family_metadata
    md.languages.append("non_Runr")  # Cabin does not support Old Nordic Runic
    assert_results_contain(check([font, fake_mdpb(tmp_path, md)]), FAIL, "sample-text")

    # TODO: expand the check to also validate rendering of
    #       text provided explicitely on the sample_text field of METADATA.pb


@pytest.mark.skip("Check not ported yet.")
@check_id("googlefonts/metadata/unsupported_subsets")
def test_check_metadata_unsupported_subsets(check):
    """Check for METADATA subsets with zero support."""

    font = TEST_FILE("librecaslontext/LibreCaslonText[wght].ttf")
    assert_PASS(check(font))

    md = Font(font).family_metadata
    md.subsets.extend(["foo"])
    assert_results_contain(
        check(MockFont(file=font, family_metadata=md)), FAIL, "unknown-subset"
    )

    del md.subsets[:]
    md.subsets.extend(["cyrillic"])
    assert_results_contain(
        check(MockFont(file=font, family_metadata=md)), FAIL, "unsupported-subset"
    )


@pytest.mark.parametrize(
    """fp,mod,result""",
    [
        # font includes condensed fvar instances so it should fail
        (TEST_FILE("cabinvfbeta/CabinVFBeta.ttf"), [], FAIL),
        # official fonts have been fixed so this should pass
        (TEST_FILE("cabinvf/Cabin[wdth,wght].ttf"), [], PASS),
        (TEST_FILE("cabinvf/Cabin-Italic[wdth,wght].ttf"), [], PASS),
        # lets inject an instance which is not a multiple of 100
        (TEST_FILE("cabinvf/Cabin[wdth,wght].ttf"), [("Book", 450)], FAIL),
    ],
)
@check_id("googlefonts/fvar_instances")
def test_check_fvar_instances(check, fp, mod, result):
    """Check font fvar instances are correct"""
    from fontTools.ttLib.tables._f_v_a_r import NamedInstance

    ttFont = TTFont(fp)
    expected = expected_font_names(ttFont, [])
    if mod:
        for name, wght_val in mod:
            inst = NamedInstance()
            inst.subfamilyNameID = ttFont["name"].addName(name)
            inst.coordinates = {"wght": wght_val, "wdth": 100}
            ttFont["fvar"].instances.append(inst)

    if result == PASS:
        assert_PASS(check(ttFont), "with a good font")
    elif result == FAIL:
        assert_results_contain(
            check(ttFont),
            FAIL,
            "bad-fvar-instances",
            "with a bad font",
        )


@pytest.mark.parametrize(
    """fp,mod,result,code""",
    [
        (TEST_FILE("cabinvf/Cabin[wdth,wght].ttf"), [], PASS, None),
        # Drop weight has so this should fail since gf version has it
        (
            TEST_FILE("cabinvf/Cabin[wdth,wght].ttf"),
            ["wght", None, None],
            FAIL,
            "missing-axes",
        ),
        # Change ranges of weight axis to 500-600, this should fail since gf version has 400-700
        (
            TEST_FILE("cabinvf/Cabin[wdth,wght].ttf"),
            ["wght", 500, None],
            FAIL,
            "axis-min-out-of-range",
        ),
        (
            TEST_FILE("cabinvf/Cabin[wdth,wght].ttf"),
            ["wght", None, 600],
            FAIL,
            "axis-max-out-of-range",
        ),
    ],
)
@check_id("googlefonts/axes_match")
def test_check_axes_match(check, fp, mod, result, code):
    """Check if the axes match between the font and the Google Fonts version."""

    ttFont = TTFont(fp)
    mdpb = TEST_FILE("cabinvf/METADATA.pb")
    if mod:
        name, min_val, max_val = mod
        if not min_val and not max_val:
            ttFont["fvar"].axes = [a for a in ttFont["fvar"].axes if a.axisTag != name]
        else:
            axis = next(a for a in ttFont["fvar"].axes if a.axisTag == name)
            axis.minValue = min_val or axis.minValue
            axis.maxValue = max_val or axis.maxValue

    if result == PASS:
        assert_PASS(check([ttFont, mdpb]), "with a good font")
    elif result == FAIL:
        assert_results_contain(
            check([ttFont, mdpb]),
            FAIL,
            code,
            "with a bad font",
        )


@pytest.mark.parametrize(
    """fps,new_stat,result""",
    [
        # Fail (we didn't really know what we were doing at this stage)
        (
            [
                TEST_FILE("cabinvf/Cabin[wdth,wght].ttf"),
                TEST_FILE("cabinvf/Cabin-Italic[wdth,wght].ttf"),
            ],
            [],
            FAIL,
        ),
        # Fix previous test for Cabin[wdth,wght].ttf
        (
            [
                TEST_FILE("cabinvf/Cabin[wdth,wght].ttf"),
                TEST_FILE("cabinvf/Cabin-Italic[wdth,wght].ttf"),
            ],
            # STAT for Cabin[wdth,wght].ttf
            [
                {
                    "name": "Weight",
                    "tag": "wght",
                    "values": [
                        {
                            "value": 400,
                            "name": "Regular",
                            "linkedValue": 700.0,
                            "flags": 0x2,
                        },
                        {"value": 500, "name": "Medium"},
                        {"value": 600, "name": "SemiBold"},
                        {"value": 700, "name": "Bold"},
                    ],
                },
                {
                    "name": "Width",
                    "tag": "wdth",
                    "values": [
                        {"value": 75, "name": "Condensed"},
                        {"value": 87.5, "name": "SemiCondensed"},
                        {"value": 100, "name": "Normal", "flags": 0x2},
                    ],
                },
                {
                    "name": "Italic",
                    "tag": "ital",
                    "values": [
                        {
                            "value": 0.0,
                            "name": "Normal",
                            "linkedValue": 1.0,
                            "flags": 0x2,
                        }
                    ],
                },
            ],
            PASS,
        ),
    ],
)
@check_id("googlefonts/STAT/compulsory_axis_values")
def test_check_STAT(check, fps, new_stat, result):
    """Check STAT table Axis Values are correct"""
    # more comprehensive checks are available in the axisregistry:
    # https://github.com/googlefonts/axisregistry/blob/main/tests/test_names.py#L442
    # this check merely exists to check that everything is hooked up correctly
    from fontTools.otlLib.builder import buildStatTable

    ttFonts = [TTFont(f) for f in fps]
    ttFont = ttFonts[0]
    if new_stat:
        buildStatTable(ttFont, new_stat)

    if result == PASS:
        assert_PASS(
            check(ttFont),
            "with a good font",
        )
    elif result == FAIL:
        assert_results_contain(
            check(ttFont),
            FAIL,
            "bad-axis-values",
            "with a bad font",
        )


