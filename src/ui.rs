use crate::components::button::render_button;
use crate::components::dropdown::render_dropdown;
use crate::components::editor_preview::render_editor_preview;
use crate::components::form_field::render_form_field;
// use crate::components::gradient_bar::render_gradient_bar;
use crate::AppView;
use crate::components::panel::render_panel;
use crate::state::SimulationState;
use gpui::{Context, IntoElement, div, prelude::*, rems};

const SHOW_THEME_HINT_FOOTER: bool = true;

pub fn render_interactive_ui(
    view: &mut crate::AppView,
    window: &mut gpui::Window,
    cx: &mut Context<crate::AppView>,
) -> impl IntoElement {
    let app_state = cx.global::<crate::AppState>().clone();
    let active_theme = &app_state.active_theme;
    let surface_background = active_theme.0.get("surface.background").unwrap().hsla;
    let text_color = active_theme.0.get("text").unwrap().hsla;

    let start_focused = view.start_dropdown_state.focus_handle.is_focused(window);
    let end_focused = view.end_dropdown_state.focus_handle.is_focused(window);
    let sleep_focused = view
        .sleep_input_state
        .input
        .read(cx)
        .focus_handle
        .is_focused(window);
    let fade_focused = view
        .fade_input_state
        .input
        .read(cx)
        .focus_handle
        .is_focused(window);
    let fps_focused = view
        .fps_input_state
        .input
        .read(cx)
        .focus_handle
        .is_focused(window);

    // let start_theme = &app_state.themes[app_state.start_theme_index];
    // let end_theme = &app_state.themes[app_state.end_theme_index];

    let is_running = view.simulation_state != SimulationState::Idle;

    // let key_colors = [
    //     "editor.background",
    //     "surface.background",
    //     "text",
    //     "text.accent",
    //     "element.selected",
    //     "border.focused",
    //     "element.background",
    //     "element.hover",
    //     "border",
    // ];

    div()
        .track_focus(&view.root_focus_handle)
        .on_mouse_down(
            gpui::MouseButton::Left,
            cx.listener(|view, _, window, cx| {
                view.focus_root(window, cx);
            }),
        )
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
                .child(div().w_64().flex_shrink_0().child(render_panel(
                    "left-panel",
                    rems(0.0).into(), // gap_0
                    active_theme,
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
                                is_running,
                                active_theme,
                                |view, _, _, cx| view.toggle_start_dropdown(cx),
                                |index, view, _, _, cx| view.select_start_theme(index, cx),
                                cx,
                            ),
                            is_running,
                            start_focused,
                            active_theme,
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
                                is_running,
                                active_theme,
                                |view, _, _, cx| view.toggle_end_dropdown(cx),
                                |index, view, _, _, cx| view.select_end_theme(index, cx),
                                cx,
                            ),
                            is_running,
                            end_focused,
                            active_theme,
                        )
                        .into_any_element(),
                        render_form_field(
                            "Sleep Duration (s):",
                            view.sleep_input_state.validation_message.clone(),
                            view.sleep_input_state.input.clone(),
                            is_running,
                            sleep_focused,
                            active_theme,
                        )
                        .into_any_element(),
                        render_form_field(
                            "Fade Duration (s):",
                            view.fade_input_state.validation_message.clone(),
                            view.fade_input_state.input.clone(),
                            is_running,
                            fade_focused,
                            active_theme,
                        )
                        .into_any_element(),
                        render_form_field(
                            "Target FPS:",
                            view.fps_input_state.validation_message.clone(),
                            view.fps_input_state.input.clone(),
                            is_running,
                            fps_focused,
                            active_theme,
                        )
                        .into_any_element(),
                        render_button(
                            "run-simulation-button",
                            if is_running {
                                "Running..."
                            } else {
                                "Run Simulation"
                            },
                            Some("RunButton"),
                            &view.run_simulation_focus_handle,
                            is_running,
                            |view, _, _, cx| {
                                view.run_simulation(cx);
                            },
                            cx,
                        )
                        .into_any_element(),
                        div()
                            .id("simulation-status")
                            .h_6() // Fixed height to prevent layout shift
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_sm()
                            .child(view.simulation_state.display())
                            .into_any_element(),
                    ],
                )))
                .child(div().flex_1().child(render_panel(
                    "right-panel",
                    rems(0.0).into(), // No gap
                    active_theme,
                    vec![render_editor_preview(active_theme, cx).into_any_element()],
                ))),
        )
        .when(SHOW_THEME_HINT_FOOTER, |parent| {
            parent.child(
                div()
                    .w_full()
                    .text_center()
                    .text_xs()
                    .p_1()
                    .text_color(text_color)
                    .opacity(0.6)
                    .child(format!(
                        "Themes found in ~/.config/zed/themes loaded at startup."
                    )),
            )
        })
}
