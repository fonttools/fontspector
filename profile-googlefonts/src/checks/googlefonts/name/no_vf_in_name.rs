use fontations::{
    skrifa::raw::{types::NameId, TableProvider},
    write::tables::name::{Name, NameRecord},
};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

const ILLEGAL_PARTICLES: &[&str] = &["VF"];

/// Check if a word looks like a version number (e.g. "v1", "V2.0", "v1.2.3")
fn is_version_number(word: &str) -> bool {
    let stripped = word.strip_prefix(['v', 'V']);
    match stripped {
        Some(rest) if !rest.is_empty() => {
            rest.chars().all(|c| c.is_ascii_digit() || c == '.')
                && rest.chars().next().is_some_and(|c| c.is_ascii_digit())
        }
        _ => false,
    }
}

/// Find illegal particles in a name string. Returns the matching particles.
fn find_illegal_particles(name: &str) -> Vec<String> {
    let mut found = vec![];
    for word in name.split_whitespace() {
        if ILLEGAL_PARTICLES.contains(&word) {
            found.push(word.to_string());
        }
        if is_version_number(word) {
            found.push(word.to_string());
        }
    }
    found
}

/// Remove illegal particles from a name string.
fn remove_illegal_particles(name: &str) -> String {
    name.split_whitespace()
        .filter(|word| !ILLEGAL_PARTICLES.contains(word) && !is_version_number(word))
        .collect::<Vec<_>>()
        .join(" ")
}

#[check(
    id = "googlefonts/name/illegal_particles",
    title = "Ensure family name does not contain illegal particles like 'VF' or version numbers.",
    rationale = "
        Google Fonts does not allow certain particles in family names.

        The particle 'VF' should not be used because many environments
        and applications do not support variable fonts, so including 'VF'
        in the name is confusing to users and implies the font only works
        as a variable font.

        Version numbers (e.g. 'v1.0', 'V2') should not be included in
        the family name because they cause confusion when fonts are updated.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/648",
    hotfix = fix_illegal_particles,
)]
fn illegal_particles(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    let name_ids = [NameId::FAMILY_NAME, NameId::TYPOGRAPHIC_FAMILY_NAME];

    for name_id in name_ids {
        for name in f.get_name_entry_strings(name_id) {
            let particles = find_illegal_particles(&name);
            if !particles.is_empty() {
                let cleaned = remove_illegal_particles(&name);
                let particle_list = particles.join(", ");
                let message = format!(
                    "Family name '{name}' (nameID {name_id}) contains \
                     illegal particle(s): {particle_list}. \
                     Should be '{cleaned}'."
                );
                let mut status = Status::fail("illegal-particle", &message);
                status.add_metadata(Metadata::TableProblem {
                    table_tag: "name".to_string(),
                    field_name: Some(format!("nameID {name_id}")),
                    actual: Some(json!(name)),
                    expected: Some(json!(cleaned)),
                    message,
                });
                problems.push(status);
            }
        }
    }
    return_result(problems)
}

fn fix_illegal_particles(t: &mut Testable, _replies: Option<MoreInfoReplies>) -> Result<FixResult, FontspectorError> {
    let f = testfont!(t);
    let name_table = f.font().name()?;
    let name_ids_to_fix = [NameId::FAMILY_NAME, NameId::TYPOGRAPHIC_FAMILY_NAME];

    let new_records: Vec<NameRecord> = name_table
        .name_record()
        .iter()
        .map(|r| {
            let original = r
                .string(name_table.string_data())
                .map(|s| s.chars().collect::<String>())
                .unwrap_or_default();
            let new_string = if name_ids_to_fix.contains(&r.name_id()) {
                remove_illegal_particles(&original)
            } else {
                original
            };
            NameRecord::new(
                r.platform_id(),
                r.encoding_id(),
                r.language_id(),
                r.name_id(),
                new_string.into(),
            )
        })
        .collect();

    let new_name = Name::new(new_records);
    t.set(f.rebuild_with_new_table(&new_name)?);
    Ok(FixResult::Fixed)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, set_name_entry, test_able},
        StatusCode,
    };

    use super::*;

    #[test]
    fn test_pass_normal_name() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(illegal_particles, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_fail_vf_in_name() {
        let mut testable = test_able("mada/Mada-Regular.ttf");
        set_name_entry(
            &mut testable,
            3,
            1,
            0x0409,
            NameId::FAMILY_NAME,
            "Mada VF".to_string(),
        );
        let results = run_check(illegal_particles, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("illegal-particle".to_string()),
        );
    }

    #[test]
    fn test_fail_version_in_name() {
        let mut testable = test_able("mada/Mada-Regular.ttf");
        set_name_entry(
            &mut testable,
            3,
            1,
            0x0409,
            NameId::FAMILY_NAME,
            "Mada v2.0".to_string(),
        );
        let results = run_check(illegal_particles, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("illegal-particle".to_string()),
        );
    }

    #[test]
    fn test_no_false_positives() {
        // "VF" embedded in a word should NOT match
        assert!(find_illegal_particles("Avfont").is_empty());
        assert!(find_illegal_particles("MyFontVFoo").is_empty());
        // "vector" shouldn't match version pattern
        assert!(find_illegal_particles("vector").is_empty());
    }

    #[test]
    fn test_version_detection() {
        assert!(is_version_number("v1"));
        assert!(is_version_number("V2.0"));
        assert!(is_version_number("v1.2.3"));
        assert!(!is_version_number("vector"));
        assert!(!is_version_number("v"));
        assert!(!is_version_number("V"));
        assert!(!is_version_number("valve"));
    }

    #[test]
    fn test_remove_particles() {
        assert_eq!(remove_illegal_particles("Mada VF"), "Mada");
        assert_eq!(remove_illegal_particles("Mada v2.0"), "Mada");
        assert_eq!(remove_illegal_particles("VF Font v1"), "Font");
    }
}
