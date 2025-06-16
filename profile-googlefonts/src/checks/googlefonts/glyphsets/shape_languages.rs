use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use google_fonts_glyphsets::{get_glyphset_coverage, languages_per_glyphset};
use hashbrown::HashMap;
use shaperglot::{Checker, Languages, ResultCode};
use tabled::builder::Builder;

fn table_of_results(
    context: &Context,
    _title: &str,
    results: &HashMap<String, Vec<String>>,
) -> String {
    let mut table = Builder::from_iter(
        std::iter::once(vec!["Message".to_string(), "Languages".to_string()]).chain(
            results.iter().map(|(message, languages)| {
                vec![message.to_string(), bullet_list(context, languages)]
            }),
        ),
    )
    .build();
    table.with(tabled::settings::Style::markdown()).to_string()
}
#[check(
    id = "googlefonts/glyphsets/shape_languages",
    rationale = "
        This check uses a heuristic to determine which GF glyphsets a font supports.
        Then it checks the font for correct shaping behaviour for all languages in
        those glyphsets.
    ",
    proposal = "https://github.com/googlefonts/fontbakery/issues/4147",
    title = "Shapes languages in all GF glyphsets."
)]
fn shape_languages(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let checker =
        Checker::new(&t.contents).map_err(|e| FontspectorError::General(e.to_string()))?;
    let languages = Languages::new();
    let codepoints = f.codepoints(Some(context));
    let mut warns = HashMap::new();
    let mut fails = HashMap::new();
    let mut any_glyphset_supported = false;
    for (glyphset, coverage) in get_glyphset_coverage(&codepoints).iter() {
        if coverage.fraction > 0.8 {
            any_glyphset_supported = true;
            for language_code in languages_per_glyphset(glyphset)
                .map_err(|e| FontspectorError::General(e.to_string()))?
                .iter()
            {
                if let Some(language) = languages.get_language(language_code) {
                    let reporter = checker.check(language);
                    let name = language.name();
                    let language_string = format!("{} ({})", language_code, name);
                    for result in reporter.iter() {
                        let message = result.to_string();
                        if result.status == ResultCode::Warn {
                            warns
                                .entry(message)
                                .or_insert(vec![])
                                .push(language_string.clone());
                        } else if result.status == ResultCode::Fail {
                            fails
                                .entry(message)
                                .or_insert(vec![])
                                .push(language_string.clone());
                        }
                    }
                }
            }
        }
    }

    let mut problems = vec![];
    if !fails.is_empty() {
        problems.push(Status::fail(
            "failed-language-shaping",
            &format!(
                "Failed language shaping:\n\n{}",
                table_of_results(context, "FAIL", &fails)
            ),
        ));
    }
    if !warns.is_empty() {
        problems.push(Status::warn(
            "warning-language-shaping",
            &format!(
                "Warning language shaping:\n\n{}",
                table_of_results(context, "WARN", &warns)
            ),
        ));
    }
    if !any_glyphset_supported {
        problems.push(Status::fail("no-glyphset-supported",
            "No GF glyphset was found to be supported >80%, so language shaping support couldn't get checked.",
        ));
    }

    return_result(problems)
}
