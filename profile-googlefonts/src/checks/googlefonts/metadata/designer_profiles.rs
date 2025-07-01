use crate::{
    checks::googlefonts::metadata::{family_proto, DesignerInfoProto},
    network_conditions::{is_designer_listed, DESIGNER_INFO_RAW_URL},
};
use fontspector_checkapi::{prelude::*, skip};

const NAME_DEUNICODIZATION: [(&str, &str); 19] = [
    ("á", "a"),
    ("é", "e"),
    ("í", "i"),
    ("ó", "o"),
    ("ú", "u"),
    ("à", "a"),
    ("è", "e"),
    ("ì", "i"),
    ("ò", "o"),
    ("ù", "u"),
    ("ń", "n"),
    ("ø", "o"),
    ("ř", "r"),
    ("ś", "s"),
    ("ß", "ss"),
    ("ł", "l"),
    ("ã", "a"),
    ("ı", "i"),
    ("ü", "ue"),
];

fn normalize_designer_name(designer: &str) -> String {
    designer
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else {
                NAME_DEUNICODIZATION
                    .iter()
                    .find(|(from, _)| *from == c.to_string())
                    .map(|(_, to)| to.chars())
                    .unwrap_or("".chars())
                    .next()
            }
        })
        .collect()
}

#[check(
    id = "googlefonts/metadata/designer_profiles",
    rationale = "
        
        Google Fonts has a catalog of designers.

        This check ensures that the online entries of the catalog can be found based
        on the designer names listed on the METADATA.pb file.

        It also validates the URLs and file formats are all correctly set.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/3083",
    title = "METADATA.pb: Designers are listed correctly on the Google Fonts catalog?",
    applies_to = "MDPB"
)]
fn designer_profiles(c: &Testable, context: &Context) -> CheckFnResult {
    let msg = family_proto(c).map_err(|e| {
        FontspectorError::General(format!("METADATA.pb is not a valid FamilyProto: {e:?}"))
    })?;
    skip!(
        context.skip_network,
        "network-disabled",
        "Skipping network check"
    );
    let mut problems = vec![];
    for designer in msg.designer().split(",") {
        let designer = normalize_designer_name(designer.trim());
        if designer == "multipledesigners" {
            problems.push(Status::fail(
                    "multiple-designers",
                    &format!(
                        "Font family {} does not explicitely mention the names of its designers on its METADATA.pb file.",
                        msg.name()
                    ),
                ),
            );
            continue;
        }
        let listed = is_designer_listed(context, &designer)?;
        if let Some(profile) = listed {
            let designer_profile = protobuf::text_format::parse_from_str::<DesignerInfoProto>(
                &profile,
            )
            .map_err(|e| FontspectorError::General(format!("Error parsing info.pb: {e}")))?;
            if normalize_designer_name(designer_profile.designer()) != designer {
                problems.push(Status::warn(
                        "mismatch",
                        &format!(
                            "Designer name at METADATA.pb ({}) is not the same as listed on the designers catalog ({}) available at {}/{}/info.pb",
                            designer,
                            normalize_designer_name(designer_profile.designer()),
                            DESIGNER_INFO_RAW_URL, designer
                        ),
                    ),
                );
            }
            if !designer_profile.link().is_empty() {
                problems.push(Status::warn(
                        "link-field",
                        "Currently the link field is not used by the GFonts API. Designer webpage links should, for now, be placed directly on the bio.html file.",
                    ),
                );
            }
            if designer_profile.avatar.file_name().is_empty() && designer != "Google" {
                problems.push(Status::warn(
                    "missing-avatar",
                    &format!(
                        "Designer {designer} still does not have an avatar image. Please provide one."
                    ),
                ));
            } else {
                let avatar_url = format!(
                    "{}{}/{}",
                    DESIGNER_INFO_RAW_URL,
                    designer,
                    designer_profile.avatar.file_name()
                );
                let response = reqwest::blocking::get(&avatar_url).map_err(|e| {
                    FontspectorError::General(format!(
                        "Error fetching avatar image from {avatar_url}: {e}"
                    ))
                })?;
                if !response.status().is_success() {
                    problems.push(Status::warn(
                        "bad-avatar-filename",
                        &format!(
                            "The avatar filename provided seems to be incorrect: ({avatar_url})"
                        ),
                    ));
                }
            }
        } else {
            problems.push(Status::warn(
                    "profile-not-found",
                    &format!(
                        "It seems that {designer} is still not listed on the designers catalog. Please submit a photo and a link to a webpage where people can learn more about the work of this designer/typefoundry."
                    ),
                ),
            );
            continue;
        }
    }

    return_result(problems)
}
