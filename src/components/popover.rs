use gpui::{deferred, div, AnyElement, IntoElement};

/// Renders a child element as a popover, which is drawn in an overlay
/// outside the normal layout flow.
///
/// This is useful for dropdown menus, tooltips, etc., that need to appear
/// on top of other UI elements.
pub fn render_popover(
    is_open: bool,
    child: impl IntoElement,
) -> AnyElement {
    if is_open {
        deferred(child).into_any_element()
    } else {
        div().into_any_element()
    }
}
