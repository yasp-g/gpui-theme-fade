use gpui::{div, hsla, prelude::*, Context, FocusHandle, IntoElement};
use crate::AppView;

pub fn render_button(
    id: &'static str,
    label: impl IntoElement,
    key_context: Option<&'static str>,
    focus_handle: &FocusHandle,
    on_click_callback: impl Fn(&mut AppView, &gpui::ClickEvent, &mut gpui::Window, &mut Context<AppView>) + 'static + Clone,
    cx: &mut Context<AppView>,
) -> impl IntoElement {
    let button_focus_handle = focus_handle.clone();
    let app_state = cx.global::<crate::AppState>().clone();
    let active_theme = &app_state.active_theme;
    let focus_color = active_theme
        .0
        .get("border.focused")
        .map_or(gpui::blue(), |color| color.hsla);

    div()
        .id(id)
        .when(key_context.is_some(), |this| this.key_context(key_context.unwrap()))
        .track_focus(focus_handle)
        .focus(|s| s.border_color(focus_color))
        .p_2()
        .border_1()
        .border_color(hsla(0., 0., 1., 0.2))
        .rounded_md()
        .text_center()
        .hover(|style| style.bg(hsla(0., 0., 1., 0.1)))
        .on_mouse_down(gpui::MouseButton::Left, move |_, window, cx| {
            cx.stop_propagation();
            window.focus(&button_focus_handle);
        })
        .on_click(cx.listener(on_click_callback))
        .child(label)
}