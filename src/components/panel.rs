use crate::theme::InterpolatableTheme;
use gpui::{IntoElement, Rems, div, hsla, prelude::*};

pub fn render_panel(
    id: &'static str,
    gap: Rems,
    theme: &InterpolatableTheme,
    children: impl IntoIterator<Item = impl IntoElement>,
) -> impl IntoElement {
    let border_color = theme
        .0
        .get("border")
        .map_or(hsla(0., 0., 1., 0.2), |c| c.hsla);

    div()
        .id(id)
        .flex_1()
        .flex()
        .flex_col()
        .gap(gap)
        .p_4()
        .border_1()
        .border_color(border_color)
        .rounded_md()
        .children(children)
}
