# Development Notes

## Architectural Decisions

### Theme Transition Strategy: CPU vs. GPU

**Decision:**
The application implements theme transitions using a **CPU-based state update** mechanism rather than a GPU-accelerated shader approach.

**Mechanism:**
1.  A background thread calculates the interpolated color values for every key in the theme at the target frame rate (e.g., 60 FPS).
2.  This new "frame" of theme data is sent to the main thread.
3.  The global `AppState` is updated, and the UI is notified (`cx.notify()`).
4.  GPUI re-renders the interface using the new color values.

**Justification:**
*   **Framework constraints:** GPUI's public API does not currently support injecting custom shaders into standard elements (like `div` or `text`) without modifying the engine's source code. Implementing a "true" GPU fade would require forking GPUI and rewriting the rendering pipeline for every primitive type (quads, shadows, glyphs, etc.).
*   **Integration goals:** As a Zed extension, this tool must modify the editor's actual settings source of truth (the JSON theme definition) to affect the entire editor globally. A visual-only shader effect would not propagate to other parts of the editor.
*   **Performance viability:** Unlike web-based editors (Electron), Zed and GPUI are native and highly optimized. The overhead of applying a global state change 60 times a second is negligible in this environment, allowing for smooth animations via CPU interpolation that might cause stuttering in a DOM-based architecture.

**Trade-offs:**
*   *Pros:* Simplicity, compatibility with all standard UI elements, and correct integration with Zed's settings system.
*   *Cons:* Higher CPU usage during transitions compared to a shader-based approach (though well within acceptable limits for modern hardware).

---

## GPUI-Specific Learnings

### 1. Unresolved Imports: `gpui::View` and `gpui::WindowContext`

- **Problem:** A recurring compilation error `error[E0432]: unresolved imports` occurs when trying to use `gpui::View` or `gpui::WindowContext` in the function signatures of component render functions or their callbacks. These types are not available in the public API in the way they might seem.

- **Solution:** Do not use generic type parameters like `<V: View>` for component functions. Instead, use the concrete view type directly (e.g., `AppView`). For contexts and callbacks, use `Context<AppView>` instead of `WindowContext`.

- **Example (Correct):** The `render_dropdown` component's `on_toggle` callback signature correctly uses `AppView` and `Context<AppView>`:

  ```rust
  // In src/components/dropdown.rs
  pub fn render_dropdown(
      // ...
      on_toggle: impl Fn(&mut AppView, &ClickEvent, &mut Window, &mut Context<AppView>) + 'static,
      // ...
      cx: &mut Context<AppView>,
  ) -> impl IntoElement
  ```

- **Example (Incorrect):**
  ```rust
  // This will not compile.
  pub fn render_dropdown<V: View>(
      // ...
      on_toggle: impl Fn(&mut V, &mut WindowContext) + 'static,
      // ...
      cx: &mut Context<V>,
  ) -> impl IntoElement
  ```
  This pattern ensures that components are correctly wired to the application's specific `AppView` and its context.

### 2. Manual Scrollbar Implementation

- **Problem:** Applying `.overflow_y_scroll()` to a `div` makes it scrollable with a mouse wheel, but no visible scrollbar (thumb or track) appears. There are no simple style properties like `scrollbar_color` to make it visible.

- **Discovery:** The base `gpui` framework does not automatically render a scrollbar UI. It only provides the scrolling _mechanics_. The developer is responsible for implementing the visual scrollbar as a separate component. The `data_table.rs` example in the GPUI source code is the canonical example of this pattern.

- **Solution:**
  1.  **Create a `ScrollHandle`:** State that needs to persist for the scrollable element (like the scroll offset) requires a handle. This handle should be stored in a persistent location, such as the `AppState` global, and passed down to the component that needs to scroll.
  2.  **Track the Element:** Use the `.track_scroll(&scroll_handle)` method on the scrollable `div` to associate it with the handle.
  3.  **Build a Scrollbar Component:** Create a separate component (e.g., `render_scrollbar`) that also takes the `ScrollHandle`.
  4.  **Render Manually:** Inside this component, read the state from the handle (`scroll_handle.bounds()`, `scroll_handle.max_offset()`) to calculate the size and position of the scrollbar thumb. Render the thumb as a `div` with an absolute position.
  5.  **Layout:** Render the scrollable `div` and the `render_scrollbar` component as siblings inside a parent `div` that has `relative()` positioning.

### 3. Implementing Custom `gpui::Element`s

- **Problem:** For complex, stateful, and custom-drawn UI components (like a draggable scrollbar thumb), simply returning a `div()` chain is insufficient. Direct drawing and fine-grained event handling are required.

- **Solution:** Implement the `gpui::Element` trait for a custom struct. This provides access to the low-level rendering pipeline and event system.

- **Key Learnings for `gpui::Element` Implementation:**
  - **Struct Definition:** Define a struct (e.g., `ScrollbarElement`) to hold the component's data (e.g., `id`, `scroll_handle`).
  - **`IntoElement` Trait:** Implement `IntoElement` for your custom struct, returning `Self`.
  - **`Element` Trait Implementation:**
    - **`id(&self) -> Option<ElementId>`:** Required. Returns the unique ID for the element.
    - **`source_location(&self) -> Option<&'static std::panic::Location<'static>>`:** Required. Can return `None` for simple cases.
    - **`request_layout(...)`:** Defines the element's layout properties. The `_inspector_id` parameter is `Option<&InspectorElementId>`.
    - **`prepaint(...)`:** Calculates geometry and state needed for painting. The `_inspector_id` parameter is `Option<&InspectorElementId>`. Store calculated values in `Self::PrepaintState`.
    - **`paint(...)`:** Performs the actual drawing and event handling.
      - **Method Signature:** The `paint` method takes 8 parameters, including `_inspector_id: Option<&InspectorElementId>`, `_bounds: Bounds<Pixels>`, `_layout: &mut Self::RequestLayoutState`, `prepaint_state: &mut Self::PrepaintState`, `window: &mut Window`, and `cx: &mut App`.
      - **Drawing:** Use `window.paint_quad(...)` with `gpui::quad(...)` to draw shapes.
      - **State Management:** Use `window.use_keyed_state(self.id.clone(), cx, |_, _| { ... })` to create or retrieve persistent state for the element. This returns an `Entity<YourState>`.
      - **Event Handling:** Use `window.on_mouse_event(move |event, phase, window, cx| { ... })` for fine-grained mouse interaction.
      - **`cx.refresh()` vs. `window.refresh()`:** When updating state within an `Element`'s event handler, use `window.refresh()` to trigger a redraw.

### 4. `ElementId` Creation

- **Problem:** Creating unique `ElementId`s for components, especially when dynamically generated or nested, can be tricky.
- **Solution:** `ElementId` implements `From` for several tuple types. The most reliable pattern for combining a static string with a dynamic identifier is `(static_str: &str, dynamic_id: usize)`.
- **Incorrect Patterns:** Directly using `String` from `format!` or `(&str, &str)` tuples are not supported conversions for `ElementId`.

### 5. Rust Ownership and Cloning in Closures

- **Problem:** When using `move` closures, especially for event handlers, variables captured from the outer scope are moved into the closure. If multiple closures need the same non-`Copy` variable, only the first one can take ownership, leading to "use of moved value" errors.
- **Solution:** For each `move` closure that needs a variable, create a `.clone()` of that variable _before_ the closure is defined. This ensures each closure receives its own independent copy, satisfying Rust's ownership rules.

  ```rust
  let original_var = ...;
  // For Closure 1
  let var_for_closure1 = original_var.clone();
  window.on_mouse_event(move |...| { /* use var_for_closure1 */ });

  // For Closure 2
  let var_for_closure2 = original_var.clone();
  window.on_mouse_event(move |...| { /* use var_for_closure2 */ });
  ```

This document contains the historical development notes and decision-making process that were previously in `ROADMAP.md`.

---

# Implementation Notes & Refinements (2025-11-11)

- **Robust Theme Parsing:** Fixed a bug that caused a flood of warnings on startup.
  - **Problem:** The theme parser was attempting to interpret any string value in the theme files (e.g., `"italic"`, `"opaque"`) as a hex color, leading to numerous "Invalid hex color" warnings.
  - **Solution:** The `flatten_colors` function in `src/theme.rs` was updated to be more selective. It now checks if a string value starts with a `#` before attempting to parse it as a color, correctly ignoring non-color style properties.

- **Theme Animation Bug Fix:** Resolved a major bug where the theme transition animation would not render smoothly.
  - **Problem:** The UI would only update during the theme fade when the user interacted with it (e.g., moving the mouse), as the background thread was not signaling the UI thread to redraw.
  - **Solution:** A call to `cx.refresh()` was added to the simulation task in `src/main.rs`. This call is made immediately after the global `active_theme` is updated, forcing a UI redraw for each step of the animation and resulting in a smooth transition.

# Implementation Notes & Refinements (2025-11-11) - _Decision Update & Course Correction_

- **New, Simplified Approach for Dropdowns (`.occlude()`):** After a deep investigation of the GPUI source code, the previous plan to use a modal system has been found to be **incorrect**. The `window.present_modal()` API does not exist, and the `ManagedView` trait is not used in the way previously assumed. A much simpler and more idiomatic solution has been discovered.
  - **Problem:** The dropdown menu visually appears on top of other elements but does not intercept mouse clicks.
  - **Discovery:** The correct way to implement this behavior in GPUI is to use the `.occlude()` method on the `div` element that serves as the dropdown's container. This method sets the underlying `HitboxBehavior` to `BlockMouse`, which explicitly prevents mouse events from "passing through" to elements rendered underneath it. This is the intended mechanism for overlays and popovers.
  - **New Plan:**
    1.  In `src/components/popover.rs`, locate the root `div` that renders the popover content.
    2.  Add the `.occlude()` method to this `div`'s style chain.
    3.  This single change is expected to resolve the layering bug without any complex refactoring.

- **(DEPRECATED) Old Approach for Dropdowns (Overlay System):**
  - **Reason for Deprecation:** The plan below was based on a misunderstanding of the GPUI API. It is preserved here for historical context.
  - ~~**Discovery:** The correct way to implement this in GPUI is to use the `window.present_modal()` API. This renders a view into a separate, top-level overlay layer that is guaranteed to capture all mouse events within its bounds.~~
  - ~~**Plan:**~~
    1.  ~~**Create a `DropdownMenu` View:** The content of the popover (the list of items) will be extracted into its own stateful view struct.~~
    2.  ~~**Implement `ManagedView`:** The new `DropdownMenu` view will implement the `ManagedView` trait, which is required for it to be presented as a modal.~~
    3.  ~~**Refactor `dropdown` Component:** The `on_toggle` logic will be changed to instantiate and present the `DropdownMenu` view using `window.present_modal()`.~~
    4.  ~~**Handle Dismissal:** Logic will be added to call `window.dismiss_modal()` when an item is selected or the popover is otherwise cancelled.~~

# Implementation Notes & Refinements (2025-11-07) - _Decision Update_

- **New Approach for Dropdowns:** After discovering that the current dropdown implementation suffers from clipping and layering (z-index) issues, a decision was made to build a custom dropdown component from scratch.
  - **Problem:** The simple, nested `div` approach causes the dropdown menu to be rendered within the bounds of its parent container, leading to it being clipped and appearing behind other UI elements.
  - **Rationale:** Initial attempts to use a pre-built `Popover` component from an external library (`gpui-component`) revealed significant version incompatibilities with our project's `gpui` dependency. To avoid a complex and brittle dependency setup, we have opted to build our own solution.
  - **Plan:** The new goal is to investigate GPUI's low-level rendering primitives to create a proper overlay system for a generic `Dropdown` component. This will be a valuable learning exercise and will give us full control over the component's behavior.
- **Popover Component Status:**
  - `src/components/popover.rs` was created and integrated. It has since been updated and aligned with the current GPUI API, resolving several compilation issues. It now serves as a basic example of a custom GPUI component.

# Implementation Notes & Refinements (2025-11-07)

- **Independent Theme Selectors:** Refactored the theme dropdowns to be fully independent, allowing users to select a start and end theme for the simulation separately.
  - **Problem:** Both the "Start Theme" and "End Theme" selectors were previously tied to the same underlying state, causing them to conflict and behave as a single unit.
  - **Solution:** The `AppState` was updated to hold separate state for the start and end themes (`start_theme_index`, `end_theme_index`) and their dropdown visibility. The `AppView` methods and the UI were refactored to use this new state. The "Start Theme" selector now instantly updates the application's active theme for immediate visual feedback.

- **Dynamic Theme Loading:** Replaced the hardcoded theme loading mechanism with a dynamic, file-based approach.
  - **Problem:** The application previously only loaded two specific themes that were hardcoded in `main.rs`.
  - **Solution:** The `main` function now automatically reads all `.json` files from the `assets/` directory and parses all themes defined within them. This makes adding new themes as simple as dropping a new JSON file into the `assets` folder. The old hardcoded scheduler logic was also removed to simplify the startup process.

# Implementation Notes & Refinements (2025-11-07) - _Component Refactoring_

- **Component Extraction Initiative:** Initiated a dedicated effort to extract reusable UI components from `src/ui.rs` into the `src/components/` directory. A `src/components/README.md` has been created to outline the detailed roadmap for this refactoring.
- **Styled Button Component:** Extracted the "Run Simulation" button into a generic `render_button` function (`src/components/button.rs`). This involved:
    - Creating the component file and integrating it into `src/components/mod.rs`.
    - Updating `src/ui.rs` to use the new component.
    - Resolving several compilation errors related to GPUI API changes, closure signatures, lifetime requirements (`&'static str`), and return types (`impl IntoElement`).
- **Popover Component Fixes:** Addressed multiple compilation errors in the existing `src/components/popover.rs` component to align it with the current GPUI API. This included:
    - Updating the `render` method signature to match the `gpui::Render` trait.
    - Correcting the `on_click` closure's context usage (`_cx_for_method.notify()`).
    - Making the root `div` stateful with an `id()`.
    - Aligning `Popover::new` with the `AppContext` trait's generic constraints and using `cx.new` to return `Entity<Self>`.
- **Generic Dropdown Decision:** The initial plan for a `ThemeSelector` component was generalized into a more reusable `Dropdown` component, which will be the next focus for extraction.

# Implementation Notes & Refinements (2025-11-05) - _Revised Plan_

After repeated compilation failures, a new strategy for implementing keyboard navigation has been devised. The root cause of the failures was identified as a misuse of GPUI's context and action handling systems.

The previous approach attempted to handle window-specific actions (like focus changes) from the global application context in the `main` function. This created complex context-passing challenges and incorrect API usage.

The new plan is to refactor the action handlers to be methods on the `AppView` struct itself, which is the idiomatic pattern demonstrated in `sandbox/gpui-source/gpui/examples/tab_stop.rs`.

**Refactoring Steps:**

1.  **Move Action Logic to `AppView`:** The logic for `FocusNext`, `FocusPrev`, and `Submit` actions will be moved from the `cx.on_action` closures in `main.rs` into new methods on `impl AppView`.
    - `on_focus_next` will simply call `window.focus_next()`.
    - `on_focus_prev` will simply call `window.focus_prev()`.
    - `on_submit` will call the existing `self.run_simulation(cx)`.

2.  **Attach Listeners in `render`:** In `src/ui.rs`, the root `div` of the interactive UI will use `.on_action(cx.listener(..))` to link the `FocusNext`, `FocusPrev`, and `Submit` actions to their new corresponding methods on `AppView`.

3.  **Cleanup `main.rs`:** The now-redundant `cx.on_action` closures for these three actions will be removed from the `main` function, simplifying the application setup significantly.

This approach aligns our code with a proven working example and is expected to resolve the persistent compilation errors.

# Phase 2: Interactive Theme Tester UI

The current 24-hour scheduling simulation is not ideal for rapid testing. We will build a user interface to control and trigger the theme scheduler's logic manually. This UI will serve as a development and testing mode for the core scheduling engine.

## Design & Simulation Strategy

After discussion, the primary goal of the interactive UI has been clarified: **to test and validate the existing `ThemeScheduler` logic** in an accelerated, on-demand fashion, rather than creating a parallel implementation of the fade.

To achieve this, we will implement a one-shot simulation strategy:

1.  **Full Simulation:** The UI will provide inputs for `sleep_duration` and `fade_duration` to allow for a full simulation of the `ThemeScheduler::run_loop`, including both the initial sleep/wait phase and the active fade phase.
2.  **On-Demand Execution:** When the "Run Simulation" button is clicked, the application will spawn a new, temporary thread to run a single simulation.
3.  **Dynamic Schedule:** Inside this thread, a temporary two-event `schedule` will be created based on the current theme and the UI inputs.
4.  **Scheduler Modification:** The `ThemeScheduler` will be made aware of the `AppMode`. Its `run_loop` will be slightly modified to check for `AppMode::Interactive`. If this mode is detected, the loop will execute one full cycle (sleep and fade) and then `return`, cleanly terminating the simulation thread.

This approach allows us to use the real scheduler code in a controlled test environment.

# Implementation Notes & Refinements (2025-11-05)

- **Keyboard Navigation:** Implemented comprehensive keyboard control for the interactive simulator.
  - **Focus Management:** `[x]` Implemented `Tab` and `Shift+Tab` functionality to cycle focus between all interactive elements.
  - **Global "Enter" Key Submission:** `[x]` Pressing "Enter" in text fields or on the "Run Simulation" button correctly triggers the simulation.
  - **Dropdown Keyboard Control:** `[x]` Implemented full keyboard control for the theme selector dropdown.
    - `[x]` `Up Arrow` and `Down Arrow` keys now navigate the list of themes when the dropdown is open, with **auto-scrolling** to keep the highlighted item in view.
    - `[x]` `Enter` key now opens the dropdown, or selects the highlighted theme and closes it.
    - `[x]` `Escape` key handling to close the dropdown has not been implemented yet.

- **Focus Indication:** To improve UI clarity, visual indicators will be added for focused elements. Currently, the "Run Simulation" button and theme selector dropdown do not visually change when they receive focus via keyboard navigation.
  - **Implementation:** The `.focus()` style modifier will be applied to these elements in `src/ui.rs`. When focused, their border color will be updated to use the `border_focused` color from the active theme, making the indicator theme-aware.

- **Theme-Aware Styling Fix:** Resolved a runtime panic caused by incorrect theme parsing.
  - **Problem:** The `parse_zed_theme` function did not handle nested color objects (e.g., `"border": { "focused": "..." }`), causing it to fail to load certain theme colors. This led to a panic when the UI code tried to `.unwrap()` a color that hadn't been loaded.
  - **Solution:** The theme parser was updated to recursively flatten nested JSON objects, creating dot-separated keys (e.g., `"border.focused"`). The UI code was also updated to use these new keys and to provide a fallback color, preventing future panics if a key is missing from a theme.

- **Theming:** Refactored the `TextInput` component to be theme-aware. The component now pulls background and placeholder colors from the active theme data stored in the `AppState` global, resolving issues where its appearance was disconnected from the application's theme. This replaced hardcoded color values with dynamic, theme-based styling.

- **Action Handler Refactoring:** Resolved a `gpui: window not found` runtime error occurring when the theme selector dropdown was clicked.
  - **Problem:** The `ToggleDropdown` and `SelectTheme` actions were being handled by global `cx.on_action` listeners defined in the `main` function. This is the same pattern that previously caused issues with keyboard navigation, where the action handler receives a context that is disconnected from the application window.
  - **Solution:** The handlers for these actions were refactored into methods on the `AppView` struct (`on_toggle_dropdown`, `on_select_theme`). The UI now attaches these methods using `cx.listener`, ensuring all UI-related actions are handled within a valid window context. The old global handlers were removed.

# Implementation Notes & Refinements (2025-11-10)

- **Color Palette Preview:** The "Color Palette" section of the UI will be implemented as a set of gradient bars.
  - **Goal:** To provide a static, at-a-glance preview of the color transition for key theme colors (e.g., `surface.background`, `text`, etc.).
  - **Implementation:** A new `gradient_bar` component will be created using a `canvas` element to manually draw a smooth `linear-gradient` between the start and end theme colors.

# Implementation Notes & Refinements (2025-11-02)

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

- **Remaining Issues:**
  - [x] **Styling:** The input box has a hardcoded style that won't look good when we switch to a dark theme.
  - [-] **No "Enter" key handling:** Pressing enter in the text box does nothing. (Superseded by Keyboard Navigation task)

# Detailed Steps

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

# Implementation Notes & Refinements (2025-10-31)

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