use gpui::{div, hsla, prelude::*, rems, Context, Div, IntoElement};
use crate::{SelectTheme, Theme, ToggleDropdown};

// This helper function does NOT need cx, because the on_click handler provides its own.
fn render_theme_selection_dropdown(
    themes: &[Theme],
    selected_theme_index: usize,
) -> Div {
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
        .children(themes.iter().enumerate().map(|(index, theme)| {
            div()
                .id(("theme", index))
                .p_2()
                .hover(|style| style.bg(hsla(0., 0., 1., 0.1)))
                .when(index == selected_theme_index, |style| {
                    style.bg(hsla(0., 0., 1., 0.2))
                })
                .on_click(move |_, _, cx| {
                    cx.dispatch_action(&SelectTheme { theme_index: index });
                })
                .child(theme.name.clone())
        }))
}

pub fn render_interactive_ui(
    _view: &mut crate::AppView,
    cx: &mut Context<crate::AppView>,
) -> impl IntoElement {
    let app_state = cx.global::<crate::AppState>().clone();
    let active_theme = &app_state.active_theme;
    let surface_background = active_theme.0.get("surface.background").unwrap().hsla;
    let text_color = active_theme.0.get("text").unwrap().hsla;

    div()
        .id("interactive-ui")
        .flex()
        .flex_col()
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
                .child(div().text_xl().child("Theme Transition Simulator"))
                .child(
                    div()
                        .id("theme-selector")
                        .relative()
                        .child(
                            div()
                                .id("theme-selector-button")
                                .flex()
                                .items_center()
                                .gap_2()
                                .p_2()
                                .border_1()
                                .border_color(hsla(0., 0., 1., 0.2))
                                .rounded_md()
                                .hover(|style| style.bg(hsla(0., 0., 1., 0.1)))
                                .on_click(|_, _, cx| {
                                    cx.dispatch_action(&ToggleDropdown);
                                })
                                .child(app_state.themes[app_state.selected_theme_index].name.clone())
                                .child(div().child("▼")),
                        )
                        .when(app_state.dropdown_open, |d| {
                            d.child(render_theme_selection_dropdown(
                                &app_state.themes,
                                app_state.selected_theme_index,
                            ))
                        }),
                ),
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
                            div()
                                .id("run-simulation-button")
                                .p_2()
                                .border_1()
                                .border_color(hsla(0., 0., 1., 0.2))
                                .rounded_md()
                                .text_center()
                                .hover(|style| style.bg(hsla(0., 0., 1., 0.1)))
                                .on_click(cx.listener(|view, _, _, cx| {
                                    view.run_simulation(cx);
                                }))
                                .child("Run Simulation"),
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
