use chrono::Duration as ChronoDuration;
use gpui::{
    div, point, prelude::*, px, Action, App, AppContext, Application, AssetSource, Context, Entity,
    FocusHandle, Global, IntoElement, KeyBinding, Render, Result, ScrollHandle, SharedString,
    Window,
};
use rust_embed::RustEmbed;
use schemars::JsonSchema;
use serde::Deserialize;
use std::fs;

pub mod components;
pub mod scheduler;
pub mod simulation;
pub mod state;
pub mod text_input;
pub mod theme;
pub mod ui;

use crate::state::SimulationState;
use text_input::{
    Backspace, Copy, Cut, Delete, End, Home, Left, Paste, Right, SelectAll, SelectLeft,
    SelectRight, TextInput,
};
use theme::{flatten_colors, InterpolatableTheme, Theme, ZedThemeFile};

// --- 1. ACTIONS ---

#[derive(Clone, PartialEq, Action, Deserialize, JsonSchema)]
pub struct SetSleepDuration {
    pub seconds: f32,
}

#[derive(Clone, PartialEq, Action, Deserialize, JsonSchema)]
pub struct SelectNextTheme;

#[derive(Clone, PartialEq, Action, Deserialize, JsonSchema)]
pub struct SelectPrevTheme;

#[derive(Clone, PartialEq, Action, Deserialize, JsonSchema)]
pub struct ConfirmTheme;

#[derive(Clone, PartialEq, Action, Deserialize, JsonSchema)]
pub struct SetFadeDuration {
    pub seconds: f32,
}

#[derive(Clone, PartialEq, Action, Deserialize, JsonSchema)]
pub struct Submit;

#[derive(Clone, PartialEq, Action, Deserialize, JsonSchema)]
pub struct FocusNext;

#[derive(Clone, PartialEq, Action, Deserialize, JsonSchema)]
pub struct FocusPrev;

#[derive(Clone, PartialEq, Action, Deserialize, JsonSchema)]
pub struct CloseDropdowns;

#[derive(Clone, PartialEq, Action, Deserialize, JsonSchema)]
pub struct Cancel;

#[derive(Clone, PartialEq, Action, Deserialize, JsonSchema)]
pub struct Quit;

#[derive(Clone, PartialEq, Action)]
pub struct CloseWindow;

// New enum for application mode
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AppMode {
    Scheduler,
    Interactive,
}

impl Default for AppMode {
    fn default() -> Self {
        AppMode::Scheduler
    }
}

// New struct for dropdown-specific state
pub struct DropdownState {
    pub is_open: bool,
    pub preview_index: usize,
    pub scroll_handle: ScrollHandle,
    pub focus_handle: FocusHandle,
}

impl DropdownState {
    pub fn new(initial_index: usize, tab_index: usize, cx: &mut Context<AppView>) -> Self {
        Self {
            is_open: false,
            preview_index: initial_index,
            scroll_handle: ScrollHandle::new(),
            focus_handle: cx
                .focus_handle()
                .tab_index(tab_index as isize)
                .tab_stop(true),
        }
    }
}

// New struct for validated input state
pub struct ValidatedInputState {
    pub input: Entity<TextInput>,
    pub validation_message: Option<SharedString>,
}

// --- 3. GPUI GLOBAL STATE ---

/// This is the global `AppState` that our UI will read from and update.
#[derive(Clone)]
pub struct AppState {
    pub app_mode: AppMode,
    pub themes: Vec<Theme>,
    pub start_theme_index: usize,
    pub end_theme_index: usize,
    // The currently active theme, which the UI renders.
    pub active_theme: InterpolatableTheme,
}

impl Global for AppState {}

// --- 5. THE MAIN UI VIEW (REFACTORED) ---

pub struct AppView {
    pub start_dropdown_state: DropdownState,
    pub end_dropdown_state: DropdownState,
    pub sleep_input_state: ValidatedInputState,
    pub fade_input_state: ValidatedInputState,
    pub fps_input_state: ValidatedInputState,
    pub run_simulation_focus_handle: FocusHandle,
    pub root_focus_handle: FocusHandle,
    pub simulation_state: SimulationState,
    pub file_tree_scroll_handle: ScrollHandle,
    pub editor_content_scroll_handle: ScrollHandle,
}

impl AppView {
    pub fn new(
        cx: &mut Context<Self>,
        sleep_input: Entity<TextInput>,
        fade_input: Entity<TextInput>,
        fps_input: Entity<TextInput>,
    ) -> Self {
        let end_theme_index = cx.global::<AppState>().themes.len().saturating_sub(1);
        let root_focus_handle = cx.focus_handle();

        Self {
            start_dropdown_state: DropdownState::new(0, 1, cx),
            end_dropdown_state: DropdownState::new(end_theme_index, 2, cx),
            sleep_input_state: ValidatedInputState {
                input: sleep_input,
                validation_message: None,
            },
            fade_input_state: ValidatedInputState {
                input: fade_input,
                validation_message: None,
            },
            fps_input_state: ValidatedInputState {
                input: fps_input,
                validation_message: None,
            },
            run_simulation_focus_handle: cx.focus_handle().tab_index(6).tab_stop(true),
            root_focus_handle,
            simulation_state: SimulationState::Idle,
            file_tree_scroll_handle: ScrollHandle::new(),
            editor_content_scroll_handle: ScrollHandle::new(),
        }
    }

    fn on_focus_next(&mut self, _: &FocusNext, window: &mut Window, _: &mut Context<Self>) {
        window.focus_next();
    }

    fn on_focus_prev(&mut self, _: &FocusPrev, window: &mut Window, _: &mut Context<Self>) {
        window.focus_prev();
    }

    fn on_submit(&mut self, _: &Submit, _: &mut Window, cx: &mut Context<Self>) {
        self.run_simulation(cx);
    }

    pub fn focus_root(&mut self, window: &mut Window, _cx: &mut Context<Self>) {
        window.focus(&self.root_focus_handle);
    }

    pub fn toggle_start_dropdown(&mut self, cx: &mut Context<Self>) {
        self.start_dropdown_state.is_open = !self.start_dropdown_state.is_open;
        if self.start_dropdown_state.is_open {
            self.start_dropdown_state.preview_index = cx.global::<AppState>().start_theme_index;
            self.end_dropdown_state.is_open = false;
        }
    }

    pub fn toggle_end_dropdown(&mut self, cx: &mut Context<Self>) {
        self.end_dropdown_state.is_open = !self.end_dropdown_state.is_open;
        if self.end_dropdown_state.is_open {
            self.end_dropdown_state.preview_index = cx.global::<AppState>().end_theme_index;
            self.start_dropdown_state.is_open = false;
        }
    }

    pub fn close_dropdowns(&mut self, cx: &mut Context<Self>) {
        self.start_dropdown_state.is_open = false;
        self.end_dropdown_state.is_open = false;
        cx.notify();
    }

    pub fn select_start_theme(&mut self, index: usize, cx: &mut Context<Self>) {
        cx.update_global::<AppState, _>(|app_state, _| {
            if index != app_state.end_theme_index {
                app_state.start_theme_index = index;
                // Instantly update the active theme
                let theme = &app_state.themes[index].interpolatable_theme;
                app_state.active_theme = theme.clone();
            }
        });
        self.start_dropdown_state.is_open = false;
        cx.notify();
    }

    pub fn select_end_theme(&mut self, index: usize, cx: &mut Context<Self>) {
        cx.update_global::<AppState, _>(|app_state, _| {
            if index != app_state.start_theme_index {
                app_state.end_theme_index = index;
            }
        });
        self.end_dropdown_state.is_open = false;
        cx.notify();
    }

    fn scroll_dropdown_to_preview_index(window: &Window, dropdown_state: &mut DropdownState) {
        let rem_size = window.rem_size();
        let item_height = window.line_height() + rem_size;
        let scroll_handle = &dropdown_state.scroll_handle;
        let container_bounds = scroll_handle.bounds();
        if container_bounds.size.height > px(0.0) {
            let current_offset = scroll_handle.offset().y;
            let item_top = item_height * dropdown_state.preview_index as f32;
            let item_bottom = item_top + item_height;
            let visible_top = -current_offset;
            let visible_bottom = visible_top + container_bounds.size.height;

            let mut new_offset_y = current_offset;
            if item_top < visible_top {
                new_offset_y = -item_top;
            } else if item_bottom > visible_bottom {
                new_offset_y = -(item_bottom - container_bounds.size.height);
            }
            scroll_handle.set_offset(point(px(0.0), new_offset_y));
        }
    }

    pub fn select_next_theme(&mut self, window: &Window, cx: &mut Context<Self>) {
        let app_state = cx.global::<AppState>();
        let theme_count = app_state.themes.len();
        if theme_count < 2 {
            return;
        }

        let (dropdown_state, disabled_index) = if self.start_dropdown_state.is_open {
            (&mut self.start_dropdown_state, app_state.end_theme_index)
        } else if self.end_dropdown_state.is_open {
            (&mut self.end_dropdown_state, app_state.start_theme_index)
        } else {
            // If dropdowns were closed, check focus and open the correct one.
            if self.start_dropdown_state.focus_handle.is_focused(window) {
                self.toggle_start_dropdown(cx);
            } else if self.end_dropdown_state.focus_handle.is_focused(window) {
                self.toggle_end_dropdown(cx);
            }
            cx.notify();
            return;
        };

        let mut current_index = dropdown_state.preview_index;
        while current_index < theme_count - 1 {
            current_index += 1;
            if current_index != disabled_index {
                dropdown_state.preview_index = current_index;
                break;
            }
        }

        Self::scroll_dropdown_to_preview_index(window, dropdown_state);

        cx.notify();
    }

    pub fn select_prev_theme(&mut self, window: &Window, cx: &mut Context<Self>) {
        let app_state = cx.global::<AppState>();
        let theme_count = app_state.themes.len();
        if theme_count < 2 {
            return;
        }

        let (dropdown_state, disabled_index) = if self.start_dropdown_state.is_open {
            (&mut self.start_dropdown_state, app_state.end_theme_index)
        } else if self.end_dropdown_state.is_open {
            (&mut self.end_dropdown_state, app_state.start_theme_index)
        } else {
            // If dropdowns were closed, check focus and open the correct one.
            if self.start_dropdown_state.focus_handle.is_focused(window) {
                self.toggle_start_dropdown(cx);
            } else if self.end_dropdown_state.focus_handle.is_focused(window) {
                self.toggle_end_dropdown(cx);
            }
            cx.notify();
            return;
        };

        let mut current_index = dropdown_state.preview_index;
        while current_index > 0 {
            current_index -= 1;
            if current_index != disabled_index {
                dropdown_state.preview_index = current_index;
                break;
            }
        }

        Self::scroll_dropdown_to_preview_index(window, dropdown_state);

        cx.notify();
    }

    fn on_select_next_theme(
        &mut self,
        _: &SelectNextTheme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.stop_propagation();
        self.select_next_theme(window, cx);
    }

    fn on_select_prev_theme(
        &mut self,
        _: &SelectPrevTheme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.stop_propagation();
        self.select_prev_theme(window, cx);
    }

    pub fn confirm_theme(&mut self, window: &Window, cx: &mut Context<Self>) {
        if self.start_dropdown_state.is_open {
            self.select_start_theme(self.start_dropdown_state.preview_index, cx);
        } else if self.end_dropdown_state.is_open {
            self.select_end_theme(self.end_dropdown_state.preview_index, cx);
        } else if self.start_dropdown_state.focus_handle.is_focused(window) {
            self.toggle_start_dropdown(cx);
        } else if self.end_dropdown_state.focus_handle.is_focused(window) {
            self.toggle_end_dropdown(cx);
        }
    }

    fn on_confirm_theme(&mut self, _: &ConfirmTheme, window: &mut Window, cx: &mut Context<Self>) {
        cx.stop_propagation();
        self.confirm_theme(window, cx);
        cx.notify();
    }

    fn on_close_window(&mut self, _: &CloseWindow, window: &mut Window, _cx: &mut Context<Self>) {
        window.remove_window();
    }

    fn on_cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.stop_propagation();
        self.close_dropdowns(cx);
    }

    fn on_close_dropdowns(
        &mut self,
        _: &CloseDropdowns,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        println!("on_close_dropdowns called");
        self.close_dropdowns(cx);
    }
    fn run_simulation(&mut self, cx: &mut Context<Self>) {
        if self.simulation_state != SimulationState::Idle {
            return;
        }

        let sleep_content = self.sleep_input_state.input.read(cx).content.clone();
        let fade_content = self.fade_input_state.input.read(cx).content.clone();
        let fps_content = self.fps_input_state.input.read(cx).content.clone();

        // Perform validation.
        let sleep_seconds = sleep_content.parse::<f32>();
        let fade_seconds = fade_content.parse::<f32>();
        let target_fps = fps_content.parse::<u32>();

        self.sleep_input_state.validation_message = match sleep_seconds {
            Ok(s) if (0.0..=600.0).contains(&s) => None,
            _ => Some("Value must be between 0 and 600.".into()),
        };
        self.fade_input_state.validation_message = match fade_seconds {
            Ok(f) if (0.0..=600.0).contains(&f) => None,
            _ => Some("Value must be between 0 and 600.".into()),
        };
        self.fps_input_state.validation_message = match target_fps {
            Ok(f) if (1..=120).contains(&f) => None,
            _ => Some("Value must be between 1 and 120.".into()),
        };

        // Only run the simulation if all inputs are valid.
        if self.sleep_input_state.validation_message.is_none()
            && self.fade_input_state.validation_message.is_none()
            && self.fps_input_state.validation_message.is_none()
        {
            let (start_theme, end_theme) = cx.read_global(|app_state: &AppState, _| {
                (
                    app_state.themes[app_state.start_theme_index]
                        .interpolatable_theme
                        .clone(),
                    app_state.themes[app_state.end_theme_index]
                        .interpolatable_theme
                        .clone(),
                )
            });

            let sleep = sleep_seconds.unwrap();
            let fade = fade_seconds.unwrap();
            let fps = target_fps.unwrap();
            let sleep_duration = ChronoDuration::seconds(sleep as i64);
            let fade_duration = ChronoDuration::seconds(fade as i64);

            // Get theme names for logging
            let (start_theme_name, end_theme_name) = cx.read_global(|app_state: &AppState, _| {
                (
                    app_state.themes[app_state.start_theme_index].name.clone(),
                    app_state.themes[app_state.end_theme_index].name.clone(),
                )
            });

            simulation::run_simulation_core(
                cx,
                start_theme,
                end_theme,
                sleep_duration,
                fade_duration,
                fps,
                start_theme_name.into(),
                end_theme_name.into(),
            );
        }

        cx.notify();
    }

    fn render_interactive_ui(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        ui::render_interactive_ui(self, window, cx)
    }
}

impl Render for AppView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // println!("AppView::render called");
        let app_state = cx.global::<AppState>().clone();

        // Logic to close dropdowns if they lose focus
        if self.start_dropdown_state.is_open
            && !self
                .start_dropdown_state
                .focus_handle
                .contains_focused(window, cx)
        {
            self.close_dropdowns(cx);
        }
        if self.end_dropdown_state.is_open
            && !self
                .end_dropdown_state
                .focus_handle
                .contains_focused(window, cx)
        {
            self.close_dropdowns(cx);
        }

        let content = match app_state.app_mode {
            AppMode::Scheduler => div()
                .flex()
                .size_full()
                .justify_center()
                .items_center()
                .bg(app_state
                    .active_theme
                    .0
                    .get("surface.background")
                    .expect("Theme missing surface.background")
                    .hsla)
                .text_color(
                    app_state
                        .active_theme
                        .0
                        .get("text")
                        .expect("Theme missing text color")
                        .hsla,
                )
                .child(format!(
                    "Current Theme: {}",
                    app_state.themes[app_state.start_theme_index].name
                ))
                .into_any_element(),
            AppMode::Interactive => self.render_interactive_ui(window, cx).into_any_element(),
        };

        div()
            .size_full()
            .on_action(cx.listener(Self::on_close_window))
            .child(content)
    }
}

fn load_themes_from_dir(path: &std::path::Path) -> Vec<Theme> {
    if !path.exists() || !path.is_dir() {
        return Vec::new();
    }

    fs::read_dir(path)
        .expect("Failed to read directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() && path.extension()? == "json" {
                Some(path)
            } else {
                None
            }
        })
        .flat_map(|path| {
            match fs::read_to_string(&path) {
                Ok(json_data) => match serde_json::from_str::<ZedThemeFile>(&json_data) {
                    Ok(theme_file) => theme_file
                        .themes
                        .into_iter()
                        .map(|theme_def| {
                            let mut interpolatable_theme = InterpolatableTheme::default();
                            flatten_colors(&theme_def.style.colors, &mut interpolatable_theme, "");
                            Theme {
                                name: theme_def.name,
                                interpolatable_theme,
                            }
                        })
                        .collect::<Vec<Theme>>(),
                    Err(e) => {
                        eprintln!("Failed to parse theme file {:?}: {}", path, e);
                        Vec::new()
                    }
                },
                Err(e) => {
                    eprintln!("Failed to read theme file {:?}: {}", path, e);
                    Vec::new()
                }
            }
        })
        .collect()
}

#[derive(RustEmbed)]
#[folder = "assets"]
struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        Self::get(path)
            .map(|f| Ok(Some(f.data)))
            .unwrap_or(Ok(None))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| {
                if p.starts_with(path) {
                    Some(p.to_string().into())
                } else {
                    None
                }
            })
            .collect())
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // --- Parse themes ---
    let mut all_themes = load_themes_from_dir(std::path::Path::new("assets/"));

    if let Ok(home_dir) = std::env::var("HOME") {
        let mut config_path = std::path::PathBuf::from(home_dir);
        config_path.push(".config/zed/themes");
        if config_path.exists() {
            eprintln!("Loading themes from {:?}", config_path);
            let user_themes = load_themes_from_dir(&config_path);
            all_themes.extend(user_themes);
        }
    }

    // Sort themes alphabetically by name
    all_themes.sort_by_key(|theme| theme.name.clone());

    Application::new().with_assets(Assets).run(move |cx: &mut App| {
        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, Some("TextInput")),
            KeyBinding::new("delete", Delete, Some("TextInput")),
            KeyBinding::new("left", Left, Some("TextInput")),
            KeyBinding::new("right", Right, Some("TextInput")),
            KeyBinding::new("shift-left", SelectLeft, Some("TextInput")),
            KeyBinding::new("shift-right", SelectRight, Some("TextInput")),
            KeyBinding::new("cmd-a", SelectAll, Some("TextInput")),
            KeyBinding::new("home", Home, Some("TextInput")),
            KeyBinding::new("end", End, Some("TextInput")),
            KeyBinding::new("cmd-v", Paste, Some("TextInput")),
            KeyBinding::new("cmd-c", Copy, Some("TextInput")),
            KeyBinding::new("cmd-x", Cut, Some("TextInput")),
            KeyBinding::new("tab", FocusNext, Some("InteractiveUI")),
            KeyBinding::new("shift-tab", FocusPrev, Some("InteractiveUI")),
            KeyBinding::new("down", SelectNextTheme, Some("ThemeSelector")),
            KeyBinding::new("up", SelectPrevTheme, Some("ThemeSelector")),
            KeyBinding::new("enter", ConfirmTheme, Some("ThemeSelector")),
            KeyBinding::new("escape", Cancel, Some("ThemeSelector")),
            KeyBinding::new("cmd-enter", Submit, Some("InteractiveUI")),
            KeyBinding::new("enter", Submit, Some("RunButton")),
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-w", CloseWindow, None),
        ]);

        cx.on_action(|_: &Quit, cx| cx.quit());

        // --- Initialize AppState ---
        let app_mode = AppMode::Interactive; // Default to Interactive mode for now

        // Set the initial theme to the first one we loaded.
        let initial_active_theme = all_themes
            .first()
            .map(|theme| theme.interpolatable_theme.clone())
            .expect("Failed to get initial theme");

        let end_theme_index = if all_themes.len() > 1 { 1 } else { 0 };

        cx.set_global(AppState {
            app_mode,
            themes: all_themes,
            start_theme_index: 0, // Default to the first theme
            end_theme_index,      // Default to the second theme if available
            active_theme: initial_active_theme,
        });

        let sleep_duration_input = cx.new(|cx| TextInput {
            focus_handle: cx.focus_handle().tab_index(3).tab_stop(true),
            content: "10".into(),
            placeholder: "Sleep seconds...".into(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            is_selecting: false,
            cursor_visible: true,
            blink_interval: std::time::Duration::from_millis(500),
            blink_epoch: 0,
            is_blinking: false,
            was_focused: false,
        });
        let fade_duration_input = cx.new(|cx| TextInput {
            focus_handle: cx.focus_handle().tab_index(4).tab_stop(true),
            content: "10".into(),
            placeholder: "Fade seconds...".into(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            is_selecting: false,
            cursor_visible: true,
            blink_interval: std::time::Duration::from_millis(500),
            blink_epoch: 0,
            is_blinking: false,
            was_focused: false,
        });
        let fps_input = cx.new(|cx| TextInput {
            focus_handle: cx.focus_handle().tab_index(5).tab_stop(true),
            content: "60".into(),
            placeholder: "FPS...".into(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            is_selecting: false,
            cursor_visible: true,
            blink_interval: std::time::Duration::from_millis(500),
            blink_epoch: 0,
            is_blinking: false,
            was_focused: false,
        });

        // --- Open Window and Set Window-Specific Handlers ---
        let _ = cx
            .open_window(
                gpui::WindowOptions {
                    titlebar: Some(gpui::TitlebarOptions {
                        title: Some("Zed Theme Fader".into()),
                        ..Default::default()
                    }),
                    window_min_size: Some(gpui::size(gpui::px(800.0), gpui::px(500.0))),
                    ..Default::default()
                },
                |window, cx| {
                    let view =
                        cx.new(|cx| AppView::new(cx, sleep_duration_input, fade_duration_input, fps_input));
                    view.update(cx, |view, _cx| {
                        window.focus(&view.root_focus_handle);
                    });
                    view
                },
            )
            .unwrap();
    });
}
