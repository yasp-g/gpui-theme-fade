use gpui::{
    IntoElement, ScrollHandle, div, prelude::*, ElementId, Div, Stateful
};

pub fn render_scrollable_container(
    id: impl Into<ElementId>,
    scroll_handle: &ScrollHandle,
    child: impl IntoElement,
) -> Stateful<Div> {
    div()
        .id(id)
        .track_scroll(scroll_handle)
        .overflow_y_scroll()
        .child(child)
}
