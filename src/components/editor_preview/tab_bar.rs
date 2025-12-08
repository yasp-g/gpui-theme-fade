use gpui::{div, hsla, prelude::*, IntoElement, Rems};

pub fn render_tab_bar(
    theme: &crate::theme::InterpolatableTheme,
) -> impl IntoElement {
    let tab_bar_bg = theme
        .0
        .get("tab_bar.background")
        .map_or(hsla(0., 0., 0.1, 1.), |c| c.hsla);
    let tab_active_bg = theme
        .0
        .get("tab.active_background")
        .map_or(tab_bar_bg, |c| c.hsla); // Default to tab_bar_bg if editor_bg not available
    let tab_inactive_bg = theme
        .0
        .get("tab.inactive_background")
        .map_or(tab_bar_bg, |c| c.hsla);
    let text_color = theme.0.get("text").map_or(hsla(0., 0., 1., 1.), |c| c.hsla);

    div()
        .flex()
        .items_center()
        .h(Rems(2.0))
        .bg(tab_bar_bg)
        .child(render_tab("main.rs", true, tab_active_bg, text_color))
        .child(render_tab("lib.rs", false, tab_inactive_bg, text_color))
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
