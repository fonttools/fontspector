import io

import fontTools.ttLib
from fontTools.ttLib import TTFont
import fontTools.subset

from fontbakery.status import INFO, WARN, ERROR
from fontbakery.codetesting import (
    assert_PASS,
    assert_results_contain,
    TEST_FILE,
)
from conftest import check_id


@check_id("opentype/xavgcharwidth")
def test_check_xavgcharwidth(check):
    """Check if OS/2 xAvgCharWidth is correct."""

    test_font_path = TEST_FILE("nunito/Nunito-Regular.ttf")

    test_font = TTFont(test_font_path)
    assert_PASS(check(test_font))

    test_font["OS/2"].xAvgCharWidth = 556
    assert_results_contain(check(test_font), INFO, "xAvgCharWidth-close")

    test_font["OS/2"].xAvgCharWidth = 500
    assert_results_contain(
        check(test_font), WARN, "xAvgCharWidth-wrong"
    )  # FIXME: This needs a message keyword

    # XXX We can't actually save an *empty* postv4 font

    test_font = TTFont(test_font_path)
    subsetter = fontTools.subset.Subsetter()
    subsetter.populate(
        glyphs=[
            "a",
            "b",
            "c",
            "d",
            "e",
            "f",
            "g",
            "h",
            "i",
            "j",
            "k",
            "l",
            "m",
            "n",
            "o",
            "p",
            "q",
            "r",
            "s",
            "t",
            "u",
            "v",
            "w",
            "x",
            "y",
            "z",
            "space",
        ]
    )
    subsetter.subset(test_font)
    test_font["OS/2"].xAvgCharWidth = 447
    test_font["OS/2"].version = 2
    temp_file = io.BytesIO()
    test_font.save(temp_file)
    test_font = TTFont(temp_file)
    test_font.reader.file.name = "foo.ttf"
    assert_PASS(check(test_font))

    test_font["OS/2"].xAvgCharWidth = 450
    assert_results_contain(check(test_font), INFO, "xAvgCharWidth-close")

    test_font["OS/2"].xAvgCharWidth = 500
    assert_results_contain(check(test_font), WARN, "xAvgCharWidth-wrong")

    test_font = TTFont(temp_file)
    test_font.reader.file.name = "foo.ttf"
    subsetter = fontTools.subset.Subsetter()
    subsetter.populate(
        glyphs=[
            "b",
            "c",
            "d",
            "e",
            "f",
            "g",
            "h",
            "i",
            "j",
            "k",
            "l",
            "m",
            "n",
            "o",
            "p",
            "q",
            "r",
            "s",
            "t",
            "u",
            "v",
            "w",
            "x",
            "y",
            "z",
            "space",
        ]
    )
    subsetter.subset(test_font)
    assert list(check(test_font))[0].status == ERROR
