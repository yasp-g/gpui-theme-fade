use gpui::{IntoElement, Rems, div, hsla, prelude::*};

pub fn render_panel(
    id: &'static str,
    gap: Rems,
    children: impl IntoIterator<Item = impl IntoElement>,
) -> impl IntoElement {
    div()
        .id(id)
        .flex_1()
        .flex()
        .flex_col()
        .gap(gap)
        .p_4()
        .border_1()
        .border_color(hsla(0., 0., 1., 0.2))
        .rounded_md()
        .children(children)
}
