use anyhow::{anyhow, Result};
use chrono::{Duration, Local, NaiveTime, Timelike};
use gpui::{
    div, hsla, prelude::*, rgba, AppContext, Dispatcher, Global, Hsla, IntoElement, Pixels,
    Render, Rgba, SharedString, View, WindowContext,
};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use std::{collections::HashMap, fs, str::FromStr, sync::Arc, thread, time::Duration as StdDuration};
use tracing::info;

// --- 1. THEME & COLOR DEFINITIONS (REFACTORED) ---

/// A new `Color` struct that can be parsed from a hex string.
/// It stores both Rgba (for GPUI) and Hsla (for interpolation).
#[derive(Clone, Copy, Debug)]
struct Color {
    rgba: Rgba,
    hsla: Hsla,
}

// Default color if a lookup fails
impl Default for Color {
    fn default() -> Self {
        Color {
            rgba: rgba(0xff00ff), // Default to bright magenta
            hsla: hsla(0.83, 1.0, 0.5, 1.0),
        }
    }
}

/// Regex to parse #RRGGBB and #RRGGBBAA hex codes
static COLOR_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^#([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})?$").unwrap()
});

/// Implement `FromStr` to parse hex strings into our `Color`.
impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let caps = COLOR_REGEX
            .captures(s)
            .ok_or_else(|| anyhow!("Invalid hex color: {}", s))?;

        let r = u8::from_str_radix(caps.get(1).unwrap().as_str(), 16)?;
        let g = u8::from_str_radix(caps.get(2).unwrap().as_str(), 16)?;
        let b = u8::from_str_radix(caps.get(3).unwrap().as_str(), 16)?;
        let a = caps
            .get(4)
            .map_or(Ok(255), |m| u8::from_str_radix(m.as_str(), 16))?;

        let rgba = Rgba {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        };
        let hsla = rgba.to_hsla();
        Ok(Color { rgba, hsla })
    }
}

/// Linearly interpolates between two `Color` structs using their HSLA values.
fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    let hsla = Hsla {
        h: a.hsla.h + (b.hsla.h - a.hsla.h) * t,
        s: a.hsla.s + (b.hsla.s - a.hsla.s) * t,
        l: a.hsla.l + (b.hsla.l - a.hsla.l) * t,
        a: a.hsla.a + (b.hsla.a - a.hsla.a) * t,
    };
    let rgba = hsla.to_rgba();
    Color { rgba, hsla }
}

/// This is our new, interpolatable theme format.
/// It's a `HashMap` that maps Zed's style keys to our `Color` struct.
#[derive(Clone, Default, Debug)]
struct InterpolatableTheme(HashMap<String, Color>);

/// Takes two themes (HashMaps) and interpolates *every* matching key.
fn lerp_theme(a: &InterpolatableTheme, b: &InterpolatableTheme, t: f32) -> InterpolatableTheme {
    let mut new_theme = InterpolatableTheme::default();

    // Iterate over the "base" theme (A)
    for (key, color_a) in &a.0 {
        // Find the matching color in the "target" theme (B)
        if let Some(color_b) = b.0.get(key) {
            // If it exists, interpolate and store the new color
            new_theme
                .0
                .insert(key.clone(), lerp_color(*color_a, *color_b, t));
        } else {
            // If B doesn't have this key, just use A's color
            new_theme.0.insert(key.clone(), *color_a);
        }
    }
    // Note: This implementation doesn't add keys that are in B but not in A.
    // For a theme scheduler, this is usually fine as we interpolate
    // between two *complete* themes.
    new_theme
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
    appearance: String,
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
fn set_active_theme(theme: InterpolatableTheme, cx: &mut AppContext) {
    cx.update_global(|active_theme: &mut ActiveTheme, _| {
        active_theme.0 = theme;
        // `update_global` automatically notifies and triggers a re-render.
    });
}

// --- 4. THEME SCHEDULER SERVICE ---

/// This holds our schedule and the logic for the "daemon" thread.
/// This logic is identical to the previous example, but the "Theme"
/// type it passes around is now our `InterpolatableTheme`.
struct ThemeScheduler {
    schedule: Arc<Vec<ScheduleEntry>>,
    dispatcher: Dispatcher,
}

#[derive(Clone)]
struct ScheduleEntry {
    time: NaiveTime,
    theme: InterpolatableTheme,
    fade_duration: Duration,
}

impl ThemeScheduler {
    /// Spawns the scheduler on a new, detached background thread.
    pub fn spawn(dispatcher: Dispatcher, schedule: Arc<Vec<ScheduleEntry>>) {
        let scheduler = Self {
            schedule,
            dispatcher,
        };
        thread::spawn(move || {
            info!("ThemeScheduler: Background thread spawned.");
            scheduler.run_loop();
        });
    }

    /// The main loop for our scheduler "daemon".
    /// This logic is IDENTICAL to before, just with a new theme type.
    fn run_loop(&self) {
        let mut current_theme_idx = self.find_previous_event_index(Local::now().time());

        loop {
            // --- 1. Find the Next Event ---
            let now = Local::now().time();
            let prev_event = &self.schedule[current_theme_idx];
            let next_event_idx = (current_theme_idx + 1) % self.schedule.len();
            let next_event = &self.schedule[next_event_idx];

            let current_theme = prev_event.theme.clone();

            // --- 2. Calculate Timings ---
            let fade_start_time = next_event.time - next_event.fade_duration;
            let fade_end_time = next_event.time;

            info!(
                "ThemeScheduler: Next event is at {}. Fading from {}.",
                fade_end_time, fade_start_time
            );

            // --- 3. Determine State (Idle, Fading, or Post-Fade) ---
            if now < fade_start_time {
                // --- STATE 1: IDLE ---
                let sleep_dur_ms = (fade_start_time - now).num_milliseconds();
                if sleep_dur_ms > 0 {
                    info!("ThemeScheduler: Sleeping for {}ms...", sleep_dur_ms);
                    thread::sleep(StdDuration::from_millis(sleep_dur_ms as u64));
                }
                continue; // Loop to enter Fading state
            } else if now < fade_end_time {
                // --- STATE 2: FADING ---
                info!("ThemeScheduler: Starting fade...");
                self.run_fade_loop(&current_theme, next_event);
                current_theme_idx = next_event_idx; // Update current theme index
                continue; // Loop to enter Post-Fade state
            } else {
                // --- STATE 3: POST-FADE ---
                info!("ThemeScheduler: Setting final theme and finding next event.");
                self.dispatch_theme_update(next_event.theme.clone());
                current_theme_idx = next_event_idx; // Update current theme index
                // Short sleep to prevent busy-looping if schedule is broken
                thread::sleep(StdDuration::from_millis(1000));
                continue;
            }
        }
    }

    /// The "event-spam" loop that runs during a fade.
    fn run_fade_loop(&self, start_theme: &InterpolatableTheme, target_event: &ScheduleEntry) {
        let fade_start_time = target_event.time - target_event.fade_duration;
        let fade_end_time = target_event.time;
        let total_duration_ms = target_event.fade_duration.num_milliseconds() as f32;

        loop {
            let now = Local::now().time();
            if now >= fade_end_time {
                break; // Fade is complete.
            }
            let elapsed_ms = (now - fade_start_time).num_milliseconds() as f32;
            let t = (elapsed_ms / total_duration_ms).clamp(0.0, 1.0);

            // This is our JIT "lerp_theme" call
            let interpolated_theme = lerp_theme(start_theme, &target_event.theme, t);
            self.dispatch_theme_update(interpolatable_theme);

            thread::sleep(StdDuration::from_millis(16)); // Target ~60fps
        }
        info!("ThemeScheduler: Fade complete. Setting final theme.");
        self.dispatch_theme_update(target_event.theme.clone());
    }

    /// Finds the index of the event that *should* be active right now.
    fn find_previous_event_index(&self, now: NaiveTime) -> usize {
        self.schedule
            .iter()
            .enumerate()
            .filter(|(_, e)| e.time <= now)
            .last()
            .map(|(i, _)| i)
            .unwrap_or(self.schedule.len() - 1) // If it's before the first event, wrap to last
    }

    /// Helper to send our theme update to the main UI thread.
    fn dispatch_theme_update(&self, theme: InterpolatableTheme) {
        self.dispatcher
            .dispatch_global(Box::new(move |cx| {
                set_active_theme(theme, cx);
            }))
            .ok();
    }
}

// --- 5. THE MAIN UI VIEW (REFACTORED) ---

struct AppView;

impl Render for AppView {
    fn render(&mut self, cx: &mut WindowContext) -> impl IntoElement {
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
            .gap(Pixels(16.0))
            .child(
                div()
                    .text_color(text_color.rgba)
                    .text_size(Pixels(32.0))
                    .child("Theme Scheduler PoC"),
            )
            .child(
                div()
                    .px(Pixels(16.0))
                    .py(Pixels(8.0))
                    .bg(button_bg.rgba)
                    .border()
                    .border_color(accent_color.rgba) // Use accent for border
                    .text_color(button_text.rgba)
                    .rounded(Pixels(6.0))
                    .child("A Themed Button"),
            )
            .child(
                div()
                    .text_color(text_color.rgba)
                    .text_size(Pixels(14.0))
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
    // A real app would read these from files.
    let one_dark_json = fs::read_to_string("assets/one.json").expect("Failed to read one.json");
    let ayu_light_json = fs::read_to_string("assets/ayu.json").expect("Failed to read ayu.json");

    let one_dark_theme =
        parse_zed_theme(&one_dark_json, "One Dark").expect("Failed to parse One Dark");
    let ayu_light_theme =
        parse_zed_theme(&ayu_light_json, "Ayu Light").expect("Failed to parse Ayu Light");

    gpui::App::new().run(move |cx: &mut AppContext| {
        // --- This is our mock schedule ---
        let schedule = Arc::new(vec![
            ScheduleEntry {
                time: NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
                theme: ayu_light_theme.clone(),
                fade_duration: Duration::seconds(300),
            },
            ScheduleEntry {
                // For testing, let's set this 1 minute from now
                // time: (Local::now() + Duration::minutes(1)).time(),
                time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
                theme: one_dark_theme.clone(),
                fade_duration: Duration::seconds(600),
            },
        ]);

        // --- Find the correct initial theme ---
        let now = Local::now().time();
        let scheduler_for_init = ThemeScheduler {
            schedule: schedule.clone(),
            dispatcher: cx.dispatcher(),
        };
        let prev_idx = scheduler_for_init.find_previous_event_index(now);
        let prev_event = &schedule[prev_idx];
        let next_event = &schedule[(prev_idx + 1) % schedule.len()];

        let initial_theme = {
            let fade_start = next_event.time - next_event.fade_duration;
            if now >= fade_start && now < next_event.time {
                // We are mid-fade! Calculate the initial `t`.
                let total_dur = next_event.fade_duration.num_milliseconds() as f32;
                let elapsed = (now - fade_start).num_milliseconds() as f32;
                let t = (elapsed / total_dur).clamp(0.0, 1.0);
                info!("Main: Starting mid-fade (t = {}).", t);
                lerp_theme(&prev_event.theme, &next_event.theme, t)
            } else {
                // We are idle. Use the theme of the *previous* event.
                info!("Main: Starting in idle state.");
                prev_event.theme.clone()
            }
        };

        // Initialize the global with our calculated theme.
        ActiveTheme::init(cx, ActiveTheme(initial_theme));

        // Spawn the scheduler on its background thread.
        ThemeScheduler::spawn(cx.dispatcher(), schedule);

        // Open the main window.
        cx.open_window(Default::default(), |cx| View::new(cx, |_| AppView));
    });
}
