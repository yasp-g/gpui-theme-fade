use crate::{AppState, AppView};
use gpui::{div, prelude::*, Context, IntoElement};

// This module will contain the interactive UI rendering logic.

pub fn render_interactive_ui(app_state: &AppState, _cx: &mut Context<AppView>) -> impl IntoElement {
    let bg_color = app_state.active_theme.0.get("surface.background").cloned().unwrap_or_default();
    let text_color = app_state.active_theme.0.get("text").cloned().unwrap_or_default();

    div()
        .flex()
        .size_full()
        .justify_center()
        .items_center()
        .bg(bg_color.rgba)
        .text_color(text_color.rgba)
        .child("Interactive Mode")
}