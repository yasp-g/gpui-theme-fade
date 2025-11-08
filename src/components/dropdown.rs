use crate::{
    components::{popover::render_popover, scrollbar::render_scrollbar},
    AppView,
};
use gpui::{
    div, hsla, prelude::*, rems, ClickEvent, Context, FocusHandle, IntoElement, ScrollHandle,
    Window,
};

pub fn render_dropdown(
    selector_id: &'static str,
    button_id: &'static str,
    item_id_prefix: &'static str,
    scroll_container_id: &'static str,
    is_open: bool,
    focus_handle: &FocusHandle,
    scroll_handle: &ScrollHandle,
    focus_color: gpui::Hsla,
    themes: &[crate::Theme],
    selected_index: usize,
    on_toggle: impl Fn(&mut AppView, &ClickEvent, &mut Window, &mut Context<AppView>) + 'static,
    on_select: impl Fn(usize, &mut AppView, &ClickEvent, &mut Window, &mut Context<AppView>)
        + 'static
        + Clone,
    cx: &mut Context<AppView>,
) -> impl IntoElement {
    let selected_theme_name = themes[selected_index].name.clone();

    div()
        .id(selector_id)
        .key_context("ThemeSelector")
        .on_action(cx.listener(AppView::on_confirm_theme))
        .on_action(cx.listener(AppView::on_select_next_theme))
        .on_action(cx.listener(AppView::on_select_prev_theme))
        .relative()
        .child(
            div()
                .id(button_id)
                .track_focus(focus_handle)
                .focus(|s| s.border_color(focus_color))
                .flex()
                .items_center()
                .gap_2()
                .p_2()
                .border_1()
                .border_color(hsla(0., 0., 1., 0.2))
                .rounded_md()
                .hover(|style| style.bg(hsla(0., 0., 1., 0.1)))
                .on_click(cx.listener(on_toggle))
                .child(selected_theme_name)
                .child(div().child("▼")),
        )
        .child(render_popover(
            is_open,
            div()
                .absolute()
                .top(rems(2.5))
                .right_0()
                .w_48()
                .child(
                    div()
                        .id(scroll_container_id)
                        .track_scroll(scroll_handle)
                        .max_h_64() // Corresponds to `max-height: 16rem;` or 256px
                        .overflow_y_scroll()
                        .border_1()
                        .border_color(focus_color)
                        .bg(hsla(0., 0., 0., 0.8))
                        .text_color(hsla(0., 0., 1., 0.8))
                        .rounded_md()
                        .shadow_lg()
                        .children(themes.iter().enumerate().map(|(index, theme)| {
                            let on_select = on_select.clone();
                            div()
                                .id((item_id_prefix, index))
                                .p_2()
                                .hover(|style| style.bg(hsla(0., 0., 1., 0.1)))
                                .when(index == selected_index, |style| {
                                    style.bg(hsla(0., 0., 1., 0.2))
                                })
                                .on_click(cx.listener(move |view, ev, win, cx| {
                                    on_select(index, view, ev, win, cx);
                                }))
                                .child(theme.name.clone())
                        })),
                )
                .child(render_scrollbar(scroll_handle, cx)),
        ))
}