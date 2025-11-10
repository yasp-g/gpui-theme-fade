use crate::{
    AppView,
    components::{popover::render_popover, scrollbar::render_scrollbar},
    theme::{InterpolatableTheme, Theme},
};
use gpui::{
    ClickEvent, Context, FocusHandle, IntoElement, ScrollHandle, Window, div, prelude::*, rems,
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
    theme: &InterpolatableTheme,
    on_toggle: impl Fn(&mut AppView, &ClickEvent, &mut Window, &mut Context<AppView>) + 'static,
    on_select: impl Fn(usize, &mut AppView, &ClickEvent, &mut Window, &mut Context<AppView>)
    + 'static
    + Clone,
    cx: &mut Context<AppView>,
) -> impl IntoElement {
    let selected_theme_name = themes[selected_index].name.clone();

    let text_color = theme.0.get("text").map_or(gpui::black(), |c| c.hsla);
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
                .border_color(border_color)
                .rounded_md()
                .hover(|style| style.bg(element_hover))
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
                        .bg(popover_bg)
                        .text_color(text_color)
                        .rounded_md()
                        .shadow_lg()
                        .children(themes.iter().enumerate().map(|(index, theme_item)| {
                            let on_select = on_select.clone();
                            div()
                                .id((item_id_prefix, index))
                                .p_2()
                                .hover(|style| style.bg(element_hover))
                                .when(index == selected_index, |style| style.bg(element_selected))
                                .on_click(cx.listener(move |view, ev, win, cx| {
                                    on_select(index, view, ev, win, cx);
                                }))
                                .child(theme_item.name.clone())
                        })),
                )
                .child(render_scrollbar(
                    (scroll_container_id, 1 as usize),
                    scroll_handle,
                    theme,
                )),
        ))
}
