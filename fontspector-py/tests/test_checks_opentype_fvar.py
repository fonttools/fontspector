from fontTools.ttLib import TTFont
from fontTools.ttLib.tables._f_v_a_r import Axis

from fontbakery.status import FAIL, WARN, SKIP
from fontbakery.codetesting import (
    assert_PASS,
    assert_results_contain,
    TEST_FILE,
)
from conftest import check_id
import pytest


@check_id("opentype/fvar/regular_coords_correct")
def test_check_varfont_regular_wght_coord(check):
    """The variable font 'wght' (Weight) axis coordinate
    must be 400 on the 'Regular' instance."""
    # Our reference varfont CabinVFBeta.ttf
    # has a good Regular:wght coordinate
    ttFont = TTFont("data/test/cabinvfbeta/CabinVFBeta.ttf")
    assert_PASS(check(ttFont))

    # We then ensure the check detects it when we
    # introduce the problem by setting a bad value:
    ttFont["fvar"].instances[0].coordinates["wght"] = 500
    msg = assert_results_contain(check(ttFont), FAIL, "wght-not-400")

    # Reload the original font.
    ttFont = TTFont("data/test/cabinvfbeta/CabinVFBeta.ttf")
    # Change the name of the first instance from 'Regular' (nameID 258)
    # to 'Medium' (nameID 259). The font now has no Regular instance.
    ttFont["fvar"].instances[0].subfamilyNameID = 259
    msg = assert_results_contain(check(ttFont), FAIL, "no-regular-instance")
    assert msg == ('"Regular" instance not present.')

    # Test with an italic variable font. The Italic instance must also be 400
    ttFont = TTFont(TEST_FILE("varfont/OpenSans-Italic[wdth,wght].ttf"))
    assert_PASS(check(ttFont))

    # Now test with a static font.
    # The test should be skipped due to an unfulfilled condition.
    ttFont = TTFont(TEST_FILE("source-sans-pro/TTF/SourceSansPro-Bold.ttf"))
    msg = assert_results_contain(check(ttFont), SKIP, "not-variable")


@check_id("opentype/fvar/regular_coords_correct")
def test_check_varfont_regular_wdth_coord(check):
    """The variable font 'wdth' (Width) axis coordinate
    must be 100 on the 'Regular' instance."""

    # Our reference varfont CabinVFBeta.ttf
    # has a good Regular:wdth coordinate
    ttFont = TTFont("data/test/cabinvfbeta/CabinVFBeta.ttf")
    assert_PASS(check(ttFont))

    # We then ensure the check detects it when we
    # introduce the problem by setting a bad value:
    ttFont["fvar"].instances[0].coordinates["wdth"] = 0
    msg = assert_results_contain(check(ttFont), FAIL, "wdth-not-100")

    # Reload the original font.
    ttFont = TTFont("data/test/cabinvfbeta/CabinVFBeta.ttf")
    # Change the name of the first instance from 'Regular' (nameID 258)
    # to 'Medium' (nameID 259). The font now has no Regular instance.
    ttFont["fvar"].instances[0].subfamilyNameID = 259
    msg = assert_results_contain(check(ttFont), FAIL, "no-regular-instance")
    assert msg == ('"Regular" instance not present.')

    # Test with an italic variable font. The Italic instance must also be 100
    ttFont = TTFont(TEST_FILE("varfont/OpenSans-Italic[wdth,wght].ttf"))
    assert_PASS(check(ttFont))

    # Now test with a static font.
    # The test should be skipped due to an unfulfilled condition.
    ttFont = TTFont(TEST_FILE("source-sans-pro/TTF/SourceSansPro-Bold.ttf"))
    msg = assert_results_contain(check(ttFont), SKIP, "not-variable")


@check_id("opentype/fvar/regular_coords_correct")
def test_check_varfont_regular_slnt_coord(check):
    """The variable font 'slnt' (Slant) axis coordinate
    must be zero on the 'Regular' instance."""

    # Our reference varfont, CabinVFBeta.ttf, lacks a 'slnt' variation axis.
    ttFont = TTFont("data/test/cabinvfbeta/CabinVFBeta.ttf")

    # So we add one:
    new_axis = Axis()
    new_axis.axisTag = "slnt"
    ttFont["fvar"].axes.append(new_axis)
    for instance in ttFont["fvar"].instances:
        instance.coordinates["slnt"] = 0

    # and specify a bad coordinate for the Regular:
    first_instance = ttFont["fvar"].instances[0]
    first_instance.coordinates["slnt"] = 12
    # Note: I know the correct instance index for this hotfix because
    # I inspected our reference CabinVF using ttx

    # And with this the check must detect the problem:
    msg = assert_results_contain(check(ttFont), FAIL, "slnt-not-0")

    # We correct the slant coordinate value to make the check PASS.
    first_instance.coordinates["slnt"] = 0
    assert_PASS(check(ttFont))

    # Change the name of the first instance from 'Regular' (nameID 258)
    # to 'Medium' (nameID 259). The font now has no Regular instance.
    first_instance.subfamilyNameID = 259
    msg = assert_results_contain(check(ttFont), FAIL, "no-regular-instance")
    assert msg == ('"Regular" instance not present.')

    # Now test with a static font.
    # The test should be skipped due to an unfulfilled condition.
    ttFont = TTFont(TEST_FILE("source-sans-pro/TTF/SourceSansPro-Bold.ttf"))
    msg = assert_results_contain(check(ttFont), SKIP, "not-variable")


@check_id("opentype/fvar/regular_coords_correct")
def test_check_varfont_regular_ital_coord(check):
    """The variable font 'ital' (Italic) axis coordinate
    must be zero on the 'Regular' instance."""
    # Our reference varfont, CabinVFBeta.ttf, lacks an 'ital' variation axis.
    ttFont = TTFont("data/test/cabinvfbeta/CabinVFBeta.ttf")

    # So we add one:
    new_axis = Axis()
    new_axis.axisTag = "ital"
    ttFont["fvar"].axes.append(new_axis)
    for instance in ttFont["fvar"].instances:
        instance.coordinates["ital"] = 0

    # and specify a bad coordinate for the Regular:
    first_instance = ttFont["fvar"].instances[0]
    first_instance.coordinates["ital"] = 123
    # Note: I know the correct instance index for this hotfix because
    # I inspected the our reference CabinVF using ttx

    # And with this the check must detect the problem:
    msg = assert_results_contain(check(ttFont), FAIL, "ital-not-0")

    # We correct the italic coordinate value to make the check PASS.
    first_instance.coordinates["ital"] = 0
    assert_PASS(check(ttFont))

    # Change the name of the first instance from 'Regular' (nameID 258)
    # to 'Medium' (nameID 259). The font now has no Regular instance.
    first_instance.subfamilyNameID = 259
    msg = assert_results_contain(check(ttFont), FAIL, "no-regular-instance")
    assert msg == ('"Regular" instance not present.')

    # Now test with a static font.
    # The test should be skipped due to an unfulfilled condition.
    ttFont = TTFont(TEST_FILE("source-sans-pro/TTF/SourceSansPro-It.ttf"))
    msg = assert_results_contain(check(ttFont), SKIP, "not-variable")


@pytest.mark.xfail(reason="We don't do MockFonts in fontspector")
@check_id("opentype/fvar/regular_coords_correct")
def test_check_varfont_regular_opsz_coord(check):
    """The variable font 'opsz' (Optical Size) axis coordinate
    should be between 10 and 16 on the 'Regular' instance."""

    # Our reference varfont, CabinVFBeta.ttf, lacks an 'opsz' variation axis.
    ttFont = TTFont("data/test/cabinvfbeta/CabinVFBeta.ttf")

    # So we add one:
    new_axis = Axis()
    new_axis.axisTag = "opsz"
    ttFont["fvar"].axes.append(new_axis)
    for instance in ttFont["fvar"].instances:
        instance.coordinates["opsz"] = 0

    # and specify a bad coordinate for the Regular:
    first_instance = ttFont["fvar"].instances[0]
    first_instance.coordinates["opsz"] = 9
    # Note: I know the correct instance index for this hotfix because
    # I inspected the our reference CabinVF using ttx

    # Then we ensure the problem is detected:
    assert_results_contain(
        check(ttFont),
        WARN,
        "opsz-out-of-range",
        "with a bad Regular:opsz coordinate (9)...",
    )

    # We try yet another bad value
    # and the check should detect the problem:
    assert_results_contain(
        check(MockFont(ttFont=ttFont, regular_opsz_coord=17)),
        WARN,
        "opsz-out-of-range",
        "with another bad Regular:opsz value (17)...",
    )

    # We then test with good default opsz values:
    for value in [10, 11, 12, 13, 14, 15, 16]:
        assert_PASS(
            check(MockFont(ttFont=ttFont, regular_opsz_coord=value)),
            f"with a good Regular:opsz coordinate ({value})...",
        )

    # Change the name of the first instance from 'Regular' (nameID 258)
    # to 'Medium' (nameID 259). The font now has no Regular instance.
    first_instance.subfamilyNameID = 259
    msg = assert_results_contain(check(ttFont), FAIL, "no-regular-instance")
    assert msg == ('"Regular" instance not present.')


@check_id("opentype/fvar/axis_ranges_correct")
def test_check_varfont_wght_valid_range(check):
    """The variable font 'wght' (Weight) axis coordinate
    must be within spec range of 1 to 1000 on all instances."""
    # Our reference varfont CabinVFBeta.ttf
    # has all instances within the 1-1000 range
    ttFont = TTFont("data/test/cabinvfbeta/CabinVFBeta.ttf")
    assert_PASS(check(ttFont), "with a good varfont...")

    # We then introduce the problem by setting a bad value:
    ttFont["fvar"].instances[0].coordinates["wght"] = 0
    assert_results_contain(check(ttFont), FAIL, "wght-out-of-range", "with wght=0...")

    # And yet another bad value:
    ttFont["fvar"].instances[0].coordinates["wght"] = 1001
    assert_results_contain(
        check(ttFont), FAIL, "wght-out-of-range", "with wght=1001..."
    )


@check_id("opentype/fvar/axis_ranges_correct")
def test_check_varfont_wdth_valid_range(check):
    """The variable font 'wdth' (Width) axis coordinate
    must be strictly greater than zero, per the spec."""
    # Our reference varfont CabinVFBeta.ttf
    # has all instances within the 1-1000 range
    ttFont = TTFont("data/test/cabinvfbeta/CabinVFBeta.ttf")
    assert_PASS(check(ttFont), "with a good varfont...")

    # We then introduce the problem by setting a bad value:
    ttFont["fvar"].instances[0].coordinates["wdth"] = 0
    assert_results_contain(check(ttFont), FAIL, "wdth-out-of-range", "with wght=0...")

    # A valid but unusual value:
    ttFont["fvar"].instances[0].coordinates["wdth"] = 1001
    assert_results_contain(
        check(ttFont), WARN, "wdth-greater-than-1000", "with wght=1001..."
    )


@check_id("opentype/fvar/axis_ranges_correct")
def test_check_varfont_slnt_range(check):
    """The variable font 'slnt' (Slant) axis coordinate
    specifies positive values in its range?"""
    # Our reference Inter varfont has a bad slnt range
    ttFont = TTFont("data/test/varfont/inter/Inter[slnt,wght].ttf")
    assert_results_contain(
        check(ttFont),
        WARN,
        "unusual-slnt-range",
        'with a varfont that has an unusual "slnt" range.',
    )

    # We then fix the font-bug by flipping the slnt axis range:
    for i, axis in enumerate(ttFont["fvar"].axes):
        if axis.axisTag == "slnt":
            minValue, maxValue = axis.minValue, axis.maxValue
            ttFont["fvar"].axes[i].minValue = -maxValue
            ttFont["fvar"].axes[i].maxValue = -minValue

    # And it must now be good ;-)
    assert_PASS(check(ttFont))


@check_id("opentype/varfont/foundry_defined_tag_name")
def test_check_varfont_foundry_defined_tag_name(check):
    "Validate foundry-defined design-variation axis tag names."
    # Our reference varfont CabinVFBeta.ttf has registered tags.
    ttFont = TTFont("data/test/cabinvfbeta/CabinVFBeta.ttf")
    assert_PASS(check(ttFont), "with a good varfont...")

    def frob_axis_tag(tag):
        ttFont["fvar"].axes[0].axisTag = tag
        for instance in ttFont["fvar"].instances:
            instance.coordinates[tag] = 400

    frob_axis_tag("GOOD")
    assert_PASS(check(ttFont), "with a good all uppercase axis tag...")

    frob_axis_tag("G009")
    assert_PASS(check(ttFont), "with all uppercase + digits...")

    frob_axis_tag("ITAL")
    assert_results_contain(
        check(ttFont),
        WARN,
        "foundry-defined-similar-registered-name",
        "with an uppercase version of registered tag...",
    )

    frob_axis_tag("nope")
    assert_results_contain(
        check(ttFont),
        FAIL,
        "invalid-foundry-defined-tag-first-letter",
        "when first letter of axis tag is not uppercase...",
    )

    frob_axis_tag("N0pe")
    assert_results_contain(
        check(ttFont),
        FAIL,
        "invalid-foundry-defined-tag-chars",
        "when characters not all uppercase-letters or digits...",
    )


@check_id("opentype/varfont/valid_nameids")
def test_check_varfont_valid_axis_nameid(check):
    """The value of axisNameID used by each VariationAxisRecord must
    be greater than 255 and less than 32768."""
    # The axisNameID values in the reference varfont are all valid
    ttFont = TTFont("data/test/cabinvf/Cabin[wdth,wght].ttf")
    assert_PASS(check(ttFont), "with a good varfont...")

    fvar_table = ttFont["fvar"]
    wght_axis = fvar_table.axes[0]
    wdth_axis = fvar_table.axes[1]

    # Change the axes' axisNameID to the maximum and minimum allowed values
    wght_axis.axisNameID = 32767
    wdth_axis.axisNameID = 256
    assert_PASS(check(ttFont), "with a good varfont...")

    # Change the axes' axisNameID to invalid values
    # (32768 is greater than the maximum, and 255 is less than the minimum)
    wght_axis.axisNameID = 32768
    wdth_axis.axisNameID = 255
    assert_results_contain(check(ttFont), FAIL, "invalid-axis-nameid:32768")
    msg = assert_results_contain(check(ttFont), FAIL, "invalid-axis-nameid:255")

    # Another set of invalid values
    wght_axis.axisNameID = 128
    wdth_axis.axisNameID = 36000
    assert_results_contain(check(ttFont), FAIL, "invalid-axis-nameid:128")
    msg = assert_results_contain(check(ttFont), FAIL, "invalid-axis-nameid:36000")


@check_id("opentype/varfont/valid_nameids")
def test_check_varfont_valid_subfamily_nameid(check):
    """The value of subfamilyNameID used by each InstanceRecord must
    be 2, 17, or greater than 255 and less than 32768."""
    # The subfamilyNameID values in the reference varfont are all valid
    ttFont = TTFont("data/test/cabinvf/Cabin[wdth,wght].ttf")
    assert_PASS(check(ttFont), "with a good varfont...")

    fvar_table = ttFont["fvar"]
    inst_1 = fvar_table.instances[0]
    inst_2 = fvar_table.instances[1]
    inst_3 = fvar_table.instances[2]
    inst_4 = fvar_table.instances[3]

    # Change the instances' subfamilyNameID to
    # 2, 17 and to the maximum and minimum allowed values
    inst_1.subfamilyNameID = 2
    inst_2.subfamilyNameID = 17
    inst_3.subfamilyNameID = 256
    inst_4.subfamilyNameID = 32767
    assert_PASS(check(ttFont), "with a good varfont...")

    # Change two instances' subfamilyNameID to invalid values
    # (32768 is greater than the maximum, and 255 is less than the minimum)
    inst_3.subfamilyNameID = 255
    inst_4.subfamilyNameID = 32768
    assert_results_contain(check(ttFont), FAIL, "invalid-subfamily-nameid:255")
    msg = assert_results_contain(check(ttFont), FAIL, "invalid-subfamily-nameid:32768")

    # Reset two subfamilyNameID to valid values,
    # then set two other subfamilyNameID to invalid values
    inst_3.subfamilyNameID = 256  # valid
    inst_4.subfamilyNameID = 32767  # valid
    inst_1.subfamilyNameID = 3
    inst_2.subfamilyNameID = 18
    assert_results_contain(check(ttFont), FAIL, "invalid-subfamily-nameid:3")
    msg = assert_results_contain(check(ttFont), FAIL, "invalid-subfamily-nameid:18")


@check_id("opentype/varfont/valid_nameids")
def test_check_varfont_valid_postscript_nameid(check):
    """The value of postScriptNameID used by each InstanceRecord must
    be 6, 0xFFFF, or greater than 255 and less than 32768."""
    # The postScriptNameID values in the reference varfont are all valid
    ttFont = TTFont("data/test/cabinvf/Cabin[wdth,wght].ttf")
    assert_PASS(check(ttFont), "with a good varfont...")

    fvar_table = ttFont["fvar"]
    inst_1 = fvar_table.instances[0]
    inst_2 = fvar_table.instances[1]
    inst_3 = fvar_table.instances[2]
    inst_4 = fvar_table.instances[3]

    # Change the instances' postScriptNameID to
    # 6, 0xFFFF and to the maximum and minimum allowed values
    inst_1.postscriptNameID = 6
    inst_2.postscriptNameID = 0xFFFF
    inst_3.postscriptNameID = 256
    inst_4.postscriptNameID = 32767
    assert_PASS(check(ttFont), "with a good varfont...")

    # Change two instances' postScriptNameID to invalid values
    # (32768 is greater than the maximum, and 255 is less than the minimum)
    inst_3.postscriptNameID = 255
    inst_4.postscriptNameID = 32768
    assert_results_contain(check(ttFont), FAIL, "invalid-postscript-nameid:255")
    msg = assert_results_contain(check(ttFont), FAIL, "invalid-postscript-nameid:32768")

    # Reset two postScriptNameID to valid values,
    # then set two other postScriptNameID to invalid values
    inst_3.postscriptNameID = 256  # valid
    inst_4.postscriptNameID = 32767  # valid
    inst_1.postscriptNameID = 3
    inst_2.postscriptNameID = 18
    assert_results_contain(check(ttFont), FAIL, "invalid-postscript-nameid:3")
    msg = assert_results_contain(check(ttFont), FAIL, "invalid-postscript-nameid:18")


@check_id("opentype/varfont/valid_default_instance_nameids")
def test_check_varfont_valid_default_instance_nameids(check):
    """If an instance record is included for the default instance, then the instance's
    subfamilyName string should match the string of nameID 2 or nameID 17, and the
    instance's postScriptName string should match the string of nameID 6."""

    # The font's 'Regular' instance record has the same coordinates as the default
    # instance, and the record's string matches the string of nameID 2.
    ttFont_1 = TTFont(TEST_FILE("cabinvf/Cabin[wdth,wght].ttf"))
    assert_PASS(check(ttFont_1))

    # The font's 'LightCondensed' instance record has the same coordinates as the
    # default instance, and the record's string matches the string of nameID 17.
    ttFont_2 = TTFont(TEST_FILE("mutatorsans-vf/MutatorSans-VF.ttf"))
    assert_PASS(check(ttFont_2))

    # Change subfamilyNameID value of the default instance to another name ID whose
    # string doesn't match the font's Subfamily name, thus making the check fail.
    fvar_table_1 = ttFont_1["fvar"]
    dflt_inst = fvar_table_1.instances[0]
    dflt_inst.subfamilyNameID = 16  # the font doesn't have this record
    msg = assert_results_contain(
        check(ttFont_1), FAIL, "invalid-default-instance-subfamily-name"
    )
    # Restore the original ID
    dflt_inst.subfamilyNameID = 258

    fvar_table_2 = ttFont_2["fvar"]
    dflt_inst = fvar_table_2.instances[0]
    dflt_inst.subfamilyNameID = 16
    msg = assert_results_contain(
        check(ttFont_2), FAIL, "invalid-default-instance-subfamily-name"
    )
    # Restore the original ID
    dflt_inst.subfamilyNameID = 258

    # The value of postScriptNameID is 0xFFFF for all the instance records in CabinVF.
    # Change one of them, to make the check validate the postScriptNameID value of the
    # default instance (which is currently 0xFFFF).
    inst_2 = fvar_table_1.instances[1]
    inst_2.postscriptNameID = 256  # the font doesn't have this record
    msg = assert_results_contain(
        check(ttFont_1), FAIL, "invalid-default-instance-postscript-name"
    )

    # The default instance of MutatorSans-VF has the correct postScriptNameID.
    # Change it to make the check fail.
    inst_1 = fvar_table_2.instances[0]
    inst_1.postscriptNameID = 261
    msg = assert_results_contain(
        check(ttFont_2), FAIL, "invalid-default-instance-postscript-name"
    )


@check_id("opentype/varfont/same_size_instance_records")
def test_check_varfont_same_size_instance_records(check):
    """All of the instance records in a given font must have the same size,
    with all either including or omitting the postScriptNameID field. If the value
    is 0xFFFF it means that no PostScript name is provided for the instance."""
    # The value of postScriptNameID is 0xFFFF for all the instance records in the
    # reference varfont
    ttFont = TTFont("data/test/cabinvf/Cabin[wdth,wght].ttf")
    assert_PASS(check(ttFont), "with a good varfont...")

    fvar_table = ttFont["fvar"]
    inst_1 = fvar_table.instances[0]
    inst_2 = fvar_table.instances[1]
    inst_3 = fvar_table.instances[2]
    inst_4 = fvar_table.instances[3]

    # Change the postScriptNameID of one instance record
    inst_1.postscriptNameID = 256
    msg = assert_results_contain(check(ttFont), FAIL, "different-size-instance-records")
    assert msg == "Instance records don't all have the same size."

    # Change the postScriptNameID of the remaining instance records
    inst_2.postscriptNameID = 356
    inst_3.postscriptNameID = 456
    inst_4.postscriptNameID = 556
    assert_PASS(check(ttFont), "with a good varfont...")

    # Change the postScriptNameID of two instance records
    inst_1.postscriptNameID = 0xFFFF
    inst_3.postscriptNameID = 0xFFFF
    msg = assert_results_contain(check(ttFont), FAIL, "different-size-instance-records")
    assert msg == "Instance records don't all have the same size."

    fvar_table.instances = []
    msg = assert_results_contain(check(ttFont), SKIP, "no-instance-records")
    assert msg == "Font has no instance records."


@check_id("opentype/varfont/distinct_instance_records")
def test_check_varfont_distinct_instance_records(check):
    """All of the instance records in a font should have distinct coordinates
    and distinct subfamilyNameID and postScriptName ID values."""
    # All of the instance records in the reference varfont are unique
    ttFont = TTFont("data/test/cabinvf/Cabin[wdth,wght].ttf")
    assert_PASS(check(ttFont), "with a good varfont...")

    fvar_table = ttFont["fvar"]
    inst_1 = fvar_table.instances[0]
    inst_2 = fvar_table.instances[1]
    inst_3 = fvar_table.instances[2]
    inst_4 = fvar_table.instances[3]

    # Make instance 2 the same as instance 1
    inst_2.subfamilyNameID = inst_1.subfamilyNameID
    inst_2.coordinates["wght"] = inst_1.coordinates["wght"]
    msg = assert_results_contain(
        check(ttFont), WARN, "repeated-instance-record:Regular"
    )

    # Make instance 4 the same as instance 3
    inst_4.subfamilyNameID = inst_3.subfamilyNameID
    inst_4.coordinates["wght"] = inst_3.coordinates["wght"]
    msg = assert_results_contain(
        check(ttFont), WARN, "repeated-instance-record:SemiBold"
    )


@check_id("opentype/varfont/family_axis_ranges")
def test_check_varfont_family_axis_ranges(check):
    """Check that family axis ranges are indentical"""
    ttFonts = [
        TTFont("data/test/ubuntusansmono/UbuntuMono[wght].ttf"),
        TTFont("data/test/ubuntusansmono/UbuntuMono-Italic[wght].ttf"),
    ]
    assert_results_contain(check(ttFonts), FAIL, "axis-range-mismatch")

    ttFonts = [
        TTFont("data/test/cabinvf/Cabin[wdth,wght].ttf"),
        TTFont("data/test/cabinvf/Cabin-Italic[wdth,wght].ttf"),
    ]
    assert_PASS(check(ttFonts), "with good varfont...")


@check_id("opentype/slant_direction")
def test_check_slant_direction(check):
    """Checking direction of slnt axis angles."""
    font = TEST_FILE("slant_direction/Cairo_correct_slnt_axis.ttf")
    assert_PASS(check(TTFont(font)))

    font = TEST_FILE("slant_direction/Cairo_wrong_slnt_axis.ttf")
    assert_results_contain(
        check(TTFont(font)), FAIL, "positive-value-for-clockwise-lean"
    )
