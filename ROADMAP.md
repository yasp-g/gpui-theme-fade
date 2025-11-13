# Project Roadmap: GPUI Theme Scheduler

**Last Updated:** 2025-11-11

## Project Overview

An application to test and demonstrate smooth, animated theme transitions for the Zed editor. It features an interactive UI for on-demand testing and a scheduler for time-based theme changes. The primary goal is to produce a robust, maintainable, and user-friendly tool for developing and showcasing a theme-scheduling Zed extension.

## Core Objectives

1.  **Architectural Integrity:** Build a maintainable and scalable application by decoupling components and simplifying state management.
2.  **User Experience:** Provide a polished, intuitive, and responsive interface for controlling theme simulations.
3.  **Functional Robustness:** Ensure the core theme transition logic is reliable and performs smoothly under all conditions.

---

## Phase 1: Architectural Refactoring & UX Polish

This phase focuses on improving the foundational architecture of the application to make it more modular, maintainable, and scalable. These changes will simplify the codebase and make it easier to add new features in the future.

### 1. State Management Refactoring

- **Description:** Decouple ephemeral UI state from the global `AppState` by moving it into the components that own it. This is the highest-priority architectural improvement.
- **Status:** [ ] Not Started
- **Priority:** High
- **Tasks:**
  - [ ] **Simplify `AppState`:** Remove UI-specific state fields (`start_dropdown_open`, `end_dropdown_open`, `start_preview_index`, `end_preview_index`, `sleep_input_validation_message`, `fade_input_validation_message`) from the `AppState` struct in `src/main.rs`.
  - [ ] **Create Stateful `Dropdown` Component:** Refactor the `dropdown.rs` component to be a proper stateful GPUI view.
    - [ ] It should manage its own internal state, such as `is_open` and `preview_index`.
    - [ ] It will receive the list of themes and the currently selected index as props.
    - [ ] It will use a callback prop (e.g., `on_select`) to notify the parent view of a selection change.
  - [ ] **Update UI:** Modify `src/ui.rs` to use the new stateful `Dropdown` component, passing in the required props and callbacks.
  - [ ] **Create Stateful `ValidatedInput` Component:** Extract the text inputs and their validation logic into a new reusable component.
    - [ ] This component will manage its own validation message state.

### 2. `AppView` Simplification

- **Description:** Slim down the `AppView` "God Object" by extracting distinct areas of logic into more focused, testable modules.
- **Status:** [ ] Not Started
- **Priority:** Medium
- **Tasks:**
  - [ ] **Extract Simulation Logic:**
    - [ ] Create a new module (e.g., `src/simulation.rs`).
    - [ ] Move the core logic from `AppView::run_simulation` into a function within this new module. This function will take the necessary parameters (themes, durations) and handle spawning the async task.
    - [ ] `AppView::run_simulation` will now be a thin wrapper that calls this new function.
  - [ ] **Extract Scroll Logic:**
    - [ ] Create a new helper function (e.g., in `src/ui.rs` or a new `src/components/util.rs`).
    - [ ] This function will contain the duplicated manual scroll calculation logic from `AppView::select_next_theme` and `AppView::select_prev_theme`.
    - [ ] Refactor the `AppView` methods to call this new, single helper function.

### 3. UX Enhancements

- **Description:** Implement several small but high-impact UX improvements to make the application feel more polished and professional.
- **Status:** [ ] Not Started
- **Priority:** Medium
- **Tasks:**
  - [ ] **Implement "Click-Away-to-Close" for Dropdowns:**
    - [ ] When a dropdown is open, render a transparent, full-screen `div` underneath it.
    - [ ] This `div` will have an `on_click` handler that closes the active dropdown.
  - [ ] **Add Visual Cues for Disabled Dropdown Items:**
    - [ ] In the `dropdown.rs` component, when rendering the list of themes, identify if a theme is disabled (i.e., it's the currently selected theme in the _other_ dropdown).
    - [ ] Apply a distinct visual style (e.g., `opacity-50`, `text_color` change) to disabled items to make them clearly non-selectable.
  - [ ] **Add Standard Window Management Keybindings:**
    - **Description:** Implement standard, platform-conventional key commands for quitting the app, closing a window, and minimizing a window.
    - **Status:** [ ] Not Started
    - **Priority:** Medium
    - **Tasks:**
      - [ ] **1. Define Actions:** In `src/main.rs`, locate the `// --- 1. ACTIONS ---` section. Define three new, empty structs named `Quit`, `MinimizeActiveWindow`, and `CloseActiveWindow`. Each struct should derive the `Clone`, `PartialEq`, and `Action` traits.
      - [ ] **2. Bind Keys:** In the `main` function of `src/main.rs`, find the `cx.bind_keys([...])` call. Add three new `KeyBinding` instances to the array.
        - Bind the "cmd-q" keystroke to the `Quit` action.
        - Bind the "cmd-w" keystroke to the `CloseActiveWindow` action.
        - Bind the "cmd-m" keystroke to the `MinimizeActiveWindow` action.
        - Ensure these bindings are global by passing `None` for the key context argument.
      - [ ] **3. Implement Global Handlers:** In the `main` function, directly after the `cx.bind_keys` call, add three `cx.on_action(...)` closures to handle the new actions.
        - The handler for the `Quit` action should call `cx.quit()` to terminate the application.
        - The handler for `MinimizeActiveWindow` should get the active window via `cx.active_window()`. If a window is present, it should call `window.update(...)` and, within the update closure, call `window.minimize_window()`.
        - The handler for `CloseActiveWindow` should follow the same pattern as the minimize handler, but call `window.remove_window()` inside the update closure to close it.

---

## Phase 2: New Features

Once the architecture is solidified, we can begin adding new functionality.

### 1. Camouflaged Colors

- **Description:** During faders, certain theme colors blend too closely together, making them difficult to distinguish (text becomes hidden in the background, for example).
- **Status:** [ ] Not Started
- **Priority:** Medium-High
- **Tasks:**
  - [ ] **Recreate issue** Find examples of themes where this occurs (user task)
  - [ ] **Determine Root Cause**
  - [ ] **Devise Solution**
  - [ ] **Implement Solution**

### 2. Configuration Persistence

- **Description:** Save and load the user's settings to provide a consistent experience between sessions.
- **Status:** [ ] Not Started
- **Priority:** Low
- **Tasks:**
  - [ ] **Define Config Struct:** Create a new struct that can be serialized/deserialized (e.g., with `serde`) to hold settings like `start_theme_index`, `end_theme_index`, `sleep_duration`, and `fade_duration`.
  - [ ] **Implement Save Logic:** On a clean application shutdown or after a setting is changed, serialize the config struct to a file (e.g., `config.json`).
  - [ ] **Implement Load Logic:** On application startup, check for the existence of `config.json` and load its values into the initial `AppState`.

---

## Completed Milestones

- [x] **Smooth Theme Animation:** Implemented a fix to ensure the theme transition animation renders smoothly by forcing UI redraws from the background thread.
- [x] **Robust Theme Parsing:** Updated the theme parser to correctly handle non-color string values, eliminating startup warnings.
- [x] **Dynamic Theme Loading:** Replaced hardcoded themes with a dynamic system that loads all `.json` files from the `assets/` directory.
- [x] **Component Extraction:** Began refactoring the UI by extracting components like `Button`, `Panel`, and `Dropdown` into the `src/components/` directory.
- [x] **Independent Theme Selectors:** Refactored the UI to allow independent selection of start and end themes.
- [x] **Full Keyboard Navigation:** Implemented comprehensive keyboard controls for all interactive elements, including focus cycling (`Tab`), submission (`Enter`), and dropdown navigation (`Up`/`Down`/`Enter`/`Escape`).
- [x] **Auto-Scrolling Dropdowns:** Implemented manual scroll logic to ensure the highlighted item in a dropdown is always visible during keyboard navigation.
