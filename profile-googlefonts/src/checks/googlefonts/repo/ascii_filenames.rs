use fontspector_checkapi::prelude::*;

#[check(
    id = "googlefonts/repo/ascii_filenames",
    rationale = "
        The google/fonts repository should only contain files with ASCII
        filenames. Non-ASCII characters in filenames can cause issues
        across different operating systems and tools.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/167",
    title = "Ensure all filenames are ASCII-only.",
    implementation = "all"
)]
fn ascii_filenames(c: &TestableCollection, _context: &Context) -> CheckFnResult {
    let mut problems = vec![];
    for testable in c.iter() {
        if let Some(name) = testable.basename() {
            if !name.is_ascii() {
                problems.push(Status::fail(
                    "non-ascii-filename",
                    &format!("Non-ASCII filename: '{name}'"),
                ));
            }
        }
    }
    return_result(problems)
}
