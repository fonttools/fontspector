import json
import os
import tempfile
import pytest

from conftest import check_id
from fontbakery.status import FAIL, WARN
from fontbakery.codetesting import (
    assert_PASS,
    assert_SKIP,
    assert_results_contain,
    TEST_FILE,
)


@check_id("shaping/regression")
def test_check_shaping_regression(check):
    """Check that we can test shaping against expectations."""

    shaping_test = {
        "configuration": {},
        "tests": [{"input": "AV", "expectation": "A=0+664|V=1+691"}],
    }

    with tempfile.TemporaryDirectory() as tmp_gf_dir:
        json.dump(
            shaping_test,
            open(os.path.join(tmp_gf_dir, "test.json"), "w", encoding="utf-8"),
        )

        config = {"shaping": {"test_directory": tmp_gf_dir}}

        font = TEST_FILE("nunito/Nunito-Regular.ttf")
        assert_PASS(check(font, config=config), "Nunito: A=664,V=691")

        font = TEST_FILE("slabo/Slabo13px.ttf")
        assert_results_contain(
            check(font, config=config),
            FAIL,
            "shaping-regression",
            "Slabo: A!=664,V!=691",
        )


@pytest.mark.skip(
    reason="Our implementation is fine but we're getting wrong answers due to https://github.com/harfbuzz/rustybuzz/issues/156"
)
@check_id("shaping/regression")
def test_check_shaping_regression_with_variations(check):
    """Check that we can test shaping with variation settings against expectations."""

    shaping_test = {
        "configuration": {},
        "tests": [
            {
                "input": "AV",
                "expectation": "A=0+453|V=1+505",
            },
            {
                "input": "AV",
                "expectation": "A=0+517|V=1+526",
                "variations": {"wght": 700},
            },
        ],
    }

    with tempfile.TemporaryDirectory() as tmp_gf_dir:
        json.dump(
            shaping_test,
            open(os.path.join(tmp_gf_dir, "test.json"), "w", encoding="utf-8"),
        )

        config = {"shaping": {"test_directory": tmp_gf_dir}}

        font = TEST_FILE("varfont/Oswald-VF.ttf")
        assert_PASS(check(font, config=config), "Oswald: A=0+453|V=1+505")


@pytest.mark.skip("Check not ported yet")
@check_id("shaping/collides")
def test_check_shaping_collides(check):
    """Check that we can test for colliding glyphs in output."""

    shaping_test = {
        "configuration": {"collidoscope": {"area": 0, "bases": True, "marks": True}},
        "tests": [{"input": "ïï"}],
    }

    with tempfile.TemporaryDirectory() as tmp_gf_dir:
        json.dump(
            shaping_test,
            open(os.path.join(tmp_gf_dir, "test.json"), "w", encoding="utf-8"),
        )

        config = {"shaping": {"test_directory": tmp_gf_dir}}

        font = TEST_FILE("cousine/Cousine-Regular.ttf")
        assert_PASS(check(font, config=config), "ïï doesn't collide in Cousine")

        font = TEST_FILE("nunito/Nunito-Black.ttf")
        assert_results_contain(
            check(font, config=config),
            FAIL,
            "shaping-collides",
            "ïï collides in Nunito",
        )
