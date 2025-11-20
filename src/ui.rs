use crate::AppView;
use crate::components::button::render_button;
use crate::components::dropdown::render_dropdown;
use crate::components::form_field::render_form_field;
use crate::components::gradient_bar::render_gradient_bar;
use crate::components::panel::render_panel;
use gpui::{Context, IntoElement, div, prelude::*, rems};

pub fn render_interactive_ui(
    view: &mut crate::AppView,
    cx: &mut Context<crate::AppView>,
) -> impl IntoElement {
    let app_state = cx.global::<crate::AppState>().clone();
    let active_theme = &app_state.active_theme;
    let surface_background = active_theme.0.get("surface.background").unwrap().hsla;
    let text_color = active_theme.0.get("text").unwrap().hsla;

    let start_theme = &app_state.themes[app_state.start_theme_index];
    let end_theme = &app_state.themes[app_state.end_theme_index];

    let key_colors = [
        "surface.background",
        "text",
        "element.background",
        "element.hover",
        "element.selected",
        "border",
        "border.focused",
    ];

    div()
        .key_context("InteractiveUI")
        .on_action(cx.listener(AppView::on_focus_next))
        .on_action(cx.listener(AppView::on_focus_prev))
        .on_action(cx.listener(AppView::on_submit))
        .on_action(cx.listener(AppView::on_close_dropdowns))
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
                .child(render_panel(
                    "left-panel",
                    rems(1.0).into(), // gap_4
                    vec![
                        render_form_field(
                            "Start Theme:",
                            None,
                            render_dropdown(
                                "start-theme-selector",
                                "start-theme-button",
                                "start-theme",
                                "start-theme-scroll",
                                view.start_dropdown_state.is_open,
                                &view.start_dropdown_state.focus_handle,
                                &view.start_dropdown_state.scroll_handle,
                                &app_state.themes,
                                app_state.start_theme_index,
                                view.start_dropdown_state.preview_index,
                                &[app_state.end_theme_index],
                                active_theme,
                                |view, _, _, cx| view.toggle_start_dropdown(cx),
                                |index, view, _, _, cx| view.select_start_theme(index, cx),
                                cx,
                            ),
                        )
                        .into_any_element(),
                        render_form_field(
                            "End Theme:",
                            None,
                            render_dropdown(
                                "end-theme-selector",
                                "end-theme-button",
                                "end-theme",
                                "end-theme-scroll",
                                view.end_dropdown_state.is_open,
                                &view.end_dropdown_state.focus_handle,
                                &view.end_dropdown_state.scroll_handle,
                                &app_state.themes,
                                app_state.end_theme_index,
                                view.end_dropdown_state.preview_index,
                                &[app_state.start_theme_index],
                                active_theme,
                                |view, _, _, cx| view.toggle_end_dropdown(cx),
                                |index, view, _, _, cx| view.select_end_theme(index, cx),
                                cx,
                            ),
                        )
                        .into_any_element(),
                        render_form_field(
                            "Sleep Duration (s):",
                            view.sleep_input_state.validation_message.clone(),
                            view.sleep_input_state.input.clone(),
                        )
                        .into_any_element(),
                        render_form_field(
                            "Fade Duration (s):",
                            view.fade_input_state.validation_message.clone(),
                            view.fade_input_state.input.clone(),
                        )
                        .into_any_element(),
                        render_button(
                            "run-simulation-button",
                            "Run Simulation",
                            &view.run_simulation_focus_handle,
                            |view, _, _, cx| {
                                view.run_simulation(cx);
                            },
                            cx,
                        )
                        .into_any_element(),
                    ],
                ))
                .child(render_panel(
                    "right-panel",
                    rems(0.5).into(), // gap_2
                    key_colors
                        .iter()
                        .map(|&key| {
                            let start_hsla = start_theme
                                .interpolatable_theme
                                .0
                                .get(key)
                                .map_or(gpui::black(), |c| c.hsla);
                            let end_hsla = end_theme
                                .interpolatable_theme
                                .0
                                .get(key)
                                .map_or(gpui::black(), |c| c.hsla);
                            div()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .child(div().text_sm().child(key))
                                .child(render_gradient_bar(start_hsla, end_hsla))
                                .into_any_element()
                        })
                        .collect::<Vec<_>>(),
                )),
        )
}
