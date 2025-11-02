use anyhow::{anyhow, Result};
use chrono::{Duration as ChronoDuration, Local, NaiveTime};
use gpui::{
    div, prelude::*, App, AppContext, AsyncApp, Context, Global, IntoElement, Render,
    Window, Application,
};
use serde::Deserialize;
use std::{collections::HashMap, fs, str::FromStr, sync::Arc};
use futures::{channel::mpsc, StreamExt};
use tracing::info;

pub mod scheduler;
pub mod ui;
use scheduler::{
    find_previous_event_index, lerp_theme, Color, InterpolatableTheme, ScheduleEntry,
    ThemeScheduler,
};

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
fn parse_zed_theme(json_data: &str, theme_name: &str) -> Result<InterpolatableTheme, anyhow::Error> {
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
#[derive(Clone, Default, Debug)]
pub struct AppState {
    pub app_mode: AppMode,
    pub themes: Vec<Theme>,
    pub selected_theme_index: usize,
    pub sleep_duration_seconds: f32,
    pub fade_duration_seconds: f32,
    pub dropdown_open: bool,
    // The currently active theme, which the UI renders.
    pub active_theme: InterpolatableTheme,
}

impl Global for AppState {}

/// A simple function to update the active theme within the global AppState.
fn set_active_theme<T: AppContext + gpui::BorrowAppContext>(theme: InterpolatableTheme, cx: &mut T) {
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
                .bg(app_state.active_theme.0.get("surface.background").expect("Theme missing surface.background").hsla)
                .text_color(app_state.active_theme.0.get("text").expect("Theme missing text color").hsla)
                .child(format!("Current Theme: {}", app_state.themes[app_state.selected_theme_index].name))
                .into_any_element(),
            AppMode::Interactive => ui::render_interactive_ui(&app_state, cx).into_any_element(),
        }
    }
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

        cx.set_global(AppState {
            app_mode,
            themes: all_themes,
            selected_theme_index: 0, // Default to the first theme
            sleep_duration_seconds: 5.0, // Default value
            fade_duration_seconds: 5.0, // Default value
            dropdown_open: false,
            active_theme: initial_active_theme,
        });

        // Spawn a task to manage the theme scheduling
        cx.spawn(move |async_cx: &mut AsyncApp| {
            let async_cx = async_cx.clone();
            let schedule = schedule.clone();
            async move {
                let (theme_sender, mut theme_receiver) = mpsc::channel(32);

                // Get app_mode from the global AppState
                let current_app_mode = async_cx.read_global::<AppState, _>(|app_state, _| app_state.app_mode).expect("Should be able to read AppState");

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
