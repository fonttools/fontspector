from fontTools.ttLib import TTFont

from fontbakery.codetesting import (
    assert_PASS,
    assert_results_contain,
    TEST_FILE,
)
from fontbakery.status import FAIL
from conftest import check_id
import pytest


def test_check_fvar_name_entries():
    """Check variable font instances."""
    # TODO: Implement-me!


def test_check_varfont_consistent_axes():
    """Check variable font instances."""
    # TODO: Implement-me!


@check_id("varfont/duplexed_axis_reflow")
def test_check_varfont_duplexed_axis_reflow(check):
    """Ensure VFs with the GRAD axis do not vary horizontal advance."""
    ttFont = TTFont(TEST_FILE("BadGrades/BadGrades-VF.ttf"))
    assert_results_contain(check(ttFont), FAIL, "grad-causes-reflow")

    # Zero out the horizontal advances
    gvar = ttFont["gvar"]
    for glyph, deltas in gvar.variations.items():
        for delta in deltas:
            if "GRAD" not in delta.axes:
                continue
            if delta.coordinates:
                delta.coordinates = delta.coordinates[:-4] + [(0, 0)] * 4

    # But the kern rules should still be a problem
    assert_results_contain(check(ttFont), FAIL, "duplexed-kern-causes-reflow")

    ttFont["GPOS"].table.LookupList.Lookup = []
    assert_PASS(check(ttFont))

    ttFont = TTFont(TEST_FILE("bad_fonts/reflowing_ROND/BadRoundness-VF.ttf"))
    assert_results_contain(check(ttFont), FAIL, "rond-causes-reflow")
