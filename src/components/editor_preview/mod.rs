use crate::AppView;
use crate::components::scrollable_container::render_scrollable_container;
use crate::theme::InterpolatableTheme;
use gpui::{Context, IntoElement, Rems, SharedString, div, hsla, prelude::*, px};

pub mod breadcrumbs;
pub mod file_tree;
pub mod status_bar;
pub mod terminal;

use breadcrumbs::render_breadcrumbs;
use file_tree::render_file_tree;
use status_bar::render_status_bar;
use terminal::render_terminal;

#[derive(Clone, Copy)]
struct Token {
    text: &'static str,
    syntax: &'static str,
    is_error: bool,
}

impl Token {
    const fn new(text: &'static str, syntax: &'static str) -> Self {
        Self {
            text,
            syntax,
            is_error: false,
        }
    }

    const fn error(text: &'static str, syntax: &'static str) -> Self {
        Self {
            text,
            syntax,
            is_error: true,
        }
    }
}

fn get_example_code() -> Vec<Vec<Token>> {
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
        Token::new("Applying theme: {}", "string"),
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
        Token::new("Retrying...", "string"),
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

pub fn render_editor_preview(
    theme: &InterpolatableTheme,
    file_tree_scroll_handle: &gpui::ScrollHandle,
    editor_content_scroll_handle: &gpui::ScrollHandle,
    cx: &mut Context<AppView>,
) -> impl IntoElement {
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
    let error_color = theme
        .0
        .get("error")
        .map_or(hsla(0., 1.0, 0.5, 1.), |c| c.hsla);

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
                .overflow_hidden() // This constrains the container
                .min_h_0() // This allows the container to shrink below content size
                .child(render_file_tree(theme, file_tree_scroll_handle)) // Sidebar
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
                                .flex_1() // Allow it to take flex space
                                .min_h_0() // Allow shrinking
                                .relative()
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
                                        .font_family("Menlo")
                                        .children(
                                            (1..=code_lines.len())
                                                .map(|i| div().h(Rems(1.25)).child(i.to_string())),
                                        ),
                                )
                                .child(
                                    // Code Content
                                    render_scrollable_container(
                                        "editor-content-scroll-container",
                                        editor_content_scroll_handle,
                                        div()
                                            .flex_1()
                                            // .h_full() // Removed to allow content to exceed container height
                                            .p_2()
                                            .text_sm()
                                            .text_color(text_color)
                                            .font_family("Menlo")
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
                                                                    theme.0.get(&format!(
                                                                        "{}.color",
                                                                        key
                                                                    ))
                                                                })
                                                                .map(|c| c.hsla)
                                                                .unwrap_or(text_color)
                                                        };
                                                        div()
                                                            .text_color(color)
                                                            .when(token.is_error, |s| {
                                                                s.text_decoration_1()
                                                                    .text_decoration_color(
                                                                        error_color,
                                                                    )
                                                                    .text_decoration_wavy()
                                                            })
                                                            .child(token.text)
                                                    }),
                                                )
                                            })),
                                    )
                                    .size_full()
                                    .max_h_full(),
                                )
                                .child(
                                    // The scrollbar component for editor content
                                    crate::components::scrollbar::render_scrollbar(
                                        "editor-content-scrollbar",
                                        editor_content_scroll_handle,
                                        theme,
                                    ),
                                ),
                        )
                        .child(render_terminal(theme, cx)),
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
