mod intercharacter_spacing;
use std::str::FromStr;

use fontspector_checkapi::{pens::BezGlyph, TestFont, DEFAULT_LOCATION};
pub use intercharacter_spacing::intercharacter_spacing;
mod interline_spacing;
pub use interline_spacing::interline_spacing;
mod interword_spacing;
pub use interword_spacing::interword_spacing;
mod proportions;
use kurbo::{ParamCurve, Shape};
pub use proportions::proportions;
mod stem_width;
use fontations::skrifa::{
    prelude::{LocationRef, Size},
    MetadataProvider,
};
use rustybuzz::{Face, UnicodeBuffer};
pub use stem_width::stem_width;

fn find_stem_width(f: &TestFont) -> Option<f64> {
    let l_glyph = f.font().charmap().map('l')?;
    let intersections = x_height_intersections(f, l_glyph)?;
    if intersections.len() != 2 {
        return None;
    }
    #[allow(clippy::indexing_slicing)]
    Some(intersections[1] - intersections[0])
}

fn x_height_intersections(f: &TestFont, glyph_id: fontations::skrifa::GlyphId) -> Option<Vec<f64>> {
    let mut bezpen = BezGlyph(vec![]);
    f.draw_glyph(glyph_id, &mut bezpen, DEFAULT_LOCATION).ok()?;
    let all_bounds = bezpen
        .0
        .iter()
        .map(|p| p.bounding_box())
        .reduce(|a, b| a.union(b))?;
    let x_height = f
        .font()
        .metrics(Size::unscaled(), LocationRef::new(&[]))
        .x_height? as f64;
    let ray = kurbo::Line::new(
        (all_bounds.min_x(), x_height),
        (all_bounds.max_x(), x_height),
    );
    let mut intersections = vec![];
    for path in bezpen.0.iter() {
        for seg in path.segments() {
            let segintersections = seg.intersect_line(ray);
            let points = segintersections.iter().map(|i| ray.eval(i.line_t).x);
            intersections.extend(points);
        }
    }
    intersections.sort_by(|a, b| a.total_cmp(b));
    Some(intersections)
}

fn pair_kerning(contents: &[u8], left: char, right: char) -> Option<i32> {
    let face = Face::from_slice(contents, 0)?;
    // let plan = rustybuzz::ShapePlan::new(
    //     &face,
    //     rustybuzz::Direction::LeftToRight,
    //     Some(rustybuzz::script::LATIN),
    //     None,
    //     &[],
    // );
    // let kern = f.font().kern()?.pair(left_id, right_id).unwrap_or(0);
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(&format!("{left}{right}"));
    #[allow(clippy::unwrap_used)] // Static
    let buffer_with = rustybuzz::shape(
        &face,
        &[rustybuzz::Feature::from_str("+kern").unwrap()],
        buffer,
    );
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(&format!("{left}{right}"));
    #[allow(clippy::unwrap_used)] // Static
    let buffer_without = rustybuzz::shape(
        &face,
        &[rustybuzz::Feature::from_str("-kern").unwrap()],
        buffer,
    );
    let advance_with = buffer_with.glyph_positions().first()?.x_advance;
    let advance_without = buffer_without.glyph_positions().first()?.x_advance;
    Some(advance_with - advance_without)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_kerning() {
        let contents = include_bytes!(
            "../../../../fontspector-py/data/test/montserrat/Montserrat-Regular.ttf"
        );
        let Some(kern) = pair_kerning(contents, 'A', 'V') else {
            panic!("Failed to get kerning value");
        };
        assert_eq!(kern, 679 - 726);
    }
}
