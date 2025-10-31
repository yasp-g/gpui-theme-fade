use anyhow::{anyhow, Result};
use chrono::{Duration, Local, NaiveTime};
use futures::channel::mpsc;
use gpui::{hsla, rgba, Hsla, Rgba};
use once_cell::sync::Lazy;
use regex::Regex;
use std::{collections::HashMap, str::FromStr, sync::Arc, thread, time::Duration as StdDuration};
use tracing::info;

use crate::AppMode; // New import

// --- THEME & COLOR DEFINITIONS ---

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub rgba: Rgba,
    pub hsla: Hsla,
}

impl Default for Color {
    fn default() -> Self {
        Color {
            rgba: rgba(0xff00ff),
            hsla: hsla(0.83, 1.0, 0.5, 1.0),
        }
    }
}

static COLOR_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^#([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})?$").unwrap()
});

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
        let hsla = Hsla::from(rgba);
        Ok(Color { rgba, hsla })
    }
}

pub fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    let hsla = Hsla {
        h: a.hsla.h + (b.hsla.h - a.hsla.h) * t,
        s: a.hsla.s + (b.hsla.s - a.hsla.s) * t,
        l: a.hsla.l + (b.hsla.l - a.hsla.l) * t,
        a: a.hsla.a + (b.hsla.a - a.hsla.a) * t,
    };
    let rgba = Rgba::from(hsla);
    Color { rgba, hsla }
}

#[derive(Clone, Default, Debug)]
pub struct InterpolatableTheme(pub HashMap<String, Color>);

pub fn lerp_theme(a: &InterpolatableTheme, b: &InterpolatableTheme, t: f32) -> InterpolatableTheme {
    let mut new_theme = InterpolatableTheme::default();

    for (key, color_a) in &a.0 {
        if let Some(color_b) = b.0.get(key) {
            new_theme
                .0
                .insert(key.clone(), lerp_color(*color_a, *color_b, t));
        } else {
            new_theme.0.insert(key.clone(), *color_a);
        }
    }
    new_theme
}


// --- THEME SCHEDULER SERVICE ---

pub struct ThemeScheduler {
    schedule: Arc<Vec<ScheduleEntry>>,
    theme_sender: mpsc::Sender<InterpolatableTheme>,
    app_mode: AppMode, // New field
}

#[derive(Clone)]
pub struct ScheduleEntry {
    pub time: NaiveTime,
    pub theme: InterpolatableTheme,
    pub fade_duration: Duration,
}

impl ThemeScheduler {
    pub fn spawn(
        theme_sender: mpsc::Sender<InterpolatableTheme>,
        schedule: Arc<Vec<ScheduleEntry>>,
        app_mode: AppMode, // New parameter
    ) {
        let mut scheduler = Self {
            schedule,
            theme_sender,
            app_mode, // Store the new parameter
        };
        thread::spawn(move || {
            info!("ThemeScheduler: Background thread spawned.");
            scheduler.run_loop();
        });
    }

    fn run_loop(&mut self) {
        let mut current_theme_idx = find_previous_event_index(Local::now().time(), &self.schedule);

        loop {
            let now = Local::now().time();
            let prev_event = &self.schedule[current_theme_idx];
            let next_event_idx = (current_theme_idx + 1) % self.schedule.len();
            let next_event = self.schedule[next_event_idx].clone();

            let current_theme = prev_event.theme.clone();

            let fade_start_time = next_event.time - next_event.fade_duration;
            let fade_end_time = next_event.time;

            info!(
                "ThemeScheduler: Next event is at {}. Fading from {}.",
                fade_end_time, fade_start_time
            );

            if now < fade_start_time {
                let sleep_dur_ms = (fade_start_time - now).num_milliseconds();
                if sleep_dur_ms > 0 {
                    info!("ThemeScheduler: Sleeping for {}ms...", sleep_dur_ms);
                    thread::sleep(StdDuration::from_millis(sleep_dur_ms as u64));
                }
                continue;
            } else if now < fade_end_time {
                info!("ThemeScheduler: Starting fade...");
                self.run_fade_loop(&current_theme, &next_event);
                current_theme_idx = next_event_idx;

                // Conditional return for Interactive mode
                if self.app_mode == crate::AppMode::Interactive {
                    info!("ThemeScheduler: Interactive simulation complete. Exiting thread.");
                    return;
                }

                continue;
            } else {
                info!("ThemeScheduler: Setting final theme and finding next event.");
                self.dispatch_theme_update(next_event.theme.clone());
                current_theme_idx = next_event_idx;
                thread::sleep(StdDuration::from_millis(1000));

                // Conditional return for Interactive mode if it somehow gets here
                if self.app_mode == crate::AppMode::Interactive {
                    info!("ThemeScheduler: Interactive simulation complete (after catch-up). Exiting thread.");
                    return;
                }

                continue;
            }
        }
    }

    fn run_fade_loop(&mut self, start_theme: &InterpolatableTheme, target_event: &ScheduleEntry) {
        let fade_start_time = target_event.time - target_event.fade_duration;
        let fade_end_time = target_event.time;
        let total_duration_ms = target_event.fade_duration.num_milliseconds() as f32;

        loop {
            let now = Local::now().time();
            if now >= fade_end_time {
                break;
            }
            let elapsed_ms = (now - fade_start_time).num_milliseconds() as f32;
            let t = (elapsed_ms / total_duration_ms).clamp(0.0, 1.0);

            let interpolated_theme = lerp_theme(start_theme, &target_event.theme, t);
            self.dispatch_theme_update(interpolated_theme);

            thread::sleep(StdDuration::from_millis(16));
        }
        info!("ThemeScheduler: Fade complete. Setting final theme.");
        self.dispatch_theme_update(target_event.theme.clone());
    }

    fn dispatch_theme_update(&mut self, theme: InterpolatableTheme) {
        if let Err(e) = self.theme_sender.try_send(theme) {
            tracing::warn!("Failed to send theme update: {}", e);
        }
    }
}

pub fn find_previous_event_index(now: NaiveTime, schedule: &[ScheduleEntry]) -> usize {
    schedule
        .iter()
        .enumerate()
        .filter(|(_, e)| e.time <= now)
        .last()
        .map(|(i, _)| i)
        .unwrap_or(schedule.len() - 1)
}
