use anyhow::{anyhow, Result};
use chrono::{Duration, Local, NaiveTime};
use futures::{channel::mpsc, StreamExt};
use gpui::{
    div, hsla, prelude::*, px, rgba, App, AsyncApp, Context, Global, Hsla, IntoElement, Render, Rgba,
    SharedString, Window, Application,
};
use serde::Deserialize;
use std::{collections::HashMap, fs, str::FromStr, sync::Arc};
use tracing::info;

pub mod scheduler;
use scheduler::{
    find_previous_event_index, lerp_theme, Color, InterpolatableTheme, ScheduleEntry,
    ThemeScheduler,
};


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
fn parse_zed_theme(json_data: &str, theme_name: &str) -> Result<InterpolatableTheme> {
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

/// This is the global `ActiveTheme` that our UI will read from.
/// It now holds our HashMap-based theme.
#[derive(Clone, Default, Debug)]
struct ActiveTheme(InterpolatableTheme);

impl Global for ActiveTheme {}

/// A simple function to update the global theme.
fn set_active_theme(theme: InterpolatableTheme, cx: &mut App) {
    cx.update_global(|active_theme: &mut ActiveTheme, _| {
        active_theme.0 = theme;
        // `update_global` automatically notifies and triggers a re-render.
    });
}


// --- 5. THE MAIN UI VIEW (REFACTORED) ---

struct AppView;

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Read the *current* active theme (the HashMap) from the global context.
        let theme_map = &cx.global::<ActiveTheme>().0 .0;

        // --- We now look up colors by their string key ---
        // We provide a `.unwrap_or_default()` as a fallback
        // in case a theme is missing a key.
        let bg_color = theme_map
            .get("surface.background")
            .cloned()
            .unwrap_or_default();
        let text_color = theme_map
            .get("text") // "text" is a common key
            .cloned()
            .unwrap_or_default();
        let accent_color = theme_map
            .get("border.focused") // Use a more interesting color
            .cloned()
            .unwrap_or_default();
        let button_bg = theme_map
            .get("element.background")
            .cloned()
            .unwrap_or_default();
        let button_text = theme_map
            .get("text")
            .cloned()
            .unwrap_or_default();


        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(bg_color.rgba) // Use the .rgba field for GPUI
            .justify_center()
            .items_center()
            .gap(px(16.0))
            .child(
                div()
                    .text_color(text_color.rgba)
                    .text_size(px(32.0))
                    .child("Theme Scheduler PoC"),
            )
            .child(
                div()
                    .px(px(16.0))
                    .py(px(8.0))
                    .bg(button_bg.rgba)
                    .border_t(px(1.0))
                    .border_b(px(1.0))
                    .border_l(px(1.0))
                    .border_r(px(1.0))
                    .border_color(accent_color.rgba) // Use accent for border
                    .text_color(button_text.rgba)
                    .rounded(px(6.0))
                    .child("A Themed Button"),
            )
            .child(
                div()
                    .text_color(text_color.rgba)
                    .text_size(px(14.0))
                    .child(SharedString::from(format!(
                        "BG Lightness: {:.2}%",
                        bg_color.hsla.l * 100.0
                    ))),
            )
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
                fade_duration: Duration::seconds(300),
            },
            ScheduleEntry {
                time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
                theme: one_dark_theme.clone(),
                fade_duration: Duration::seconds(600),
            },
        ]);

        // --- Find the correct initial theme ---
        let now = Local::now().time();
        let prev_idx = find_previous_event_index(now, &schedule);
        let prev_event = &schedule[prev_idx];
        let next_event = &schedule[(prev_idx + 1) % schedule.len()];

        let initial_theme = {
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

        // Initialize the global with our calculated theme.
        cx.set_global(ActiveTheme(initial_theme));

        // Spawn a task to manage the theme scheduling
        cx.spawn(move |async_cx: &mut AsyncApp| {
            let async_cx = async_cx.clone();
            let schedule = schedule.clone();
            async move {
                let (theme_sender, mut theme_receiver) = mpsc::channel(32);

                // Spawn the scheduler on its background thread.
                ThemeScheduler::spawn(theme_sender, schedule);

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