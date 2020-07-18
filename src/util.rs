use crate::CasingScheme;
use std::collections::HashSet;

pub const KEYWORDS: &[&str] = &[
    "abstract", "alignof", "as", "async", "await", "become", "box", "break", "const", "continue",
    "crate", "do", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in",
    "let", "loop", "macro", "match", "mod", "move", "mut", "offsetof", "override", "priv", "proc",
    "pub", "pure", "ref", "return", "self", "Self", "sizeof", "static", "struct", "super", "trait",
    "true", "try", "type", "typeof", "unsafe", "unsized", "use", "virtual", "where", "while",
    "yield",
];

pub fn fix_name(name: &str, used: &mut HashSet<String>, casing: CasingScheme) -> String {
    // TODO ascii-fy everything until rust allows utf-8 identifiers
    let name = name.trim();
    let mut out = match name.chars().next() {
        Some(c) if c.is_ascii() && c.is_numeric() => casing.convert(&format!("n{}", name)),
        _ => casing.convert(name),
    };

    if KEYWORDS.contains(&&*out) {
        out.push_str("_");
    }

    assert!(!out.is_empty());

    let mut i = 1;
    // clone so we have the original string to try a new suffix with
    let mut temp = out.clone();
    loop {
        if !used.contains(&temp) {
            used.insert(temp.clone());
            break temp;
        }
        i += 1;
        temp = format!("{}{}", out, i);
    }
}
