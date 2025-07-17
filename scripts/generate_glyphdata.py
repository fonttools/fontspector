from collections import Counter

"""Generate FontBakery's desired_glyph_data.json file.

The desired_glyph_data.json file contains the 'recommended' countour count
for encoded glyphs. The contour counts are derived from fonts which were
chosen for their quality and unique design decisions for particular glyphs.

Why make this?
Visually QAing thousands of glyphs by hand is tiring. Most glyphs can only
be constructured in a handful of ways. This means a glyph's contour count
will only differ slightly amongst different fonts, e.g a 'g' could either
be 2 or 3 contours, depending on whether its double story or single story.
However, a quotedbl should have 2 contours, unless the font belongs to a
display family.

In the future, additional glyph data can be included. A good addition would
be the 'recommended' anchor counts for each glyph.
"""

import json
import os
import sys
import glob
from tqdm import tqdm

from fontTools.ttLib import TTFont
from fontbakery.utils import glyph_contour_count

MAX_OPTIONS = 3


def get_font_glyph_data(font: TTFont, dataset):
    """Return information for each glyph in a font"""
    cmap_reversed = font["cmap"].buildReversed()

    for glyph_name in font.getGlyphOrder():
        if "cid" in glyph_name:
            continue
        contours = glyph_contour_count(font, glyph_name)
        if glyph_name in cmap_reversed:
            uni_glyph = cmap_reversed[glyph_name]
            for unicode in uni_glyph:
                if unicode not in dataset["by_unicode"]:
                    dataset["by_unicode"][unicode] = Counter()
                dataset["by_unicode"][unicode][contours] += 1
        if glyph_name not in dataset["by_name"]:
            dataset["by_name"][glyph_name] = Counter()
        dataset["by_name"][glyph_name][contours] += 1


banned = ["noto", "bitcount", "nabla"]


def main():
    dataset = {
        "by_name": {},
        "by_unicode": {},
    }
    for font_ttf in tqdm(glob.glob(os.environ["GF_ROOT"] + "/ofl/*/*ttf")):
        if any(ban in font_ttf for ban in banned):
            continue
        font = TTFont(font_ttf)
        # Not CJK
        if len(font.getGlyphOrder()) > 5000:
            continue
        get_font_glyph_data(font, dataset)

    for topic in dataset.values():
        for key in list(topic.keys()):
            # Thin out the dataset; remove any entries that have less than 5 total appearances.
            if topic[key].total() < 10:
                del topic[key]
                continue
            # See if there is an "obvious" way to do it; i.e. capture the top 95% of uses.
            most_common = [
                (contours, count / topic[key].total())
                for (contours, count) in topic[key].most_common()
            ]
            contours_to_keep = []
            cum_prob = 0.0
            for contours, prob in most_common:
                cum_prob += prob
                contours_to_keep.append(contours)
                if cum_prob > 0.95:
                    break
            if len(contours_to_keep) <= MAX_OPTIONS:
                topic[key] = contours_to_keep
            else:  # No clear winner
                del topic[key]

    print("Collating font data into glyph data file")

    script_path = os.path.dirname(__file__)
    glyph_data_path = os.path.join(script_path, "..", "desired_glyph_data.json")

    # Sort them.
    dataset["by_name"] = {
        glyph_name: list(contours)
        for glyph_name, contours in sorted(
            dataset["by_name"].items(), key=lambda x: x[0]
        )
    }
    dataset["by_unicode"] = {
        unicode: list(contours)
        for unicode, contours in sorted(
            dataset["by_unicode"].items(), key=lambda x: x[0]
        )
    }
    print(f"Saving to {glyph_data_path}")
    with open(glyph_data_path, "w") as glyph_file:
        json.dump(dataset, glyph_file, indent=4)
    print("done")


if __name__ == "__main__":
    sys.exit(main())
