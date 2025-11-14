# Project Roadmap: GPUI Theme Scheduler

**Last Updated:** 2025-11-12

## Project Overview

This project serves as a development environment for a theme scheduling library intended for the Zed editor. It includes a standalone test harness UI to demonstrate and interact with the core scheduling and theme interpolation logic. The primary goal is to produce a robust, modular, and Zed-ready library for theme management.

## Core Objectives

1.  **Architectural Integrity:** Build a modular, testable, and scalable library by decoupling the core logic from any specific UI implementation.
2.  **Functional Robustness:** Ensure the core theme scheduling and interpolation logic is reliable and performs smoothly.
3.  **Developer Experience:** Provide a clear and effective test harness for rapid development and demonstration of the core library's features.

---

## Phase 1: Core Logic & Architectural Refactoring

This phase focuses on separating the core, UI-agnostic logic from the test harness UI. The goal is to create a modular library that can be easily integrated into a Zed extension.

### 1. State Management Refactoring

- **Description:** Decouple ephemeral UI state from the global `AppState` by moving it into the UI components that own it. This is the highest-priority architectural improvement.
- **Status:** [x] Completed
- **Priority:** High
- **Tasks:**
  - [x] **Simplify `AppState`:** Remove UI-specific state fields (`start_dropdown_open`, `end_dropdown_open`, `start_preview_index`, `end_preview_index`, `sleep_input_validation_message`, `fade_input_validation_message`) from the `AppState` struct in `src/main.rs`.
  - [x] **Create Stateful `Dropdown` Component:** Refactor the `dropdown.rs` component to be a proper stateful GPUI view.
    - [x] Its ephemeral UI state (`is_open`, `preview_index`, `focus_handle`, `scroll_handle`) is now managed by `AppView` via a `DropdownState` struct and passed to the stateless `render_dropdown` function.
    - [x] It receives the list of themes and the currently selected index as props.
    - [x] It uses callbacks (e.g., `on_select`) to notify the parent view of a selection change.
  - [x] **Update UI:** Modified `src/ui.rs` to use the `DropdownState` from `AppView` when calling the `render_dropdown` function, passing in the required props and callbacks.
  - [x] **Create Stateful `ValidatedInput` Component:** Extract the text inputs and their validation logic into a new reusable component.
    - [x] The `TextInput` entities and their corresponding `validation_message` are now owned by `AppView` via a `ValidatedInputState` struct.
    - [x] The validation logic in `run_simulation` now mutates the state on `AppView` directly, removing the need to pass state through the global `AppState`.

### 2. Core Logic Extraction

- **Description:** Slim down the `AppView` "God Object" by extracting distinct areas of logic into more focused, UI-agnostic modules.
- **Status:** [ ] Not Started
- **Priority:** Medium
- **Tasks:**
  - [ ] **Extract Simulation Logic:**
    - [ ] Create a new module (e.g., `src/simulation.rs`).
    - [ ] Move the core logic from `AppView::run_simulation` into a function within this new module. This function will take the necessary parameters (themes, durations) and handle spawning the async task.
    - [ ] `AppView::run_simulation` will now be a thin wrapper that calls this new function.
  - [ ] **Isolate UI-Specific Logic:**
    - [ ] Create a new helper function (e.g., in `src/ui.rs` or a new `src/components/util.rs`).
    - [ ] This function will contain the duplicated manual scroll calculation logic from `AppView::select_next_theme` and `AppView::select_prev_theme`.
    - [ ] Refactor the `AppView` methods to call this new, single helper function to better isolate this UI-only code.

---

## Phase 2: Core Feature Development

Once the architecture is solidified, we can begin adding new, core library functionality.

### 1. Theme Integration with Zed Settings

- **Description:** Integrate the theme scheduler with Zed's settings system to allow programmatic theme changes. This is the primary mechanism for the final Zed extension to function.
- **Status:** [ ] Not Started
- **Priority:** High
- **Tasks:**
  - [ ] Adapt the core scheduling logic to call `settings::update_settings_file` when a theme change is due.
  - [ ] Inside the `update_settings_file` closure, use the `theme::settings::set_theme` helper to modify the `SettingsContent` with the new theme name.
  - [ ] Ensure the scheduler has access to the necessary `Fs` and `AppContext` handles to perform this operation.

### 2. Zed Extension Structure and Theme Access

- **Description:** Define the project's structure as a Zed extension and adapt its theme loading mechanism to use Zed's internal theme registry.
- **Status:** [ ] Not Started
- **Priority:** High
- **Tasks:**
  - [ ] Investigate Zed's extension loading mechanism and define the `Cargo.toml` and entry point for our extension.
  - [ ] Understand how Zed extensions access the available themes from Zed's `ThemeRegistry`.
  - [ ] Adapt our theme loading mechanism to use Zed's API instead of reading theme files directly from the `assets/` directory.

### 3. Configuration Persistence

- **Description:** Save and load the user's settings to provide a consistent experience between sessions. This will be designed as a core feature that the Zed extension can leverage.
- **Status:** [ ] Not Started
- **Priority:** Low
- **Tasks:**
  - [ ] **Define Config Struct:** Create a new struct that can be serialized/deserialized (e.g., with `serde`) to hold settings like `start_theme_index`, `end_theme_index`, `sleep_duration`, and `fade_duration`.
  - [ ] **Implement Save/Load Logic:** On application startup and shutdown, serialize/deserialize the config struct to a file (e.g., `config.json`).

---

## Phase 3 (Optional): Test Harness Polish

These tasks improve the standalone test application but do not contribute directly to the Zed extension. They should only be worked on if deemed necessary for improving the development experience.

### 1. UX Enhancements

- **Description:** Implement several small but high-impact UX improvements to make the test application feel more polished.
- **Status:** [ ] Not Started
- **Priority:** Very Low
- **Tasks:**
  - [ ] **Implement "Click-Away-to-Close" for Dropdowns.**
  - [ ] **Add Visual Cues for Disabled Dropdown Items.**
  - [ ] **Add Standard Window Management Keybindings (`cmd-q`, `cmd-w`, `cmd-m`).**

---

## Completed Milestones

- [x] **Smooth Theme Animation:** Implemented a fix to ensure the theme transition animation renders smoothly by forcing UI redraws from the background thread.
- [x] **Robust Theme Parsing:** Updated the theme parser to correctly handle non-color string values, eliminating startup warnings.
- [x] **Dynamic Theme Loading:** Replaced hardcoded themes with a dynamic system that loads all `.json` files from the `assets/` directory.
- [x] **Component Extraction:** Began refactoring the UI by extracting components like `Button`, `Panel`, and `Dropdown` into the `src/components/` directory.
- [x] **Independent Theme Selectors:** Refactored the UI to allow independent selection of start and end themes.
- [x] **Full Keyboard Navigation:** Implemented comprehensive keyboard controls for all interactive elements.
- [x] **Auto-Scrolling Dropdowns:** Implemented manual scroll logic to ensure the highlighted item in a dropdown is always visible.
