use fontspector_checkapi::{prelude::*, skip};
use image::image_dimensions;
use scraper::{Html, Selector};

fn image_requirements(extension: &str) -> Option<(u32, u32, &str, usize)> {
    match extension {
        "svg" => Some((2048, 1024, "vector", 1750 * 1024)),
        "png" | "jpg" | "jpeg" | "jxl" | "gif" => Some((2048, 1024, "raster", 800 * 1024)),
        _ => None,
    }
}
#[check(
    id = "googlefonts/article/images",
    rationale = "
        
        The purpose of this check is to ensure images (either raster or vector files)
        are not excessively large in filesize and resolution.

        These constraints are loosely based on infrastructure limitations under
        default configurations.

        It also ensures that the article page has a minimum length and includes
        at least one visual asset.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4594",
    title = "Validate size, and resolution of article images,
and ensure article page has minimum length and includes visual assets.",
    implementation = "all"
)]
fn images(c: &TestableCollection, _context: &Context) -> CheckFnResult {
    let Some(article) = c.get_file("ARTICLE.en_us.html") else {
        skip!("no-article", "ARTICLE.en_us.html not present")
    };
    let mut problems = vec![];
    let fragment = Html::parse_fragment(std::str::from_utf8(&article.contents)?);
    let mut has_visuals = false;
    #[allow(clippy::unwrap_used)] // it's a constant
    let selector = Selector::parse("img,svg,video,iframe").unwrap();
    for element in fragment.select(&selector) {
        if let Some(src) = element.value().attr("src") {
            let src = src.to_lowercase();
            let extension = src.split('.').next_back().unwrap_or_default();
            if let Some((max_width, max_height, image_type, max_size)) =
                image_requirements(extension)
            {
                has_visuals = true;
                let Some(file) = c.get_file(&src) else {
                    problems.push(Status::error(
                        Some("missing-visual-file"),
                        &format!("Visual asset file is missing: {src}"),
                    ));
                    continue;
                };
                let filesize = file.contents.len();
                if filesize > max_size {
                    problems.push(Status::fail(
                        "filesize",
                        &format!(
                            "`{src}` has `{filesize}` bytes, but the maximum filesize for {image_type} images is `{max_size}` bytes."
                        ),
                    ));
                }
                if let Ok((w, h)) = image_dimensions(&file.filename) {
                    if w > max_width || h > max_height {
                        problems.push(Status::fail(
                        "image-too-large",
                        &format!(
                            "Image {src}, is too large: `{w}` x `{h}` pixels`\n\nMax resolution allowed: `{max_width}` x `{max_height}` pixels"
                        ),
                    ));
                    }
                }
            } else {
                // Bad extension. Currently we ignore this
            }
        }
    }
    if !has_visuals {
        problems.push(Status::warn(
            "missing-visual-asset",
            "Article page lacks visual assets.",
        ));
    }
    return_result(problems)
}
