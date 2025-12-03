mod forbidden;
mod regression;
pub(crate) mod schema;
use std::str::FromStr;

use fontspector_checkapi::{Context, FontspectorError, Testable};
pub use forbidden::forbidden;
pub use regression::regression;

use harfrust::{GlyphBuffer, Shaper, ShaperData, UnicodeBuffer};
use schema::{ShapingConfig, ShapingInput, ShapingOptions, ShapingTest};

pub(crate) struct FailedCheck {
    test: ShapingTest,
    detail: String,
}

pub(crate) fn create_buffer_and_run(
    shaper: &harfrust::Shaper,
    input: &str,
    options: &ShapingOptions,
) -> Result<GlyphBuffer, FontspectorError> {
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(input);
    buffer.guess_segment_properties();
    if let Some(script) = options.script.as_deref() {
        buffer.set_script(
            harfrust::Script::from_str(script)
                .map_err(|e| FontspectorError::Shaping(format!("Bad 'script' argument {e}")))?,
        );
    }
    if let Some(language) = options.language.as_deref() {
        buffer.set_language(
            harfrust::Language::from_str(language)
                .map_err(|e| FontspectorError::Shaping(format!("Bad 'language' argument {e}")))?,
        );
    }
    if let Some(direction) = options.direction.as_deref() {
        buffer.set_direction(
            harfrust::Direction::from_str(direction)
                .map_err(|e| FontspectorError::Shaping(format!("Bad 'direction' argument {e}")))?,
        );
    }
    let features = options
        .features
        .clone() // Urgh, to avoid partial move
        .unwrap_or_default()
        .iter()
        .map(|(tag, value)| {
            harfrust::Tag::new_checked(tag.as_bytes())
                .map(|tag| harfrust::Feature::new(tag, *value as u32, ..))
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| FontspectorError::Shaping(format!("Bad feature tag: {e}")))?;

    Ok(shaper.shape(buffer, &features))
}

pub(crate) trait ShapingCheck {
    fn run(
        &self,
        t: &Testable,
        context: &Context,
    ) -> Result<Vec<(String, Vec<FailedCheck>)>, FontspectorError> {
        let fontref = harfrust::FontRef::new(&t.contents)
            .map_err(|e| FontspectorError::Shaping(format!("Failed to load font file: {e}")))?;

        let basename = t.basename().unwrap_or_default();
        let mut results = vec![];
        let shaping_file = context
            .configuration
            .get("shaping")
            .and_then(|shaping| shaping.as_object())
            .and_then(|shaping| shaping.get("test_directory"))
            .and_then(|test_directory: &serde_json::Value| test_directory.as_str())
            .ok_or(FontspectorError::skip(
                "no-tests",
                "Shaping test directory not defined in configuration file",
            ))?;
        let files = glob::glob(&format!("{shaping_file}/*.json"))
        .map_err(|_| {
            FontspectorError::General("Invalid pattern in glob for shaping tests (shaping directory in configuration file was bad?)".to_string())
        })?
        .flatten();

        for file in files {
            let file_contents = std::fs::read_to_string(&file)?;
            let input: ShapingInput = serde_json::from_str(&file_contents)?;
            let config = input.configuration;
            let mut failed_checks = vec![];
            for test in input.tests {
                if !self.applies(&config, &test) || test.excluded(&basename) {
                    continue;
                }
                let options = test.options.fill_from_defaults(&config);

                // Create shaper (we can't make this a separate function because of lifetime horrors)
                let shaper_data = ShaperData::new(&fontref);
                let mut shaper_builder = shaper_data.shaper(&fontref);
                let hr_variations: Vec<_> = if let Some(ref variations) = options.variations {
                    variations
                        .iter()
                        .map(|(tag, value)| {
                            harfrust::Tag::new_checked(tag.as_bytes())
                                .map(|tag| harfrust::Variation { tag, value: *value })
                        })
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|e| FontspectorError::Shaping(format!("Bad variation tag: {e}")))?
                } else {
                    vec![]
                };
                let shaper_instance =
                    harfrust::ShaperInstance::from_variations(&fontref, hr_variations);
                shaper_builder = shaper_builder.instance(Some(&shaper_instance));
                let shaper = shaper_builder.build();
                // Run the test
                let glyph_buffer = create_buffer_and_run(&shaper, &test.input, &options)?;
                if let Some(res) = self.pass_fail(&test, &config, &glyph_buffer, &shaper) {
                    failed_checks.push(FailedCheck {
                        test: test.clone(),
                        detail: res,
                    });
                }
            }
            results.push((file.to_string_lossy().to_string(), failed_checks));
        }
        Ok(results)
    }

    fn applies(&self, configuration: &ShapingConfig, test: &ShapingTest) -> bool;

    fn pass_fail(
        &self,
        test: &ShapingTest,
        configuration: &ShapingConfig,
        buffer: &GlyphBuffer,
        shaper: &Shaper,
    ) -> Option<String>;
}
