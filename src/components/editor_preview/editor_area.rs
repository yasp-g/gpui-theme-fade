use crate::components::scrollable_container::render_scrollable_container;
use crate::theme::InterpolatableTheme;
use gpui::{div, hsla, prelude::*, IntoElement, Rems, px};
// Removed breadcrumbs import
use super::dummy_data::get_example_code;
use super::dummy_data::Token; // Import Token

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
                .items_end()
                .pr_2()
                .text_color(line_number_color)
                .text_sm()
                .bg(editor_bg)
                .font_family("Menlo")
                .children(
                    (1..=code_lines.len()).map(|i| div().h(Rems(1.25)).child(i.to_string())),
                ),
        )
        .child(
            // Code Content
            render_scrollable_container(
                "editor-content-scroll-container",
                editor_content_scroll_handle,
                div()
                    .flex_1()
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
        )
}
