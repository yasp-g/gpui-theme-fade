use gpui::{div, hsla, prelude::*, AppContext, Context, Div, FocusHandle, IntoElement};
use crate::AppView;

pub fn render_button(
    id: &'static str,
    label: impl IntoElement,
    focus_handle: &FocusHandle,
    on_click_callback: impl Fn(&mut AppView, &gpui::ClickEvent, &mut gpui::Window, &mut Context<AppView>) + 'static + Clone,
    cx: &mut Context<AppView>,
) -> impl IntoElement {
    let app_state = cx.global::<crate::AppState>().clone();
    let active_theme = &app_state.active_theme;
    let focus_color = active_theme
        .0
        .get("border.focused")
        .map_or(gpui::blue(), |color| color.hsla);

    div()
        .id(id)
        .track_focus(focus_handle)
        .focus(|s| s.border_color(focus_color))
        .p_2()
        .border_1()
        .border_color(hsla(0., 0., 1., 0.2))
        .rounded_md()
        .text_center()
        .hover(|style| style.bg(hsla(0., 0., 1., 0.1)))
        .on_click(cx.listener(on_click_callback))
        .child(label)
}