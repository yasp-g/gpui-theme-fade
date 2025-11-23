# Project Roadmap: GPUI Theme Scheduler

**Last Updated:** 2025-11-20

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

This phase focuses on fixing usability bugs and adding quality-of-life features to make the standalone demo a polished and shareable showcase of the core logic.

### 1. Implement Standard Form Submission UX

- **Description:** Adopt a standard form submission model where `Cmd+Enter` submits globally, and `Enter` only submits when the "Run Simulation" button is focused. This prevents accidental submissions while typing in text fields.
- **Status:** `[ ] Not Started`
- **Priority:** High
- **Tasks:**
  - [ ] Replace the global `Enter` -> `Submit` binding with `Cmd+Enter` -> `Submit` in `main.rs`.
  - [ ] Assign a specific key context (e.g., "RunButton") to the "Run Simulation" button in `ui.rs`.
  - [ ] Bind `Enter` -> `Submit` specifically for the "RunButton" context in `main.rs`.

### 2. Fix Enter Key Handling in Dropdowns

- **Description:** Enter key events are not handled correctly by the dropdown components.
- **Flashing Bug:** When a closed dropdown is focused and Enter is hit, the menu flashes open and immediately closes. When the menu is already opened and Enter is hit, the menu flashes closed and immediately reopens, but the new selection (if one has been navigated to with the arrow keys) is recorded.
- **Focus Bug:** After closing a dropdown with Enter, pressing Enter again does not re-open it. After opening a dropdown with Enter, pressing Enter again does not close it unless another interaction (like arrow keys or mouse hover) occurs first.
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

### 5. Implement Simulation State Machine

- **Description:** Introduce a formal state machine to manage the application's state during a simulation. This will provide clear, real-time UX feedback and prevent users from starting multiple simulations at once.
- **Status:** `[ ] Not Started`
- **Priority:** Medium
- **Tasks:**
  - [ ] **Create `src/state.rs` Module:**
    - [ ] Define a public `SimulationState` enum with variants: `Idle`, `Sleeping { end_time: DateTime<Utc> }`, and `Fading { end_time: DateTime<Utc>, from: String, to: String }`.
    - [ ] Implement a `display()` method on `SimulationState` that returns the formatted status string for the UI (e.g., "Status: Sleeping for 3.2s", "Status: Fading... (55%)").
  - [ ] **Refactor `AppView`:**
    - [ ] Add a `simulation_state: SimulationState` field to the `AppView` struct in `main.rs`.
  - [ ] **Update `ThemeScheduler`:**
    - [ ] Modify the scheduler to send `SimulationState` updates over the `mpsc` channel instead of `InterpolatableTheme`.
  - [ ] **Update UI (`ui.rs`):**
    - [ ] The UI will now render conditionally based on `AppView.simulation_state`.
    - [ ] When not `Idle`, the "Run Simulation" button and all inputs will be disabled.
    - [ ] A new `div` will be added to display the formatted status string from `simulation_state.display()`.

### 6. Improve Post-Simulation UX

- **Description:** After a simulation concludes, update the UI to reflect the new state logically and prevent user confusion.
- **Status:** `[ ] Not Started`
- **Priority:** Low
- **Tasks:**
  - [ ] When the `ThemeScheduler` finishes, it will send a final `SimulationState::Idle` message.
  - [ ] Upon receiving this message, `AppView` will:
    - [ ] Set `app_state.start_theme_index = app_state.end_theme_index`.
    - [ ] Advance `app_state.end_theme_index` to the next theme in the list (wrapping around if necessary) to ensure the start and end themes are different.

---

## Bugs / Missing Features Noticed During Development

### 1. Focus Navigation Initiation

- `tab` and `shift + tab` do nothing until a item is focused with the mouse
- If nothing is focused, `tab` should focus the top component, `shift + tab` should focus the last element

### 2. Dropdown Enter Bug

- After closing a dropdown menu with Enter, immediately pressing Enter does nothing (i.e. it does not open the dropdown again)
- After opening a dropdown menu with Enter, immediately pressing Enter does nothing (i.e. it does not close the dropdown (with the same selection))

### 3. `cmd + W` Does nothing

- Common keybindings like `cmd + W` and `cmd + M` are missing
- Is there a common set that are standard and simple to include?

### 4. Multi-Click Text Control

- We should implement the standard UX for double and triple clicking text for the textinputs
- If text in a textinput is highlighted, clicking away (even to another textinput) doesn't un-do the highlight (this feels like incorrect UX, right?)
