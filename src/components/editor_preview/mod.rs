use crate::theme::InterpolatableTheme;
use gpui::{IntoElement, Rems, div, hsla, prelude::*};

pub mod breadcrumbs;
pub mod file_tree;
pub mod status_bar;

use breadcrumbs::render_breadcrumbs;
use file_tree::render_file_tree;
use status_bar::render_status_bar;

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

pub fn render_editor_preview(theme: &InterpolatableTheme) -> impl IntoElement {
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
    let text_color = theme.0.get("text").map_or(hsla(0., 0., 1., 1.), |c| c.hsla);
    let line_number_color = theme
        .0
        .get("editor.line_number")
        .map_or(hsla(0., 0., 0.5, 1.), |c| c.hsla);

    let code_lines = get_example_code();

    // Full IDE Layout
    div()
        .flex()
        .flex_col() // Top to bottom
        .size_full()
        .border_1()
        .border_color(
            theme
                .0
                .get("border")
                .map_or(hsla(0., 0., 0., 1.), |c| c.hsla),
        )
        .rounded_md()
        .child(
            // Main Body (Sidebar | Editor)
            div()
                .flex()
                .flex_1()
                .child(render_file_tree(theme)) // Sidebar
                .child(
                    // Editor Column
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .bg(editor_bg)
                        .child(
                            // Tab Bar
                            div()
                                .flex()
                                .items_center()
                                .h(Rems(2.0))
                                .bg(tab_bar_bg)
                                .child(render_tab("main.rs", true, tab_active_bg, text_color))
                                .child(render_tab("lib.rs", false, tab_inactive_bg, text_color)),
                        )
                        .child(render_breadcrumbs(theme)) // Breadcrumbs
                        .child(
                            // Editor Area (Gutter + Code)
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
                                        .font_family(".SystemUIFont")
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
                                        .font_family(".SystemUIFont")
                                        .children(code_lines.into_iter().map(|tokens| {
                                            div().h(Rems(1.25)).flex().children(
                                                tokens.into_iter().map(|token| {
                                                    let color = if token.syntax == "text" {
                                                        text_color
                                                    } else {
                                                        let key =
                                                            format!("syntax.{}", token.syntax);
                                                        theme
                                                            .0
                                                            .get(&key)
                                                            .or_else(|| {
                                                                theme
                                                                    .0
                                                                    .get(&format!("{}.color", key))
                                                            })
                                                            .map(|c| c.hsla)
                                                            .unwrap_or(text_color)
                                                    };
                                                    div().text_color(color).child(token.text)
                                                }),
                                            )
                                        })),
                                ),
                        ),
                ),
        )
        .child(render_status_bar(theme)) // Status Bar at bottom
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
