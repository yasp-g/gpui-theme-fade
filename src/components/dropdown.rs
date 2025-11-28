use crate::{
    AppView,
    components::{popover::render_popover, scrollbar::render_scrollbar},
    theme::{InterpolatableTheme, Theme},
};
use gpui::{
    Context, FocusHandle, IntoElement, MouseDownEvent, ScrollHandle, Window, div, hsla, prelude::*,
    ClickEvent,
};

pub fn render_dropdown(
    selector_id: &'static str,
    button_id: &'static str,
    item_id_prefix: &'static str,
    scroll_container_id: &'static str,
    is_open: bool,
    focus_handle: &FocusHandle,
    scroll_handle: &ScrollHandle,
    themes: &[Theme],
    selected_index: usize,
    preview_index: usize,
    disabled_indices: &[usize],
    disabled: bool,
    theme: &InterpolatableTheme,
    on_toggle: impl Fn(&mut AppView, &MouseDownEvent, &mut Window, &mut Context<AppView>) + 'static,
    on_select: impl Fn(usize, &mut AppView, &ClickEvent, &mut Window, &mut Context<AppView>)
        + 'static
        + Clone,
    cx: &mut Context<AppView>,
) -> impl IntoElement {
    let header_focus_handle = focus_handle.clone();
    let selected_theme_name = themes[selected_index].name.clone();

    let text_color = theme.0.get("text").map_or(gpui::black(), |c| c.hsla);
    let text_disabled_color = theme
        .0
        .get("text.disabled")
        .map_or(hsla(0.0, 0.0, 0.5, 1.0), |c| c.hsla);
    let border_color = theme.0.get("border").map_or(gpui::black(), |c| c.hsla);
    let focus_color = theme
        .0
        .get("border.focused")
        .map_or(gpui::blue(), |c| c.hsla);
    let popover_bg = theme
        .0
        .get("elevated_surface.background")
        .map_or(gpui::black(), |c| c.hsla);
    let element_hover = theme.0.get("element.hover").map_or(gpui::red(), |c| c.hsla);
    let element_selected = theme
        .0
        .get("element.selected")
        .map_or(gpui::blue(), |c| c.hsla);

    div()
        .id(selector_id)
        .key_context("ThemeSelector")
        .on_action(cx.listener(AppView::on_confirm_theme))
        .on_action(cx.listener(AppView::on_select_next_theme))
        .on_action(cx.listener(AppView::on_select_prev_theme))
        .on_action(cx.listener(AppView::on_cancel))
        .relative()
        .child(
            div()
                .id(button_id)
                .w_full()
                .flex()
                .justify_between()
                .items_center()
                .gap_2()
                .p_1()
                .rounded_md()
                .when(disabled, |s| s.opacity(0.5).cursor(gpui::CursorStyle::OperationNotAllowed))
                .when(!disabled, |s| {
                    s.track_focus(focus_handle)
                        .hover(|style| style.bg(element_hover))
                        .on_mouse_down(
                            gpui::MouseButton::Left,
                            cx.listener(move |view, event, window, cx| {
                                cx.stop_propagation();
                                window.focus(&header_focus_handle);
                                on_toggle(view, event, window, cx);
                            }),
                        )
                })
                .child(selected_theme_name)
                .child(div().child("▼")),
        )
        .child(render_popover(
            is_open,
            div()
                .size_full()
                .children([
                    div()
                        .id("dropdown-backdrop")
                        .absolute()
                        .size_full()
                        .on_click(|_, _, cx| {
                            cx.dispatch_action(&crate::CloseDropdowns);
                        })
                        .into_any_element(),
                    div()
                        .id("dropdown-content")
                        .occlude()
                        .absolute()
                        .right_0()
                        .w_full()
                        .border_1()
                        .border_color(focus_color)
                        .bg(popover_bg)
                        .text_color(text_color)
                        .rounded_md()
                        .shadow_lg()
                        .on_click(|_, _, app_context| app_context.stop_propagation())
                        .child(
                            div()
                                .id(scroll_container_id)
                                .track_scroll(scroll_handle)
                                .max_h_64() // Corresponds to `max-height: 16rem;` or 256px
                                .overflow_y_scroll()
                                .children(themes.iter().enumerate().map(|(index, theme_item)| {
                                    let on_select = on_select.clone();
                                    let is_disabled = disabled_indices.contains(&index);

                                    div()
                                        .id((item_id_prefix, index))
                                        .p_2()
                                        .when(!is_disabled, |s| {
                                            s.hover(|style| style.bg(element_hover)).on_click(
                                                cx.listener(move |view, ev, win, cx| {
                                                    on_select(index, view, ev, win, cx);
                                                }),
                                            )
                                        })
                                        .when(index == preview_index, |style| {
                                            style.bg(element_selected)
                                        })
                                        .when(is_disabled, |s| s.text_color(text_disabled_color))
                                        .child(theme_item.name.clone())
                                })),
                        )
                        .child(render_scrollbar(
                            (scroll_container_id, 1 as usize),
                            scroll_handle,
                            theme,
                        ))
                        .into_any_element(),
                ]),
        ))
}