use chrono::{Duration, Local, NaiveTime};
use futures::channel::mpsc;
use std::{sync::Arc, thread, time::Duration as StdDuration};
use tracing::info;

use crate::{
    theme::{lerp_theme, InterpolatableTheme},
    AppMode,
};

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
