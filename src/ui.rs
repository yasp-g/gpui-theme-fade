use crate::components::button::render_button;
use crate::components::dropdown::render_dropdown;
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
                                .child(render_dropdown(
                                    "start-theme-selector",
                                    "start-theme-button",
                                    "start-theme",
                                    "start-theme-scroll",
                                    app_state.start_dropdown_open,
                                    &app_state.theme_selector_focus_handle,
                                    &app_state.start_theme_scroll_handle,
                                    focus_color,
                                    &app_state.themes,
                                    app_state.start_theme_index,
                                    |view, _, _, cx| view.toggle_start_dropdown(cx),
                                    |index, view, _, _, cx| view.select_start_theme(index, cx),
                                    cx,
                                )),
                        )
                        .child(
                            // End Theme Selector
                            div()
                                .flex()
                                .w_full()
                                .justify_between()
                                .items_center()
                                .child(div().child("End Theme:"))
                                .child(render_dropdown(
                                    "end-theme-selector",
                                    "end-theme-button",
                                    "end-theme",
                                    "end-theme-scroll",
                                    app_state.end_dropdown_open,
                                    &app_state.end_theme_selector_focus_handle,
                                    &app_state.end_theme_scroll_handle,
                                    focus_color,
                                    &app_state.themes,
                                    app_state.end_theme_index,
                                    |view, _, _, cx| view.toggle_end_dropdown(cx),
                                    |index, view, _, _, cx| view.select_end_theme(index, cx),
                                    cx,
                                )),
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
