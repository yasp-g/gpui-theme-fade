use gpui::{div, Hsla, IntoElement, linear_gradient, linear_color_stop, prelude::*};

pub fn render_gradient_bar(
    start_color: Hsla,
    end_color: Hsla,
) -> impl IntoElement {
    div()
        .h_4() // Give the bar some height
        .rounded_md() // Make it look nice
        .bg(linear_gradient(
            // Angle: 90 degrees creates a left-to-right horizontal gradient.
            90.,
            // Start Stop: The first color at the very beginning (0%).
            linear_color_stop(start_color, 0.0),
            // End Stop: The second color at the very end (100%).
            linear_color_stop(end_color, 1.0),
        ))
}
