use gpui::{div, prelude::*, AppContext, EventEmitter, Entity, Render};

pub struct Popover {
    is_open: bool,
}

impl Popover {
    pub fn new<T: AppContext<Result<Entity<Self>> = Entity<Self>>>(cx: &mut T) -> Entity<Self> {
        cx.new(|_cx| Self { is_open: false })
    }
}

impl Render for Popover {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("popover-trigger")
            .on_click(cx.listener(|this, _, _, _cx_for_method| {
                this.is_open = !this.is_open;
                _cx_for_method.notify();
            }))
            .child("Click me")
            .when(self.is_open, |d| {
                // Later, this will use defer_draw to render the content in an overlay.
                d.child("I am open")
            })
    }
}

impl EventEmitter<DismissEvent> for Popover {}

#[derive(Clone, Debug, PartialEq)]
pub struct DismissEvent;
