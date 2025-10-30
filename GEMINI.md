# GPUI Theme Scheduler

This project is a proof-of-concept for a theme scheduler using the `gpui` framework. It demonstrates how to dynamically switch between themes with a smooth, animated transition.

## Purpose

The primary goal of this application is to serve as a prototype for a potential Zed extension. The core functionality revolves around time-based theme switching, allowing for automatic changes between light and dark themes at specified times.

## Key Features

*   **Theme Interpolation:** The application smoothly interpolates between two themes, "One Dark" and "Ayu Light," to create a fading transition effect.
*   **Time-Based Scheduling:** A background thread manages a schedule to switch themes at predefined times (e.g., 7:00 AM and 5:00 PM).
*   **Zed Theme Compatibility:** The project parses theme data from JSON strings that follow the Zed theme schema.
*   **`gpui` Integration:** It leverages the `gpui` library for creating the user interface and managing the application state.

## Project Structure

*   `src/main.rs`: Contains the entire application logic, including theme parsing, color interpolation, the theme scheduler, and the UI view.
*   `Cargo.toml`: Defines the project dependencies, including `gpui`, `chrono`, `serde`, and other necessary crates.
*   `GEMINI.md`: This file, providing a high-level overview of the project for easy future reference.

## Development Notes

*   **GPUI Source:** The source code for the version of GPUI used in this project has been cloned from Zed Industries' repository into `sandbox/gpui-source/gpui/` for reference.
*   **Compilation:** The immediate focus is on achieving a successful compilation. Use `cargo build` to check for errors before attempting to run the application with `cargo run`.