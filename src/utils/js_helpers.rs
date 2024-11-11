const JAVASCRIPT_KEYWORDS: [&str; 12] = [
    "let",
    "var",
    "const",
    "class",
    "function",
    "volatile",
    "new",
    "boolean",
    "package",
    "byte",
    "arguments",
    "abstract",
];

pub fn is_javascript_keyword(word: &str) -> bool {
    JAVASCRIPT_KEYWORDS.contains(&word)
}