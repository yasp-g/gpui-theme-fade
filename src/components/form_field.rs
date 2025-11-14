use gpui::{div, rems, IntoElement, prelude::*, hsla, SharedString};

pub fn render_form_field(
    label: &'static str,
    validation_message: Option<SharedString>,
    child: impl IntoElement,
) -> impl IntoElement {
    let is_valid = validation_message.is_none();
    div()
        .flex()
        .w_full()
        .justify_between()
        .items_start()
        .child(div().child(label))
        .child(
            div()
                .flex()
                .flex_col()
                .w(rems(12.0))
                .gap_1()
                .items_end()
                .child(
                    div()
                        .w_full()
                        .border_1()
                        .border_color(if is_valid {
                            hsla(0., 0., 1., 0.2)
                        } else {
                            gpui::red()
                        })
                        .rounded_md()
                        .child(child)
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(gpui::red())
                        .min_h(rems(1.0)) // Reserve space for the message to prevent layout shifts
                        .children(validation_message)
                )
        )
}
