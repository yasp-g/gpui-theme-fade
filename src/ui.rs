use crate::{AppView, AppState};
use gpui::{div, prelude::*, Context, IntoElement};

// This module will contain the interactive UI rendering logic.

pub fn render_interactive_ui(app_state: &AppState, cx: &mut Context<AppView>) -> impl IntoElement {
    div()
        .flex()
        .size_full()
        .justify_center()
        .items_center()
        .bg(app_state.active_theme.base.background.color)
        .text_color(app_state.active_theme.base.foreground.color)
        .child("Interactive Mode")
}