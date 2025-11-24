# Project Roadmap: GPUI Theme Scheduler

**Last Updated:** 2025-11-23

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
- **Status:** [x] Completed
- **Priority:** Medium
- **Tasks:**
  - [x] **Extract Simulation Logic:**
    - [x] Created a new module `src/simulation.rs`.
    - [x] Moved the core simulation spawning and theme application logic from `AppView::run_simulation` into `simulation::run_simulation_core`.
    - [x] `AppView::run_simulation` is now a thin wrapper that handles validation and calls `simulation::run_simulation_core`.
  - [x] **Isolate UI-Specific Logic:**
    - [x] Created a private helper method `AppView::scroll_dropdown_to_preview_index` within `src/main.rs`.
    - [x] Moved the duplicated manual scroll calculation logic from `AppView::select_next_theme` and `AppView::select_prev_theme` into this new helper.
    - [x] Refactored both `AppView::select_next_theme` and `AppView::select_prev_theme` to call this single helper function.

---

## Completed Milestones

- [x] **Smooth Theme Animation:** Implemented a fix to ensure the theme transition animation renders smoothly by forcing UI redraws from the background thread.
- [x] **Robust Theme Parsing:** Updated the theme parser to correctly handle non-color string values, eliminating startup warnings.
- [x] **Dynamic Theme Loading:** Replaced hardcoded themes with a dynamic system that loads all `.json` files from the `assets/` directory.
- [x] **Component Extraction:** Began refactoring the UI by extracting components like `Button`, `Panel`, and `Dropdown` into the `src/components/` directory.
- [x] **Independent Theme Selectors:** Refactored the UI to allow independent selection of start and end themes.
- [x] **Full Keyboard Navigation:** Implemented comprehensive keyboard controls for all interactive elements.
- [x] **Auto-Scrolling Dropdowns:** Implemented manual scroll logic to ensure the highlighted item in a dropdown is always visible.

---

## Phase 2: Demo Polish & UX Enhancements

This phase focuses on fixing usability bugs, standardizing application behavior, and adding quality-of-life features to make the standalone demo a polished and shareable showcase.

### 1. Implement Standard Form Submission UX

- **Description:** Adopt a standard form submission model where `Cmd+Enter` submits globally, and `Enter` only submits when the "Run Simulation" button is focused. This prevents accidental submissions while typing in text fields.
- **Status:** `[x] Completed`
- **Priority:** High
- **Tasks:**
  - `[x]` Replace the global `Enter` -> `Submit` binding with `Cmd+Enter` -> `Submit` in `main.rs`.
  - `[x]` Assign a specific key context (e.g., "RunButton") to the "Run Simulation" button in `ui.rs`.
  - `[x]` Bind `Enter` -> `Submit` specifically for the "RunButton" context in `main.rs`.

### 2. Fix Enter Key Handling in Dropdowns

- **Description:** Enter key events are not handled correctly by the dropdown components.
- **Status:** `[x] Completed`
- **Priority:** High
- **Tasks:**
  - `[x]` Investigate the interaction between `on_confirm_theme` and dropdown state.
  - `[x]` Ensure `Enter` key press consistently opens/closes the dropdown without flashing.
  - `[x]` Ensure focus is correctly managed after opening/closing with Enter.
  - `[x]` Verify that selection is correctly applied when `Enter` is used to confirm.

### 3. Implement "Click-Away-to-Close" for Dropdowns

- **Description:** Add the standard UX behavior of closing a dropdown menu when the user clicks anywhere outside of it.
- **Status:** `[x] Completed`
- **Priority:** Medium (Easy Win)
- **Tasks:**
  - `[x]` In `AppView::render`, when a dropdown is open, register a one-time global mouse-down listener.
  - `[x]` This listener will check if the click occurred outside the bounds of the open dropdown.
  - `[x]` If the click is outside, it will dispatch an action to set the dropdown's `is_open` state to `false`.

### 4. Implement Static Gradient Previews

- **Description:** Decouple the color gradient bars from the live theme animation to make them static previews of the selected start and end themes.
- **Status:** `[x] Completed`
- **Priority:** High (Easy Win)
- **Tasks:**
  - [x] In `src/ui.rs`, modify the `render_gradient_bar` calls.
  - [x] The start color will be sourced from `app_state.themes[app_state.start_theme_index]`.
  - [x] The end color will be sourced from `app_state.themes[app_state.end_theme_index]`.

### 5. Robust Focus Management

- **Description:** Establish a clear focus hierarchy to ensure keyboard navigation works immediately upon launch and that clicking the background correctly resets focus to a neutral state.
- **Status:** `[x] Completed`
- **Priority:** High
- **Tasks:**
  - `[x]` Implement a "Root Focus Handle" for the `InteractiveUI` container.
  - `[x]` Request focus for the root handle on application startup to enable immediate Tab navigation.
  - `[x]` Ensure clicking the background focuses the root handle, releasing focus from specific inputs.

### 6. Fix Dropdown Toggle Logic (The "Enter" Bug)

- **Description:** Fix persistent issues where the `Enter` key fails to toggle the dropdown open/closed reliably after certain interactions (e.g., after closing it, immediate re-opening fails).
- **Status:** `[ ] Not Started`
- **Priority:** High
- **Tasks:**
  - [ ] Investigate the `on_confirm_theme` vs. `on_toggle` logic.
  - [ ] Ensure consistent state transitions regardless of whether the action was triggered via Mouse Click or Enter Key.

### 7. Standard Application Behaviors

- **Description:** Implement standard OS-level application behaviors and identity.
- **Status:** `[x] Completed`
- **Priority:** Medium
- **Tasks:**
  - [x] **Window Controls:** Add standard key bindings for `Cmd+W` (Close Window) and `Cmd+Q` (Quit).
  - [x] **App Bundle ID:** Investigated. Setting a custom Bundle ID requires a full App Bundle structure (Info.plist), which is outside the scope of `cargo run`. Skipped for now.

### 8. Text Input Polish

- **Description:** Implement standard text selection behaviors to match user expectations.
- **Status:** `[x] Completed`
- **Priority:** Low
- **Tasks:**
  - [x] **Double-Click:** Select the word under the cursor.
  - [x] **Triple-Click:** Select the entire line/content.
  - [ ] **Blur Behavior:** Ensure selection visibility behaves correctly when the input loses focus (optional, but nice to have).

---

## Phase 3: Simulation Logic & State Management

This phase focuses on the core business logic of the theme scheduler, introducing a formal state machine and improving the feedback loop.

### 1. Implement Simulation State Machine

- **Description:** Introduce a formal state machine to manage the application's state during a simulation. This will provide clear, real-time UX feedback and prevent users from starting multiple simulations at once.
- **Status:** `[x] Completed`
- **Priority:** Medium
- **Tasks:**
  - [x] **Create `src/state.rs` Module:**
    - [x] Define a public `SimulationState` enum with variants: `Idle`, `Sleeping { end_time: DateTime<Utc> }`, and `Fading { end_time: DateTime<Utc>, from: String, to: String }`.
    - [x] Implement a `display()` method on `SimulationState` that returns the formatted status string for the UI (e.g., "Status: Sleeping for 3.2s", "Status: Fading... (55%)").
  - [x] **Refactor `AppView`:**
    - [x] Add a `simulation_state: SimulationState` field to the `AppView` struct in `main.rs`.
  - [x] **Update `ThemeScheduler`:**
    - [x] Modify the scheduler to send `SimulationState` updates over the `mpsc` channel instead of `InterpolatableTheme`.
  - [x] **Update UI (`ui.rs`):**
    - [x] The UI will now render conditionally based on `AppView.simulation_state`.
    - [x] When not `Idle`, the "Run Simulation" button and all inputs will be disabled.
    - [x] A new `div` will be added to display the formatted status string from `simulation_state.display()`.

### 2. Improve Post-Simulation UX

- **Description:** After a simulation concludes, update the UI to reflect the new state logically and prevent user confusion.
- **Status:** `[ ] Not Started`
- **Priority:** Low
- **Tasks:**
  - [ ] When the `ThemeScheduler` finishes, it will send a final `SimulationState::Idle` message.
  - [ ] Upon receiving this message, `AppView` will:
    - [ ] Set `app_state.start_theme_index = app_state.end_theme_index`.
    - [ ] Advance `app_state.end_theme_index` to the next theme in the list (wrapping around if necessary) to ensure the start and end themes are different.

---

## Known Issues

- **Background Scheduler Race Condition:** When closing the window with `Cmd+W`, the application process may persist, and the console may log `ERROR gpui: window not found`. This occurs because the background scheduler thread outlives the UI window and attempts to dispatch updates to a closed window. `Cmd+Q` avoids this by terminating the process immediately.
