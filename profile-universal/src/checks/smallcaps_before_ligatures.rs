use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

#[check(
    id = "smallcaps_before_ligatures",
    rationale = "
        OpenType small caps should be defined before ligature lookups to ensure
        proper functionality.

        Rainer Erich Scheichelbauer (a.k.a. MekkaBlue) pointed out in a tweet
        (https://twitter.com/mekkablue/status/1297486769668132865) that the ordering
        of small caps and ligature lookups can lead to bad results such as the example
        he provided of the word \"WAFFLES\" in small caps, but with an unfortunate
        lowercase ffl ligature substitution.
	
        This check attempts to detect this kind of mistake.",
    proposal = "https://github.com/fonttools/fontbakery/issues/3020",
    title = "Ensure 'smcp' (small caps) lookups are defined before ligature lookups in the 'GSUB' table."
)]
fn smallcaps_before_ligatures(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    // Skip if no smcp
    let smcp_lookups = f
        .feature_records(true)
        .filter(|(r, _l)| r.feature_tag() == "smcp")
        .flat_map(|(_r, l)| l)
        .flat_map(|l| l.lookup_list_indices())
        .collect::<Vec<_>>();
    let liga_lookups = f
        .feature_records(true)
        .filter(|(r, _l)| r.feature_tag() == "liga")
        .flat_map(|(_r, l)| l)
        .flat_map(|l| l.lookup_list_indices())
        .collect::<Vec<_>>();
    skip!(smcp_lookups.is_empty(), "no-smcp", "No smcp feature");
    skip!(liga_lookups.is_empty(), "no-liga", "No liga feature");
    #[allow(clippy::unwrap_used)] // We know that the vecs are not empty
    let first_smcp_lookup = smcp_lookups.iter().min().unwrap();
    #[allow(clippy::unwrap_used)] // We know that the vecs are not empty
    let first_liga_lookup = liga_lookups.iter().min().unwrap();
    if first_smcp_lookup < first_liga_lookup {
        return Ok(Status::just_one_pass());
    }
    return Ok(Status::just_one_fail(
        "feature-ordering",
        "'smcp' lookups are not defined before 'liga' lookups.",
    ));
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        FileTypeConvert, StatusCode, Testable, TTF,
    };
    use skrifa::raw::TableProvider;
    use write_fonts::{
        from_obj::ToOwnedTable,
        tables::{
            gsub::Gsub,
            layout::{Feature, FeatureList, FeatureRecord},
        },
        types::Tag,
    };

    /// Replace the GSUB FeatureList with a pair of `smcp` and `liga` records
    /// pointing at the given lookup list indices.
    ///
    /// Records are emitted in alphabetical tag order so the table stays valid;
    /// the check only inspects lookup indices, not record order.
    fn set_smcp_liga_lookups(t: &mut Testable, smcp_lookup: u16, liga_lookup: u16) {
        let f = TTF.from_testable(t).unwrap();
        let mut gsub: Gsub = f.font().gsub().unwrap().to_owned_table();
        let feature_list = FeatureList::new(vec![
            FeatureRecord::new(Tag::new(b"liga"), Feature::new(None, vec![liga_lookup])),
            FeatureRecord::new(Tag::new(b"smcp"), Feature::new(None, vec![smcp_lookup])),
        ]);
        gsub.feature_list = feature_list.into();
        let new_bytes = f.rebuild_with_new_table(&gsub).unwrap();
        t.set(new_bytes);
    }

    #[test]
    fn test_smallcaps_before_ligatures_skip_no_smcp() {
        // Mada Regular has no 'smcp' feature, so check should SKIP
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(super::smallcaps_before_ligatures, testable);
        assert_results_contain(&results, StatusCode::Skip, Some("no-smcp".to_string()));
    }

    #[test]
    fn test_smallcaps_before_ligatures_pass() {
        // smcp uses an earlier lookup than liga
        let mut testable = test_able("mada/Mada-Regular.ttf");
        set_smcp_liga_lookups(&mut testable, 0, 1);
        let results = run_check(super::smallcaps_before_ligatures, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_smallcaps_before_ligatures_fail_feature_ordering() {
        // liga uses an earlier lookup than smcp
        let mut testable = test_able("mada/Mada-Regular.ttf");
        set_smcp_liga_lookups(&mut testable, 1, 0);
        let results = run_check(super::smallcaps_before_ligatures, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("feature-ordering".to_string()),
        );
    }
}
