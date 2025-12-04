use chrono::{Duration, Local, NaiveTime};
use futures::channel::mpsc;
use std::{sync::Arc, thread, time::Duration as StdDuration};
use tracing::info;

use crate::{
    state::SimulationState,
    theme::{lerp_theme, InterpolatableTheme},
    AppMode,
};

pub enum SchedulerEvent {
    ThemeUpdate(InterpolatableTheme),
    StateChange(SimulationState),
    Finished,
}

// --- THEME SCHEDULER SERVICE ---

pub struct ThemeScheduler {
    schedule: Arc<Vec<ScheduleEntry>>,
    event_sender: mpsc::Sender<SchedulerEvent>,
    app_mode: AppMode,
    target_fps: u32,
}

#[derive(Clone)]
pub struct ScheduleEntry {
    pub time: NaiveTime,
    pub theme: InterpolatableTheme,
    pub fade_duration: Duration,
}

impl ThemeScheduler {
    pub fn spawn(
        event_sender: mpsc::Sender<SchedulerEvent>,
        schedule: Arc<Vec<ScheduleEntry>>,
        app_mode: AppMode,
        target_fps: u32,
    ) {
        let mut scheduler = Self {
            schedule,
            event_sender,
            app_mode,
            target_fps,
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
                // Sleep Phase
                loop {
                    let now = Local::now().time();
                    if now >= fade_start_time {
                        break;
                    }
                    let remaining_ms = (fade_start_time - now).num_milliseconds();
                    let seconds = (remaining_ms as f32 / 1000.0).ceil() as usize;
                    
                    // Dispatch status update
                    self.dispatch_event(SchedulerEvent::StateChange(SimulationState::Sleeping {
                        seconds_remaining: seconds,
                    }));

                    // Sleep a bit (e.g. 100ms)
                    thread::sleep(StdDuration::from_millis(100));
                }
            }
            
            // Double check we are ready to fade
            let now = Local::now().time();
            if now < fade_end_time {
                info!("ThemeScheduler: Starting fade...");
                self.run_fade_loop(&current_theme, &next_event);
                current_theme_idx = next_event_idx;

                if self.app_mode == AppMode::Interactive {
                    info!("ThemeScheduler: Interactive simulation complete. Exiting thread.");
                    self.dispatch_event(SchedulerEvent::Finished);
                    return;
                }
                continue;
            } else {
                // We missed the window or it's time to set final
                info!("ThemeScheduler: Setting final theme and finding next event.");
                self.dispatch_event(SchedulerEvent::ThemeUpdate(next_event.theme.clone()));
                current_theme_idx = next_event_idx;
                
                // Small delay to avoid tight loop if logic is off
                thread::sleep(StdDuration::from_millis(100));

                if self.app_mode == AppMode::Interactive {
                    info!("ThemeScheduler: Interactive simulation complete (after catch-up). Exiting thread.");
                    self.dispatch_event(SchedulerEvent::Finished);
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
        let sleep_ms = (1000.0 / self.target_fps as f32).max(1.0) as u64;

        loop {
            let now = Local::now().time();
            if now >= fade_end_time {
                break;
            }
            let elapsed_ms = (now - fade_start_time).num_milliseconds() as f32;
            let t = (elapsed_ms / total_duration_ms).clamp(0.0, 1.0);

            let interpolated_theme = lerp_theme(start_theme, &target_event.theme, t);
            
            // Update Theme
            self.dispatch_event(SchedulerEvent::ThemeUpdate(interpolated_theme));
            // Update Status
            self.dispatch_event(SchedulerEvent::StateChange(SimulationState::Fading { progress: t }));

            thread::sleep(StdDuration::from_millis(sleep_ms));
        }
        info!("ThemeScheduler: Fade complete. Setting final theme.");
        self.dispatch_event(SchedulerEvent::ThemeUpdate(target_event.theme.clone()));
        self.dispatch_event(SchedulerEvent::StateChange(SimulationState::Fading { progress: 1.0 }));
    }

    fn dispatch_event(&mut self, event: SchedulerEvent) {
        if let Err(e) = self.event_sender.try_send(event) {
            tracing::warn!("Failed to send scheduler event: {}", e);
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