# Copyright 2016 The Fontbakery Authors
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# This module within fontspectorapi was largely taken from Fontbakery to improve
# the migration experience. See:
# - https://github.com/fonttools/fontbakery/blob/9a85e003d36ebfbbfe68c6d362e5db5a6434332c/Lib/fontbakery/utils.py
# - https://github.com/fonttools/fontbakery/blob/9a85e003d36ebfbbfe68c6d362e5db5a6434332c/Lib/fontbakery/constants.py
# Some functions were removed, some things inlined, and type hints added
# throughout.

from __future__ import annotations

import os
import subprocess
from typing import Any, Iterable
from fontTools.ttLib import TTFont
from fontTools.ttLib.tables._n_a_m_e import NameRecord
from fontTools.ttLib.tables._f_v_a_r import Axis

from fontspectorapi import FAIL, PASS, Message, StatusYield

UNICODERANGE_DATA: list[list[tuple[int, str, int, int]]] = [
    [(0, "Basic Latin", 0x00000, 0x0007F)],
    [(1, "Latin-1 Supplement", 0x00080, 0x000FF)],
    [(2, "Latin Extended-A", 0x00100, 0x0017F)],
    [(3, "Latin Extended-B", 0x00180, 0x0024F)],
    [
        (4, "IPA Extensions", 0x00250, 0x002AF),
        (4, "Phonetic Extensions", 0x01D00, 0x01D7F),
        (4, "Phonetic Extensions Supplement", 0x01D80, 0x01DBF),
    ],
    [
        (5, "Spacing Modifier Letters", 0x002B0, 0x002FF),
        (5, "Modifier Tone Letters", 0x0A700, 0x0A71F),
    ],
    [
        (6, "Combining Diacritical Marks", 0x00300, 0x0036F),
        (6, "Combining Diacritical Marks Supplement", 0x01DC0, 0x01DFF),
    ],
    [(7, "Greek and Coptic", 0x00370, 0x003FF)],
    [(8, "Coptic", 0x02C80, 0x02CFF)],
    [
        (9, "Cyrillic", 0x00400, 0x004FF),
        (9, "Cyrillic Supplement", 0x00500, 0x0052F),
        (9, "Cyrillic Extended-A", 0x02DE0, 0x02DFF),
        (9, "Cyrillic Extended-B", 0x0A640, 0x0A69F),
    ],
    [(10, "Armenian", 0x00530, 0x0058F)],
    [(11, "Hebrew", 0x00590, 0x005FF)],
    [(12, "Vai", 0x0A500, 0x0A63F)],
    [(13, "Arabic", 0x00600, 0x006FF), (13, "Arabic Supplement", 0x00750, 0x0077F)],
    [(14, "NKo", 0x007C0, 0x007FF)],
    [(15, "Devanagari", 0x00900, 0x0097F)],
    [(16, "Bengali", 0x00980, 0x009FF)],
    [(17, "Gurmukhi", 0x00A00, 0x00A7F)],
    [(18, "Gujarati", 0x00A80, 0x00AFF)],
    [(19, "Oriya", 0x00B00, 0x00B7F)],
    [(20, "Tamil", 0x00B80, 0x00BFF)],
    [(21, "Telugu", 0x00C00, 0x00C7F)],
    [(22, "Kannada", 0x00C80, 0x00CFF)],
    [(23, "Malayalam", 0x00D00, 0x00D7F)],
    [(24, "Thai", 0x00E00, 0x00E7F)],
    [(25, "Lao", 0x00E80, 0x00EFF)],
    [(26, "Georgian", 0x010A0, 0x010FF), (26, "Georgian Supplement", 0x02D00, 0x02D2F)],
    [(27, "Balinese", 0x01B00, 0x01B7F)],
    [(28, "Hangul Jamo", 0x01100, 0x011FF)],
    [
        (29, "Latin Extended Additional", 0x01E00, 0x01EFF),
        (29, "Latin Extended-C", 0x02C60, 0x02C7F),
        (29, "Latin Extended-D", 0x0A720, 0x0A7FF),
    ],
    [(30, "Greek Extended", 0x01F00, 0x01FFF)],
    [
        (31, "General Punctuation", 0x02000, 0x0206F),
        (31, "Supplemental Punctuation", 0x02E00, 0x02E7F),
    ],
    [(32, "Superscripts And Subscripts", 0x02070, 0x0209F)],
    [(33, "Currency Symbols", 0x020A0, 0x020CF)],
    [(34, "Combining Diacritical Marks For Symbols", 0x020D0, 0x020FF)],
    [(35, "Letterlike Symbols", 0x02100, 0x0214F)],
    [(36, "Number Forms", 0x02150, 0x0218F)],
    [
        (37, "Arrows", 0x02190, 0x021FF),
        (37, "Supplemental Arrows-A", 0x027F0, 0x027FF),
        (37, "Supplemental Arrows-B", 0x02900, 0x0297F),
        (37, "Miscellaneous Symbols and Arrows", 0x02B00, 0x02BFF),
    ],
    [
        (38, "Mathematical Operators", 0x02200, 0x022FF),
        (38, "Supplemental Mathematical Operators", 0x02A00, 0x02AFF),
        (38, "Miscellaneous Mathematical Symbols-A", 0x027C0, 0x027EF),
        (38, "Miscellaneous Mathematical Symbols-B", 0x02980, 0x029FF),
    ],
    [(39, "Miscellaneous Technical", 0x02300, 0x023FF)],
    [(40, "Control Pictures", 0x02400, 0x0243F)],
    [(41, "Optical Character Recognition", 0x02440, 0x0245F)],
    [(42, "Enclosed Alphanumerics", 0x02460, 0x024FF)],
    [(43, "Box Drawing", 0x02500, 0x0257F)],
    [(44, "Block Elements", 0x02580, 0x0259F)],
    [(45, "Geometric Shapes", 0x025A0, 0x025FF)],
    [(46, "Miscellaneous Symbols", 0x02600, 0x026FF)],
    [(47, "Dingbats", 0x02700, 0x027BF)],
    [(48, "CJK Symbols And Punctuation", 0x03000, 0x0303F)],
    [(49, "Hiragana", 0x03040, 0x0309F)],
    [
        (50, "Katakana", 0x030A0, 0x030FF),
        (50, "Katakana Phonetic Extensions", 0x031F0, 0x031FF),
    ],
    [(51, "Bopomofo", 0x03100, 0x0312F), (51, "Bopomofo Extended", 0x031A0, 0x031BF)],
    [(52, "Hangul Compatibility Jamo", 0x03130, 0x0318F)],
    [(53, "Phags-pa", 0x0A840, 0x0A87F)],
    [(54, "Enclosed CJK Letters And Months", 0x03200, 0x032FF)],
    [(55, "CJK Compatibility", 0x03300, 0x033FF)],
    [(56, "Hangul Syllables", 0x0AC00, 0x0D7AF)],
    [(57, "Non-Plane 0 *", 0x10000, 0x10FFFF)],
    [(58, "Phoenician", 0x10900, 0x1091F)],
    [
        (59, "CJK Unified Ideographs", 0x04E00, 0x09FFF),
        (59, "CJK Radicals Supplement", 0x02E80, 0x02EFF),
        (59, "Kangxi Radicals", 0x02F00, 0x02FDF),
        (59, "Ideographic Description Characters", 0x02FF0, 0x02FFF),
        (59, "CJK Unified Ideographs Extension A", 0x03400, 0x04DBF),
        (59, "CJK Unified Ideographs Extension B", 0x20000, 0x2A6DF),
        (59, "Kanbun", 0x03190, 0x0319F),
    ],
    [(60, "Private Use Area (plane 0)", 0x0E000, 0x0F8FF)],
    [
        (61, "CJK Strokes", 0x031C0, 0x031EF),
        (61, "CJK Compatibility Ideographs", 0x0F900, 0x0FAFF),
        (61, "CJK Compatibility Ideographs Supplement", 0x2F800, 0x2FA1F),
    ],
    [(62, "Alphabetic Presentation Forms", 0x0FB00, 0x0FB4F)],
    [(63, "Arabic Presentation Forms-A", 0x0FB50, 0x0FDFF)],
    [(64, "Combining Half Marks", 0x0FE20, 0x0FE2F)],
    [
        (65, "Vertical Forms", 0x0FE10, 0x0FE1F),
        (65, "CJK Compatibility Forms", 0x0FE30, 0x0FE4F),
    ],
    [(66, "Small Form Variants", 0x0FE50, 0x0FE6F)],
    [(67, "Arabic Presentation Forms-B", 0x0FE70, 0x0FEFF)],
    [(68, "Halfwidth And Fullwidth Forms", 0x0FF00, 0x0FFEF)],
    [(69, "Specials", 0x0FFF0, 0x0FFFF)],
    [(70, "Tibetan", 0x00F00, 0x00FFF)],
    [(71, "Syriac", 0x00700, 0x0074F)],
    [(72, "Thaana", 0x00780, 0x007BF)],
    [(73, "Sinhala", 0x00D80, 0x00DFF)],
    [(74, "Myanmar", 0x01000, 0x0109F)],
    [
        (75, "Ethiopic", 0x01200, 0x0137F),
        (75, "Ethiopic Supplement", 0x01380, 0x0139F),
        (75, "Ethiopic Extended", 0x02D80, 0x02DDF),
    ],
    [(76, "Cherokee", 0x013A0, 0x013FF)],
    [(77, "Unified Canadian Aboriginal Syllabics", 0x01400, 0x0167F)],
    [(78, "Ogham", 0x01680, 0x0169F)],
    [(79, "Runic", 0x016A0, 0x016FF)],
    [(80, "Khmer", 0x01780, 0x017FF), (80, "Khmer Symbols", 0x019E0, 0x019FF)],
    [(81, "Mongolian", 0x01800, 0x018AF)],
    [(82, "Braille Patterns", 0x02800, 0x028FF)],
    [(83, "Yi Syllables", 0x0A000, 0x0A48F), (83, "Yi Radicals", 0x0A490, 0x0A4CF)],
    [
        (84, "Tagalog", 0x01700, 0x0171F),
        (84, "Hanunoo", 0x01720, 0x0173F),
        (84, "Buhid", 0x01740, 0x0175F),
        (84, "Tagbanwa", 0x01760, 0x0177F),
    ],
    [(85, "Old Italic", 0x10300, 0x1032F)],
    [(86, "Gothic", 0x10330, 0x1034F)],
    [(87, "Deseret", 0x10400, 0x1044F)],
    [
        (88, "Byzantine Musical Symbols", 0x1D000, 0x1D0FF),
        (88, "Musical Symbols", 0x1D100, 0x1D1FF),
        (88, "Ancient Greek Musical Notation", 0x1D200, 0x1D24F),
    ],
    [(89, "Mathematical Alphanumeric Symbols", 0x1D400, 0x1D7FF)],
    [
        (90, "Private Use (plane 15)", 0xFF000, 0xFFFFD),
        (90, "Private Use (plane 16)", 0x100000, 0x10FFFD),
    ],
    [
        (91, "Variation Selectors", 0x0FE00, 0x0FE0F),
        (91, "Variation Selectors Supplement", 0xE0100, 0xE01EF),
    ],
    [(92, "Tags", 0xE0000, 0xE007F)],
    [(93, "Limbu", 0x01900, 0x0194F)],
    [(94, "Tai Le", 0x01950, 0x0197F)],
    [(95, "New Tai Lue", 0x01980, 0x019DF)],
    [(96, "Buginese", 0x01A00, 0x01A1F)],
    [(97, "Glagolitic", 0x02C00, 0x02C5F)],
    [(98, "Tifinagh", 0x02D30, 0x02D7F)],
    [(99, "Yijing Hexagram Symbols", 0x04DC0, 0x04DFF)],
    [(100, "Syloti Nagri", 0x0A800, 0x0A82F)],
    [
        (101, "Linear B Syllabary", 0x10000, 0x1007F),
        (101, "Linear B Ideograms", 0x10080, 0x100FF),
        (101, "Aegean Numbers", 0x10100, 0x1013F),
    ],
    [(102, "Ancient Greek Numbers", 0x10140, 0x1018F)],
    [(103, "Ugaritic", 0x10380, 0x1039F)],
    [(104, "Old Persian", 0x103A0, 0x103DF)],
    [(105, "Shavian", 0x10450, 0x1047F)],
    [(106, "Osmanya", 0x10480, 0x104AF)],
    [(107, "Cypriot Syllabary", 0x10800, 0x1083F)],
    [(108, "Kharoshthi", 0x10A00, 0x10A5F)],
    [(109, "Tai Xuan Jing Symbols", 0x1D300, 0x1D35F)],
    [
        (110, "Cuneiform", 0x12000, 0x123FF),
        (110, "Cuneiform Numbers and Punctuation", 0x12400, 0x1247F),
    ],
    [(111, "Counting Rod Numerals", 0x1D360, 0x1D37F)],
    [(112, "Sundanese", 0x01B80, 0x01BBF)],
    [(113, "Lepcha", 0x01C00, 0x01C4F)],
    [(114, "Ol Chiki", 0x01C50, 0x01C7F)],
    [(115, "Saurashtra", 0x0A880, 0x0A8DF)],
    [(116, "Kayah Li", 0x0A900, 0x0A92F)],
    [(117, "Rejang", 0x0A930, 0x0A95F)],
    [(118, "Cham", 0x0AA00, 0x0AA5F)],
    [(119, "Ancient Symbols", 0x10190, 0x101CF)],
    [(120, "Phaistos Disc", 0x101D0, 0x101FF)],
    [
        (121, "Carian", 0x102A0, 0x102DF),
        (121, "Lycian", 0x10280, 0x1029F),
        (121, "Lydian", 0x10920, 0x1093F),
    ],
    [(122, "Domino Tiles", 0x1F030, 0x1F09F), (122, "Mahjong Tiles", 0x1F000, 0x1F02F)],
]


def remove_white_space(s: str) -> str:
    s = s.replace(" ", "")
    s = s.replace("\t", "")
    s = s.replace("\n", "")
    return s


def get_apple_terminal_bg_color() -> str:
    """Runs an AppleScript snippet that returns the RGB values of the
    background color of the active Apple Terminal window."""
    line_1 = 'tell application "Terminal"'
    line_2 = "    get background color of selected tab of window 1"
    line_3 = "end tell"
    output = subprocess.run(
        ["osascript", "-e", line_1, "-e", line_2, "-e", line_3],
        text=True,
        check=True,
        stdout=subprocess.PIPE,
    ).stdout
    return output.strip()


def apple_terminal_bg_is_white() -> bool:
    """Returns a boolean indicating if the background color
    of Apple's Terminal is white."""
    is_apple_terminal = os.getenv("TERM_PROGRAM") == "Apple_Terminal"
    if is_apple_terminal:
        bg_color = get_apple_terminal_bg_color()
        if bg_color == "65535, 65535, 65535":
            return True
    return False


def unindent_and_unwrap_rationale(rationale: str) -> str:
    """Takes the 'rationale' docstring of a check and removes indents and hard line
    breaks that were added to long lines."""
    content = ""

    for line in rationale.split("\n"):
        soft_return = line.endswith("⏎")  # U+23CE
        stripped_line = line.strip()
        new_paragraph = len(stripped_line) == 0

        if new_paragraph:
            content = content.rstrip()
            content += "\n\n"

        else:
            content += stripped_line

            if soft_return:
                content = f"{content[:-1]}\n"
            else:
                content += " "

    return f"\n{content.strip()}\n"


def split_camel_case(camelcase: str) -> str:
    chars = []
    for i, char in enumerate(camelcase):
        if char.isupper() and i > 0:
            chars.append(" ")
        chars.append(char)

    return "".join(chars)


def pretty_print_list(
    values: list[Any],
    *,
    shorten: int | None = 10,
    sep: str = ", ",
    glue: str = " and ",
    quiet: bool = False,
    full_lists: bool = False,
) -> str:
    if len(values) == 1:
        return str(values[0])

    if full_lists:
        shorten = None

    if shorten and len(values) > shorten + 2:
        joined_items_str = sep.join(map(str, values[:shorten]))
        msg = f"{joined_items_str}{glue}{len(values) - shorten} more."
        if not quiet:
            msg += "\n\nUse -F or --full-lists to disable shortening of long lists."
        return msg
    else:
        joined_items_str = sep.join(map(str, values[:-1]))
        return f"{joined_items_str}{glue}{str(values[-1])}"


def bullet_list(
    items: list[Any],
    *,
    bullet: str = "-",
    indentation: str = "\t",
    full_lists: bool = False,
) -> str:
    return f"{indentation}{bullet} " + pretty_print_list(
        items,
        sep=f"\n\n{indentation}{bullet} ",
        glue=f"\n\n{indentation}{bullet} ",
        full_lists=full_lists,
    )


def markdown_table(items: list[dict]) -> str:
    """Format a list of dicts into a markdown table.

    >>> markdown_table(
    >>>    [{"name": "Sam", "age": 30}, {"name": "Ash", "age": 25}]
    >>> )
    ...
    | name | age  |
    | :--- | :--- |
    | Sam  | 30   |
    | Ash  | 25   |
    """
    res = []
    header = "| " + " | ".join(items[0].keys()) + " |"
    res.append(header)
    lb = "|" + " :--- |" * len(items[0])
    res.append(lb)
    for row in items:
        vals = list(row.values())
        r = "| " + " | ".join(map(str, vals)) + " |"
        res.append(r)
    return "\n".join(res)


def get_regular(fonts: Iterable[TTFont]) -> TTFont | None:
    for font in fonts:
        if "-Regular.ttf" in font.file:  # type: ignore
            return font


def filesize_formatting(s: int) -> str:
    if s < 1024:
        return f"{s} bytes"
    elif s < 1024 * 1024:
        return f"{s / 1024:.1f}kb"
    else:
        return f"{s / (1024 * 1024):.1f}Mb"


def get_bounding_box(font: TTFont) -> tuple[int, int]:
    """Returns max and min bbox of given truetype font"""
    ymin = 0
    ymax = 0
    if font.sfntVersion == "OTTO":
        ymin = font["head"].yMin
        ymax = font["head"].yMax
    else:
        for g in font["glyf"].glyphs:
            char = font["glyf"][g]
            if hasattr(char, "yMin") and ymin > char.yMin:
                ymin = char.yMin
            if hasattr(char, "yMax") and ymax < char.yMax:
                ymax = char.yMax
    return ymin, ymax


def get_name_entries(
    font: TTFont,
    nameID: int,
    platformID: int | None = None,
    encodingID: int | None = None,
    langID: int | None = None,
) -> list[NameRecord]:
    results = []
    for entry in font["name"].names:
        if (
            entry.nameID == nameID
            and (platformID is None or entry.platformID == platformID)
            and (encodingID is None or entry.platEncID == encodingID)
            and (langID is None or entry.langID == langID)
        ):
            results.append(entry)
    return results


def get_name_entry_strings(
    font: TTFont,
    nameID: int,
    platformID: int | None = None,
    encodingID: int | None = None,
    langID: int | None = None,
) -> list[str]:
    entries = get_name_entries(font, nameID, platformID, encodingID, langID)
    return list(map(lambda e: e.string.decode(e.getEncoding()), entries))


def get_glyph_name(font: TTFont, codepoint: int) -> str | None:
    next_best_cmap = font.getBestCmap()
    assert next_best_cmap is not None

    if codepoint in next_best_cmap:
        return next_best_cmap[codepoint]

    return None


def glyph_contour_count(font: TTFont, name: str) -> int:
    """Contour count for specified glyph.
    This implementation will also return contour count for
    composite glyphs.
    """
    contour_count = 0
    items = [font["glyf"][name]]

    while items:
        g = items.pop(0)
        if g.isComposite():
            for comp in g.components:
                if comp.glyphName != ".ttfautohint":
                    items.append(font["glyf"][comp.glyphName])
        if g.numberOfContours != -1:
            contour_count += g.numberOfContours
    return contour_count


def check_bit_entry(
    ttFont: TTFont, table: str, attr: str, expected: int, bitmask: int, bitname: str
) -> StatusYield:
    value = getattr(ttFont[table], attr)
    name_str = f"{table} {attr} {bitname} bit"
    if bool(value & bitmask) == expected:
        return PASS, f"{name_str} is properly set."
    else:
        if expected:
            expected_str = "set"
        else:
            expected_str = "unset"
        return FAIL, Message(f"bad-{bitname}", f"{name_str} should be {expected_str}.")


def cff_glyph_has_ink(font: TTFont, glyph_name: str) -> bool:
    if "CFF2" in font:
        top_dict = font["CFF2"].cff.topDictIndex[0]
    else:
        top_dict = font["CFF "].cff.topDictIndex[0]
    char_strings = top_dict.CharStrings
    char_string = char_strings[glyph_name]
    bounds = char_string.calcBounds(char_strings)
    return bounds is not None


def ttf_glyph_has_ink(font: TTFont, name: str) -> bool:
    glyph = font["glyf"].glyphs[name]
    glyph.expand(font["glyf"])

    if not glyph.isComposite():
        if glyph.numberOfContours == 0:
            return False
        (coords, _, _) = glyph.getCoordinates(font["glyf"])
        # you need at least 3 points to draw
        return len(coords) > 2

    # Check for ink in each sub-component.
    for glyph_name in glyph.getComponentNames(glyph.components):
        if glyph_has_ink(font, glyph_name):
            return True

    return False


def unicoderange_bit_name(bit: int) -> str:
    return UNICODERANGE_DATA[bit][0][1]


def get_preferred_cmap(ttFont: TTFont) -> dict[int, str] | None:
    cmaps = {}
    for table in ttFont["cmap"].tables:
        cmaps[table.format] = table.cmap

    if 12 in cmaps:
        return cmaps[12]

    elif 4 in cmaps:
        return cmaps[4]

    else:
        return None


def chars_in_range(ttFont: TTFont, bit: int) -> list[int]:
    cmap = get_preferred_cmap(ttFont)
    assert cmap is not None
    chars = []
    for c in sorted(cmap):
        for entry in UNICODERANGE_DATA[bit]:
            if c >= entry[2] and c <= entry[3]:
                chars.append(c)
    return chars


def compute_unicoderange_bits(ttFont: TTFont) -> int:
    cmap = get_preferred_cmap(ttFont)
    assert cmap is not None
    result = 0
    for c in sorted(cmap):
        for bit in range(len(UNICODERANGE_DATA)):
            for entry in UNICODERANGE_DATA[bit]:
                bit = entry[0]
                if c >= entry[2] and c <= entry[3]:
                    result |= 1 << bit
    return result


def glyph_has_ink(font: TTFont, glyph_name: str) -> bool:
    """Checks if specified glyph has any ink.

    That is, that it has at least one defined contour associated.
    Composites are considered to have ink if any of their components have ink.
    Args:
        font:       the font
        glyph_name: The name of the glyph to check for ink.
    Returns:
        True if the font has at least one contour associated with it.
    """
    if "glyf" in font:
        return ttf_glyph_has_ink(font, glyph_name)
    elif ("CFF " in font) or ("CFF2" in font):
        return cff_glyph_has_ink(font, glyph_name)
    else:
        raise Exception("Could not find 'glyf', 'CFF ', or 'CFF2' table.")


def filenames_ending_in(suffix: str, root: str) -> list[str]:
    """
    Returns a list of the filenames of all files in a given directory subtree
    that have the given filename suffix. Example: List all ".json" files.
    """
    filenames = []
    for f in os.listdir(root):
        fullpath = os.path.join(root, f)
        if f.endswith(suffix):
            filenames.append(fullpath)
        if os.path.isdir(fullpath):
            filenames.extend(filenames_ending_in(suffix, fullpath))
    return filenames


def axis(ttFont: TTFont, tag: str) -> Axis | None:
    """Return the axis with the given tag."""
    for axis in ttFont["fvar"].axes:
        if axis.axisTag == tag:
            return axis


def show_inconsistencies(
    dictionary: dict[str, list[str]], *, full_lists: bool = False
) -> str:
    """Display an 'inconsistencies dictionary' as a bullet list. Turns:

        { "value1": ["file1", "file2"], "value2": ["file3"] }

    into

        - value1: file1 and file2
        - value2: file3
    """

    return bullet_list(
        [
            f"{value}: {pretty_print_list(files, full_lists=full_lists)}"
            for value, files in dictionary.items()
        ],
        full_lists=full_lists,
    )


def get_family_name(ttFont: TTFont) -> str | None:
    """
    Get the family name from the name table.

    TODO: For now, this is just name ID 1. It should be expanded to at least
    check IDs 16 & 21, and ideally do the whole font differentiator heuristic.
    """
    family_name = ttFont["name"].getName(1, 3, 1, 0x0409)
    if family_name is None:
        return None
    return family_name.toUnicode()


def get_subfamily_name(ttFont: TTFont) -> str | None:
    """
    Get the subfamily name from the name table.

    TODO: For now, this is just name ID 2. It should be expanded to at least
    check IDs 17 & 22, and ideally do the whole font differentiator heuristic.
    """
    subfamily_name = ttFont["name"].getName(2, 3, 1, 0x0409)
    if subfamily_name is None:
        return None
    return subfamily_name.toUnicode()


def feature_tags(ttFont: TTFont) -> set[str]:
    in_this_font = set()
    for table in ["GSUB", "GPOS"]:
        if ttFont.get(table) and ttFont[table].table.FeatureList:  # type: ignore
            for fr in ttFont[table].table.FeatureList.FeatureRecord:  # type: ignore
                in_this_font.add(fr.FeatureTag)
    return in_this_font


def language_tags(ttFont: TTFont) -> set[str]:
    in_this_font = set()
    for table in ["GSUB", "GPOS"]:
        if ttFont.get(table) and ttFont[table].table.ScriptList:  # type: ignore
            for fr in ttFont[table].table.ScriptList.ScriptRecord:  # type: ignore
                for lsr in fr.Script.LangSysRecord:
                    in_this_font.add(lsr.LangSysTag)
    return in_this_font


def script_tags(ttFont: TTFont) -> set[str]:
    in_this_font = set()
    for table in ["GSUB", "GPOS"]:
        if ttFont.get(table) and ttFont[table].table.ScriptList:  # type: ignore
            for fr in ttFont[table].table.ScriptList.ScriptRecord:  # type: ignore
                in_this_font.add(fr.ScriptTag)
    return in_this_font


def get_mark_class_glyphnames(ttFont: TTFont) -> set[str]:
    class_defs = ttFont["GDEF"].table.GlyphClassDef.classDefs.items()
    return {name for (name, value) in class_defs if value == 3}


def is_non_spacing_mark_char(charcode: int) -> bool | None:
    from fontTools import unicodedata

    category = unicodedata.category(chr(charcode))
    if category.startswith("C"):
        # skip control characters
        return None
    else:
        # Non spacing marks either have the Unicode General_category:
        # Mn, Nonspacing_Mark
        # Me, Enclosing_Mark
        # Characters with the category Mc, Spacing_Mark should not be considered
        # as non spacing marks.
        return category in ("Mn", "Me")


def get_advance_width_for_char(ttFont: TTFont, ch: str) -> int | None:
    cp = ord(ch)
    cmap = ttFont.getBestCmap()
    if cmap is None or cp not in cmap:
        return None
    return ttFont["hmtx"][cmap[cp]][0]


def unicoderange(ttFont: TTFont) -> int:
    """Get an integer bitmap representing the UnicodeRange fields in the os/2 table."""
    os2 = ttFont["OS/2"]
    return (
        os2.ulUnicodeRange1
        | os2.ulUnicodeRange2 << 32
        | os2.ulUnicodeRange3 << 64
        | os2.ulUnicodeRange4 << 96
    )


def is_icon_font(ttFont: TTFont) -> bool:
    return (
        "OS/2" in ttFont and ttFont["OS/2"].panose.bFamilyType == 5  # type: ignore
    )


def git_rootdir(family_dir: str) -> str | None:
    assert family_dir

    root_dir = None

    try:
        git_cmd = ["git", "-C", family_dir, "rev-parse", "--show-toplevel"]
        git_output = subprocess.check_output(git_cmd, stderr=subprocess.STDOUT)
        root_dir = git_output.decode("utf-8").strip()

    except (OSError, IOError, subprocess.CalledProcessError):
        pass  # Not a git repo, or git is not installed.

    return root_dir


def typo_metrics_enabled(ttFont: TTFont) -> bool:
    return ttFont["OS/2"].fsSelection & 0b10000000 > 0  # type: ignore


def close_but_not_on(
    value_expected: int | float, value_true: int | float, tolerance: int | float
) -> bool:
    if value_expected == value_true:
        return False
    if abs(value_expected - value_true) <= tolerance:
        return True
    return False


def mark_glyphs(ttFont: TTFont) -> list[str]:
    marks = []
    if "GDEF" in ttFont and ttFont["GDEF"].table.GlyphClassDef:
        class_def = ttFont["GDEF"].table.GlyphClassDef.classDefs
        glyphOrder = ttFont.getGlyphOrder()
        for name in glyphOrder:
            if name in class_def and class_def[name] == 3:
                marks.append(name)
    return marks
