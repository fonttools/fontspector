use fontations::skrifa::raw::TableProvider;
use fontations::skrifa::string::StringId;
use fontspector_checkapi::{prelude::*, FileTypeConvert};

#[check(
    id = "googlefonts/name/license",
    rationale = "
        
        A known licensing description must be provided in the NameID 14
        (LICENSE DESCRIPTION) entries of the name table.

        The source of truth for this check (to determine which license is in use) is
        a file placed side-by-side to your font project including the licensing terms.

        Depending on the chosen license, one of the following string snippets is
        expected to be found on the NameID 13 (LICENSE DESCRIPTION) entries of the
        name table:

        - \"This Font Software is licensed under the SIL Open Font License, Version 1.1.
          This license is available with a FAQ at: openfontlicense.org\"

        - \"Licensed under the Apache License, Version 2.0\"

        - \"Licensed under the Ubuntu Font Licence 1.0.\"


        Currently accepted licenses are Apache or Open Font License. For a small set
        of legacy families the Ubuntu Font License may be acceptable as well.

        When in doubt, please choose OFL for new font projects.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Check copyright namerecords match license file.",
    implementation = "all"
)]
fn license(c: &TestableCollection, _context: &Context) -> CheckFnResult {
    let mut problems = vec![];
    let mut http_warn = false;
    let mut entry_found: bool = false;

    let license_filename = c
        .get_file("OFL.txt")
        .or_else(|| c.get_file("UFL.txt"))
        .or_else(|| c.get_file("LICENSE.txt"))
        .ok_or_else(|| CheckError::skip("license-file-missing", "A license file was not found."))?
        .basename()
        .ok_or(CheckError::Error("This should never happen!".to_string()))?;

    let placeholder = match license_filename.as_str() {
        "UFL.txt" => "Licensed under the Ubuntu Font Licence 1.0.",
        "OFL.txt" => "This Font Software is licensed under the SIL Open Font License, Version 1.1. This license is available with a FAQ at: https://openfontlicense.org",
        "LICENSE.txt" => "Licensed under the Apache License, Version 2.0",
        _ => "", // Impossible to happen.
    };

    let fonts = TTF.from_collection(c);
    for f in fonts {
        let name = f.font().name()?;
        for record in name.name_record().iter() {
            let string = record.string(name.string_data())?;
            if record.name_id() == StringId::LICENSE_DESCRIPTION {
                let mut license_description = string.to_string();
                entry_found = true;
                if license_description.contains("http://") {
                    problems.push(Status::warn(
                        "http-in-description",
                        &format!(
                            "Please consider using HTTPS URLs at name table entry [plat={}, enc={}, name={}]",
                            record.platform_id(),
                            record.encoding_id(),
                            record.name_id()
                        ),
                    ));
                    license_description = license_description.replace("http://", "https://");
                    http_warn = true;
                }

                if license_description.contains("scripts.sil.org/OFL") {
                    problems.push(Status::warn(
                        "old-url",
                        "Please consider updating the url from 'https://scripts.sil.org/OFL' to 'https://openfontlicense.org'.",
                    ));
                    continue;
                }
                if license_description != placeholder {
                    problems.push(Status::fail(
                        "wrong",
                        &format!(
                            "License file {} exists but NameID 13 (LICENSE DESCRIPTION) value on platform {} is not specified for that.\nValue was: \"{}\"\nMust be changed to \"{}\"",
                            license_filename,
                            record.platform_id(),
                            license_description,
                            placeholder
                        ),
                    ));
                }
            }
        }

        if http_warn {
            problems.push(Status::warn(
                "http",
                "For now we're still accepting http URLs, but you should consider using https instead.",
            ));
        }

        if !entry_found {
            problems.push(Status::fail(
                "missing",
                "Font lacks NameID 13 (LICENSE DESCRIPTION). A proper licensing entry must be set.",
            ));
        }
    }
    return_result(problems)
}
