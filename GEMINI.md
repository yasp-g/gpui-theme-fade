# GPUI Theme Scheduler

This project serves as a development environment for a theme scheduler designed for the Zed code editor. It demonstrates how to dynamically switch between themes with a smooth, animated transition, and will culminate in a demo application.

## Purpose

The primary goal of this application is to develop and showcase a theme scheduling feature that can be packaged as a Zed extension. The code is being developed to be "turn-key" for this purpose. The application includes both a time-based scheduler and an interactive mode for rapid testing and demonstration.

## Key Features

*   **Theme Interpolation:** Smoothly interpolates between themes to create a fading transition effect.
*   **Time-Based Scheduling:** A background thread manages a schedule to switch themes at predefined times.
*   **Interactive Theme Tester:** A user interface to manually trigger and configure theme transitions for development and debugging.
*   **Zed Theme Compatibility:** Parses theme data from JSON strings that follow the Zed theme schema.
*   **`gpui` Integration:** Leverages the `gpui` library for the user interface and application state management.

## Project Structure

*   `src/main.rs`: Contains the application's entry point, state management, and the primary UI view.
*   `src/scheduler.rs`: Houses the `ThemeScheduler` for time-based theme switching.
*   `assets/`: Contains theme definition files (e.g., `one.json`, `ayu.json`).
*   `sandbox/gpui-source/gpui/`: Local clone of the `gpui` framework source code, available for reference during development.
*   `Cargo.toml`: Defines project dependencies.
*   `GEMINI.md`: This file, providing a high-level overview of the project.

## Development Notes

*   **GPUI Source:** The source code for the version of GPUI used in this project has been cloned from Zed Industries' repository into `sandbox/gpui-source/gpui/` for reference.
*   **Compilation:** The immediate focus is on achieving a successful compilation. Use `cargo build` to check for errors before attempting to run the application with `cargo run`.