use crate::theme::InterpolatableTheme;
use gpui::{div, hsla, prelude::*, IntoElement, Rems};

struct Token {
    text: &'static str,
    syntax: &'static str,
}

impl Token {
    const fn new(text: &'static str, syntax: &'static str) -> Self {
        Self { text, syntax }
    }
}

fn get_example_code() -> Vec<Vec<Token>> {
    vec![
        // Line 1: use std::collections::HashMap;
        vec![
            Token::new("use", "keyword"),
            Token::new(" ", "text"),
            Token::new("std", "variable"),
            Token::new("::", "punctuation.delimiter"),
            Token::new("collections", "variable"),
            Token::new("::", "punctuation.delimiter"),
            Token::new("HashMap", "type"),
            Token::new(";", "punctuation.delimiter"),
        ],
        // Line 2:
        vec![],
        // Line 3: struct User {
        vec![
            Token::new("struct", "keyword"),
            Token::new(" ", "text"),
            Token::new("User", "type"),
            Token::new(" ", "text"),
            Token::new("{", "punctuation.bracket"),
        ],
        // Line 4:     id: usize,
        vec![
            Token::new("    ", "text"),
            Token::new("id", "property"),
            Token::new(":", "punctuation.delimiter"),
            Token::new(" ", "text"),
            Token::new("usize", "type"),
            Token::new(",", "punctuation.delimiter"),
        ],
        // Line 5:     name: String,
        vec![
            Token::new("    ", "text"),
            Token::new("name", "property"),
            Token::new(":", "punctuation.delimiter"),
            Token::new(" ", "text"),
            Token::new("String", "type"),
            Token::new(",", "punctuation.delimiter"),
        ],
        // Line 6:
        vec![Token::new("}", "punctuation.bracket")],
        // Line 7:
        vec![],
        // Line 8: fn main() {
        vec![
            Token::new("fn", "keyword"),
            Token::new(" ", "text"),
            Token::new("main", "function"),
            Token::new("()", "punctuation.bracket"),
            Token::new(" ", "text"),
            Token::new("{", "punctuation.bracket"),
        ],
        // Line 9:     let user = User {
        vec![
            Token::new("    ", "text"),
            Token::new("let", "keyword"),
            Token::new(" ", "text"),
            Token::new("user", "variable"),
            Token::new(" ", "text"),
            Token::new("=", "operator"),
            Token::new(" ", "text"),
            Token::new("User", "type"),
            Token::new(" ", "text"),
            Token::new("{", "punctuation.bracket"),
        ],
        // Line 10:
        vec![
            Token::new("        ", "text"),
            Token::new("id", "property"),
            Token::new(":", "punctuation.delimiter"),
            Token::new(" ", "text"),
            Token::new("1", "number"),
            Token::new(",", "punctuation.delimiter"),
        ],
        // Line 11:
        vec![
            Token::new("        ", "text"),
            Token::new("name", "property"),
            Token::new(":", "punctuation.delimiter"),
            Token::new(" ", "text"),
            Token::new("\"", "string"),
            Token::new("Alice", "string"),
            Token::new("\"", "string"),
            Token::new(".", "punctuation.delimiter"),
            Token::new("to_string", "function"),
            Token::new("()", "punctuation.bracket"),
            Token::new(",", "punctuation.delimiter"),
        ],
        // Line 12:
        vec![
            Token::new("    ", "text"),
            Token::new("}", "punctuation.bracket"),
            Token::new(";", "punctuation.delimiter"),
        ],
        // Line 13:
        vec![
            Token::new("    ", "text"),
            Token::new("println!", "function"),
            Token::new("(", "punctuation.bracket"),
            Token::new("\"", "string"),
            Token::new("Hello, {}!", "string"),
            Token::new("\"", "string"),
            Token::new(", ", "punctuation.delimiter"),
            Token::new("user", "variable"),
            Token::new(".", "punctuation.delimiter"),
            Token::new("name", "property"),
            Token::new(");", "punctuation.bracket"),
        ],
        // Line 14:
        vec![Token::new("}", "punctuation.bracket")],
    ]
}

pub fn render_code_preview(theme: &InterpolatableTheme) -> impl IntoElement {
    let editor_bg = theme
        .0
        .get("editor.background")
        .map_or(hsla(0., 0., 0., 1.), |c| c.hsla);
    let tab_bar_bg = theme
        .0
        .get("tab_bar.background")
        .map_or(hsla(0., 0., 0.1, 1.), |c| c.hsla);
    let tab_active_bg = theme
        .0
        .get("tab.active_background")
        .map_or(editor_bg, |c| c.hsla);
    let tab_inactive_bg = theme
        .0
        .get("tab.inactive_background")
        .map_or(tab_bar_bg, |c| c.hsla);
    let text_color = theme
        .0
        .get("text")
        .map_or(hsla(0., 0., 1., 1.), |c| c.hsla);
    let line_number_color = theme
        .0
        .get("editor.line_number")
        .map_or(hsla(0., 0., 0.5, 1.), |c| c.hsla);

    let code_lines = get_example_code();

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(editor_bg)
        .border_1()
        .border_color(
            theme
                .0
                .get("border")
                .map_or(hsla(0., 0., 0., 1.), |c| c.hsla),
        )
        .rounded_md()
        .child(
            // Tab Bar
            div()
                .flex()
                .items_center()
                .h(Rems(2.0)) // 32px
                .bg(tab_bar_bg)
                .child(render_tab("main.rs", true, tab_active_bg, text_color))
                .child(render_tab("lib.rs", false, tab_inactive_bg, text_color)),
        )
        .child(
            // Editor Area
            div()
                .flex()
                .flex_1()
                .child(
                    // Gutter
                    div()
                        .w(Rems(3.0))
                        .py_2()
                        .flex()
                        .flex_col()
                        .items_end()
                        .pr_2()
                        .text_color(line_number_color)
                        .text_sm()
                        .bg(editor_bg)
                        .font_family(".SystemUIFont") // Monospace for alignment
                        .children(
                            (1..=code_lines.len())
                                .map(|i| div().h(Rems(1.25)).child(i.to_string())),
                        ),
                )
                .child(
                    // Code Content
                    div()
                        .flex_1()
                        .p_2()
                        .text_sm()
                        .text_color(text_color)
                        .font_family(".SystemUIFont") // Need monospace
                        .children(code_lines.into_iter().map(|tokens| {
                            div()
                                .h(Rems(1.25)) // Fixed line height to match gutter
                                .flex()
                                .children(tokens.into_iter().map(|token| {
                                    let color = if token.syntax == "text" {
                                        text_color
                                    } else {
                                        theme
                                            .0
                                            .get(&format!("syntax.{}", token.syntax))
                                            .map(|c| c.hsla)
                                            .unwrap_or(text_color)
                                    };
                                    div().text_color(color).child(token.text)
                                }))
                        })),
                ),
        )
}

fn render_tab(
    name: impl Into<gpui::SharedString>,
    active: bool,
    bg: gpui::Hsla,
    text_color: gpui::Hsla,
) -> impl IntoElement {
    let name: gpui::SharedString = name.into();
    div()
        .px_3()
        .h_full()
        .flex()
        .items_center()
        .bg(bg)
        .text_color(text_color)
        .text_sm()
        .when(!active, |s| s.opacity(0.7))
        .child(name)
}