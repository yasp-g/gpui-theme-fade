use crate::components::scrollable_container::render_scrollable_container;
use crate::theme::InterpolatableTheme;
use gpui::{div, hsla, prelude::*, IntoElement, Rems, px};
// Removed breadcrumbs import
use super::dummy_data::get_example_code;
 // Import Token

// Renamed from render_editor_pane
pub fn render_editor_area(
    theme: &InterpolatableTheme,
    editor_content_scroll_handle: &gpui::ScrollHandle,
) -> impl IntoElement {
    let editor_bg = theme
        .0
        .get("editor.background")
        .map_or(hsla(0., 0., 0., 1.), |c| c.hsla);
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

    // Editor Area (Gutter + Code)
    div()
        .relative()
        .h(px(250.0)) // Force height constraint
        .flex()
        .child(
            // Gutter
            div()
                .w(Rems(3.0))
                .py_2()
                .flex()
                .flex_col()
                .text_sm()
                .bg(editor_bg)
                .font_family("Menlo")
                .children((0..code_lines.len()).map(|i| {
                    let is_active = i == 9;
                    let active_line_bg = theme
                        .0
                        .get("editor.active_line.background")
                        .map_or(hsla(0., 0., 0., 0.), |c| c.hsla);
                    let active_line_number_color = theme
                        .0
                        .get("editor.active_line_number")
                        .map_or(hsla(0., 0., 0., 1.), |c| c.hsla);

                    div()
                        .h(Rems(1.25))
                        .w_full() // Ensure background fills width
                        .flex()
                        .justify_end() // Align text to right
                        .pr_2() // Move padding inside the row
                        .when(is_active, |d| d.bg(active_line_bg))
                        .text_color(if is_active {
                            active_line_number_color
                        } else {
                            line_number_color
                        })
                        .child((i + 1).to_string())
                })),
        )
        .child(
            // Code Content
            render_scrollable_container(
                "editor-content-scroll-container",
                editor_content_scroll_handle,
                div()
                    .flex_1()
                    .py_2() // Only vertical padding on container
                    .text_sm()
                    .text_color(text_color)
                    .font_family("Menlo")
                    .children(
                        code_lines
                            .into_iter()
                            .enumerate() // Add enumerate to get the index
                            .map(|(line_idx, tokens)| {
                                let line_div = div().h(Rems(1.25)).flex().px_2(); // Padding inside row
                                let active_line_bg = theme
                                    .0
                                    .get("editor.active_line.background")
                                    .map_or(hsla(0., 0., 0., 0.), |c| c.hsla);

                                line_div
                                    .when(line_idx == 9, |d| d.bg(active_line_bg)) // Highlight line 10 (0-indexed)
                                    .children(
                                        tokens.into_iter().map(|token| {
                                            let color = if token.syntax == "text" {
                                                text_color
                                            } else {
                                                let key = format!("syntax.{}", token.syntax);
                                                theme
                                                    .0
                                                    .get(&key)
                                                    .or_else(|| theme.0.get(&format!("{}.color", key)))
                                                    .map(|c| c.hsla)
                                                    .unwrap_or(text_color)
                                            };
                                            div()
                                                .text_color(color)
                                                .when(token.is_error, |s| {
                                                    s.text_decoration_1()
                                                        .text_decoration_color(error_color)
                                                        .text_decoration_wavy()
                                                })
                                                .child(token.text)
                                        }),
                                    )
                            }),
                    ),
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
        )
}
