from fontTools.ttLib import TTFont
import pytest

from conftest import check_id
from fontbakery.status import FAIL, WARN, SKIP
from fontbakery.codetesting import (
    assert_PASS,
    assert_results_contain,
    TEST_FILE,
)


@check_id("contour_count")
def test_check_contour_count(check):
    """Check glyphs contain the recommended contour count"""
    ttFont = TTFont(TEST_FILE("ibmplexsans-vf/IBMPlexSansVar-Roman.ttf"))

    assert_PASS(check(ttFont))

    # Lets swap the glyf 'Idotaccent' (3 contours) with glyf 'c' (1 contour)
    ttFont["glyf"]["Idotaccent"] = ttFont["glyf"]["c"]
    msg = assert_results_contain(check(ttFont), WARN, "contour-count")

    # Lets swap the glyf 'Idotaccent' (3 contours) with space (0 contour) to get a FAIL
    ttFont["glyf"]["Idotaccent"] = ttFont["glyf"]["space"]
    msg = assert_results_contain(check(ttFont), FAIL, "no-contour")
