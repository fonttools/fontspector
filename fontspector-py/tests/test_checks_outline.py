from fontTools.ttLib import TTFont

from conftest import check_id
from fontbakery.status import WARN, SKIP
from fontbakery.codetesting import (
    assert_results_contain,
    TEST_FILE,
    assert_PASS,
)


@check_id("outline_alignment_miss")
def test_check_outline_alignment_os2_old(check):
    """Test that the outline_alignment_miss check works when
    the OS/2 table has a low version and does not have the
    xHeight and CapHeight fields that are normally used."""

    ttFont = TTFont(TEST_FILE("merriweather/Merriweather-Regular.ttf"))

    assert ttFont["OS/2"].version == 3

    results = check(ttFont)
    assert not any([r.status == WARN for r in results])
    # Passes (but only because there are too many near misses)
    assert_PASS(check(ttFont))

    # Downgrade OS/2 version
    ttFont["OS/2"].version = 2

    results = check(ttFont)
    assert not any([r.status == WARN for r in results])
    # Passes (but only because there are too many near misses)
    assert_PASS(check(ttFont))

    # Downgrade OS/2 to version 1
    ttFont["OS/2"].version = 1
    del ttFont["OS/2"].sxHeight
    del ttFont["OS/2"].sCapHeight
    del ttFont["OS/2"].usDefaultChar
    del ttFont["OS/2"].usBreakChar
    del ttFont["OS/2"].usMaxContext

    assert_results_contain(check(ttFont), WARN, "skip-cap-x-height-alignment")

















