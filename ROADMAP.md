# Project Roadmap

**Last Updated:** 2025-10-30
**Current Goal:** Begin the implementation of the detailed steps outlined for Phase 2.

**Next Goal:** Refactor the application from a passive, time-based scheduler into an interactive tool for testing theme transitions. This will accelerate development and debugging.

---

## Phase 2: Interactive Theme Tester UI

The current 24-hour scheduling simulation is not ideal for rapid testing. We will build a user interface to control and trigger the theme scheduler's logic manually. This UI will serve as a development and testing mode for the core scheduling engine.

### Design & Simulation Strategy

After discussion, the primary goal of the interactive UI has been clarified: **to test and validate the existing `ThemeScheduler` logic** in an accelerated, on-demand fashion, rather than creating a parallel implementation of the fade.

To achieve this, we will implement a one-shot simulation strategy:

1.  **Full Simulation:** The UI will provide inputs for `sleep_duration` and `fade_duration` to allow for a full simulation of the `ThemeScheduler::run_loop`, including both the initial sleep/wait phase and the active fade phase.
2.  **On-Demand Execution:** When the "Run Simulation" button is clicked, the application will spawn a new, temporary thread to run a single simulation.
3.  **Dynamic Schedule:** Inside this thread, a temporary two-event `schedule` will be created based on the current theme and the UI inputs.
4.  **Scheduler Modification:** The `ThemeScheduler` will be made aware of the `AppMode`. Its `run_loop` will be slightly modified to check for `AppMode::Interactive`. If this mode is detected, the loop will execute one full cycle (sleep and fade) and then `return`, cleanly terminating the simulation thread.

This approach allows us to use the real scheduler code in a controlled test environment.

### Implementation Notes & Refinements

During the initial refactoring, several unforeseen tasks were completed:

- **`AppState` as a `Global`:** The application's state management was centralized by creating a new `AppState` struct. This struct was made a `gpui::Global`, replacing the previous, simpler `ActiveTheme` global. This provides a single source of truth for the UI.
- **`Theme` Struct:** A new `Theme` struct was created to encapsulate a theme's name (`String`) and its `InterpolatableTheme` data. This is used in the `AppState.themes` vector.
- **`Default` Trait Implementation:** To allow `AppState` to derive the `Default` trait, a manual `impl Default for AppMode` was added, setting `AppMode::Scheduler` as the default.
- **Handling `Result` from `read_global`:** The `gpui::AsyncAppContext::read_global` function returns a `Result`. The implementation now correctly handles this by using `.expect()` to unwrap the value, ensuring that a failure to read the global state will result in a controlled panic.

### Detailed Steps

- [x] **State Management (`AppState`)**
    - [x] Modify the `AppState` struct in `src/main.rs`.
    - [x] Add an `app_mode: AppMode` enum field. This enum will have two variants: `Scheduler` and `Interactive`.
    - [x] Add a `themes: Vec<Theme>` field to store all available themes.
    - [x] Add a `selected_theme_index: usize` field for the interactive mode's dropdown.
    - [x] Add a `sleep_duration_seconds: f32` field for the simulation's sleep duration.
    - [x] Add a `fade_duration_seconds: f32` field for the simulation's fade duration.
    - [x] Add a `dropdown_open: bool` field to manage the visibility of the theme selector dropdown.

- [x] **Logic Overhaul & Modularization**
    - [x] **Create `scheduler` module:**
        - [x] Create a new file: `src/scheduler.rs`.
        - [x] Move the `ThemeScheduler` struct and its related implementation from `src/main.rs` into this new file.
        - [x] Declare `pub mod scheduler;` in `src/main.rs`.
    - [x] **Scheduler Refinement:**
        - [x] Add `app_mode: AppMode` to the `ThemeScheduler` struct.
        - [x] Modify the `run_loop` to accept the `app_mode` and include a conditional `return` to exit the loop after one cycle in `Interactive` mode.

- [x] **Conditional Logic in `main`:**
    - [x] In the `main` function, after initializing the `AppState`, check the value of `app_mode`.
    - [x] If `app_mode` is `AppMode::Scheduler`, spawn the `ThemeScheduler` background task to run continuously.
    - [x] If `app_mode` is `AppMode::Interactive`, the `ThemeScheduler` is **not** started automatically. It will be triggered on-demand by the UI.

- [ ] **Interactive UI (`AsyncApp::render`)**
    - [ ] The `render` method will conditionally render the UI based on the `app_mode`.
    - [ ] If `app_mode` is `AppMode::Scheduler`, the view can remain as it is (or show a simple status).
    - [ ] If `app_mode` is `AppMode::Interactive`, render the following controls:
        - [ ] **Main Container:** A root `div` element.
        - [ ] **Current Theme Display:** A `text` element showing the name of the current base theme.
        - [ ] **Next Theme Selector (Dropdown):**
            - [ ] A clickable `div` to toggle the `dropdown_open` state.
            - [ ] A conditionally rendered `list` of available themes.
            - [ ] Each item in the list will be clickable to update `selected_theme_index`.
        - [ ] **Sleep Duration Input:** A `TextInput` view for `sleep_duration_seconds`.
        - [ ] **Fade Duration Input:** A `TextInput` view for `fade_duration_seconds`.
        - [ ] **"Run Simulation" Button:**
            - [ ] A clickable `div` with a `text` label.
            - [ ] The `on_click` handler will spawn a new thread to run the one-shot simulation as described in the "Design & Simulation Strategy" section.


---

<details>
<summary><b>Archive: Initial Compilation Fixes</b></summary>

The initial development phase focused on resolving a critical compilation deadlock in `src/main.rs`.

- **Problem:** A `cx.spawn` closure created a lifetime conflict. The `async move` block required a `'static` lifetime, but it was being passed a non-static `&mut AsyncApp` reference, which it could not hold across an `.await` point.
- **Solution:** The issue was resolved by cloning the `AsyncApp` context inside the closure. This created an owned value that could be safely moved into the `async` block, satisfying the borrow checker.

</details>
