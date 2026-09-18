import io
import os
from unittest.mock import MagicMock, patch

import pytest
import requests
from conftest import check_id
from fontbakery.codetesting import (
    TEST_FILE,
    MockFont,
    assert_PASS,
    assert_results_contain,
    assert_SKIP,
)
from fontbakery.status import ERROR, FAIL, INFO, SKIP, WARN
from fontbakery.testable import Font
from fontbakery.utils import glyph_has_ink
from fontTools.ttLib import TTFont


@pytest.fixture
def montserrat_ttFonts():
    paths = [
        TEST_FILE("montserrat/Montserrat-Black.ttf"),
        TEST_FILE("montserrat/Montserrat-BlackItalic.ttf"),
        TEST_FILE("montserrat/Montserrat-Bold.ttf"),
        TEST_FILE("montserrat/Montserrat-BoldItalic.ttf"),
        TEST_FILE("montserrat/Montserrat-ExtraBold.ttf"),
        TEST_FILE("montserrat/Montserrat-ExtraBoldItalic.ttf"),
        TEST_FILE("montserrat/Montserrat-ExtraLight.ttf"),
        TEST_FILE("montserrat/Montserrat-ExtraLightItalic.ttf"),
        TEST_FILE("montserrat/Montserrat-Italic.ttf"),
        TEST_FILE("montserrat/Montserrat-Light.ttf"),
        TEST_FILE("montserrat/Montserrat-LightItalic.ttf"),
        TEST_FILE("montserrat/Montserrat-Medium.ttf"),
        TEST_FILE("montserrat/Montserrat-MediumItalic.ttf"),
        TEST_FILE("montserrat/Montserrat-Regular.ttf"),
        TEST_FILE("montserrat/Montserrat-SemiBold.ttf"),
        TEST_FILE("montserrat/Montserrat-SemiBoldItalic.ttf"),
        TEST_FILE("montserrat/Montserrat-Thin.ttf"),
        TEST_FILE("montserrat/Montserrat-ThinItalic.ttf"),
    ]
    return [TTFont(path) for path in paths]


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


@pytest.fixture
def cabin_ttFonts():
    return [TTFont(path) for path in cabin_fonts]


@pytest.fixture
def cabin_condensed_ttFonts():
    return [TTFont(path) for path in cabin_condensed_fonts]


@check_id("name/trailing_spaces")
def test_check_name_trailing_spaces(check):
    """Name table entries must not have trailing spaces."""
    # Our reference Cabin Regular is known to be good:
    ttFont = TTFont(TEST_FILE("cabin/Cabin-Regular.ttf"))
    assert_PASS(check(ttFont), "with a good font...")

    for i, entry in enumerate(ttFont["name"].names):
        good_string = ttFont["name"].names[i].toUnicode()
        bad_string = good_string + " "
        ttFont["name"].names[i].string = bad_string.encode(entry.getEncoding())
        assert_results_contain(
            check(ttFont),
            FAIL,
            "trailing-space",
            f'with a bad name table entry ({i}: "{bad_string}")...',
        )

        # restore good entry before moving to the next one:
        ttFont["name"].names[i].string = good_string.encode(entry.getEncoding())


@check_id("mandatory_glyphs")
def test_check_mandatory_glyphs(check):
    """Font contains the first few mandatory glyphs (.null or NULL, CR and space)?"""
    from fontTools import subset

    ttFont = TTFont(TEST_FILE("nunito/Nunito-Regular.ttf"))
    assert_PASS(check(ttFont))

    options = subset.Options()
    options.glyph_names = True  # Preserve glyph names
    # By default, the subsetter keeps the '.notdef' glyph but removes its outlines
    subsetter = subset.Subsetter(options)
    subsetter.populate(text="mn")  # Arbitrarily remove everything except 'm' and 'n'
    subsetter.subset(ttFont)
    message = assert_results_contain(check(ttFont), FAIL, "notdef-is-blank")
    assert message == "The '.notdef' glyph should contain a drawing, but it is blank."

    options.notdef_glyph = False  # Drop '.notdef' glyph
    subsetter = subset.Subsetter(options)
    subsetter.populate(text="mn")
    subsetter.subset(ttFont)
    message = assert_results_contain(check(ttFont), WARN, "notdef-not-found")
    assert message == "Font should contain the '.notdef' glyph."

    # XXX The below is enough to fool fontTools, but when the font is
    # compiled to binary and saved it does not fully rename the .notdef glyph.

    # Change the glyph name from 'n' to '.notdef'
    # (Must reload the font here since we already decompiled the glyf table)
    # ttFont = TTFont(TEST_FILE("nunito/Nunito-Regular.ttf"))
    # ttFont.glyphOrder = ["m", ".notdef"]
    # for subtable in ttFont["cmap"].tables:
    #     if subtable.isUnicode():
    #         subtable.cmap[110] = ".notdef"
    #         if 0 in subtable.cmap:
    #             del subtable.cmap[0]
    # results = check(ttFont)
    # message = assert_results_contain([results[0]], WARN, "notdef-not-first")
    # assert message == "The '.notdef' should be the font's first glyph."

    # message = assert_results_contain([results[1]], WARN, "notdef-has-codepoint")
    # assert message == (
    #     "The '.notdef' glyph should not have a Unicode codepoint value assigned,"
    #     " but has 0x006E."
    # )


def _remove_cmap_entry(font, cp):
    """Helper method that removes a codepoint entry from all the tables in cmap."""
    for subtable in font["cmap"].tables:
        subtable.cmap.pop(cp, None)


@check_id("whitespace_ink")
def test_check_whitespace_ink(check):
    """Whitespace glyphs have ink?"""
    test_font = TTFont(TEST_FILE("nunito/Nunito-Regular.ttf"))
    assert_PASS(check(test_font))

    test_font["cmap"].tables[0].cmap[0x1680] = "a"
    assert_PASS(check(test_font), "because Ogham space mark does have ink.")

    test_font["cmap"].tables[0].cmap[0x0020] = "uni1E17"
    assert_results_contain(
        check(test_font),
        FAIL,
        "has-ink",
        "for whitespace character having composites (with ink).",
    )

    test_font["cmap"].tables[0].cmap[0x0020] = "scedilla"
    assert_results_contain(
        check(test_font),
        FAIL,
        "has-ink",
        "for whitespace character having outlines (with ink).",
    )

    import fontTools.pens.ttGlyphPen

    pen = fontTools.pens.ttGlyphPen.TTGlyphPen(test_font.getGlyphSet())
    pen.addComponent("space", (1, 0, 0, 1, 0, 0))
    test_font["glyf"].glyphs["uni200B"] = pen.glyph()
    assert_results_contain(
        check(test_font),
        FAIL,
        "has-ink",  # should we give is a separate keyword? This looks wrong.
        "for whitespace character having composites (without ink).",
    )


mada_fonts = [
    # ⚠️ 'test_check_family_win_ascent_and_descent' expects the Regular font to be first
    TEST_FILE("mada/Mada-Regular.ttf"),
    TEST_FILE("mada/Mada-Black.ttf"),
    TEST_FILE("mada/Mada-Bold.ttf"),
    TEST_FILE("mada/Mada-ExtraLight.ttf"),
    TEST_FILE("mada/Mada-Light.ttf"),
    TEST_FILE("mada/Mada-Medium.ttf"),
    TEST_FILE("mada/Mada-SemiBold.ttf"),
]


@pytest.fixture
def mada_ttFonts():
    return [TTFont(path) for path in mada_fonts]


@check_id("family/win_ascent_and_descent")
def test_check_family_win_ascent_and_descent(mada_ttFonts, check):
    """Checking OS/2 usWinAscent & usWinDescent."""
    # Mada Regular is know to be bad
    # single font input
    ttFont = TTFont(mada_fonts[0])
    message = assert_results_contain(check(ttFont), FAIL, "ascent")
    assert message == (
        "OS/2.usWinAscent value should be"
        " equal or greater than 880, but got 776 instead."
    )
    # multi font input
    check_results = check(mada_ttFonts)
    message = assert_results_contain([check_results[0]], FAIL, "ascent")
    assert message == (
        "OS/2.usWinAscent value should be"
        " equal or greater than 918, but got 776 instead."
    )
    message = assert_results_contain([check_results[1]], FAIL, "descent")
    assert message == (
        "OS/2.usWinDescent value should be"
        " equal or greater than 406, but got 322 instead."
    )

    # Fix usWinAscent
    ttFont["OS/2"].usWinAscent = 880
    assert_PASS(check(ttFont))

    # Make usWinAscent too large
    ttFont["OS/2"].usWinAscent = 880 * 2 + 1
    message = assert_results_contain(check(ttFont), FAIL, "ascent")
    assert message == (
        "OS/2.usWinAscent value 1761 is too large. "
        "It should be less than double the yMax. Current yMax value is 880."
    )

    # Make usWinDescent too large
    ttFont["OS/2"].usWinDescent = 292 * 2 + 1
    message = assert_results_contain(check(ttFont), FAIL, "descent")
    assert message == (
        "OS/2.usWinDescent value 585 is too large."
        " It should be less than double the yMin. Current absolute yMin value is 292."
    )

    # Delete OS/2 table
    del ttFont["OS/2"]
    assert check(ttFont)[0].status == ERROR


@check_id("os2_metrics_match_hhea")
def test_check_os2_metrics_match_hhea(check):
    """Checking OS/2 Metrics match hhea Metrics."""
    # Our reference Mada Regular is know to be faulty here.
    ttFont = TTFont(TEST_FILE("mada/Mada-Regular.ttf"))
    assert_results_contain(
        check(ttFont),
        FAIL,
        "lineGap",
        "OS/2 sTypoLineGap (100) and hhea lineGap (96) must be equal.",
    )

    # Our reference Mada Black is know to be good here.
    ttFont = TTFont(TEST_FILE("mada/Mada-Black.ttf"))

    assert_PASS(check(ttFont), "with a good font...")

    # Now we break it:
    correct = ttFont["hhea"].ascent
    ttFont["OS/2"].sTypoAscender = correct + 1
    assert_results_contain(
        check(ttFont), FAIL, "ascender", "with a bad OS/2.sTypoAscender font..."
    )

    # Restore good value:
    ttFont["OS/2"].sTypoAscender = correct

    # And break it again, now on sTypoDescender value:
    correct = ttFont["hhea"].descent
    ttFont["OS/2"].sTypoDescender = correct + 1
    assert_results_contain(
        check(ttFont), FAIL, "descender", "with a bad OS/2.sTypoDescender font..."
    )

    # Delete OS/2 table
    del ttFont["OS/2"]
    assert check(ttFont)[0].status == ERROR


@check_id("family/vertical_metrics")
def test_check_family_vertical_metrics(montserrat_ttFonts, check):
    assert_PASS(check(montserrat_ttFonts), "with multiple good fonts...")

    montserrat_ttFonts[0]["OS/2"].sTypoAscender = 3333
    montserrat_ttFonts[1]["OS/2"].usWinAscent = 4444
    results = check(montserrat_ttFonts)
    msg = assert_results_contain(results, FAIL, "sTypoAscender-mismatch")
    assert "Montserrat-Black.ttf: 3333" in msg
    msg = assert_results_contain(results, FAIL, "usWinAscent-mismatch")
    assert "Montserrat-BlackItalic.ttf: 4444" in msg

    del montserrat_ttFonts[2]["OS/2"]
    del montserrat_ttFonts[3]["hhea"]
    results = check(montserrat_ttFonts)
    msg = assert_results_contain([results[0]], FAIL, "lacks-OS/2")
    assert msg == "Montserrat-Bold.ttf lacks an 'OS/2' table."
    msg = assert_results_contain([results[1]], FAIL, "lacks-hhea")
    assert msg == "Montserrat-BoldItalic.ttf lacks a 'hhea' table."


@pytest.mark.skip(reason="Check not yet implemented")
@check_id("superfamily/list")
def test_check_superfamily_list(check):
    msg = assert_results_contain(
        check(MockFont(superfamily=[cabin_fonts])), INFO, "family-path"
    )
    assert msg == os.path.normpath("data/test/cabin")


@pytest.mark.skip(reason="Check not yet implemented")
@check_id("superfamily/vertical_metrics")
def test_check_superfamily_vertical_metrics(
    montserrat_ttFonts, cabin_ttFonts, cabin_condensed_ttFonts, check
):
    msg = assert_SKIP(check(MockFont(superfamily_ttFonts=[cabin_ttFonts[0]])))
    assert msg == "Sibling families were not detected."

    assert_PASS(
        check(MockFont(superfamily_ttFonts=[cabin_ttFonts, cabin_condensed_ttFonts])),
        "with multiple good families...",
    )

    assert_results_contain(
        check(MockFont(superfamily_ttFonts=[cabin_ttFonts, montserrat_ttFonts])),
        WARN,
        "superfamily-vertical-metrics",
        "with families that diverge on vertical metric values...",
    )


@check_id("cjk_chws_feature")
def test_check_cjk_chws_feature(check):
    """Does the font contain chws and vchw features?"""
    cjk_font = TEST_FILE("cjk/YujiBoku-Regular.ttf")
    results = check(cjk_font)
    assert_results_contain(results, WARN, "missing-chws-feature", "for Yuji Boku")

    assert_results_contain(results, WARN, "missing-vchw-feature", "for Yuji Boku")

    # Insert them.
    from fontTools.ttLib.tables.otTables import FeatureRecord

    ttFont = TTFont(cjk_font)
    chws = FeatureRecord()
    chws.FeatureTag = "chws"
    vchw = FeatureRecord()
    vchw.FeatureTag = "vchw"
    ttFont["GSUB"].table.FeatureList.FeatureRecord.extend([chws, vchw])

    assert_PASS(check(ttFont))


@check_id("sfnt_version")
def test_check_sfnt_version(check):
    """Ensure that the font has the proper sfntVersion value."""
    # Valid TrueType font; the check must PASS.
    ttFont = TTFont(TEST_FILE("cabinvf/Cabin[wdth,wght].ttf"))
    assert_PASS(check(ttFont))

    # Change the sfntVersion to an improper value for TrueType fonts.
    # The check should FAIL.
    ttFont.sfntVersion = "OTTO"
    msg = assert_results_contain(check(ttFont), FAIL, "wrong-sfnt-version-ttf")
    assert msg == "Font with TrueType outlines has incorrect sfntVersion value: 'OTTO'"

    # Valid CFF font; the check must PASS.
    ttFont = TTFont(TEST_FILE("source-sans-pro/OTF/SourceSansPro-Bold.otf"))
    assert_PASS(check(ttFont))

    # Change the sfntVersion to an improper value for CFF fonts. The check should FAIL.
    ttFont.sfntVersion = "\x00\x01\x00\x00"
    msg = assert_results_contain(check(ttFont), FAIL, "wrong-sfnt-version-cff")
    assert msg == (
        "Font with CFF data has incorrect sfntVersion value: '\\x00\\x01\\x00\\x00'"
    )

    # Valid CFF2 font; the check must PASS.
    ttFont = TTFont(TEST_FILE("source-sans-pro/VAR/SourceSansVariable-Roman.otf"))
    assert_PASS(check(ttFont))

    # Change the sfntVersion to an improper value for CFF fonts. The check should FAIL.
    ttFont.sfntVersion = "\x00\x01\x00\x00"
    msg = assert_results_contain(check(ttFont), FAIL, "wrong-sfnt-version-cff")
    assert msg == (
        "Font with CFF data has incorrect sfntVersion value: '\\x00\\x01\\x00\\x00'"
    )


@check_id("whitespace_widths")
def test_check_whitespace_widths(check):
    """Whitespace glyphs have coherent widths?"""
    ttFont = TTFont(TEST_FILE("nunito/Nunito-Regular.ttf"))
    assert_PASS(check(ttFont))

    ttFont["hmtx"].metrics["space"] = (0, 1)
    assert_results_contain(check(ttFont), FAIL, "different-widths")


@check_id("linegaps")
def test_check_linegaps(check):
    """Checking Vertical Metric Linegaps."""
    # Our reference Mada Regular is know to be bad here.
    ttFont = TTFont(TEST_FILE("mada/Mada-Regular.ttf"))

    # But just to be sure, we first explicitely set
    # the values we're checking for:
    ttFont["hhea"].lineGap = 1
    ttFont["OS/2"].sTypoLineGap = 0
    assert_results_contain(check(ttFont), WARN, "hhea", "with non-zero hhea.lineGap...")

    # Then we run the check with a non-zero OS/2.sTypoLineGap:
    ttFont["hhea"].lineGap = 0
    ttFont["OS/2"].sTypoLineGap = 1
    assert_results_contain(
        check(ttFont), WARN, "OS/2", "with non-zero OS/2.sTypoLineGap..."
    )

    # And finaly we fix it by making both values equal to zero:
    ttFont["hhea"].lineGap = 0
    ttFont["OS/2"].sTypoLineGap = 0
    assert_PASS(check(ttFont))

    # Confirm the check yields FAIL if the font doesn't have a required table
    del ttFont["OS/2"]
    assert check(ttFont)[0].status == ERROR


@check_id("STAT_in_statics")
def test_check_STAT_in_statics(check):
    """Checking STAT table on static fonts."""
    ttFont = TTFont(TEST_FILE("cabin/Cabin-Regular.ttf"))
    msg = assert_results_contain(check(ttFont), SKIP, "no-stat")
    assert "No STAT table." in msg

    ttFont = TTFont(TEST_FILE("varfont/RobotoSerif[GRAD,opsz,wdth,wght].ttf"))
    msg = assert_results_contain(check(ttFont), SKIP, "variable-font")
    assert "This is a variable font." in msg

    # Remove fvar table to make FontBakery think it is dealing with a static font
    del ttFont["fvar"]

    # We know that our reference RobotoSerif varfont (which the check is induced
    # here to think it is a static font) has multiple records per design axis in its
    # STAT table:
    msg = assert_results_contain(check(ttFont), FAIL, "multiple-STAT-entries")
    assert (
        "'wght' axis (9)" in msg or "'opsz' axis (5)" in msg or "'wdth' axis (8)" in msg
    )

    # Remove all entries except the very first one:
    stat = ttFont["STAT"].table
    stat.AxisValueArray.AxisCount = 1
    stat.AxisValueArray.AxisValue = [stat.AxisValueArray.AxisValue[0]]

    # It should PASS now
    assert_PASS(check(ttFont))


@pytest.mark.skip(reason="Check not yet implemented")
@check_id("caps_vertically_centered")
def test_check_caps_vertically_centered(check):
    """Check if uppercase glyphs are vertically centered."""

    ttFont = TTFont(TEST_FILE("shantell/ShantellSans[BNCE,INFM,SPAC,wght].ttf"))
    assert_PASS(check(ttFont))

    ttFont = TTFont(TEST_FILE("cjk/SourceHanSans-Regular.otf"))
    assert_SKIP(check(ttFont))

    # FIXME: review this test-case
    # ttFont = TTFont(TEST_FILE("cairo/CairoPlay-Italic.leftslanted.ttf"))
    # assert_results_contain(check(ttFont), WARN, "vertical-metrics-not-centered")


@pytest.mark.skip(reason="Check not yet implemented")
@check_id("gsub/smallcaps_before_ligatures")
def test_check_gsub_smallcaps_before_ligatures(check):
    """Ensure 'smcp' lookups are defined before 'liga' lookups in the 'GSUB' table."""
    from fontTools.ttLib.tables.otTables import Feature, FeatureRecord

    ttFont = TTFont(TEST_FILE("mada/Mada-Regular.ttf"))

    smcp_feature = Feature()
    smcp_feature.LookupListIndex = [0]
    liga_feature = Feature()
    liga_feature.LookupListIndex = [1]

    smcp_record = FeatureRecord()
    smcp_record.FeatureTag = "smcp"
    smcp_record.Feature = smcp_feature

    liga_record = FeatureRecord()
    liga_record.FeatureTag = "liga"
    liga_record.Feature = liga_feature

    # Test both 'smcp' and 'liga' lookups are present
    ttFont["GSUB"].table.FeatureList.FeatureRecord = [smcp_record, liga_record]
    assert_PASS(check(ttFont))

    # Test 'liga' lookup before 'smcp' lookup
    smcp_feature.LookupListIndex = [1]
    liga_feature.LookupListIndex = [0]
    assert_results_contain(check(ttFont), FAIL, "feature-ordering")


@pytest.mark.skip("Check not ported yet.")
@check_id("varfont/bold_wght_coord")
def test_check_varfont_bold_wght_coord(check):
    """The variable font 'wght' (Weight) axis coordinate
    must be 700 on the 'Bold' instance."""

    # Our reference varfont CabinVFBeta.ttf
    # has a good Bold:wght coordinate
    ttFont = TTFont("data/test/cabinvfbeta/CabinVFBeta.ttf")
    assert_PASS(check(ttFont), "with a good Bold:wght coordinate...")

    # We then change the value to ensure the problem is properly detected by the check:
    ttFont["fvar"].instances[3].coordinates["wght"] = 600
    assert_results_contain(
        check(ttFont), FAIL, "wght-not-700", "with a bad Bold:wght coordinage (600)..."
    )

    # Check we skip when we don't have a 700 weight.
    ttFont = TTFont("data/test/cabinvfbeta/CabinVFBeta.ttf")
    del ttFont["fvar"].instances[3]
    ttFont["fvar"].axes[0].maxValue = 600
    assert_results_contain(check(ttFont), SKIP, "no-bold-weight")


@pytest.mark.skip("Check not ported yet.")
@check_id("varfont/duplicate_instance_names")
def test_check_varfont_duplicate_instance_names(check, vf_ttFont):
    assert_PASS(
        check(vf_ttFont), "with a variable font which has unique instance names."
    )

    from copy import copy

    vf_ttFont2 = copy(vf_ttFont)
    duplicate_instance_name = (
        vf_ttFont2["name"]
        .getName(
            vf_ttFont2["fvar"].instances[0].subfamilyNameID,
            PlatformID.WINDOWS,
            WindowsEncodingID.UNICODE_BMP,
            WindowsLanguageID.ENGLISH_USA,
        )
        .toUnicode()
    )
    vf_ttFont2["name"].setName(
        string=duplicate_instance_name,
        nameID=vf_ttFont2["fvar"].instances[1].subfamilyNameID,
        platformID=PlatformID.WINDOWS,
        platEncID=WindowsEncodingID.UNICODE_BMP,
        langID=WindowsLanguageID.ENGLISH_USA,
    )
    assert_results_contain(check(vf_ttFont2), FAIL, "duplicate-instance-names")

    # Change the nameID of the 3rd named instance to 456,
    # and don't create a name record with that nameID.
    name_id = 456
    vf_ttFont2["fvar"].instances[2].subfamilyNameID = name_id
    msg = assert_results_contain(check(vf_ttFont2), FAIL, "name-record-not-found")
    assert f" and nameID {name_id} was not found." in msg
