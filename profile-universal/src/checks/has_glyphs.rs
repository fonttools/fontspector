use fontations::skrifa::GlyphNames;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use hashbrown::HashSet;

#[check(
    id = "has_glyphs",
    rationale = "Some foundries want to know that a font contains certain glyphs.

    This check expects to find a table of glyph names in the configuration file, and checks to ensure that the font includes these glyphs.

    Example:

    ```
    has_glyphs = [ \"a\", \"b\", \"c\" ]
    ```

    Alternatively, the configuration can be specialized on a per-font basis:

    ```
    [has_glyphs]
    \"Foo-Regular.ttf\" = [ \"a\", \"b\", \"c\" ]
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/408",
    title = "Ensure that the font contains the glyphs specified in the configuration file."
)]
fn has_glyphs(t: &Testable, context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    let config = context.local_config("has_glyphs");
    skip!(
        config.is_null(),
        "unconfigured",
        "No configuration found for has_glyphs"
    );
    let font_config = if config.is_object() {
        println!("Config is an object: {:?}", config);
        let font_name = t.basename().unwrap_or("<Unnamed Font>".to_string());
        // If the config is a table of tables, specialize it by font filename
        if let Some(specific) = config.as_object().and_then(|o| o.get(&font_name)) {
            specific
        } else {
            skip!(
                "unconfigured",
                &format!("No specific configuration found for {}", font_name)
            );
        }
    } else {
        &config
    };
    let glyphnames = GlyphNames::new(&font.font())
        .iter()
        .map(|(_, glyphname)| glyphname.to_string())
        .collect::<HashSet<String>>();
    if let Some(config_for_this_font) = font_config.as_array() {
        let mut problems = vec![];
        for glyph in config_for_this_font {
            if let Some(glyph) = glyph.as_str() {
                if !glyphnames.contains(glyph) {
                    problems.push(Status::fail(
                        "missing-glyph",
                        &format!("Font is missing required glyph {}", glyph),
                    ));
                }
            }
        }
        return_result(problems)
    } else {
        return Err(FontspectorError::General(
            "Configuration for has_glyphs is not an object or a list".to_string(),
        ));
    }
}
