use gpui::{div, prelude::*, Context, IntoElement, px, Hsla};

use crate::AppView;
use crate::theme::Theme;

pub fn render_terminal(active_theme: &Theme, _cx: &mut Context<AppView>) -> impl IntoElement {
    let prompt_color = active_theme.terminal.bright_green;
    let command_color = active_theme.terminal.white;
    let output_color = active_theme.terminal.foreground;
    let cursor_color = active_theme.terminal.cyan; // A common color for terminal cursors

    div()
        .flex_1()
        .flex()
        .flex_col()
        .font_mono()
        .text_sm()
        .bg(active_theme.terminal.background)
        .text_color(output_color)
        .p_2() // Padding around the terminal content
        .child(
            div()
                .flex()
                .child(div().text_color(prompt_color).child("~ "))
                .child(div().text_color(command_color).child("ls -laF")),
        )
        .child(div().text_color(output_color).child("total 16"))
        .child(div().text_color(output_color).child("drwxr-xr-x   5 user group   160 Dec  4 10:00 ./"))
        .child(div().text_color(output_color).child("drwxr-xr-x  14 user group   448 Dec  3 16:30 ../"))
        .child(div().text_color(output_color).child("-rw-r--r--   1 user group  1024 Dec  4 10:00 file1.txt"))
        .child(div().text_color(output_color).child("drwxr-xr-x   2 user group    64 Dec  2 09:15 my_dir/"))
        .child(
            div()
                .flex()
                .child(div().text_color(prompt_color).child("~ "))
                .child(div().text_color(command_color).child("git status")),
        )
        .child(div().text_color(output_color).child("On branch main"))
        .child(div().text_color(output_color).child("Your branch is up to date with 'origin/main'."))
        .child(div().text_color(output_color).child("nothing to commit, working tree clean"))
        .child(
            div()
                .flex()
                .child(div().text_color(prompt_color).child("~ "))
                .child(div().text_color(command_color).child("")), // Empty command for cursor line
        )
        .child(
            // Simulate a blinking cursor
            div()
                .w_2p5() // Width of cursor, e.g., 2.5 pixels
                .h_3p5() // Height of cursor, adjust as needed
                .bg(cursor_color)
                .ml_0p5() // Margin left to simulate spacing
                .relative()
                .top_px(-px(2.0)) // Adjust vertically to align with text baseline
        )
