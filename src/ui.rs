use crate::components::button::render_button;
use crate::AppView;
use gpui::{Context, IntoElement, div, hsla, prelude::*, rems};

pub fn render_interactive_ui(
    _view: &mut crate::AppView,
    cx: &mut Context<crate::AppView>,
) -> impl IntoElement {
    let app_state = cx.global::<crate::AppState>().clone();
    let active_theme = &app_state.active_theme;
    let surface_background = active_theme.0.get("surface.background").unwrap().hsla;
    let text_color = active_theme.0.get("text").unwrap().hsla;
    let focus_color = active_theme
        .0
        .get("border.focused")
        .map_or(gpui::blue(), |color| color.hsla);

    div()
        .key_context("InteractiveUI")
        .on_action(cx.listener(AppView::on_focus_next))
        .on_action(cx.listener(AppView::on_focus_prev))
        .on_action(cx.listener(AppView::on_submit))
        .size_full()
        .bg(surface_background)
        .text_color(text_color)
        .p_4()
        .gap_4()
        .child(
            // Header
            div()
                .flex()
                .justify_between()
                .items_center()
                .child(div().text_xl().child("Theme Transition Simulator")),
        )
        .child(
            // Main Content
            div()
                .flex_1()
                .flex()
                .gap_4()
                .child(
                    // Left Panel
                    div()
                        .id("left-panel")
                        .flex_1()
                        .flex()
                        .flex_col()
                        .gap_4()
                        .p_4()
                        .border_1()
                        .border_color(hsla(0., 0., 1., 0.2))
                        .rounded_md()
                        .child(
                            // Start Theme Selector
                            div()
                                .flex()
                                .w_full()
                                .justify_between()
                                .items_center()
                                .child(div().child("Start Theme:"))
                                .child(
                                    div()
                                        .id("start-theme-selector")
                                        .key_context("ThemeSelector")
                                        .on_action(cx.listener(AppView::on_confirm_theme))
                                        .on_action(cx.listener(AppView::on_select_next_theme))
                                        .on_action(cx.listener(AppView::on_select_prev_theme))
                                        .relative()
                                        .child(
                                            div()
                                                .id("start-theme-button")
                                                .track_focus(&app_state.theme_selector_focus_handle)
                                                .focus(|s| s.border_color(focus_color))
                                                .flex()
                                                .items_center()
                                                .gap_2()
                                                .p_2()
                                                .border_1()
                                                .border_color(hsla(0., 0., 1., 0.2))
                                                .rounded_md()
                                                .hover(|style| style.bg(hsla(0., 0., 1., 0.1)))
                                                .on_click(cx.listener(|view, _, _, cx| {
                                                    view.toggle_start_dropdown(cx);
                                                }))
                                                .child(app_state.themes[app_state.start_theme_index].name.clone())
                                                .child(div().child("▼")),
                                        )
                                        .when(app_state.start_dropdown_open, |d| {
                                            d.child(
                                                div()
                                                    .absolute()
                                                    .top(rems(2.5))
                                                    .right_0()
                                                    .w_48()
                                                    .bg(hsla(0., 0., 0., 0.8))
                                                    .text_color(hsla(0., 0., 1., 0.8))
                                                    .border_1()
                                                    .border_color(hsla(0., 0., 1., 0.2))
                                                    .rounded_md()
                                                    .shadow_lg()
                                                    .children(app_state.themes.iter().enumerate().map(|(index, theme)| {
                                                        div()
                                                            .id(("start-theme", index))
                                                            .p_2()
                                                            .hover(|style| style.bg(hsla(0., 0., 1., 0.1)))
                                                            .when(index == app_state.start_theme_index, |style| {
                                                                style.bg(hsla(0., 0., 1., 0.2))
                                                            })
                                                            .on_click(cx.listener(move |view, _, _, cx| {
                                                                view.select_start_theme(index, cx);
                                                            }))
                                                            .child(theme.name.clone())
                                                    }))
                                            )
                                        }),
                                ),
                        )
                        .child(
                            // End Theme Selector
                            div()
                                .flex()
                                .w_full()
                                .justify_between()
                                .items_center()
                                .child(div().child("End Theme:"))
                                .child(
                                    div()
                                        .id("end-theme-selector")
                                        .key_context("ThemeSelector")
                                        .on_action(cx.listener(AppView::on_confirm_theme))
                                        .on_action(cx.listener(AppView::on_select_next_theme))
                                        .on_action(cx.listener(AppView::on_select_prev_theme))
                                        .relative()
                                        .child(
                                            div()
                                                .id("end-theme-button")
                                                .track_focus(&app_state.end_theme_selector_focus_handle)
                                                .focus(|s| s.border_color(focus_color))
                                                .flex()
                                                .items_center()
                                                .gap_2()
                                                .p_2()
                                                .border_1()
                                                .border_color(hsla(0., 0., 1., 0.2))
                                                .rounded_md()
                                                .hover(|style| style.bg(hsla(0., 0., 1., 0.1)))
                                                .on_click(cx.listener(|view, _, _, cx| {
                                                    view.toggle_end_dropdown(cx);
                                                }))
                                                .child(app_state.themes[app_state.end_theme_index].name.clone())
                                                .child(div().child("▼")),
                                        )
                                        .when(app_state.end_dropdown_open, |d| {
                                            d.child(
                                                div()
                                                    .absolute()
                                                    .top(rems(2.5))
                                                    .right_0()
                                                    .w_48()
                                                    .bg(hsla(0., 0., 0., 0.8))
                                                    .text_color(hsla(0., 0., 1., 0.8))
                                                    .border_1()
                                                    .border_color(hsla(0., 0., 1., 0.2))
                                                    .rounded_md()
                                                    .shadow_lg()
                                                    .children(app_state.themes.iter().enumerate().map(|(index, theme)| {
                                                        div()
                                                            .id(("end-theme", index))
                                                            .p_2()
                                                            .hover(|style| style.bg(hsla(0., 0., 1., 0.1)))
                                                            .when(index == app_state.end_theme_index, |style| {
                                                                style.bg(hsla(0., 0., 1., 0.2))
                                                            })
                                                            .on_click(cx.listener(move |view, _, _, cx| {
                                                                view.select_end_theme(index, cx);
                                                            }))
                                                            .child(theme.name.clone())
                                                    }))
                                            )
                                        }),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .w_full()
                                .justify_between()
                                .items_center()
                                .child(div().child("Sleep Duration (s):"))
                                .child(
                                    div()
                                        .w(rems(12.0))
                                        .border_1()
                                        .border_color(if app_state.sleep_input_is_valid {
                                            hsla(0., 0., 1., 0.2)
                                        } else {
                                            gpui::red()
                                        })
                                        .child(app_state.sleep_duration_input.clone()),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .w_full()
                                .justify_between()
                                .items_center()
                                .child(div().child("Fade Duration (s):"))
                                .child(
                                    div()
                                        .w(rems(12.0))
                                        .border_1()
                                        .border_color(if app_state.fade_input_is_valid {
                                            hsla(0., 0., 1., 0.2)
                                        } else {
                                            gpui::red()
                                        })
                                        .child(app_state.fade_duration_input.clone()),
                                ),
                        )
                        .child(
                            render_button(
                                "run-simulation-button",
                                "Run Simulation",
                                &app_state.run_simulation_focus_handle,
                                |view, _, _, cx| {
                                    view.run_simulation(cx);
                                },
                                cx,
                            ),
                        ),
                )
                .child(
                    // Right Panel
                    div()
                        .id("right-panel")
                        .flex_1()
                        .p_4()
                        .border_1()
                        .border_color(hsla(0., 0., 1., 0.2))
                        .rounded_md()
                        .child(
                            div().child("Color Palette"), // Placeholder
                        ),
                ),
        )
}
