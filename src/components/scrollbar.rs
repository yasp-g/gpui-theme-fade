use crate::AppView;
use gpui::{
    div, hsla, prelude::*, px, AnyElement, Bounds, Context, IntoElement, Pixels, Point,
    ScrollHandle, Size,
};

const SCROLLBAR_THUMB_WIDTH: Pixels = px(8.0);
const MIN_THUMB_HEIGHT: Pixels = px(24.0);

pub fn render_scrollbar(
    scroll_handle: &ScrollHandle,
    cx: &mut Context<AppView>,
) -> AnyElement {
    let scroll_track_bounds = scroll_handle.bounds();
    let content_size = scroll_handle.max_offset() + scroll_track_bounds.size;
    let scrollable_height = content_size.height;

    if scrollable_height <= scroll_track_bounds.size.height {
        return div().into_any(); // No scrollbar needed
    }

    let thumb_height =
        (scroll_track_bounds.size.height / scrollable_height) * scroll_track_bounds.size.height;
    let thumb_height = thumb_height.max(MIN_THUMB_HEIGHT);

    let scroll_top = scroll_handle.offset().y.abs();
    let scroll_ratio = scroll_top / (scrollable_height - scroll_track_bounds.size.height);

    let thumb_top =
        scroll_ratio * (scroll_track_bounds.size.height - thumb_height);

    div()
        .id("scrollbar-thumb")
        .absolute()
        .top(thumb_top)
        .right_0()
        .w(SCROLLBAR_THUMB_WIDTH)
        .h(thumb_height)
        .bg(hsla(0.0, 0.0, 0.5, 0.5))
        .rounded_md()
        .into_any()
}