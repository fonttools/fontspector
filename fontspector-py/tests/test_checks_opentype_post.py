import tempfile

from fontTools.ttLib import TTFont

from fontbakery.status import WARN, FAIL, PASS
from fontbakery.codetesting import (
    assert_PASS,
    assert_results_contain,
    TEST_FILE,
)
from conftest import check_id


@check_id("opentype/italic_angle")
def test_check_italic_angle(check):
    """Checking post.italicAngle value."""
    ttFont = TTFont(TEST_FILE("cabin/Cabin-Regular.ttf"))

    # italic-angle, style, fail_message
    test_cases = [
        [1, "Italic", WARN, "positive"],
        [0, "Regular", PASS, None],  # This must PASS as it is a non-italic
        [-21, "ThinItalic", WARN, "over-20-degrees"],
        [-30, "ThinItalic", WARN, "over-20-degrees"],
        [-31, "ThinItalic", WARN, "over-30-degrees"],
        [-91, "ThinItalic", FAIL, "over-90-degrees"],
        [0, "Italic", FAIL, "zero-italic"],
        [-1, "ExtraBold", FAIL, "non-zero-upright"],
    ]

    for value, style, expected_result, expected_msg in test_cases:
        ttFont["post"].italicAngle = value
        with tempfile.NamedTemporaryFile(suffix="-" + style + ".ttf") as temp:
            ttFont.save(temp)

            if expected_result != PASS:
                assert_results_contain(
                    check(temp.name),
                    expected_result,
                    expected_msg,
                    f"with italic-angle:{value} style:{style}...",
                )
            else:
                assert_PASS(
                    check(temp.name),
                    f"with italic-angle:{value} style:{style}...",
                )

    # Cairo, check left and right-leaning explicitly
    ttFont = TTFont(TEST_FILE("cairo/CairoPlay-Italic.rightslanted.ttf"))
    with tempfile.NamedTemporaryFile(suffix="-Italic.ttf") as temp:
        ttFont.save(temp)
        assert_PASS(check(temp.name))
    with tempfile.NamedTemporaryFile(suffix="-Italic.ttf") as temp:
        ttFont["post"].italicAngle *= -1
        ttFont.save(temp)
        assert_results_contain(check(temp.name), WARN, "positive")

    ttFont = TTFont(TEST_FILE("cairo/CairoPlay-Italic.leftslanted.ttf"))
    with tempfile.NamedTemporaryFile(suffix="-Italic.ttf") as temp:
        ttFont.save(temp)
        assert_PASS(check(temp.name))
    ttFont["post"].italicAngle *= -1
    with tempfile.NamedTemporaryFile(suffix="-Italic.ttf") as temp:
        ttFont.save(temp)
        assert_results_contain(check(temp.name), WARN, "negative")

    ttFont = TTFont(TEST_FILE("cairo/CairoPlay-Italic.rightslanted.ttf"))
    with tempfile.NamedTemporaryFile(suffix="-Italic.ttf") as temp:
        ttFont.save(temp)
        assert_PASS(check(temp.name))
    with tempfile.NamedTemporaryFile(suffix="-Italic.ttf") as temp:
        ttFont["glyf"]["I"].endPtsOfContours = []
        ttFont["glyf"]["I"].coordinates = []
        ttFont["glyf"]["I"].flags = []
        ttFont["glyf"]["I"].numberOfContours = 0
        ttFont.save(temp)
        assert_results_contain(check(temp.name), WARN, "empty-glyphs")
