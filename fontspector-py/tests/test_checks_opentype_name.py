import os

import pytest
from conftest import check_id
from fontbakery.codetesting import (
    TEST_FILE,
    assert_PASS,
    assert_results_contain,
    assert_SKIP,
    portable_path,
)
from fontbakery.constants import (
    MacintoshEncodingID,
    MacintoshLanguageID,
    NameID,
    PlatformID,
    WindowsEncodingID,
    WindowsLanguageID,
)
from fontbakery.message import Message
from fontbakery.result import Subresult
from fontbakery.status import FAIL, INFO, PASS, SKIP, WARN
from fontTools.ttLib import TTFont


@check_id("opentype/name/match_familyname_fullfont")
def test_check_name_match_familyname_fullfont(check):
    """Does full font name begin with the font family name?"""
    # Our reference Mada Regular is known to be good
    ttFont = TTFont(TEST_FILE("mada/Mada-Regular.ttf"))

    # So it must PASS the check:
    assert_PASS(check(ttFont))

    EXPECTED_NAME_STRING = "Mada"
    BAD_PREFIX = "bad-prefix"
    name_table = ttFont["name"]
    platform_id = 3
    encoding_id = 1
    language_id = 0x0409
    family_name_id = NameID.FONT_FAMILY_NAME
    full_name_id = NameID.FULL_FONT_NAME

    # Alter the font's full_name string and re-run the check.
    # 1. Retrieve the existing name strings and assert that they're the expected ones.
    family_name = name_table.getName(
        family_name_id, platform_id, encoding_id, language_id
    ).toUnicode()
    assert family_name == EXPECTED_NAME_STRING

    full_name_before = name_table.getName(
        full_name_id, platform_id, encoding_id, language_id
    ).toUnicode()
    assert full_name_before == EXPECTED_NAME_STRING

    # 2. Prefix the full_name string, and update the font's name record.
    name_table.setName(
        f"{BAD_PREFIX}{full_name_before}",
        full_name_id,
        platform_id,
        encoding_id,
        language_id,
    )

    # 3. Retrieve the updated name string, and assert that it's the expected one.
    full_name_after = name_table.getName(
        full_name_id, platform_id, encoding_id, language_id
    ).toUnicode()
    assert full_name_after != family_name
    assert full_name_after == f"{BAD_PREFIX}{EXPECTED_NAME_STRING}"
    assert full_name_after.startswith(BAD_PREFIX)

    # 4. Now re-run the check. It should yield FAIL because the full_name string
    # no longer starts with the family_name string.
    msg = assert_results_contain(check(ttFont), FAIL, "mismatch-font-names")
    assert msg == (
        f"Full font name {full_name_after!r}"
        f" does not start with the family name {family_name!r}"
    )

    # Remove the modified full name record and re-run the check.
    # It should yield FAIL because the name table won't contain a full name string
    # to compare with the family name string.
    name_table.removeNames(full_name_id, platform_id, encoding_id, language_id)
    msg = assert_results_contain(check(ttFont), FAIL, "missing-full-name")

    # Run the check on a CJK font. The font's 'name' table contains
    # English-US (1033/0x0409) and Chinese (1028/0x0404) records. It should PASS.
    ttFont = TTFont(TEST_FILE("cjk/Iansui-Regular.ttf"))
    assert_PASS(check(ttFont))

    name_table = ttFont["name"]
    decode_error_msg_prefix = (
        f"On the 'name' table, the name record"
        f" for platformID {platform_id},"
        f" encodingID {encoding_id},"
        f" languageID {language_id}({language_id:04X}),"
    )


@check_id("opentype/family/max_4_fonts_per_family_name")
def test_check_family_max_4_fonts_per_family_name(check):
    base_path = portable_path("data/test/source-sans-pro/OTF")

    font_names = [
        "SourceSansPro-Black.otf",
        "SourceSansPro-BlackItalic.otf",
        "SourceSansPro-Bold.otf",
        "SourceSansPro-BoldItalic.otf",
        "SourceSansPro-ExtraLight.otf",
        "SourceSansPro-ExtraLightItalic.otf",
        "SourceSansPro-Italic.otf",
        "SourceSansPro-Light.otf",
        "SourceSansPro-LightItalic.otf",
        "SourceSansPro-Regular.otf",
        "SourceSansPro-Semibold.otf",
        "SourceSansPro-SemiboldItalic.otf",
    ]

    font_paths = [os.path.join(base_path, n) for n in font_names]

    test_fonts = [TTFont(x) for x in font_paths]

    # try fonts with correct family name grouping
    assert_PASS(check(test_fonts))

    # now set 5 of the fonts to have the same family name
    for font in test_fonts[:5]:
        name_records = font["name"].names
        for name_record in name_records:
            if name_record.nameID == 1:
                # print(repr(name_record.string))
                name_record.string = "foobar".encode("utf-16be")

    assert_results_contain(check(test_fonts), FAIL, "too-many")


@check_id("opentype/family/consistent_family_name")
def test_check_consistent_font_family_name(check):
    base_path = portable_path("data/test/source-sans-pro/OTF")

    font_names = [
        "SourceSansPro-Black.otf",
        "SourceSansPro-BlackItalic.otf",
        "SourceSansPro-Bold.otf",
        "SourceSansPro-BoldItalic.otf",
        "SourceSansPro-ExtraLight.otf",
        "SourceSansPro-ExtraLightItalic.otf",
        "SourceSansPro-Italic.otf",
        "SourceSansPro-Light.otf",
        "SourceSansPro-LightItalic.otf",
        "SourceSansPro-Regular.otf",
        "SourceSansPro-Semibold.otf",
        "SourceSansPro-SemiboldItalic.otf",
    ]

    font_paths = [os.path.join(base_path, n) for n in font_names]

    test_fonts = [TTFont(x) for x in font_paths]

    # try fonts with consistent family names
    assert_PASS(check(test_fonts))

    # now set 5 of the fonts to have different family names
    for i in range(1, 6):
        if i in [1, 2, 3]:
            target_nameID = 1
        elif i in [4, 5]:
            target_nameID = 16
        name_records = test_fonts[i]["name"].names
        wrong_name = f"wrong-name-{9 % i}"
        for name_record in name_records:
            if name_record.nameID == target_nameID:
                name_record.string = wrong_name.encode("utf-16be")

    msg = assert_results_contain(check(test_fonts), FAIL, "inconsistent-family-name")
    assert "4 different family names were found" in msg
    assert "'Source Sans Pro' (found" in msg
    assert "'wrong-name-1' (found" in msg
