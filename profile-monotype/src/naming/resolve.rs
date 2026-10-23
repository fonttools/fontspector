//
// resolve.rs
//
// Copyright © 2026 Monotype Imaging, Inc. All rights reserved
//

use crate::naming::known_names::{
    is_design, is_modifier, is_slope, is_weight, is_width,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ResolvedStyle {
    pub weight: Option<String>,
    pub width: Option<String>,
    pub slope: Option<String>,
    pub design: Option<String>,
    pub extra: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LastName {
    None,
    Family,
    Style,
    Modifier,
    Extra,
}

fn starts_with_digits(name: &str) -> bool {
    name.chars().next().is_some_and(|c| c.is_ascii_digit())
}

fn flush_modifier_to_family(family: &str, temp_modifier: &str) -> String {
    if family.is_empty() {
        temp_modifier.to_string()
    } else {
        format!("{family} {temp_modifier}")
    }
}

fn apply_modifier(temp_modifier: &str, space: &str, found_name: &str) -> String {
    format!("{temp_modifier}{space}{found_name}")
}

fn normalized_modifier_for(name: &str) -> Option<&'static str> {
    if name == "Semi" || name == "Sm" {
        Some("Semi")
    } else {
        is_modifier(name)
    }
}

fn append_style_field(current: &mut Option<String>, new_value: String) {
    if let Some(existing) = current {
        existing.push(' ');
        existing.push_str(&new_value);
    } else {
        *current = Some(new_value);
    }
}

fn set_weight(weight: &mut Option<String>, new_weight: String, found_weight: &str) {
    if weight.is_none() {
        *weight = Some(new_weight);
    } else if weight.as_deref() == Some("Regular") && found_weight != "Regular" {
        *weight = Some(new_weight);
    } else if weight.as_deref() != Some("Regular") && found_weight == "Regular" {
        // keep existing non-Regular weight
    } else {
        let existing = weight.as_deref().map_or(String::new(), |w| w.to_string());
        *weight = Some(format!("{existing} {new_weight}"));
    }
}

pub fn resolve_names(
    name_tuple: &[String],
    search_design: bool,
    normalized_names: bool,
) -> (String, ResolvedStyle) {
    let mut last_name = LastName::None;
    let mut family = String::new();
    let mut weight: Option<String> = None;
    let mut width: Option<String> = None;
    let mut slope: Option<String> = None;
    let mut design: Option<String> = None;
    let mut extra = String::new();
    let mut temp_modifier = String::new();
    let mut maybe_part_of_family = false;
    let space = if normalized_names { "" } else { " " };

    for name in name_tuple {
        if let Some(normalized_modifier) = normalized_modifier_for(name) {
            let found_modifier = if normalized_names {
                normalized_modifier
            } else {
                name.as_str()
            };
            if last_name == LastName::Modifier {
                temp_modifier.push_str(name);
            } else if last_name == LastName::Family {
                temp_modifier = found_modifier.to_string();
                maybe_part_of_family = true;
                last_name = LastName::Modifier;
            } else if last_name == LastName::Style {
                temp_modifier = found_modifier.to_string();
                last_name = LastName::Modifier;
            }
            continue;
        }

        if let Some(normalized_weight_name) = is_weight(name) {
            let found_weight = if normalized_names {
                normalized_weight_name
            } else {
                name.as_str()
            };
            let new_weight = if last_name == LastName::Modifier {
                let combined = apply_modifier(&temp_modifier, space, found_weight);
                temp_modifier.clear();
                maybe_part_of_family = false;
                combined
            } else {
                found_weight.to_string()
            };
            set_weight(&mut weight, new_weight, found_weight);
            last_name = LastName::Style;
            continue;
        }

        if let Some(normalized_width_name) = is_width(name) {
            let found_width = if normalized_names {
                normalized_width_name
            } else {
                name.as_str()
            };
            let new_width = if last_name == LastName::Modifier {
                let combined = apply_modifier(&temp_modifier, space, found_width);
                temp_modifier.clear();
                maybe_part_of_family = false;
                combined
            } else {
                found_width.to_string()
            };
            append_style_field(&mut width, new_width);
            last_name = LastName::Style;
            continue;
        }

        if let Some(normalized_slope_name) = is_slope(name) {
            if last_name == LastName::Modifier && maybe_part_of_family {
                family = flush_modifier_to_family(&family, &temp_modifier);
                temp_modifier.clear();
                maybe_part_of_family = false;
            }
            let found_slope = if normalized_names {
                normalized_slope_name
            } else {
                name.as_str()
            };
            append_style_field(&mut slope, found_slope.to_string());
            last_name = LastName::Style;
            continue;
        }

        if search_design {
            if let Some(normalized_design_name) = is_design(name) {
                let found_design = if normalized_names {
                    normalized_design_name
                } else {
                    name.as_str()
                };
                let new_design = if last_name == LastName::Modifier {
                    let combined = apply_modifier(&temp_modifier, space, found_design);
                    temp_modifier.clear();
                    maybe_part_of_family = false;
                    combined
                } else {
                    found_design.to_string()
                };
                append_style_field(&mut design, new_design);
                last_name = LastName::Style;
                continue;
            }
        }

        let has_styles = weight.is_some() || width.is_some() || slope.is_some() || design.is_some();

        if family.is_empty() && !has_styles {
            family = name.clone();
            last_name = LastName::Family;
        } else if !family.is_empty()
            && !has_styles
            && last_name != LastName::Extra
            && !starts_with_digits(name)
        {
            if maybe_part_of_family {
                family = flush_modifier_to_family(&family, &temp_modifier);
                temp_modifier.clear();
                maybe_part_of_family = false;
            }
            if !family.is_empty() {
                family.push(' ');
            }
            family.push_str(name);
            last_name = LastName::Family;
        } else if !family.is_empty() && !has_styles && starts_with_digits(name) {
            if maybe_part_of_family {
                family = flush_modifier_to_family(&family, &temp_modifier);
                temp_modifier.clear();
                maybe_part_of_family = false;
            }
            if last_name == LastName::Extra {
                extra.push(' ');
                extra.push_str(name);
            } else {
                extra = name.clone();
            }
            last_name = LastName::Extra;
        } else if family.is_empty() && has_styles && last_name == LastName::Style {
            family = name.clone();
            last_name = LastName::Family;
        } else if !family.is_empty() && has_styles && matches!(last_name, LastName::Style | LastName::Extra) {
            if !extra.is_empty() {
                extra.push(' ');
                extra.push_str(name);
                last_name = LastName::Extra;
            } else {
                extra = name.clone();
                last_name = LastName::Extra;
            }
        }
    }

    if let Some(ref mut w) = weight {
        if w.contains(' ') {
            let filtered: Vec<&str> = w.split(' ').filter(|part| *part != "Regular").collect();
            *w = filtered.join(" ");
        }
    }

    (
        family,
        ResolvedStyle {
            weight,
            width,
            slope,
            design,
            extra,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parts(tokens: &[&str]) -> Vec<String> {
        tokens.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn prensa_semi_bold_it() {
        let name_parts = parts(&["Prensa", "Display", "Comp", "Semi", "Bold", "It"]);
        let (family, style) = resolve_names(&name_parts, true, true);
        assert_eq!(family, "Prensa");
        assert_eq!(style.weight.as_deref(), Some("SemiBold"));
        assert_eq!(style.width.as_deref(), Some("Compressed"));
        assert_eq!(style.slope.as_deref(), Some("Italic"));
        assert_eq!(style.design.as_deref(), Some("Display"));
        assert_eq!(style.extra, "");
    }

    #[test]
    fn prensa_semi_bold_italic_same_as_it() {
        let name_parts = parts(&["Prensa", "Display", "Comp", "Semi", "Bold", "Italic"]);
        let (family, style) = resolve_names(&name_parts, true, true);
        assert_eq!(family, "Prensa");
        assert_eq!(style.weight.as_deref(), Some("SemiBold"));
        assert_eq!(style.width.as_deref(), Some("Compressed"));
        assert_eq!(style.slope.as_deref(), Some("Italic"));
        assert_eq!(style.design.as_deref(), Some("Display"));
        assert_eq!(style.extra, "");
    }

    #[test]
    fn search_design_false_keeps_display_in_family() {
        let name_parts = parts(&["Prensa", "Display", "Comp", "Semi", "Bold", "Italic"]);
        let (family, style) = resolve_names(&name_parts, false, true);
        assert_eq!(family, "Prensa Display");
        assert_eq!(style.weight.as_deref(), Some("SemiBold"));
        assert_eq!(style.width.as_deref(), Some("Compressed"));
        assert_eq!(style.slope.as_deref(), Some("Italic"));
        assert_eq!(style.design, None);
        assert_eq!(style.extra, "");
    }

    #[test]
    fn prensa_without_slope() {
        let name_parts = parts(&["Prensa", "Display", "Comp", "Semi", "Bold"]);
        let (family, style) = resolve_names(&name_parts, true, true);
        assert_eq!(family, "Prensa");
        assert_eq!(style.weight.as_deref(), Some("SemiBold"));
        assert_eq!(style.width.as_deref(), Some("Compressed"));
        assert_eq!(style.slope, None);
        assert_eq!(style.design.as_deref(), Some("Display"));
        assert_eq!(style.extra, "");
    }

    #[test]
    fn search_design_false_without_slope() {
        let name_parts = parts(&["Prensa", "Display", "Comp", "Semi", "Bold"]);
        let (family, style) = resolve_names(&name_parts, false, true);
        assert_eq!(family, "Prensa Display");
        assert_eq!(style.weight.as_deref(), Some("SemiBold"));
        assert_eq!(style.width.as_deref(), Some("Compressed"));
        assert_eq!(style.slope, None);
        assert_eq!(style.design, None);
        assert_eq!(style.extra, "");
    }

    #[test]
    fn itsl_goes_to_extra() {
        let name_parts = parts(&["Prensa", "Display", "Comp", "Semi", "Bold", "Itsl"]);
        let (family, style) = resolve_names(&name_parts, true, true);
        assert_eq!(family, "Prensa");
        assert_eq!(style.weight.as_deref(), Some("SemiBold"));
        assert_eq!(style.width.as_deref(), Some("Compressed"));
        assert_eq!(style.slope, None);
        assert_eq!(style.design.as_deref(), Some("Display"));
        assert_eq!(style.extra, "Itsl");
    }

    #[test]
    fn multi_style_accumulation() {
        let name_parts = parts(&[
            "Prensa",
            "Display",
            "Comp",
            "Semi",
            "Bold",
            "Italic",
            "Black",
            "Wide",
            "Oblique",
            "Script",
        ]);
        let (family, style) = resolve_names(&name_parts, true, true);
        assert_eq!(family, "Prensa");
        assert_eq!(style.weight.as_deref(), Some("SemiBold Black"));
        assert_eq!(style.width.as_deref(), Some("Compressed Wide"));
        assert_eq!(style.slope.as_deref(), Some("Italic Oblique"));
        assert_eq!(style.design.as_deref(), Some("Display Script"));
        assert_eq!(style.extra, "");
    }

    #[test]
    fn numeric_extra_with_extra_modifier_word() {
        let name_parts = parts(&["Prensa", "style", "extra", "75"]);
        let (family, style) = resolve_names(&name_parts, true, true);
        assert_eq!(family, "Prensa style Extra");
        assert_eq!(style.weight, None);
        assert_eq!(style.width, None);
        assert_eq!(style.slope, None);
        assert_eq!(style.design, None);
        assert_eq!(style.extra, "75");
    }

    #[test]
    fn numeric_extra_only() {
        let name_parts = parts(&["Prensa", "style", "75"]);
        let (family, style) = resolve_names(&name_parts, true, true);
        assert_eq!(family, "Prensa style");
        assert_eq!(style.weight, None);
        assert_eq!(style.width, None);
        assert_eq!(style.slope, None);
        assert_eq!(style.design, None);
        assert_eq!(style.extra, "75");
    }

    #[test]
    fn empty_tuple() {
        let name_parts: Vec<String> = vec![];
        let (family, style) = resolve_names(&name_parts, true, true);
        assert_eq!(family, "");
        assert_eq!(style.weight, None);
        assert_eq!(style.width, None);
        assert_eq!(style.slope, None);
        assert_eq!(style.design, None);
        assert_eq!(style.extra, "");
    }
}
