use regex::Regex;
use std::sync::LazyLock;

use super::known_names::{FOUNTRIES, NONSENSE, PRODUCT_VARIETIES};

const CAMEL_CASE_FIXES: &[(&str, &str)] = &[
    ("LTW1G", "LT W1G"),
    ("Lightitalique", "Light Italique"),
];

static RE_HYPHEN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(.+)-([A-Z][a-z]+)").unwrap_or_else(|_| unreachable!()));
static RE_FIRST_CAP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(.)([A-Z][a-z]+)").unwrap_or_else(|_| unreachable!()));
static RE_ALL_CAP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([a-z])([A-Z])").unwrap_or_else(|_| unreachable!()));
static RE_NUMS_BETWEEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"([A-Za-z]+)([0-9]+)[A-Za-z]*").unwrap_or_else(|_| unreachable!())
});

fn camel_to_tuple(
    camel: &str,
    normalized_names: bool,
    is_font_name: bool,
) -> Vec<String> {
    if !normalized_names {
        return camel
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();
    }

    let mut camel = camel.to_string();
    if !is_font_name {
        camel = camel.replace('-', " ");
    }
    camel = camel.replace('/', " ");
    camel = camel.replace('_', " ");
    camel = camel.replace('"', " ");

    let mut converted = RE_HYPHEN.replace_all(&camel, "$1 - $2").to_string();
    converted = RE_FIRST_CAP.replace_all(&converted, "$1 $2").to_string();
    converted = RE_ALL_CAP.replace_all(&converted, "$1 $2").to_string();
    converted = RE_NUMS_BETWEEN.replace_all(&converted, "$1 $2").to_string();

    while converted.contains("  ") {
        converted = converted.replace("  ", " ");
    }

    for (fragment, replacement) in CAMEL_CASE_FIXES {
        if converted.contains(fragment) {
            converted = converted.replace(fragment, replacement);
        }
    }

    let mut names = Vec::new();
    for name in converted.split(' ') {
        if name.is_empty() {
            continue;
        }
        if name
            .chars()
            .next()
            .is_some_and(|c| c.is_lowercase())
        {
            let mut s = name.to_string();
            if let Some(first) = s.get_mut(0..1) {
                first.make_ascii_uppercase();
            }
            names.push(s);
        } else {
            names.push(name.to_string());
        }
    }
    names
}

fn split_fontname_tuple(fontname_tuple: &[String]) -> (Vec<String>, Vec<String>) {
    let mut family = Vec::new();
    let mut style = Vec::new();
    let mut in_family = true;
    for name in fontname_tuple {
        if name == "-" {
            in_family = false;
            continue;
        }
        if in_family {
            family.push(name.clone());
        } else {
            style.push(name.clone());
        }
    }
    (family, style)
}

fn extract_foundry(name_tuple: &[String]) -> (String, Vec<String>) {
    let mut foundry = String::new();
    let mut return_names = Vec::new();
    for name in name_tuple {
        if FOUNTRIES.contains_key(name.as_str()) {
            foundry = FOUNTRIES[name.as_str()].to_string();
            if name == "Linotype" {
                return_names.push(name.clone());
            }
        } else {
            return_names.push(name.clone());
        }
    }
    (foundry, return_names)
}

fn delete_nonsense(name_tuple: &[String]) -> Vec<String> {
    name_tuple
        .iter()
        .filter(|name| !NONSENSE.contains(name.as_str()))
        .cloned()
        .collect()
}

fn extract_product_variety(name_tuple: &[String]) -> (String, Vec<String>) {
    let mut return_names = Vec::new();
    let mut product_variety = String::new();
    for name in name_tuple {
        if PRODUCT_VARIETIES.contains(name.as_str()) {
            product_variety = name.clone();
        } else {
            return_names.push(name.clone());
        }
    }
    (product_variety, return_names)
}

pub fn get_name_tuples(
    name: &str,
    style: Option<&str>,
    normalized_names: bool,
    is_font_name: bool,
) -> (String, String, Vec<String>, Vec<String>) {
    let mut names_tuple = camel_to_tuple(name, normalized_names, is_font_name);

    let style_tuple = if let Some(style_str) = style {
        camel_to_tuple(style_str, normalized_names, is_font_name)
    } else {
        Vec::new()
    };

    let mut style_tuple = style_tuple;
    if is_font_name {
        let (family, style) = split_fontname_tuple(&names_tuple);
        names_tuple = family;
        style_tuple = style;
    }

    let (foundry, names_tuple) = extract_foundry(&names_tuple);
    let (product_variety, names_tuple) = extract_product_variety(&names_tuple);
    let names_tuple = delete_nonsense(&names_tuple);

    (foundry, product_variety, names_tuple, style_tuple)
}

#[cfg(test)]
mod tests {
    use super::get_name_tuples;

    #[test]
    fn present_lt_std_light_italic() {
        let (foundry, pv, family, style) =
            get_name_tuples("Present LT Std", Some("light italic"), true, false);
        assert_eq!(foundry, "Linotype");
        assert_eq!(pv, "Std");
        assert_eq!(family, vec!["Present".to_string()]);
        assert_eq!(style, vec!["Light".to_string(), "Italic".to_string()]);
    }

    #[test]
    fn empty_inputs() {
        let (foundry, pv, family, style) = get_name_tuples("", Some(""), true, false);
        assert_eq!(foundry, "");
        assert_eq!(pv, "");
        assert!(family.is_empty());
        assert!(style.is_empty());
    }

    #[test]
    fn present_linotype_keeps_linotype_in_family() {
        let (foundry, pv, family, style) =
            get_name_tuples("Present Linotype", Some("Light"), true, false);
        assert_eq!(foundry, "Linotype");
        assert_eq!(pv, "");
        assert_eq!(family, vec!["Present".to_string(), "Linotype".to_string()]);
        assert_eq!(style, vec!["Light".to_string()]);
    }

    #[test]
    fn prensa_display_comp_fullname() {
        let (foundry, pv, family, style) =
            get_name_tuples("PrensaDisplayComp SemiBold Italic", None, true, false);
        assert_eq!(foundry, "");
        assert_eq!(pv, "");
        assert_eq!(
            family,
            vec!["Prensa", "Display", "Comp", "Semi", "Bold", "Italic"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
        assert!(style.is_empty());
    }

    #[test]
    fn prensa_font_name_with_hyphen() {
        let (foundry, pv, family, style) =
            get_name_tuples("PrensaDisplayCompLTW1G-Lightitalique", None, true, true);
        assert_eq!(foundry, "");
        assert_eq!(pv, "");
        assert_eq!(
            family,
            vec!["Prensa", "Display", "Comp", "LTW", "1"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            style,
            vec!["Light".to_string(), "Italique".to_string()]
        );
    }
}
