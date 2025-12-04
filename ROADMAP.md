# Project Roadmap: GPUI Theme Scheduler

**Last Updated:** 2025-12-04

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
  - [x] Replace the global `Enter` -> `Submit` binding with `Cmd+Enter` -> `Submit` in `main.rs`.
  - [x] Assign a specific key context (e.g., "RunButton") to the "Run Simulation" button in `ui.rs`.
  - [x] Bind `Enter` -> `Submit` specifically for the "RunButton" context in `main.rs`.

### 2. Fix Enter Key Handling in Dropdowns

- **Description:** Enter key events are not handled correctly by the dropdown components.
- **Status:** `[x] Completed`
- **Priority:** High
- **Tasks:**
  - [x] Investigate the interaction between `on_confirm_theme` and dropdown state.
  - [x] Ensure `Enter` key press consistently opens/closes the dropdown without flashing.
  - [x] Ensure focus is correctly managed after opening/closing with Enter.
  - [x] Verify that selection is correctly applied when `Enter` is used to confirm.

### 3. Implement "Click-Away-to-Close" for Dropdowns

- **Description:** Add the standard UX behavior of closing a dropdown menu when the user clicks anywhere outside of it.
- **Status:** `[x] Completed`
- **Priority:** Medium (Easy Win)
- **Tasks:**
  - [x] In `AppView::render`, when a dropdown is open, register a one-time global mouse-down listener.
  - [x] This listener will check if the click occurred outside the bounds of the open dropdown.
  - [x] If the click is outside, it will dispatch an action to set the dropdown's `is_open` state to `false`.

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
  - [x] Implement a "Root Focus Handle" for the `InteractiveUI` container.
  - [x] Request focus for the root handle on application startup to enable immediate Tab navigation.
  - [x] Ensure clicking the background focuses the root handle, releasing focus from specific inputs.

### 6. Fix Dropdown Toggle Logic (The "Enter" Bug)

- **Description:** Fix persistent issues where the `Enter` key fails to toggle the dropdown open/closed reliably after certain interactions (e.g., after closing it, immediate re-opening fails).
- **Status:** `[x] Completed`
- **Priority:** High
- **Tasks:**
  - [x] Investigate the `on_confirm_theme` vs. `on_toggle` logic.
  - [x] Ensure consistent state transitions regardless of whether the action was triggered via Mouse Click or Enter Key.

### 7. Standard Application Behaviors

- **Description:** Implement standard OS-level application behaviors and identity.
- **Status:** `[x] Completed`
- **Priority:** Medium
- **Tasks:**
  - [x] **Window Controls:** Add standard key bindings for `Cmd+W` (Close Window) and `Cmd+Q` (Quit).
  - [ ] **App Bundle ID:** Investigated. Setting a custom Bundle ID requires a full App Bundle structure (Info.plist), which is outside the scope of `cargo run`. Skipped for now.

### 8. Text Input Polish

- **Description:** Implement standard text selection behaviors to match user expectations.
- **Status:** `[x] Completed`
- **Priority:** Low
- **Tasks:**
  - [x] **Double-Click:** Select the word under the cursor.
  - [x] **Triple-Click:** Select the entire line/content.
  - [x] **Blur Behavior:** Ensure selection visibility behaves correctly when the input loses focus (optional, but nice to have).

### 9. Theme Consistency Audit

- **Description:** Ensure all UI components use colors from the active theme instead of hardcoded values.
- **Status:** `[x] Completed`
- **Priority:** Low
- **Tasks:**
  - [x] **Button Component:** Update `render_button` to use themed border and hover colors.
  - [x] **Panel Component:** Update `render_panel` to accept theme/cx and use themed border color.
  - [x] **Form Field Component:** Update `render_form_field` to accept theme/cx and use themed border and error colors.

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
- **Status:** `[x] Completed`
- **Priority:** Low
- **Tasks:**
  - [x] When the `ThemeScheduler` finishes, it will send a final `SimulationState::Idle` message.
  - [x] Upon receiving this message, `AppView` will:
    - [x] Set `app_state.start_theme_index = app_state.end_theme_index`.
    - [x] Advance `app_state.end_theme_index` to the next theme in the list (wrapping around if necessary) to ensure the start and end themes are different.

---

## Phase 4: Advanced Features & Polish

This phase focuses on deepening the application's utility and polishing the visual experience.

### 1. Window Configuration

- **Description:** Ensure the application window behaves like a native desktop app.
- **Status:** `[x] Completed`
- **Priority:** Medium
- **Tasks:**
  - [x] Set the window title to "Theme Scheduler".
  - [x] Enforce a minimum window size (e.g., 400x600) to prevent layout breakage.

### 2. Blinking Cursor

- **Description:** Implement a standard blinking cursor in text inputs for a more natural typing experience.
- **Status:** `[x] Completed`
- **Priority:** Low
- **Tasks:**
  - [x] Implement an animation loop or timer in `TextInput` to toggle cursor visibility (approx. 500ms interval).
  - [x] Ensure blinking pauses/resets on user typing.

### 3. Zed Theme Integration

- **Description:** Seamlessly integrate with the user's existing Zed environment and allow testing of external theme files.
- **Status:** `[x] Completed`
- **Priority:** High
- **Tasks:**
  - [x] **Auto-Load:** At startup, detect and scan the standard Zed theme directory (e.g., `~/.config/zed/themes`) for JSON theme files.

### 4. Text Input Focus UX

- **Description:** Improve user experience when navigating between inputs.
- **Status:** `[x] Completed`
- **Priority:** Low
- **Tasks:**
  - [x] Implement "Select All" behavior when a text input gains focus via keyboard navigation (Tab), allowing immediate overwriting of the value.

### 5. Dropdown Menu Ordering

- **Description:** Implement consistent and user-friendly sorting for themes displayed in dropdown menus.
- **Status:** `[ ] Not Started`
- **Priority:** Medium
- **Tasks:**
  - [ ] Sort the combined list of themes (assets + user-loaded) alphabetically by theme name.

### 6. Theme Hinting

- **Description:** Provide subtle, persistent in-app guidance on how to load custom themes from the user's Zed configuration directory.
- **Status:** `[x] Completed`
- **Priority:** Low
- **Tasks:**
  - [x] Implement a subtle footer text at the bottom of the UI, indicating the path for custom theme files (~/.config/zed/themes).

### 7. Centralized Focus & Border Styling

- **Description:** Improve visual accessibility and consistency by centralizing border and focus state logic for inputs in the `form_field` component.
- **Status:** `[x] Completed`
- **Priority:** High
- **Tasks:**
  - [x] **Refactor `render_form_field`:** Update to accept `is_focused: bool` and manage all border and focus ring styling (using 2px width for focus).
  - [x] **Update `render_dropdown`:** Remove internal border styling from the dropdown button, delegating it to the parent `form_field`.
  - [x] **Update `ui.rs`:** Pass `is_focused` state (derived from focus handles) to all `render_form_field` calls.
  - [x] **Update `render_button`:** Ensure standalone buttons match the new 2px focus style for consistency.

### 8. Vertical Layout Exploration

- **Description:** Refactor the UI to use a vertical stacking layout (Top Label, Full-width Input) to improve visual balance and accommodate larger previews.
- **Status:** `[x] Completed`
- **Priority:** Medium
- **Tasks:**
  - [x] Refactor `render_form_field` to use a vertical flex column (Label on top of Input).
  - [x] Remove fixed width constraints on inputs to allow full-width expansion.
  - [x] Integrate gradient previews into the main flow (optional).

### 9. Gradient Preview Refinement

- **Description:** Audit and expand the set of theme keys visualized in the gradient panel to better represent the theme's character.
- **Status:** `[x] Completed`
- **Priority:** Low
- **Tasks:**
  - [x] Update `key_colors` list with high-impact keys (`editor.background`, `text.accent`).
  - [x] Reorder gradients by visual priority.

## Phase 5: Editor Preview

This phase focuses on creating a realistic "Code Editor" preview to demonstrate how the theme applies to syntax highlighting and UI elements like tabs and file trees.

### 1. CodePreview Component Scaffolding

- **Description:** Create the basic structure of the editor preview component, including the container, tab bar, and line number gutter.
- **Status:** `[x] Completed`
- **Priority:** High
- **Tasks:**
  - [x] Create `src/components/code_preview.rs`.
  - [x] Implement layout: Header (Tabs), Sidebar (Gutter), Main Content (Code).
  - [x] Apply theme backgrounds (`editor.background`, `tab.active.background`).

### 2. Static Code Data

- **Description:** Define the internal data structures to represent a syntax-highlighted code snippet.
- **Status:** `[x] Completed`
- **Priority:** High
- **Tasks:**
  - [x] Define `Token` struct (text, color_key).
  - [x] Hardcode a representative Rust code snippet using these tokens.

### 3. Token Rendering

- **Description:** Implement the rendering logic to display the code snippet with correct theme colors.
- **Status:** `[x] Completed`
- **Priority:** High
- **Tasks:**
  - [x] Map theme syntax keys (e.g., `keyword`, `string`, `function`) to colors.
  - [x] Render lines and tokens with proper indentation.

### 4. Integration

- **Description:** Replace the existing Gradient Preview panel with the new Editor Preview component.
- **Status:** `[x] Completed`
- **Priority:** Medium
- **Tasks:**
  - [x] Replace `render_gradient_bar` loop in `ui.rs` with `render_code_preview`.
  - [x] Ensure layout responsiveness.

### 5. Editor Preview: Structure Refactor

- **Description:** Reorganize the code preview component into a dedicated module folder to support sub-components.
- **Status:** `[x] Completed`
- **Priority:** High
- **Tasks:**
  - [x] Move `code_preview.rs` to `src/components/editor_preview/mod.rs`.
  - [x] Update module registration and imports.

### 6. Editor Preview: Sidebar Components

- **Description:** Implement the File Tree to simulate the left side of the IDE.
- **Status:** `[x] Completed`
- **Priority:** Medium
- **Tasks:**
  - [x] Implement `file_tree` (hierarchical file list).
  - [x] Integrate into main preview layout.
  - [x] (Activity Bar was deprecated in favor of Status Bar toggle integration).

### 7. Editor Preview: Decor Components

- **Description:** Implement "Breadcrumbs" and a high-fidelity "Status Bar" to complete the IDE look.
- **Status:** `[x] Completed`
- **Priority:** Low
- **Tasks:**
  - [x] Implement `breadcrumbs` (path navigation).
  - [x] Implement `status_bar` with interactive-style icon buttons matching Zed's design.
  - [x] Final assembly of the 3-pane layout.

### 8. Terminal Tab Integration

- **Description:** Add a "Terminal" tab to the Editor Preview to demonstrate terminal-specific theme keys and provide a complete IDE feel. Using `Terminal.tsx`, `TerminalShell.tsx`, `TerminalTabs.tsx` files in `sandbox/zed-themes-source/zed-themes/app/components/preview/components/` as inspiration.
- **Status:** [ ] Pending
- **Priority:** High
- **Tasks:**
  - [x] Create `src/components/editor_preview/terminal.rs`.
  - [x] Implement `render_terminal` with a mock shell session (prompt, command, output).
  - [ ] Update `editor_preview/mod.rs` to handle tab switching logic (Code vs Terminal).
  - [ ] Map terminal theme keys (e.g., `terminal.background`, `terminal.foreground`) to the new component.

## Phase 6: Simulation Control & Preview Enhancements

This phase focuses on adding user control over simulation parameters and expanding the editor preview with more realistic features.

### 1. Simulation Frame Rate Control

- **Description:** Allow users to configure the simulation refresh rate (FPS) to test performance impact or smoother animations.
- **Status:** [x] Completed
- **Priority:** Medium
- **Tasks:**
  - [x] Add `fps` field to `AppState` (default 60) and a new `ValidatedInputState`.
  - [x] Add a new `ValidatedInput` to the UI for "Target FPS" (Range: 1-120).
  - [x] Pass `target_fps` to `simulation::run_simulation_core` and `ThemeScheduler`.
  - [x] Update `ThemeScheduler::run_fade_loop` to calculate thread sleep duration based on the user's requested FPS.

---

## Known Issues

- **Zed Mono Font Availability:** The "Zed Mono" font is currently not bundled with the application and relies on system-wide installation. For consistent rendering, especially across different operating systems or environments without Zed installed, the font files (IBM Plex Sans and Lilez) need to be properly integrated and loaded within the GPUI application. This should be addressed for a production-ready Zed extension.
  - **Proposed Solution:** Bundle the font files in `assets/` and use GPUI's font loading API to make them available at startup.

- **Background Scheduler Race Condition:** When closing the window with `Cmd+W`, the application process may persist, and the console may log `ERROR gpui: window not found`. This occurs because the background scheduler thread outlives the UI window and attempts to dispatch updates to a closed window. `Cmd+Q` avoids this by terminating the process immediately.
