use crate::{scheduler::ThemeScheduler, theme::InterpolatableTheme, AppState, AppView};
use chrono::{Duration as ChronoDuration, Local};
use futures::{channel::mpsc, StreamExt};
use gpui::{Context, SharedString, AsyncApp};
use std::sync::Arc;
use tracing::info;

pub fn run_simulation_core(
    cx: &mut Context<AppView>,
    start_theme: InterpolatableTheme,
    end_theme: InterpolatableTheme,
    sleep_duration: ChronoDuration,
    fade_duration: ChronoDuration,
    start_theme_name: SharedString,
    end_theme_name: SharedString,
) {
    info!(
        "Running simulation: Start='{}', End='{}'",
        start_theme_name, end_theme_name
    );

    cx.spawn::<_, ()>(move |_, async_cx_param: &mut AsyncApp| {
        let async_cx = async_cx_param.clone();
        async move {
            let (theme_sender, mut theme_receiver) = mpsc::channel(32);
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
                theme_sender.clone(),
                sim_schedule,
                crate::AppMode::Interactive,
            );

            while let Some(theme) = theme_receiver.next().await {
                async_cx.update_global::<AppState, _>(|app_state, _| {
                    app_state.active_theme = theme.clone();
                })
                .ok();
                async_cx.refresh().ok();
            }
            info!("Simulation finished and channel closed.");
        }
    })
    .detach();
}