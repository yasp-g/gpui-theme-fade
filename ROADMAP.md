# Project Roadmap

**Last Updated:** 2025-10-30

**Current Goal:** Refactor the application from a passive, time-based scheduler into an interactive tool for testing theme transitions. This will accelerate development and debugging.

---

## Phase 2: Interactive Theme Tester UI

The current 24-hour scheduling simulation is not ideal for rapid testing. We will build a user interface to control the theme transitions manually.

### Core Features

1.  **UI Controls:**
    *   **Current Theme Display:** A simple text label to show the name of the active theme (e.g., "One Dark").
    *   **Next Theme Selector:** A dropdown menu to select the target theme for the transition.
    *   **Fade Duration Input:** A number or text input to specify the transition duration in seconds.
    *   **"Run Transition" Button:** A button to trigger the fade from the current to the selected next theme.

2.  **State Management:**
    *   The main application state will need to manage the list of available themes, the user's selections for the next theme and duration, and the name of the current base theme.

3.  **Logic Overhaul:**
    *   The existing `ThemeScheduler` and its background thread will be removed.
    *   The time-based scheduling logic in `main` will be replaced.
    *   The "Run Transition" button's click handler will spawn a new, self-contained `async` task. This task will perform the theme interpolation over the specified duration and send updates to the UI thread, much like the old `run_fade_loop`.

---

<details>
<summary><b>Archive: Initial Compilation Fixes</b></summary>

The initial development phase focused on resolving a critical compilation deadlock in `src/main.rs`.

-   **Problem:** A `cx.spawn` closure created a lifetime conflict. The `async move` block required a `'static` lifetime, but it was being passed a non-static `&mut AsyncApp` reference, which it could not hold across an `.await` point.
-   **Solution:** The issue was resolved by cloning the `AsyncApp` context inside the closure. This created an owned value that could be safely moved into the `async` block, satisfying the borrow checker.

</details>