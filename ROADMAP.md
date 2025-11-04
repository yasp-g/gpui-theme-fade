# Project Roadmap

**Last Updated:** 2025-11-02
**Current Goal:** Resolve the final compilation error.

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

### Implementation Notes & Refinements (2025-11-02)

Significant progress has been made, and we are down to a single compilation error.

The latest round of fixes addressed a number of issues:

- **Dependency Management:** Added missing dependencies (`futures`, `once_cell`, `regex`) to `Cargo.toml` and corrected the `schemars` version to `1.0.4` to resolve a version conflict with `gpui`. The invalid `typely` crate was also removed.
- **GPUI API Corrections:**
  - **Actions:** Corrected `dispatch_action` calls to pass references (`&MyAction`) instead of values, which was causing `mismatched types` errors.
  - **Styling:** Replaced incorrect `.border()` calls with the correct `.border_1()`.
  - **Stateful Elements:** Made clickable `div`s stateful by adding unique IDs with `.id()`, which is required to use `on_click` handlers.
  - **Event Listeners:** Corrected the closure signature for `on_click` listeners to match the expected four arguments.
  - **Return Types:** Adjusted a function's return type from `Div` to `impl IntoElement` to correctly handle the `Stateful<Div>` type returned after adding an `.id()`.
  - **Placeholders:** Replaced an incorrect `List::new()` with `div()` for a placeholder element.
  - **Invalid Methods:** Removed a call to `.z_index()`, which is not a valid method on `Div`. Layering will be addressed separately.

- **Remaining Issue:** The final compilation error is in `src/main.rs`, where a `.unwrap()` call is being incorrectly applied to a tuple returned from `cx.read_global`. This will be the next focus.

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

- [x] **Interactive UI (`AsyncApp::render`)**
  - [x] The `render` method will conditionally render the UI based on the `app_mode`.
  - [x] If `app_mode` is `AppMode::Scheduler`, the view can remain as it is (or show a simple status).
  - [x] If `app_mode` is `AppMode::Interactive`, render the following controls:
    - [x] **Main Container:** A root `div` element.
    - [x] **Current Theme Display:** A `text` element showing the name of the current base theme.
    - [x] **Next Theme Selector (Dropdown):**
      - [x] A clickable `div` to toggle the `dropdown_open` state.
      - [x] A conditionally rendered `list` of available themes.
      - [x] Each item in the list will be clickable to update `selected_theme_index`.
    - [x] **Sleep Duration Input:** A `TextInput` view for `sleep_duration_seconds`.
    - [x] **Fade Duration Input:** A `TextInput` view for `fade_duration_seconds`.
    - [x] **"Run Simulation" Button:**
      - [x] A clickable `div` with a `text` label.
      - [x] The `on_click` handler will spawn a new thread to run the one-shot simulation as described in the "Design & Simulation Strategy" section.

### Implementation Notes & Refinements (2025-10-31)

After setting up the initial UI module, we encountered a series of complex compilation errors.

Key takeaways and fixes:

- **Standardizing Async Runtimes:** A major source of errors was the use of two different MPSC channel implementations (`tokio` vs. `futures`). We resolved this by standardizing on `futures::channel::mpsc` throughout the application, as it was already in use by the `ThemeScheduler`. This is a common issue when integrating different async libraries.

- **GPUI Trait System:**
  - **`AppContext` and Trait Bounds:** We discovered that a generic parameter `<T: AppContext>` is not always sufficient. For methods like `update_global`, the more specific `gpui::BorrowAppContext` trait bound was required.
  - **`dyn` Compatibility:** We learned that `AppContext` is not "dyn-safe" (cannot be made into a `dyn AppContext` trait object) because it has generic associated types. This forced us to use generics (`<T: ...>`) in our function signatures.
  - **Redundant `refresh()`:** We found that `cx.update_global(...)` automatically triggers a UI refresh, making a subsequent call to `cx.refresh()` unnecessary.

- **The Borrow Checker in `render`:** We resolved a classic borrow-checker error (`E0502`) in `AppView::render`. The context `cx` was being borrowed immutably to get the `app_state`, and then mutably to be passed to a child UI function. We fixed this by `.clone()`-ing the `app_state`, breaking the first borrow and allowing the second mutable borrow to succeed.

- **Public API Visibility:** A `private_interfaces` warning taught us that a public function (`render_interactive_ui`) cannot have a private type (`AppView`) in its signature. We resolved this by making `AppView` a `pub struct`.

---

<details>
<summary><b>Archive: Initial Compilation Fixes</b></summary>

The initial development phase focused on resolving a critical compilation deadlock in `src/main.rs`.

- **Problem:** A `cx.spawn` closure created a lifetime conflict. The `async move` block required a `'static` lifetime, but it was being passed a non-static `&mut AsyncApp` reference, which it could not hold across an `.await` point.
- **Solution:** The issue was resolved by cloning the `AsyncApp` context inside the closure. This created an owned value that could be safely moved into the `async` block, satisfying the borrow checker.

</details>
