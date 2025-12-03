use gpui::{div, hsla, prelude::*, Rems};
use crate::theme::InterpolatableTheme;

// Data structure inspired by zed-themes rust.tsx
struct FileItem {
    name: &'static str,
    indent: usize,
    icon: &'static str, // Path to SVG
    selected: bool,
}

fn get_example_files() -> Vec<FileItem> {
    vec![
        FileItem { name: "zed", indent: 0, icon: "icons/folder_open.svg", selected: false },
        FileItem { name: "src", indent: 2, icon: "icons/folder.svg", selected: false },
        FileItem { name: "main.rs", indent: 4, icon: "icons/rust.svg", selected: true },
        FileItem { name: "lib.rs", indent: 4, icon: "icons/rust.svg", selected: false },
        FileItem { name: "modules", indent: 4, icon: "icons/folder.svg", selected: false },
        FileItem { name: "user_service.rs", indent: 6, icon: "icons/rust.svg", selected: false },
        FileItem { name: "Cargo.toml", indent: 2, icon: "icons/toml.svg", selected: false },
        FileItem { name: ".gitignore", indent: 2, icon: "icons/file.svg", selected: false },
        FileItem { name: "README.md", indent: 2, icon: "icons/file.svg", selected: false },
    ]
}

pub fn render_file_tree(theme: &InterpolatableTheme) -> impl IntoElement {
    let bg_color = theme.0.get("sidebar.background").map_or(hsla(0., 0., 0.15, 1.), |c| c.hsla);
    let text_color = theme.0.get("text").map_or(hsla(0., 0., 0.8, 1.), |c| c.hsla);
    let selected_bg = theme.0.get("element.selected").map_or(hsla(0., 0., 0.25, 1.), |c| c.hsla);
    let selected_text = theme.0.get("text.selected").map_or(text_color, |c| c.hsla);
    let icon_color = theme.0.get("icon").map_or(text_color, |c| c.hsla);

    div()
        .w(Rems(15.0)) // 240px width
        .h_full()
        .flex()
        .flex_col()
        .bg(bg_color)
        .text_color(text_color)
        .text_sm()
        .p_2()
        .children(get_example_files().into_iter().map(|file| {
            div()
                .flex()
                .items_center()
                .w_full()
                .py_1()
                .gap_1()
                .when(file.selected, |s| s.bg(selected_bg).text_color(selected_text))
                .child(
                    div()
                        .w(Rems(file.indent as f32 * 0.5)) // Indent size
                )
                .child(
                    gpui::svg()
                        .path(file.icon)
                        .size(Rems(1.0))
                        .text_color(if file.selected { selected_text } else { icon_color })
                )
                .child(div().child(file.name))
        }))
}