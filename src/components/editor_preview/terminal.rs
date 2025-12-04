use gpui::{div, prelude::*, Context, IntoElement};

use crate::AppView;
use crate::theme::Theme;

pub fn render_terminal(active_theme: &Theme, _cx: &mut Context<AppView>) -> impl IntoElement {
    div()
        .flex_1()
        .bg(active_theme.terminal.background)
        .text_color(active_theme.terminal.foreground)
        .child("Terminal Content Here") // Placeholder content
        .p_4()
}
