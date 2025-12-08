use crate::AppView;
use crate::theme::InterpolatableTheme;
use gpui::{Context, IntoElement, div, hsla, prelude::*};

pub mod breadcrumbs;
pub mod file_tree;
pub mod status_bar;
pub mod terminal;
pub mod dummy_data;
pub mod editor_area; // Renamed
pub mod tab_bar; // New

use file_tree::render_file_tree;
use status_bar::render_status_bar;
use terminal::render_terminal;
use editor_area::render_editor_area; // Renamed
use tab_bar::render_tab_bar; // New
use breadcrumbs::render_breadcrumbs; // Import breadcrumbs directly

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
                        .child(render_tab_bar(theme)) // Tab Bar
                        .child(render_breadcrumbs(theme)) // Breadcrumbs
                        .child(
                            // Editor Content Area (Code + Terminal Flex Container)
                            div()
                                .flex()
                                .flex_col()
                                .flex_1() // Take all remaining space
                                .child(render_editor_area(theme, editor_content_scroll_handle))
                                .child(render_terminal(theme, cx)),
                        ),
                ),
        )
        .child(render_status_bar(theme)) // Status Bar at bottom
}