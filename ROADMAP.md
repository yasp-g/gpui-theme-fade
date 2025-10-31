# Project Roadmap

**Last Updated:** 2025-10-30
**Current Goal:** Begin the implementation of the detailed steps outlined for Phase 2.

**Next Goal:** Refactor the application from a passive, time-based scheduler into an interactive tool for testing theme transitions. This will accelerate development and debugging.

---

## Phase 2: Interactive Theme Tester UI

The current 24-hour scheduling simulation is not ideal for rapid testing. We will build a user interface to control the theme transitions manually. This UI will serve as a development and testing mode, while the core time-based scheduler functionality will be preserved.

### Detailed Steps

- [ ] **State Management (`AppState`)**
    - [ ] Modify the `AppState` struct in `src/main.rs`.
    - [ ] Add an `app_mode: AppMode` enum field. This enum will have two variants: `Scheduler` and `Interactive`.
    - [ ] Add a `themes: Vec<Theme>` field to store all available themes.
    - [ ] Add a `selected_theme_index: usize` field for the interactive mode's dropdown.
    - [ ] Add a `fade_duration_seconds: f32` field for the interactive mode's text input.
    - [ ] Add a `dropdown_open: bool` field to manage the visibility of the theme selector dropdown.

- [x] **Logic Overhaul & Modularization**
    - [x] **Create `scheduler` module:**
        - [x] Create a new file: `src/scheduler.rs`.
        - [x] Move the `ThemeScheduler` struct and its related implementation from `src/main.rs` into this new file.
        - [x] Declare `pub mod scheduler;` in `src/main.rs`.
    - [ ] **Conditional Logic in `main`:**
        - [ ] In the `main` function, after initializing the `AppState`, check the value of `app_mode`.
        - [ ] If `app_mode` is `AppMode::Scheduler`, spawn the `ThemeScheduler` background task as the application currently does.
        - [ ] If `app_mode` is `AppMode::Interactive`, the `ThemeScheduler` will not be started.

- [ ] **Interactive UI (`AsyncApp::render`)**
    - [ ] The `render` method of the `AsyncApp` view will conditionally render the UI based on the `app_mode`.
    - [ ] If `app_mode` is `AppMode::Scheduler`, the view can remain as it is (or show a simple status).
    - [ ] If `app_mode` is `AppMode::Interactive`, render the following controls:
        - [ ] **Main Container:** A root `div` element.
        - [ ] **Current Theme Display:** A `text` element showing the name of the current base theme.
        - [ ] **Next Theme Selector (Dropdown):**
            - [ ] A clickable `div` to toggle the `dropdown_open` state.
            - [ ] A conditionally rendered `list` of available themes.
            - [ ] Each item in the list will be clickable to update `selected_theme_index`.
        - [ ] **Fade Duration Input (Custom `TextInput` view):**
            - [ ] Define a new `TextInput` struct with `text: String` and a `focus_handle: FocusHandle`.
            - [ ] Implement the `Render` trait to draw the text.
            - [ ] Implement the `EntityInputHandler` trait to handle text input from the keyboard.
            - [ ] Instantiate this view in `AsyncApp` and register it for input handling using `cx.handle_input()`.
        - [ ] **"Run Transition" Button:**
            - [ ] A clickable `div` with a `text` label.
            - [ ] The `on_click` handler will spawn a new `async` task.
            - [ ] This task will perform the theme fade using the `selected_theme_index` and `fade_duration_seconds` from the state.


---

<details>
<summary><b>Archive: Initial Compilation Fixes</b></summary>

The initial development phase focused on resolving a critical compilation deadlock in `src/main.rs`.

- **Problem:** A `cx.spawn` closure created a lifetime conflict. The `async move` block required a `'static` lifetime, but it was being passed a non-static `&mut AsyncApp` reference, which it could not hold across an `.await` point.
- **Solution:** The issue was resolved by cloning the `AsyncApp` context inside the closure. This created an owned value that could be safely moved into the `async` block, satisfying the borrow checker.

</details>
