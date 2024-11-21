const JAVASCRIPT_KEYWORDS: [&str; 11] = [
    "let",
    "var",
    "const",
    "class",
    "function",
    "volatile",
    "boolean",
    "package",
    "byte",
    "arguments",
    "abstract",
];

pub fn is_javascript_keyword(word: &str) -> bool {
    JAVASCRIPT_KEYWORDS.contains(&word)
}