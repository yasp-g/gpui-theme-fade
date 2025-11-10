use chrono::{Duration as ChronoDuration, Local};
use futures::{StreamExt, channel::mpsc};
use gpui::{
    point, px, Action, App, AppContext, Application, AsyncApp, Context, Entity, FocusHandle,
    Global, IntoElement, KeyBinding, Render, ScrollHandle, SharedString, Window, div, prelude::*,
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::{fs, sync::Arc};
use tracing::info;

pub mod scheduler;
pub mod text_input;
pub mod ui;
pub mod components;
pub mod theme;

use scheduler::{ScheduleEntry, ThemeScheduler};
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

#[derive(Clone, PartialEq, Action)]
pub struct Cancel;

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

// --- 3. GPUI GLOBAL STATE ---

/// This is the global `AppState` that our UI will read from and update.
#[derive(Clone)]
pub struct AppState {
    pub app_mode: AppMode,
    pub themes: Vec<Theme>,
    pub start_theme_index: usize,
    pub end_theme_index: usize,
    pub start_preview_index: usize,
    pub end_preview_index: usize,
    pub sleep_duration_input: Entity<TextInput>,
    pub fade_duration_input: Entity<TextInput>,
    pub theme_selector_focus_handle: FocusHandle,
    pub end_theme_selector_focus_handle: FocusHandle,
    pub start_theme_scroll_handle: ScrollHandle,
    pub end_theme_scroll_handle: ScrollHandle,
    pub run_simulation_focus_handle: FocusHandle,
    pub sleep_input_is_valid: bool,
    pub fade_input_is_valid: bool,
    pub start_dropdown_open: bool,
    pub end_dropdown_open: bool,
    // The currently active theme, which the UI renders.
    pub active_theme: InterpolatableTheme,
}

impl Global for AppState {}

// --- 5. THE MAIN UI VIEW (REFACTORED) ---

pub struct AppView;

impl AppView {
    fn on_focus_next(&mut self, _: &FocusNext, window: &mut Window, _: &mut Context<Self>) {
        window.focus_next();
    }

    fn on_focus_prev(&mut self, _: &FocusPrev, window: &mut Window, _: &mut Context<Self>) {
        window.focus_prev();
    }

    fn on_submit(&mut self, _: &Submit, _: &mut Window, cx: &mut Context<Self>) {
        self.run_simulation(cx);
    }

    pub fn toggle_start_dropdown(&mut self, cx: &mut Context<Self>) {
        cx.update_global::<AppState, _>(|app_state, _| {
            app_state.start_dropdown_open = !app_state.start_dropdown_open;
            if app_state.start_dropdown_open {
                app_state.start_preview_index = app_state.start_theme_index;
                app_state.end_dropdown_open = false;
            }
        });
    }

    pub fn toggle_end_dropdown(&mut self, cx: &mut Context<Self>) {
        cx.update_global::<AppState, _>(|app_state, _| {
            app_state.end_dropdown_open = !app_state.end_dropdown_open;
            if app_state.end_dropdown_open {
                app_state.end_preview_index = app_state.end_theme_index;
                app_state.start_dropdown_open = false;
            }
        });
    }

    pub fn close_dropdowns(&mut self, cx: &mut Context<Self>) {
        cx.update_global::<AppState, _>(|app_state, _| {
            app_state.start_dropdown_open = false;
            app_state.end_dropdown_open = false;
        });
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
            // Always close dropdown on selection attempt
            app_state.start_dropdown_open = false;
        });
    }

    pub fn select_end_theme(&mut self, index: usize, cx: &mut Context<Self>) {
        cx.update_global::<AppState, _>(|app_state, _| {
            if index != app_state.start_theme_index {
                app_state.end_theme_index = index;
            }
            // Always close dropdown on selection attempt
            app_state.end_dropdown_open = false;
        });
    }

    pub fn select_next_theme(&mut self, window: &Window, cx: &mut Context<Self>) {
        cx.update_global::<AppState, _>(|app_state, _| {
            let theme_count = app_state.themes.len();
            if theme_count < 2 {
                return;
            }

            if app_state.start_dropdown_open {
                let disabled_index = app_state.end_theme_index;
                let mut current_index = app_state.start_preview_index;
                while current_index < theme_count - 1 {
                    current_index += 1;
                    if current_index != disabled_index {
                        app_state.start_preview_index = current_index;
                        return; // Found it, update and exit
                    }
                }
            } else if app_state.end_dropdown_open {
                let disabled_index = app_state.start_theme_index;
                let mut current_index = app_state.end_preview_index;
                while current_index < theme_count - 1 {
                    current_index += 1;
                    if current_index != disabled_index {
                        app_state.end_preview_index = current_index;
                        return; // Found it, update and exit
                    }
                }
            }
        });

        let app_state = cx.global::<AppState>();

        // If dropdowns were closed, check focus and open the correct one.
        if !app_state.start_dropdown_open && !app_state.end_dropdown_open {
            if app_state.theme_selector_focus_handle.is_focused(window) {
                self.toggle_start_dropdown(cx);
            } else if app_state.end_theme_selector_focus_handle.is_focused(window) {
                self.toggle_end_dropdown(cx);
            }
        } else {
            // --- Manual Scroll Calculation ---
            let rem_size = window.rem_size();
            // An item has p-2, which is 0.5rem top and 0.5rem bottom padding.
            let item_height = window.line_height() + rem_size;

            if app_state.start_dropdown_open {
                let scroll_handle = &app_state.start_theme_scroll_handle;
                let container_bounds = scroll_handle.bounds();
                if container_bounds.size.height > px(0.0) {
                    let current_offset = scroll_handle.offset().y;
                    let item_top = item_height * app_state.start_preview_index as f32;
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
            } else if app_state.end_dropdown_open {
                let scroll_handle = &app_state.end_theme_scroll_handle;
                let container_bounds = scroll_handle.bounds();
                if container_bounds.size.height > px(0.0) {
                    let current_offset = scroll_handle.offset().y;
                    let item_top = item_height * app_state.end_preview_index as f32;
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
        }

        cx.notify();
    }

    pub fn select_prev_theme(&mut self, window: &Window, cx: &mut Context<Self>) {
        cx.update_global::<AppState, _>(|app_state, _| {
            if app_state.themes.len() < 2 {
                return;
            }

            if app_state.start_dropdown_open {
                let disabled_index = app_state.end_theme_index;
                let mut current_index = app_state.start_preview_index;
                while current_index > 0 {
                    current_index -= 1;
                    if current_index != disabled_index {
                        app_state.start_preview_index = current_index;
                        return; // Found it, update and exit
                    }
                }
            } else if app_state.end_dropdown_open {
                let disabled_index = app_state.start_theme_index;
                let mut current_index = app_state.end_preview_index;
                while current_index > 0 {
                    current_index -= 1;
                    if current_index != disabled_index {
                        app_state.end_preview_index = current_index;
                        return; // Found it, update and exit
                    }
                }
            }
        });

        let app_state = cx.global::<AppState>();

        // If dropdowns were closed, check focus and open the correct one.
        if !app_state.start_dropdown_open && !app_state.end_dropdown_open {
            if app_state.theme_selector_focus_handle.is_focused(window) {
                self.toggle_start_dropdown(cx);
            } else if app_state.end_theme_selector_focus_handle.is_focused(window) {
                self.toggle_end_dropdown(cx);
            }
        } else {
            // --- Manual Scroll Calculation ---
            let rem_size = window.rem_size();
            // An item has p-2, which is 0.5rem top and 0.5rem bottom padding.
            let item_height = window.line_height() + rem_size;

            if app_state.start_dropdown_open {
                let scroll_handle = &app_state.start_theme_scroll_handle;
                let container_bounds = scroll_handle.bounds();
                if container_bounds.size.height > px(0.0) {
                    let current_offset = scroll_handle.offset().y;
                    let item_top = item_height * app_state.start_preview_index as f32;
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
            } else if app_state.end_dropdown_open {
                let scroll_handle = &app_state.end_theme_scroll_handle;
                let container_bounds = scroll_handle.bounds();
                if container_bounds.size.height > px(0.0) {
                    let current_offset = scroll_handle.offset().y;
                    let item_top = item_height * app_state.end_preview_index as f32;
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
        }

        cx.notify();
    }

    fn on_select_next_theme(
        &mut self,
        _: &SelectNextTheme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_next_theme(window, cx);
    }

    fn on_select_prev_theme(
        &mut self,
        _: &SelectPrevTheme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_prev_theme(window, cx);
    }

    pub fn confirm_theme(&mut self, window: &Window, cx: &mut Context<Self>) {
        let app_state = cx.global::<AppState>();
        if app_state.start_dropdown_open {
            self.select_start_theme(app_state.start_preview_index, cx);
        } else if app_state.end_dropdown_open {
            self.select_end_theme(app_state.end_preview_index, cx);
        } else if app_state.theme_selector_focus_handle.is_focused(window) {
            self.toggle_start_dropdown(cx);
        } else if app_state.end_theme_selector_focus_handle.is_focused(window) {
            self.toggle_end_dropdown(cx);
        }
    }

    fn on_confirm_theme(&mut self, _: &ConfirmTheme, window: &mut Window, cx: &mut Context<Self>) {
        self.confirm_theme(window, cx);
        cx.notify();
    }

    fn on_cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        self.close_dropdowns(cx);
    }

    fn run_simulation(&mut self, cx: &mut Context<Self>) {
        // First, get the necessary state from the global state.
        let (sleep_input_handle, fade_input_handle, start_theme, end_theme) = cx.read_global(
            |app_state: &AppState, _|
                (
                    app_state.sleep_duration_input.clone(),
                    app_state.fade_duration_input.clone(),
                    app_state.themes[app_state.start_theme_index]
                        .interpolatable_theme
                        .clone(),
                    app_state.themes[app_state.end_theme_index]
                        .interpolatable_theme
                        .clone(),
                ),
        );

        // Now, use the window context `cx` to read the entity state.
        let sleep_content = sleep_input_handle.read(cx).content.clone();
        let fade_content = fade_input_handle.read(cx).content.clone();

        // Perform validation.
        let sleep_seconds = sleep_content.parse::<f32>();
        let fade_seconds = fade_content.parse::<f32>();

        let sleep_is_valid = sleep_seconds.is_ok();
        let fade_is_valid = fade_seconds.is_ok();

        // Update the validity flags in AppState.
        cx.update_global(|app_state: &mut AppState, _| {
            app_state.sleep_input_is_valid = sleep_is_valid;
            app_state.fade_input_is_valid = fade_is_valid;
        });

        // Only run the simulation if both inputs are valid.
        if let (Ok(sleep), Ok(fade)) = (sleep_seconds, fade_seconds) {
            let sleep_duration = ChronoDuration::seconds(sleep as i64);
            let fade_duration = ChronoDuration::seconds(fade as i64);

            // Get theme names for logging
            let (start_theme_name, end_theme_name) = cx.read_global(
                |app_state: &AppState, _| {
                    (
                        app_state.themes[app_state.start_theme_index].name.clone(),
                        app_state.themes[app_state.end_theme_index].name.clone(),
                    )
                },
            );

            info!(
                "Running simulation: Start='{}', End='{}'",
                start_theme_name, end_theme_name
            );

            cx.spawn(move |_, async_cx: &mut AsyncApp| {
                let async_cx = async_cx.clone();
                async move {
                    let (theme_sender, mut theme_receiver) = mpsc::channel(32);

                    let now = Local::now().time();
                    let sim_schedule = Arc::new(vec![
                        ScheduleEntry {
                            time: now,
                            theme: start_theme.clone(),
                            fade_duration: ChronoDuration::seconds(0),
                        },
                        ScheduleEntry {
                            time: now + sleep_duration + fade_duration,
                            theme: end_theme.clone(),
                            fade_duration,
                        },
                    ]);

                    ThemeScheduler::spawn(theme_sender.clone(), sim_schedule, AppMode::Interactive);

                    while let Some(theme) = theme_receiver.next().await {
                        async_cx
                            .update(|cx| {
                                cx.update_global::<AppState, _>(|app_state, _| {
                                    app_state.active_theme = theme.clone();
                                });
                            })
                            .ok();
                    }
                    info!("Simulation finished and channel closed.");
                }
            })
            .detach();
        }
    }

    fn render_interactive_ui(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        ui::render_interactive_ui(self, cx)
    }
}

impl Render for AppView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let app_state = cx.global::<AppState>().clone();

        // Logic to close dropdowns if they lose focus
        if app_state.start_dropdown_open
            && !app_state
                .theme_selector_focus_handle
                .contains_focused(window, cx)
        {
            self.close_dropdowns(cx);
        }
        if app_state.end_dropdown_open
            && !app_state
                .end_theme_selector_focus_handle
                .contains_focused(window, cx)
        {
            self.close_dropdowns(cx);
        }

        match app_state.app_mode {
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
            AppMode::Interactive => self.render_interactive_ui(cx).into_any_element(),
        }
    }
}

fn create_duration_input(
    cx: &mut App,
    content: impl Into<SharedString>,
    placeholder: impl Into<SharedString>,
    tab_index: usize,
) -> Entity<TextInput> {
    cx.new(|cx| TextInput {
        focus_handle: cx
            .focus_handle()
            .tab_index(tab_index as isize)
            .tab_stop(true),
        content: content.into(),
        placeholder: placeholder.into(),
        selected_range: 0..0,
        selection_reversed: false,
        marked_range: None,
        last_layout: None,
        last_bounds: None,
        is_selecting: false,
    })
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // --- Parse our mock themes ---
    let all_themes: Vec<Theme> = fs::read_dir("assets/")
        .expect("Failed to read assets directory")
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
            let json_data = fs::read_to_string(&path).expect("Failed to read theme file");
            let theme_file: ZedThemeFile =
                serde_json::from_str(&json_data).expect("Failed to parse theme file");

            theme_file
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
                .collect::<Vec<Theme>>()
        })
        .collect();



    Application::new().run(move |cx: &mut App| {
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
            KeyBinding::new("enter", Submit, Some("InteractiveUI")),
            KeyBinding::new("enter", Submit, Some("TextInput")),
        ]);

        // --- Initialize AppState ---
        let app_mode = AppMode::Interactive; // Default to Interactive mode for now

        // Set the initial theme to the first one we loaded.
        let initial_active_theme = all_themes
            .first()
            .map(|theme| theme.interpolatable_theme.clone())
            .expect("Failed to get initial theme");

        let sleep_duration_input = create_duration_input(cx, "10", "Sleep seconds...", 3);
        let fade_duration_input = create_duration_input(cx, "10", "Fade seconds...", 4);
        let theme_selector_focus_handle = cx.focus_handle().tab_index(1).tab_stop(true);
        let end_theme_selector_focus_handle = cx.focus_handle().tab_index(2).tab_stop(true);
        let start_theme_scroll_handle = ScrollHandle::new();
        let end_theme_scroll_handle = ScrollHandle::new();
        let run_simulation_focus_handle = cx.focus_handle().tab_index(5).tab_stop(true);

        let end_theme_index = if all_themes.len() > 1 { 1 } else { 0 };

        cx.set_global(AppState {
            app_mode,
            themes: all_themes,
            start_theme_index: 0, // Default to the first theme
            end_theme_index,      // Default to the second theme if available
            start_preview_index: 0,
            end_preview_index: end_theme_index,
            sleep_duration_input,
            fade_duration_input,
            theme_selector_focus_handle,
            end_theme_selector_focus_handle,
            start_theme_scroll_handle,
            end_theme_scroll_handle,
            run_simulation_focus_handle,
            sleep_input_is_valid: true,
            fade_input_is_valid: true,
            start_dropdown_open: false,
            end_dropdown_open: false,
            active_theme: initial_active_theme,
        });

        // --- Open Window and Set Window-Specific Handlers ---
        let _ = cx
            .open_window(Default::default(), |_, cx| cx.new(|_| AppView))
            .unwrap();
    });
}
