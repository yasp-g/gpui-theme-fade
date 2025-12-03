use gpui::{div, hsla, prelude::*, Rems};
use crate::theme::InterpolatableTheme;

pub fn render_activity_bar(theme: &InterpolatableTheme) -> impl IntoElement {
    let bg_color = theme.0.get("activity_bar.background").map_or(hsla(0., 0., 0.1, 1.), |c| c.hsla);
    let icon_color = theme.0.get("icon.muted").map_or(hsla(0., 0., 0.7, 1.), |c| c.hsla);
    let active_color = theme.0.get("icon.accent").map_or(hsla(0., 0., 0.9, 1.), |c| c.hsla);

    div()
        .w(Rems(3.0)) // 48px width
        .h_full()
        .flex()
        .flex_col()
        .items_center()
        .justify_start()
        .gap_4()
        .py_4()
        .bg(bg_color)
        .child(render_activity_bar_icon("icons/file_tree.svg", active_color, true))
        .child(render_activity_bar_icon("icons/magnifying_glass.svg", icon_color, false))
        .child(div().flex_1()) // Spacer
        .child(render_activity_bar_icon("icons/settings.svg", icon_color, false))
}

fn render_activity_bar_icon(path: impl Into<gpui::SharedString>, color: gpui::Hsla, active: bool) -> impl IntoElement {
    let path: gpui::SharedString = path.into();
    div()
        .size(Rems(1.5))
        .flex()
        .items_center()
        .justify_center()
        .text_color(color)
        .when(active, |s| s.text_color(color)) // Just making sure active color applies
        .child(
            gpui::svg()
                .path(path)
                .size(Rems(1.25))
                .text_color(color)
        )
}