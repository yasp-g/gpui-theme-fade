use anyhow::{Result, anyhow};
use chrono::{Duration as ChronoDuration, Local, NaiveTime};
use futures::{StreamExt, channel::mpsc};
use gpui::{
    div, prelude::*, Action, App, AppContext, Application, AsyncApp, Context, Entity, Global, IntoElement,
    KeyBinding, Render, SharedString, Window,
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::{collections::HashMap, fs, str::FromStr, sync::Arc};
use tracing::info;

pub mod scheduler;
pub mod text_input;
pub mod ui;
use scheduler::{
    Color, InterpolatableTheme, ScheduleEntry, ThemeScheduler, find_previous_event_index,
    lerp_theme,
};
use text_input::{TextInput, Backspace, Delete, Left, Right, SelectLeft, SelectRight, SelectAll, Home, End, Paste, Cut, Copy};

// --- 1. ACTIONS ---
#[derive(Clone, PartialEq, Action, Deserialize, JsonSchema)]
pub struct ToggleDropdown;

#[derive(Clone, PartialEq, Action, Deserialize, JsonSchema)]
pub struct SelectTheme {
    pub theme_index: usize,
}

#[derive(Clone, PartialEq, Action, Deserialize, JsonSchema)]
pub struct RunSimulation;

#[derive(Clone, PartialEq, Action, Deserialize, JsonSchema)]
pub struct SetSleepDuration {
    pub seconds: f32,
}

#[derive(Clone, PartialEq, Action, Deserialize, JsonSchema)]
pub struct SetFadeDuration {
    pub seconds: f32,
}

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

// New struct to hold a theme and its name
#[derive(Clone, Debug)]
pub struct Theme {
    pub name: String,
    pub interpolatable_theme: InterpolatableTheme,
}

// --- 2. JSON PARSING STRUCTS ---

/// These structs mirror the Zed theme JSON schema.
#[derive(Deserialize, Debug)]
struct ZedThemeFile {
    themes: Vec<ThemeDefinition>,
}

#[derive(Deserialize, Debug)]
struct ThemeDefinition {
    name: String,
    style: ThemeStyle,
}

/// Use a HashMap to capture all unknown keys in the "style" block.
#[derive(Deserialize, Debug)]
struct ThemeStyle {
    #[serde(flatten)]
    colors: HashMap<String, serde_json::Value>,
}

/// A helper function to parse a JSON string and extract one
/// `InterpolatableTheme` (e.g., "One Dark").
fn parse_zed_theme(
    json_data: &str,
    theme_name: &str,
) -> Result<InterpolatableTheme, anyhow::Error> {
    let theme_file: ZedThemeFile = serde_json::from_str(json_data)?;

    let theme_def = theme_file
        .themes
        .into_iter()
        .find(|t| t.name == theme_name)
        .ok_or_else(|| anyhow!("Theme '{}' not found in JSON", theme_name))?;

    let mut interpolatable_theme = InterpolatableTheme::default();
    for (key, value) in theme_def.style.colors {
        if let Some(hex_string) = value.as_str() {
            match Color::from_str(hex_string) {
                Ok(color) => {
                    interpolatable_theme.0.insert(key, color);
                }
                Err(e) => {
                    // Log an error but don't crash
                    tracing::warn!(
                        "Failed to parse color for key '{}': {} (value: '{}')",
                        key,
                        e,
                        hex_string
                    );
                }
            }
        }
    }
    Ok(interpolatable_theme)
}

// --- 3. GPUI GLOBAL STATE ---

/// This is the global `AppState` that our UI will read from and update.
#[derive(Clone)]
pub struct AppState {
    pub app_mode: AppMode,
    pub themes: Vec<Theme>,
    pub selected_theme_index: usize,
    pub sleep_duration_input: Entity<TextInput>,
    pub fade_duration_input: Entity<TextInput>,
    pub sleep_input_is_valid: bool,
    pub fade_input_is_valid: bool,
    pub dropdown_open: bool,
    // The currently active theme, which the UI renders.
    pub active_theme: InterpolatableTheme,
}

impl Global for AppState {}

/// A simple function to update the active theme within the global AppState.
fn set_active_theme<T: AppContext + gpui::BorrowAppContext>(
    theme: InterpolatableTheme,
    cx: &mut T,
) {
    cx.update_global(|app_state: &mut AppState, _| {
        app_state.active_theme = theme;
        // `update_global` automatically notifies and triggers a re-render.
    });
}

// --- 5. THE MAIN UI VIEW (REFACTORED) ---

pub struct AppView;

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let app_state = cx.global::<AppState>().clone();

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
                    app_state.themes[app_state.selected_theme_index].name
                ))
                .into_any_element(),
            AppMode::Interactive => ui::render_interactive_ui(cx).into_any_element(),
        }
    }
}

fn create_duration_input(
    cx: &mut App,
    content: impl Into<SharedString>,
    placeholder: impl Into<SharedString>,
) -> Entity<TextInput> {
    cx.new(|cx| TextInput {
        focus_handle: cx.focus_handle(),
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
    let one_dark_json = fs::read_to_string("assets/one.json").expect("Failed to read one.json");
    let ayu_light_json = fs::read_to_string("assets/ayu.json").expect("Failed to read ayu.json");

    let one_dark_theme =
        parse_zed_theme(&one_dark_json, "One Dark").expect("Failed to parse One Dark");
    let ayu_light_theme =
        parse_zed_theme(&ayu_light_json, "Ayu Light").expect("Failed to parse Ayu Light");

    Application::new().run(move |cx: &mut App| {
        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, None),
            KeyBinding::new("delete", Delete, None),
            KeyBinding::new("left", Left, None),
            KeyBinding::new("right", Right, None),
            KeyBinding::new("shift-left", SelectLeft, None),
            KeyBinding::new("shift-right", SelectRight, None),
            KeyBinding::new("cmd-a", SelectAll, None),
            KeyBinding::new("home", Home, None),
            KeyBinding::new("end", End, None),
            KeyBinding::new("cmd-v", Paste, None),
            KeyBinding::new("cmd-c", Copy, None),
            KeyBinding::new("cmd-x", Cut, None),
        ]);

        // --- This is our mock schedule ---
        let schedule = Arc::new(vec![
            ScheduleEntry {
                time: NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
                theme: ayu_light_theme.clone(),
                fade_duration: ChronoDuration::seconds(300),
            },
            ScheduleEntry {
                time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
                theme: one_dark_theme.clone(),
                fade_duration: ChronoDuration::seconds(600),
            },
        ]);

        // --- Initialize AppState ---
        let app_mode = AppMode::Interactive; // Default to Interactive mode for now

        let all_themes = vec![
            Theme {
                name: "One Dark".to_string(),
                interpolatable_theme: one_dark_theme.clone(),
            },
            Theme {
                name: "Ayu Light".to_string(),
                interpolatable_theme: ayu_light_theme.clone(),
            },
        ];

        let initial_active_theme = {
            let now = Local::now().time();
            let prev_idx = find_previous_event_index(now, &schedule);
            let prev_event = &schedule[prev_idx];
            let next_event = &schedule[(prev_idx + 1) % schedule.len()];

            let fade_start = next_event.time - next_event.fade_duration;
            if now >= fade_start && now < next_event.time {
                let total_dur = next_event.fade_duration.num_milliseconds() as f32;
                let elapsed = (now - fade_start).num_milliseconds() as f32;
                let t = (elapsed / total_dur).clamp(0.0, 1.0);
                info!("Main: Starting mid-fade (t = {}).", t);
                lerp_theme(&prev_event.theme, &next_event.theme, t)
            } else {
                info!("Main: Starting in idle state.");
                prev_event.theme.clone()
            }
        };

        let sleep_duration_input = create_duration_input(cx, "10", "Sleep seconds...");
        let fade_duration_input = create_duration_input(cx, "10", "Fade seconds...");

        cx.set_global(AppState {
            app_mode,
            themes: all_themes,
            selected_theme_index: 0, // Default to the first theme
            sleep_duration_input,
            fade_duration_input,
            sleep_input_is_valid: true,
            fade_input_is_valid: true,
            dropdown_open: false,
            active_theme: initial_active_theme,
        });

        // --- Action Handlers ---
        cx.on_action(|_: &ToggleDropdown, cx| {
            cx.update_global(|app_state: &mut AppState, _| {
                app_state.dropdown_open = !app_state.dropdown_open;
            });
        });

        cx.on_action(|action: &SelectTheme, cx| {
            cx.update_global(|app_state: &mut AppState, _| {
                app_state.selected_theme_index = action.theme_index;
                app_state.dropdown_open = false; // Close dropdown after selection
            });
        });

        cx.on_action(|_: &RunSimulation, cx: &mut App| {
            // First, get the entity handles from the global state.
            let (sleep_input_handle, fade_input_handle, current_theme, next_theme) =
                cx.read_global(|app_state: &AppState, _| {
                    let current_theme_index =
                        (app_state.selected_theme_index + 1) % app_state.themes.len();
                    (
                        app_state.sleep_duration_input.clone(),
                        app_state.fade_duration_input.clone(),
                        app_state.themes[current_theme_index]
                            .interpolatable_theme
                            .clone(),
                        app_state.themes[app_state.selected_theme_index]
                            .interpolatable_theme
                            .clone(),
                    )
                });

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

                cx.spawn(move |async_cx: &mut AsyncApp| {
                    let async_cx = async_cx.clone();
                    async move {
                        let (theme_sender, mut theme_receiver) = mpsc::channel(32);

                        let now = Local::now().time();
                        let sim_schedule = Arc::new(vec![
                            ScheduleEntry {
                                time: now,
                                theme: current_theme.clone(),
                                fade_duration: ChronoDuration::seconds(0),
                            },
                            ScheduleEntry {
                                time: now + sleep_duration + fade_duration,
                                theme: next_theme.clone(),
                                fade_duration,
                            },
                        ]);

                        ThemeScheduler::spawn(
                            theme_sender.clone(),
                            sim_schedule,
                            AppMode::Interactive,
                        );

                        while let Some(theme) = theme_receiver.next().await {
                            async_cx.update(|cx| set_active_theme(theme, cx)).ok();
                        }
                        info!("Simulation finished and channel closed.");
                    }
                })
                .detach();
            }
        });

        // Spawn a task to manage the theme scheduling
        cx.spawn(move |async_cx: &mut AsyncApp| {
            let async_cx = async_cx.clone();
            let schedule = schedule.clone();
            async move {
                let (theme_sender, mut theme_receiver) = mpsc::channel(32);

                // Get app_mode from the global AppState
                let current_app_mode = async_cx
                    .read_global::<AppState, _>(|app_state, _| app_state.app_mode)
                    .expect("Should be able to read AppState");

                // Only spawn the scheduler if in Scheduler mode
                if current_app_mode == AppMode::Scheduler {
                    ThemeScheduler::spawn(theme_sender.clone(), schedule, current_app_mode);
                }

                // Listen for theme updates
                while let Some(theme) = theme_receiver.next().await {
                    async_cx.update(|cx| set_active_theme(theme, cx)).ok();
                }
            }
        })
        .detach();

        // Open the main window.
        let _ = cx.open_window(Default::default(), |_, cx| cx.new(|_| AppView));
    });
}
