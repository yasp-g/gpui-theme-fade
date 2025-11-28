use crate::theme::InterpolatableTheme;
use gpui::{div, rems, IntoElement, prelude::*, hsla, SharedString};

pub fn render_form_field(
    label: &'static str,
    validation_message: Option<SharedString>,
    child: impl IntoElement,
    disabled: bool,
    is_focused: bool,
    theme: &InterpolatableTheme,
) -> impl IntoElement {
    let is_valid = validation_message.is_none();
    let border_color = theme
        .0
        .get("border")
        .map_or(hsla(0., 0., 1., 0.2), |c| c.hsla);
    let focus_color = theme
        .0
        .get("border.focused")
        .map_or(gpui::blue(), |c| c.hsla);
    let error_color = theme.0.get("error").map_or(gpui::red(), |c| c.hsla);

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
                        .relative() // Needed for absolute overlay
                        .when(is_focused, |s| s.border_2().border_color(focus_color))
                        .when(!is_focused, |s| s.border_2().border_color(if is_valid {
                            border_color
                        } else {
                            error_color
                        }))
                        .rounded_md()
                        .child(child)
                        .when(disabled, |s| {
                            s.opacity(0.5)
                                .child(
                                    div()
                                        .absolute()
                                        .size_full()
                                        .top_0()
                                        .left_0()
                                        .cursor(gpui::CursorStyle::OperationNotAllowed)
                                        .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| {
                                            cx.stop_propagation()
                                        }),
                                )
                        }),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(error_color)
                        .min_h(rems(1.0)) // Reserve space for the message to prevent layout shifts
                        .children(validation_message)
                )
        )
}
