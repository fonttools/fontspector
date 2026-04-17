use fontations::skrifa::raw::types::NameId;
use fontspector_checkapi::{prelude::*, FileTypeConvert, StatusCode};

#[check(
    id = "typographic_family_name",
    rationale = "
        Check whether Name ID 16 (Typographic Family name) is consistent
        across the set of fonts.
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/4567",
    title = "Typographic Family name consistency.",
    implementation = "all"
)]
fn typographic_family_name(c: &TestableCollection, context: &Context) -> CheckFnResult {
    let ttfs = TTF.from_collection(c);
    let items: Vec<_> = ttfs
        .iter()
        .map(|f| {
            let name = f
                .get_name_entry_strings(NameId::TYPOGRAPHIC_FAMILY_NAME)
                .next()
                .unwrap_or("<missing>".to_string());
            #[allow(clippy::unwrap_used)]
            (
                name.clone(),
                name,
                f.filename.file_name().unwrap().to_string_lossy(),
            )
        })
        .collect();
    assert_all_the_same(
        context,
        &items,
        "inconsistency",
        "Name ID 16 (Typographic Family name) is not consistent across fonts.",
        StatusCode::Fail,
    )
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::typographic_family_name;
    use fontspector_checkapi::{
        codetesting::{assert_pass, run_check_with_config, test_able},
        TestableCollection, TestableType,
    };

    #[test]
    fn test_typographic_family_name_pass() {
        let testables = vec![
            test_able("cabin/Cabin-Regular.ttf"),
            test_able("cabin/Cabin-Bold.ttf"),
            test_able("cabin/Cabin-Italic.ttf"),
            test_able("cabin/Cabin-BoldItalic.ttf"),
        ];
        let collection = TestableCollection::from_testables(testables, None);
        let results = run_check_with_config(
            typographic_family_name,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_pass(&results);
    }
}
