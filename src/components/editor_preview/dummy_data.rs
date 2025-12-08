#[derive(Clone, Copy)]
pub struct Token {
    pub text: &'static str,
    pub syntax: &'static str,
    pub is_error: bool,
}

impl Token {
    pub const fn new(text: &'static str, syntax: &'static str) -> Self {
        Self {
            text,
            syntax,
            is_error: false,
        }
    }

    pub const fn error(text: &'static str, syntax: &'static str) -> Self {
        Self {
            text,
            syntax,
            is_error: true,
        }
    }
}

pub fn get_example_code() -> Vec<Vec<Token>> {
    let mut lines = Vec::new();

    // Line 1: use std::collections::HashMap;
    lines.push(vec![
        Token::new("use", "keyword"),
        Token::new(" ", "text"),
        Token::new("std", "variable"),
        Token::new("::", "punctuation.delimiter"),
        Token::new("collections", "variable"),
        Token::new("::", "punctuation.delimiter"),
        Token::new("HashMap", "type"),
        Token::new(";", "punctuation.delimiter"),
    ]);
    lines.push(vec![]);

    // Enum Definition
    lines.push(vec![
        Token::new("enum", "keyword"),
        Token::new(" ", "text"),
        Token::new("ThemeState", "type"),
        Token::new(" ", "text"),
        Token::new("{", "punctuation.bracket"),
    ]);
    lines.push(vec![
        Token::new("    ", "text"),
        Token::new("Loading", "variant"),
        Token::new(",", "punctuation.delimiter"),
    ]);
    lines.push(vec![
        Token::new("    ", "text"),
        Token::new("Active", "variant"),
        Token::new("(", "punctuation.bracket"),
        Token::new("Theme", "type"),
        Token::new(")", "punctuation.bracket"),
        Token::new(",", "punctuation.delimiter"),
    ]);
    lines.push(vec![
        Token::new("    ", "text"),
        Token::new("Error", "variant"),
        Token::new("{", "punctuation.bracket"),
        Token::new(" ", "text"),
        Token::new("msg", "property"),
        Token::new(":", "punctuation.delimiter"),
        Token::new(" ", "text"),
        Token::new("String", "type"),
        Token::new(" ", "text"),
        Token::new("}", "punctuation.bracket"),
        Token::new(",", "punctuation.delimiter"),
    ]);
    lines.push(vec![Token::new("}", "punctuation.bracket")]);
    lines.push(vec![]);

    // Function Definition
    lines.push(vec![
        Token::new("fn", "keyword"),
        Token::new(" ", "text"),
        Token::new("apply_theme", "function"),
        Token::new("(", "punctuation.bracket"),
        Token::new("state", "variable"),
        Token::new(":", "punctuation.delimiter"),
        Token::new(" ", "text"),
        Token::new("&", "operator"),
        Token::new("mut", "keyword"),
        Token::new(" ", "text"),
        Token::new("ThemeState", "type"),
        Token::new(")", "punctuation.bracket"),
        Token::new(" ", "text"),
        Token::new("->", "operator"),
        Token::new(" ", "text"),
        Token::new("Result", "type"),
        Token::new("<", "punctuation.bracket"),
        Token::new("()", "punctuation.bracket"),
        Token::new(">", "operator"),
        Token::new(" ", "text"),
        Token::new("{", "punctuation.bracket"),
    ]);

    // Match statement
    lines.push(vec![
        Token::new("    ", "text"),
        Token::new("match", "keyword"),
        Token::new(" ", "text"),
        Token::new("state", "variable"),
        Token::new(" ", "text"),
        Token::new("{", "punctuation.bracket"),
    ]);
    lines.push(vec![
        Token::new("        ", "text"),
        Token::new("ThemeState", "type"),
        Token::new("::", "punctuation.delimiter"),
        Token::new("Active", "variant"),
        Token::new("(", "punctuation.bracket"),
        Token::new("theme", "variable"),
        Token::new(")", "punctuation.bracket"),
        Token::new(" ", "text"),
        Token::new("=>", "operator"),
        Token::new(" ", "text"),
        Token::new("{", "punctuation.bracket"),
    ]);
    lines.push(vec![
        Token::new("            ", "text"),
        Token::new("println!", "function"),
        Token::new("(", "punctuation.bracket"),
        Token::new("\"", "string"),
        Token::new("Applying theme: {}\n", "string"),
        Token::new("\"", "string"),
        Token::new(",", "punctuation.delimiter"),
        Token::new(" ", "text"),
        Token::new("theme", "variable"),
        Token::new(".", "punctuation.delimiter"),
        Token::new("name", "property"),
        Token::new(")", "punctuation.bracket"),
        Token::new(";", "punctuation.delimiter"),
    ]);
    lines.push(vec![
        Token::new("        ", "text"),
        Token::new("}", "punctuation.bracket"),
        Token::new(",", "punctuation.delimiter"),
    ]);
    lines.push(vec![
        Token::new("        ", "text"),
        Token::new("_", "variable"),
        Token::new(" ", "text"),
        Token::new("=>", "operator"),
        Token::new(" ", "text"),
        Token::new("return", "keyword"),
        Token::new(" ", "text"),
        Token::new("Err", "variant"),
        Token::new("(", "punctuation.bracket"),
        Token::new("anyhow!", "function"),
        Token::new("(", "punctuation.bracket"),
        Token::new("\"", "string"),
        Token::new("Invalid state", "string"),
        Token::new("\"", "string"),
        Token::new(")", "punctuation.bracket"),
        Token::new(")", "punctuation.bracket"),
        Token::new(",", "punctuation.delimiter"),
    ]);
    lines.push(vec![
        Token::new("    ", "text"),
        Token::new("}", "punctuation.bracket"),
    ]);
    lines.push(vec![
        Token::new("    ", "text"),
        Token::new("Ok", "variant"),
        Token::new("(", "punctuation.bracket"),
        Token::new("()", "punctuation.bracket"),
        Token::new(")", "punctuation.bracket"),
    ]);
    lines.push(vec![Token::new("}", "punctuation.bracket")]);
    lines.push(vec![]);

    // Struct definition
    lines.push(vec![Token::new("#[derive(Debug, Clone)]", "attribute")]);
    lines.push(vec![
        Token::new("struct", "keyword"),
        Token::new(" ", "text"),
        Token::new("Config", "type"),
        Token::new(" ", "text"),
        Token::new("{", "punctuation.bracket"),
    ]);
    lines.push(vec![
        Token::new("    ", "text"),
        Token::new("pub", "keyword"),
        Token::new(" ", "text"),
        Token::new("opacity", "property"),
        Token::new(":", "punctuation.delimiter"),
        Token::new(" ", "text"),
        Token::new("f32", "type"),
        Token::new(",", "punctuation.delimiter"),
    ]);
    lines.push(vec![
        Token::new("    ", "text"),
        Token::new("pub", "keyword"),
        Token::new(" ", "text"),
        Token::new("retries", "property"),
        Token::new(":", "punctuation.delimiter"),
        Token::new(" ", "text"),
        Token::new("usize", "type"),
        Token::new(",", "punctuation.delimiter"),
    ]);
    lines.push(vec![Token::new("}", "punctuation.bracket")]);
    lines.push(vec![]);

    // Main with loop
    lines.push(vec![
        Token::new("fn", "keyword"),
        Token::new(" ", "text"),
        Token::new("main", "function"),
        Token::new("()", "punctuation.bracket"),
        Token::new(" ", "text"),
        Token::new("{", "punctuation.bracket"),
    ]);
    lines.push(vec![
        Token::new("    ", "text"),
        Token::new("// Initialize configuration", "comment"),
    ]);
    lines.push(vec![
        Token::new("    ", "text"),
        Token::new("let", "keyword"),
        Token::new(" ", "text"),
        Token::new("config", "variable"),
        Token::new(" ", "text"),
        Token::new("=", "operator"),
        Token::new(" ", "text"),
        Token::new("Config", "type"),
        Token::new(" ", "text"),
        Token::new("{", "punctuation.bracket"),
        Token::new(" ", "text"),
        Token::new("opacity", "property"),
        Token::new(":", "punctuation.delimiter"),
        Token::new(" ", "text"),
        Token::new("1.0", "number"),
        Token::new(",", "punctuation.delimiter"),
        Token::new(" ", "text"),
        Token::new("retries", "property"),
        Token::new(":", "punctuation.delimiter"),
        Token::new(" ", "text"),
        Token::new("3", "number"),
        Token::new(" ", "text"),
        Token::new("}", "punctuation.bracket"),
        Token::new(";", "punctuation.delimiter"),
    ]);
    lines.push(vec![]);
    lines.push(vec![
        Token::new("    ", "text"),
        Token::new("for", "keyword"),
        Token::new(" ", "text"),
        Token::new("i", "variable"),
        Token::new(" ", "text"),
        Token::new("in", "keyword"),
        Token::new(" ", "text"),
        Token::new("0", "number"),
        Token::new("..", "operator"),
        Token::new("config", "variable"),
        Token::new(".", "punctuation.delimiter"),
        Token::new("retries", "property"),
        Token::new(" ", "text"),
        Token::new("{", "punctuation.bracket"),
    ]);
    lines.push(vec![
        Token::new("        ", "text"),
        Token::new("if", "keyword"),
        Token::new(" ", "text"),
        Token::new("i", "variable"),
        Token::new(" ", "text"),
        Token::new(">", "operator"),
        Token::new(" ", "text"),
        Token::new("0", "number"),
        Token::new(" ", "text"),
        Token::new("{", "punctuation.bracket"),
    ]);
    lines.push(vec![
        Token::new("            ", "text"),
        Token::new("println!", "function"),
        Token::new("(", "punctuation.bracket"),
        Token::new("\"", "string"),
        Token::new("Retrying...\n", "string"),
        Token::new("\"", "string"),
        Token::new(")", "punctuation.bracket"),
        Token::new(";", "punctuation.delimiter"),
    ]);
    lines.push(vec![
        Token::new("        ", "text"),
        Token::new("}", "punctuation.bracket"),
    ]);
    lines.push(vec![
        Token::new("    ", "text"),
        Token::new("}", "punctuation.bracket"),
    ]);
    lines.push(vec![Token::new("}", "punctuation.bracket")]);

    // Duplicate to ensure it's long enough to scroll even on large screens
    let mut full_code = lines.clone();
    full_code.extend(lines);

    full_code
}
