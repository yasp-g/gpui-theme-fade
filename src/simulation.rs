use crate::{
    scheduler::{SchedulerEvent, ThemeScheduler},
    state::SimulationState,
    theme::InterpolatableTheme,
    AppState, AppView,
};
use chrono::{Duration as ChronoDuration, Local};
use futures::{channel::mpsc, StreamExt};
use gpui::{AsyncApp, Context, SharedString, WeakEntity, prelude::*};
use std::sync::Arc;
use tracing::info;

pub fn run_simulation_core(
    cx: &mut Context<AppView>,
    start_theme: InterpolatableTheme,
    end_theme: InterpolatableTheme,
    sleep_duration: ChronoDuration,
    fade_duration: ChronoDuration,
    target_fps: u32,
    start_theme_name: SharedString,
    end_theme_name: SharedString,
) {
    info!(
        "Running simulation: Start='{}', End='{}', FPS={}",
        start_theme_name, end_theme_name, target_fps
    );

    cx.spawn(move |view: WeakEntity<AppView>, cx: &mut AsyncApp| {
        let mut cx = cx.clone();
        async move {
            let (event_sender, mut event_receiver) = mpsc::channel(32);
            let now = Local::now().time();
            let sim_schedule = Arc::new(vec![
            crate::scheduler::ScheduleEntry {
                time: now,
                theme: start_theme.clone(),
                fade_duration: ChronoDuration::seconds(0),
            },
            crate::scheduler::ScheduleEntry {
                time: now + sleep_duration + fade_duration,
                theme: end_theme.clone(),
                fade_duration,
            },
        ]);

        ThemeScheduler::spawn(
            event_sender.clone(),
            sim_schedule,
            crate::AppMode::Interactive,
            target_fps,
        );

        while let Some(event) = event_receiver.next().await {
            // We update the view on the main thread
            let _ = view.update(&mut cx, |view, cx| {
                match event {
                    SchedulerEvent::ThemeUpdate(theme) => {
                        cx.update_global::<AppState, _>(|app_state, _| {
                            app_state.active_theme = theme;
                        });
                        // Force refresh as global update might not trigger it for everything if not tracking?
                        // update_global typically triggers a notify for things watching the global.
                        // But to be safe/smooth:
                        cx.notify(); 
                    }
                    SchedulerEvent::StateChange(state) => {
                        view.simulation_state = state;
                        cx.notify();
                    }
                    SchedulerEvent::Finished => {
                        info!("Simulation Finished Event Received");
                        
                        // UX Improvement: Swap themes
                        cx.update_global::<AppState, _>(|state, _| {
                            let old_start = state.start_theme_index;
                            let old_end = state.end_theme_index;
                            
                            state.start_theme_index = old_end;
                            state.end_theme_index = old_start;
                        });

                        view.simulation_state = SimulationState::Idle;
                        cx.notify();
                    }
                }
            });
        }
        info!("Simulation channel closed.");
        }
    })
    .detach();
}
