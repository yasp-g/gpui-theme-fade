use crate::theme::InterpolatableTheme;
use gpui::{IntoElement, Rems, div, hsla, prelude::*, px};

#[derive(Clone, Copy, PartialEq)]
enum GitStatus {
    None,
    Modified,
    Created,
    Ignored,
}

#[derive(Clone, Copy, PartialEq)]
enum DiagnosticStatus {
    None,
    Error,
    Warning,
}

#[derive(Clone, Copy)]
struct FileItem {
    name: &'static str,
    indent: usize,
    icon: &'static str,
    is_folder: bool,
    is_folded: bool, // For "src/components" style
    is_open: bool,   // Only relevant for folders
    selected: bool,
    git_status: GitStatus,
    diagnostic_status: DiagnosticStatus,
}

fn get_example_files() -> Vec<FileItem> {
    vec![
        // Folded Directory Example
        FileItem {
            name: "crates/gpui",
            indent: 0,
            icon: "icons/folder_open.svg",
            is_folder: true,
            is_folded: true,
            is_open: true,
            selected: false,
            git_status: GitStatus::None,
            diagnostic_status: DiagnosticStatus::None,
        },
        FileItem {
            name: "src",
            indent: 1,
            icon: "icons/folder_open.svg",
            is_folder: true,
            is_folded: false,
            is_open: true,
            selected: false,
            git_status: GitStatus::None,
            diagnostic_status: DiagnosticStatus::None,
        },
        FileItem {
            name: "main.rs",
            indent: 2,
            icon: "icons/rust.svg",
            is_folder: false,
            is_folded: false,
            is_open: false,
            selected: true,
            git_status: GitStatus::Modified,
            diagnostic_status: DiagnosticStatus::None,
        },
        FileItem {
            name: "lib.rs",
            indent: 2,
            icon: "icons/rust.svg",
            is_folder: false,
            is_folded: false,
            is_open: false,
            selected: false,
            git_status: GitStatus::None,
            diagnostic_status: DiagnosticStatus::None,
        },
        // Diagnostic Example
        FileItem {
            name: "broken.rs",
            indent: 2,
            icon: "icons/rust.svg",
            is_folder: false,
            is_folded: false,
            is_open: false,
            selected: false,
            git_status: GitStatus::Modified,
            diagnostic_status: DiagnosticStatus::Error,
        },
        // New File Example
        FileItem {
            name: "feature.rs",
            indent: 2,
            icon: "icons/rust.svg",
            is_folder: false,
            is_folded: false,
            is_open: false,
            selected: false,
            git_status: GitStatus::Created,
            diagnostic_status: DiagnosticStatus::None,
        },
        FileItem {
            name: "components",
            indent: 2,
            icon: "icons/folder.svg",
            is_folder: true,
            is_folded: false,
            is_open: false,
            selected: false,
            git_status: GitStatus::None,
            diagnostic_status: DiagnosticStatus::None,
        },
        // Ignored File Example
        FileItem {
            name: ".gitignore",
            indent: 1,
            icon: "icons/file.svg",
            is_folder: false,
            is_folded: false,
            is_open: false,
            selected: false,
            git_status: GitStatus::Ignored,
            diagnostic_status: DiagnosticStatus::None,
        },
        FileItem {
            name: "Cargo.toml",
            indent: 1,
            icon: "icons/toml.svg",
            is_folder: false,
            is_folded: false,
            is_open: false,
            selected: false,
            git_status: GitStatus::None,
            diagnostic_status: DiagnosticStatus::None,
        },
    ]
}

pub fn render_file_tree(theme: &InterpolatableTheme) -> impl IntoElement {
    let bg_color = theme
        .0
        .get("sidebar.background")
        .or_else(|| theme.0.get("panel.background"))
        .map_or(hsla(0., 0., 0.15, 1.), |c| c.hsla);
    let text_color = theme
        .0
        .get("text")
        .map_or(hsla(0., 0., 0.8, 1.), |c| c.hsla);

    // This container simulates the "ProjectPanel"
    div()
        .w(Rems(12.0))
        .h_full()
        .flex()
        .flex_col()
        .bg(bg_color)
        .text_color(text_color)
        .text_sm()
        .pt_2() // Top padding only, no horizontal padding
        .child(
            // We simulate the UniformList by just stacking divs
            div().flex().flex_col().children(
                get_example_files()
                    .into_iter()
                    .map(|file| render_file_item(file, theme)),
            ),
        )
}

fn render_file_item(file: FileItem, theme: &InterpolatableTheme) -> impl IntoElement {
    // Theme Colors
    let bg_color = theme
        .0
        .get("sidebar.background")
        .or_else(|| theme.0.get("panel.background"))
        .map_or(hsla(0., 0., 0.15, 1.), |c| c.hsla);
    let selected_bg = theme
        .0
        .get("element.selected")
        .map_or(hsla(0., 0., 0.25, 1.), |c| c.hsla);
    let hover_bg = theme
        .0
        .get("list.hover_background")
        .or_else(|| theme.0.get("element.hover"))
        .or_else(|| theme.0.get("ghost_element.hover"))
        .map_or(hsla(0., 0., 0.2, 1.), |c| c.hsla);
    let default_text = theme
        .0
        .get("text")
        .map_or(hsla(0., 0., 0.8, 1.), |c| c.hsla);
    let icon_color = theme.0.get("icon").map_or(default_text, |c| c.hsla);
    let muted_color = theme
        .0
        .get("text.muted")
        .map_or(hsla(0., 0., 0.5, 1.), |c| c.hsla);

    let guide_color = theme
        .0
        .get("editor.wrap_guide")
        .map_or(hsla(0., 0., 0.5, 0.1), |c| c.hsla); // Revert to original subtle color

    // Git Status Colors
    let git_modified = theme
        .0
        .get("git.status.modified")
        .map_or(hsla(0.1, 0.6, 0.6, 1.), |c| c.hsla); // Yellow-ish
    let git_created = theme
        .0
        .get("git.status.created")
        .map_or(hsla(0.3, 0.6, 0.6, 1.), |c| c.hsla); // Green-ish
    let git_ignored = theme
        .0
        .get("git.status.ignored")
        .map_or(muted_color, |c| c.hsla);

    // Diagnostic Colors
    let error_color = theme
        .0
        .get("error")
        .map_or(hsla(0., 0.8, 0.5, 1.), |c| c.hsla);
    let warning_color = theme
        .0
        .get("warning")
        .map_or(hsla(0.1, 0.8, 0.5, 1.), |c| c.hsla);

    let filename_color = match file.git_status {
        GitStatus::None => default_text,
        GitStatus::Modified => git_modified,
        GitStatus::Created => git_created,
        GitStatus::Ignored => git_ignored,
    };

    // Layout Constants
    let indent_step = 20.0;
    let content_ml = px((file.indent as f32) * indent_step);
    let item_pl = px(6.0);
    // Additional offset to align guide relative to content start
    let guide_alignment_offset = px(6.0);

    div()
        .w_full()
        .relative()
        .h(Rems(1.5)) // Standard list item height
        .flex()
        .items_center()
        // Selection & Hover
        .when(file.selected, |s| s.bg(selected_bg))
        .when(!file.selected, |s| s.hover(|h| h.bg(hover_bg)))
        // Indent Guides
        .children((0..file.indent).map(|i| {
            div()
                .absolute()
                .top_0()
                .left(px(i as f32 * indent_step) + item_pl + guide_alignment_offset)
                .w(px(1.0))
                .h_full()
                .bg(guide_color)
        }))
        // Content
        .child(
            div()
                .w_full()
                .flex()
                .items_center()
                .ml(content_ml)
                .pl(item_pl)
                .pr_2()
                .gap_1()
                // File Icon
                .child(
                    div()
                        .relative()
                        .child(gpui::svg().path(file.icon).size(Rems(1.0)).text_color(
                            if file.git_status == GitStatus::Ignored {
                                git_ignored
                            } else {
                                icon_color
                            },
                        ))
                        // Diagnostic Decoration (Dot)
                        .when(file.diagnostic_status != DiagnosticStatus::None, |parent| {
                            let dot_color = match file.diagnostic_status {
                                DiagnosticStatus::Error => error_color,
                                DiagnosticStatus::Warning => warning_color,
                                _ => hsla(0., 0., 0., 0.),
                            };
                            parent.child(
                                div()
                                    .absolute()
                                    .bottom_0()
                                    .right_0()
                                    .size(px(6.0))
                                    .rounded_full()
                                    .bg(dot_color)
                                    .border_1()
                                    .border_color(bg_color), // Cutout effect
                            )
                        }),
                )
                // Filename
                .child(div().text_color(filename_color).child(file.name)),
        )
}
