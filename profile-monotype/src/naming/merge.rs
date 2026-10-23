//
// merge.rs
//
// Copyright © 2026 Monotype Imaging, Inc. All rights reserved
//

use crate::naming::{get_name_tuples, resolve_names, ResolvedStyle};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StyleParts {
    pub weight: String,
    pub width: String,
    pub slope: String,
    pub design: String,
    pub extra: String,
}

fn list_shrinker(values: &[String]) -> Vec<String> {
    let mut unique: Vec<String> = values
        .iter()
        .filter(|v| !v.is_empty())
        .cloned()
        .collect();
    unique.sort();
    unique.dedup();
    unique
}

fn first_non_empty_shrunk(values: &[String]) -> String {
    let shrunk = list_shrinker(values);
    shrunk.first().cloned().unwrap_or_default()
}

fn merge_style_field(
    styles: &[ResolvedStyle],
    pick: fn(&ResolvedStyle) -> Option<&str>,
) -> String {
    let collected: Vec<String> = styles
        .iter()
        .filter_map(|s| pick(s).filter(|v| !v.is_empty()).map(str::to_string))
        .collect();
    list_shrinker(&collected).join(" ")
}

fn merge_styles(styles: &[ResolvedStyle]) -> StyleParts {
    StyleParts {
        weight: merge_style_field(styles, |s| s.weight.as_deref()),
        width: merge_style_field(styles, |s| s.width.as_deref()),
        slope: merge_style_field(styles, |s| s.slope.as_deref()),
        design: merge_style_field(styles, |s| s.design.as_deref()),
        extra: merge_style_field(styles, |s| {
            if s.extra.is_empty() {
                None
            } else {
                Some(s.extra.as_str())
            }
        }),
    }
}

fn resolve_path(
    name: &str,
    style: Option<&str>,
    search_design: bool,
) -> (String, String, String, ResolvedStyle) {
    let (foundry, prod_var, family_tuple, style_tuple) =
        get_name_tuples(name, style, true, false);
    let mut name_tuple = family_tuple;
    name_tuple.extend(style_tuple);
    let (family, resolved_style) = resolve_names(&name_tuple, search_design, true);
    (foundry, prod_var, family, resolved_style)
}

pub fn family_naming_info(
    fullname: &str,
    fontname: &str,
    family: &str,
    style: &str,
    search_design: bool,
) -> (String, Vec<String>, StyleParts, String, Vec<String>) {
    let (foundry, prod_var, family_resolved, styles) =
        resolve_path(family, Some(style), search_design);
    let (foundry_full, prod_var_full, full_family, full_styles) =
        resolve_path(fullname, None, search_design);
    let (foundry_font, prod_var_font, ps_family, ps_styles) =
        resolve_path(fontname, None, search_design);

    let family_names = list_shrinker(&[family_resolved, full_family, ps_family.clone()]);
    let foundries = list_shrinker(&[foundry, foundry_full, foundry_font]);
    let product_variety = first_non_empty_shrunk(&[prod_var, prod_var_full, prod_var_font]);
    let style_parts = merge_styles(&[styles, full_styles, ps_styles]);

    (ps_family, family_names, style_parts, product_variety, foundries)
}

#[cfg(test)]
mod tests {
    use super::family_naming_info;

    #[test]
    fn palatino_lt_pro_bold() {
        let (ps_family, family_names, style, pv, foundries) = family_naming_info(
            "Palatino LT Pro Bold",
            "PalatinoLTPro-Bold",
            "Palatino LT Pro",
            "Bold",
            true,
        );
        assert_eq!(ps_family, "Palatino");
        assert_eq!(family_names, vec!["Palatino".to_string()]);
        assert_eq!(style.weight, "Bold");
        assert_eq!(style.width, "");
        assert_eq!(style.slope, "");
        assert_eq!(style.design, "");
        assert_eq!(style.extra, "");
        assert_eq!(pv, "Pro");
        assert_eq!(foundries, vec!["Linotype".to_string()]);
    }
}
