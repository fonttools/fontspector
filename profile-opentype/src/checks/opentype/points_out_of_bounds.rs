use fontations::skrifa::{
    raw::{
        tables::glyf::{Glyph, PointFlags},
        types::Point,
        TableProvider,
    },
    GlyphId,
};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "opentype/points_out_of_bounds",
    rationale = "
        The glyf table specifies a bounding box for each glyph. This check
        ensures that all points in all glyph paths are within the bounding
        box. Glyphs with out-of-bounds points can cause rendering issues in
        some software, and should be corrected.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/735",
    title = "Check for points out of bounds"
)]
fn points_out_of_bounds(t: &Testable, _context: &Context) -> CheckFnResult {
    let ttf = testfont!(t);
    let font = ttf.font();
    skip!(!ttf.has_table(b"glyf"), "no-glyf", "No glyf table");
    let glyf = font.glyf()?;
    let loca = font.loca(None)?;
    let mut problems = vec![];
    for gid in 0..font.maxp()?.num_glyphs() {
        let gid = GlyphId::new(gid.into());
        if let Some(Glyph::Simple(glyph)) = loca.get_glyf(gid, &glyf)? {
            let point_count = glyph.num_points();
            let mut points: Vec<Point<i32>> = vec![Point::default(); point_count];
            let mut flags = vec![PointFlags::default(); point_count];
            glyph.read_points_fast(&mut points, &mut flags)?;
            let x_min: i32 = glyph.x_min().into();
            let x_max: i32 = glyph.x_max().into();
            let y_min: i32 = glyph.y_min().into();
            let y_max: i32 = glyph.y_max().into();
            for point in &points {
                if point.x < x_min || point.x > x_max {
                    let msg = format!(
                        "{} (x={}, bounds are {}<->{})",
                        ttf.glyph_name_for_id_synthesise(gid),
                        point.x,
                        x_min,
                        x_max
                    );
                    let mut status = Status::warn("points-out-of-bounds", &msg);
                    status.add_metadata(Metadata::GlyphProblem {
                        glyph_name: ttf.glyph_name_for_id_synthesise(gid),
                        glyph_id: gid.to_u32(),
                        userspace_location: None,
                        position: Some((point.x as f32, point.y as f32)),
                        actual: Some(json!({"x_value": point.x, "x_min": x_min, "x_max": x_max})),
                        expected: Some(json!({"x_between": [x_min, x_max]})),
                        message: "Point is out of bounds horizontally.".to_string(),
                    });
                    problems.push(status);
                }
                if point.y < y_min || point.y > y_max {
                    let msg = format!(
                        "{} (y={}, bounds are {}<->{})",
                        ttf.glyph_name_for_id_synthesise(gid),
                        point.y,
                        y_min,
                        y_max
                    );
                    let mut status = Status::warn("points-out-of-bounds", &msg);
                    status.add_metadata(Metadata::GlyphProblem {
                        glyph_name: ttf.glyph_name_for_id_synthesise(gid),
                        glyph_id: gid.to_u32(),
                        userspace_location: None,
                        position: Some((point.x as f32, point.y as f32)),
                        actual: Some(json!({"y_value": point.y, "y_min": y_min, "y_max": y_max})),
                        expected: Some(json!({"y_between": [y_min, y_max]})),
                        message: "Point is out of bounds vertically.".to_string(),
                    });
                    problems.push(status);
                }
            }
        }
    }
    return_result(problems)
}
