use gpui::{div, hsla, prelude::*, Rems};
use crate::theme::InterpolatableTheme;

pub fn render_breadcrumbs(theme: &InterpolatableTheme) -> impl IntoElement {
    let bg_color = theme.0.get("editor.background").map_or(hsla(0., 0., 0.1, 1.), |c| c.hsla); // Same as editor usually
    let text_muted = theme.0.get("text.muted").map_or(hsla(0., 0., 0.5, 1.), |c| c.hsla);
    
    let get_color = |key: &str| {
        theme.0.get(key)
            .or_else(|| theme.0.get(&format!("{}.color", key)))
            .map_or(hsla(0., 0., 0.8, 1.), |c| c.hsla)
    };

    let keyword_color = get_color("syntax.keyword");
    let function_color = get_color("syntax.function");
    let punctuation_color = get_color("syntax.punctuation.bracket");

    div()
        .flex()
        .items_center()
        .h(Rems(1.5)) // 24px
        .px_4()
        .bg(bg_color)
        .text_sm()
        .font_family(".SystemUIFont") // Monospace if possible
        .child(div().text_color(text_muted).child("src/main.rs"))
        .child(div().text_color(text_muted).px_2().child(">"))
        .child(div().text_color(keyword_color).mr_1().child("fn"))
        .child(div().text_color(function_color).child("main"))
        .child(div().text_color(punctuation_color).child("()"))
}
