use crate::AppView;
use crate::theme::InterpolatableTheme;
use gpui::{Context, IntoElement, Rems, div, hsla, prelude::*};

pub fn render_terminal(
    theme: &InterpolatableTheme,
    _cx: &mut Context<AppView>,
) -> impl IntoElement {
    let border_color = theme
        .0
        .get("border")
        .map_or(hsla(0., 0., 0., 1.), |c| c.hsla);

    div()
        .h(gpui::px(200.0)) // Reverted to default height
        .flex()
        .flex_col()
        .border_t_1()
        .border_color(border_color)
        .child(render_terminal_tabs(theme))
        .child(render_terminal_shell(theme))
}

fn render_terminal_tabs(theme: &InterpolatableTheme) -> impl IntoElement {
    let tab_bar_bg = theme
        .0
        .get("tab_bar.background")
        .map_or(hsla(0., 0., 0.1, 1.), |c| c.hsla);
    let tab_active_bg = theme
        .0
        .get("tab.active_background")
        .map_or(hsla(0., 0., 0.2, 1.), |c| c.hsla);
    let tab_inactive_bg = theme
        .0
        .get("tab.inactive_background")
        .map_or(hsla(0., 0., 0.15, 1.), |c| c.hsla);
    let text_color = theme
        .0
        .get("text")
        .map_or(hsla(0., 0., 0.9, 1.), |c| c.hsla);
    let text_muted = theme
        .0
        .get("text.muted")
        .map_or(hsla(0., 0., 0.5, 1.), |c| c.hsla);
    let border_color = theme
        .0
        .get("border")
        .map_or(hsla(0., 0., 0., 1.), |c| c.hsla);

    div()
        .flex()
        .bg(tab_bar_bg)
        .child(
            // Active Tab
            div()
                .flex()
                .items_center()
                .gap_2()
                .px_4()
                .h(Rems(2.0)) // Matches editor tab height
                .bg(tab_active_bg)
                .child(
                    gpui::svg()
                        .path("icons/terminal.svg")
                        .size(Rems(0.8))
                        .text_color(text_color),
                )
                .child(div().text_sm().text_color(text_color).child("zed -- zsh")),
        )
        .child(
            // Inactive Tab
            div()
                .flex()
                .items_center()
                .gap_2()
                .px_4()
                .h(Rems(2.0))
                .bg(tab_inactive_bg)
                .border_r_1()
                .border_color(border_color)
                .child(
                    gpui::svg()
                        .path("icons/terminal.svg")
                        .size(Rems(0.8))
                        .text_color(text_muted),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(text_muted)
                        .child("node -- watch"),
                ),
        )
        .child(
            // Filler
            div().flex_1().border_b_1().border_color(border_color),
        )
}

fn render_terminal_shell(theme: &InterpolatableTheme) -> impl IntoElement {
    let bg_color = theme
        .0
        .get("terminal.background")
        .map_or(hsla(0., 0., 0., 1.), |c| c.hsla);
    let fg_color = theme
        .0
        .get("terminal.foreground")
        .map_or(hsla(0., 0., 1., 1.), |c| c.hsla);
    let prompt_color = theme
        .0
        .get("terminal.ansi.green")
        .map_or(hsla(0.3, 0.8, 0.5, 1.), |c| c.hsla); // Default green-ish
    let command_color = theme
        .0
        .get("terminal.ansi.white")
        .map_or(hsla(0., 0., 1., 1.), |c| c.hsla);
    let blue_color = theme
        .0
        .get("terminal.ansi.blue")
        .map_or(hsla(0.6, 0.8, 0.5, 1.), |c| c.hsla);
    let cursor_color = theme
        .0
        .get("terminal.ansi.cyan")
        .map_or(hsla(0.5, 1., 0.5, 1.), |c| c.hsla);

    div()
        .flex_1()
        .flex()
        .flex_col()
        .font_family("Menlo")
        .text_sm()
        .bg(bg_color)
        .text_color(fg_color)
        .p_2()
        .child(
            div()
                .flex()
                .gap_1()
                .child(div().text_color(prompt_color).child("zed ➤"))
                .child(div().text_color(command_color).child("ls -laF")),
        )
        .child(div().text_color(fg_color).child("total 16"))
        .child(
            div()
                .flex()
                .gap_2()
                .child(div().text_color(blue_color).child("drwxr-xr-x"))
                .child(
                    div()
                        .text_color(fg_color)
                        .child("5 user group   160 Dec  4 10:00 ./ "),
                ),
        )
        .child(
            div()
                .flex()
                .gap_2()
                .child(div().text_color(blue_color).child("drwxr-xr-x"))
                .child(
                    div()
                        .text_color(fg_color)
                        .child("14 user group   448 Dec  3 16:30 ../"),
                ),
        )
        .child(
            div()
                .text_color(fg_color)
                .child("-rw-r--r--   1 user group  1024 Dec  4 10:00 file1.txt"),
        )
        .child(
            div()
                .flex()
                .gap_1()
                .child(div().text_color(prompt_color).child("zed ➤"))
                .child(div().text_color(command_color).child("git status")),
        )
        .child(div().text_color(fg_color).child("On branch main"))
        .child(
            div()
                .text_color(fg_color)
                .child("Your branch is up to date with 'origin/main'."),
        )
        .child(
            div()
                .text_color(fg_color)
                .child("nothing to commit, working tree clean"),
        )
        .child(
            div()
                .flex()
                .gap_1()
                .child(div().text_color(prompt_color).child("zed ➤"))
                .child(div().text_color(command_color).child("")) // Empty command
                .child(
                    // Cursor
                    div().w_2p5().h(Rems(1.0)).bg(cursor_color).opacity(0.7),
                ),
        )
}
