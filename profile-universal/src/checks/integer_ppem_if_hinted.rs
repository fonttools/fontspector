use fontations::{skrifa::raw::TableProvider, write::from_obj::ToOwnedTable};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

#[check(
    id = "integer_ppem_if_hinted",
    rationale = "
        Hinted fonts must have head table flag bit 3 set.

        Per https://docs.microsoft.com/en-us/typography/opentype/spec/head,
        bit 3 of Head::flags decides whether PPEM should be rounded. This bit should
        always be set for hinted fonts.

        Note:
        Bit 3 = Force ppem to integer values for all internal scaler math;
                May use fractional ppem sizes if this bit is clear;
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2338",
    title = "PPEM must be an integer on hinted fonts.",
    hotfix = fix_integer_ppem_if_hinted
)]
fn integer_ppem_if_hinted(f: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(f);
    skip!(
        !font.has_table(b"fpgm"),
        "no-hints",
        "Font does not have fpgm table."
    );
    Ok(if font.font().head()?.flags() & 0b1000 == 0 {
        Status::just_one_fail("bad-flags",
        "This is a hinted font, so it must have bit 3 set on the flags of the head table, so that PPEM values will be rounded into an integer value."
    )
    } else {
        Status::just_one_pass()
    })
}

fn fix_integer_ppem_if_hinted(t: &mut Testable) -> FixFnResult {
    let f = testfont!(t);
    let mut head: fontations::write::tables::head::Head = f.font().head()?.to_owned_table();
    head.flags |= 0b1000;
    t.set(f.rebuild_with_new_table(&head)?);
    Ok(true)
}
