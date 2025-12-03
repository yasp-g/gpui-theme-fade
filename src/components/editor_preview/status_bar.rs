use crate::theme::InterpolatableTheme;
use gpui::{Rems, SharedString, div, hsla, prelude::*};

pub fn render_status_bar(theme: &InterpolatableTheme) -> impl IntoElement {
    let bg_color = theme
        .0
        .get("status_bar.background")
        .map_or(hsla(0., 0., 0.15, 1.), |c| c.hsla);
    let text_color = theme
        .0
        .get("status_bar.foreground")
        .map_or(hsla(0., 0., 0.6, 1.), |c| c.hsla);
    let icon_muted = theme.0.get("icon.muted").map_or(text_color, |c| c.hsla);

    div()
        .flex()
        .items_center()
        .justify_between()
        .h(Rems(1.75))
        .px_2()
        .bg(bg_color)
        .text_color(text_color)
        .text_xs()
        .child(
            // Left Tools
            div()
                .flex()
                .gap_1()
                .items_center()
                .child(render_icon_button("icons/file_tree.svg", true, theme))
                .child(render_icon_button("icons/git_branch.svg", false, theme))
                .child(render_icon_button("icons/list_tree.svg", false, theme))
                .child(render_icon_button("icons/user_group.svg", false, theme))
                .child(render_icon_button(
                    "icons/magnifying_glass.svg",
                    false,
                    theme,
                ))
                .child(div().w_px().h_3().bg(icon_muted).opacity(0.2).mx_1())
                .child(render_icon_button("icons/library.svg", false, theme))
                .child(render_icon_button("icons/check.svg", false, theme)),
        )
        .child(
            // Right Tools
            div()
                .flex()
                .gap_1()
                .items_center()
                .child(render_text_button("11:5", false, theme))
                .child(render_text_button("Rust", false, theme))
                .child(div().w_px().h_3().bg(icon_muted).opacity(0.2).mx_1())
                .child(render_icon_button("icons/terminal.svg", false, theme))
                .child(render_icon_button("icons/debug.svg", false, theme))
                .child(render_icon_button("icons/ai.svg", false, theme))
                .child(render_icon_button("icons/bell.svg", false, theme)),
        )
}

fn render_button_container(
    active: bool,
    theme: &InterpolatableTheme,
    child: impl IntoElement,
) -> impl IntoElement {
    let default_bg = theme
        .0
        .get("ghost_element.background")
        .map_or(hsla(0., 0., 0., 0.), |c| c.hsla);
    let active_bg = theme
        .0
        .get("ghost_element.selected")
        .map_or(hsla(0., 0., 0.2, 1.), |c| c.hsla);
    let hover_bg = theme
        .0
        .get("ghost_element.hover")
        .map_or(hsla(0., 0., 0.2, 1.), |c| c.hsla);

    div()
        .p_1()
        .rounded_md()
        .cursor_pointer()
        .bg(if active { active_bg } else { default_bg })
        .hover(|s| s.bg(hover_bg))
        .child(child)
}

fn render_icon_button(
    path: &'static str,
    active: bool,
    theme: &InterpolatableTheme,
) -> impl IntoElement {
    let icon_color = theme
        .0
        .get("icon")
        .map_or(hsla(0., 0., 0.8, 1.), |c| c.hsla);
    let icon_muted = theme
        .0
        .get("icon.muted")
        .map_or(hsla(0., 0., 0.5, 1.), |c| c.hsla);
    let path: SharedString = path.into();

    render_button_container(
        active,
        theme,
        gpui::svg()
            .path(path)
            .size(Rems(1.0))
            .text_color(if active { icon_color } else { icon_muted }),
    )
}

fn render_text_button(text: &str, active: bool, theme: &InterpolatableTheme) -> impl IntoElement {
    let text_color = theme.0.get("text").map_or(hsla(0., 0., 0.8, 1.), |c| c.hsla);
    let text_muted = theme.0.get("text.muted").map_or(hsla(0., 0., 0.5, 1.), |c| c.hsla);

    render_button_container(active, theme,
        div()
            .text_xs()
            .line_height(Rems(1.0)) // Force line height to match icon size
            .text_color(if active { text_color } else { text_muted })
            .child(text.to_string())
    )
}
