The overarching purpose of the project is to help me learn Rust and broaden my ability as a developer. I have strong Python experience and also have some experience with other languages like C++ and Javascript. But, I've never worked with Rust before this project, so please keep my experience level and background that i bring in mind and please be sure to take every opportunity to help me learn Rust as best and fast as possible. Although your main goal is to help me develop the app explained below, you will also take on the roll of teacher and mentor me as i learn Rust.

---

## Core Development Principle: Trust the Source

**GPUI is in active development.** This means its API can change, and my internal knowledge may become outdated. To ensure accuracy and avoid repeated errors, the primary source of truth for GPUI development must be the existing, working code in this project and the GPUI source code cloned in `sandbox/gpui-source/gpui/`.

## When encountering a compilation error or implementing a new feature, the first step is always to **look for existing patterns within the project**. If a pattern is discovered or a new solution is found, **this `GEMINI.md` file must be updated** to document the finding for future reference.

# GPUI Theme Scheduler

This project serves as a development environment for a theme scheduler designed for the Zed code editor. It demonstrates how to dynamically switch between themes with a smooth, animated transition, and will culminate in a demo application.

## Purpose

The primary goal of this application is to develop and showcase a theme scheduling feature that can be packaged as a Zed extension. The code is being developed to be "turn-key" for this purpose. The application includes both a time-based scheduler and an interactive mode for rapid testing and demonstration.

## Key Features

- **Theme Interpolation:** Smoothly interpolates between themes to create a fading transition effect.
- **Time-Based Scheduling:** A background thread manages a schedule to switch themes at predefined times.
- **Interactive Theme Tester:** A user interface to manually trigger and configure theme transitions for development and debugging.
- **Zed Theme Compatibility:** Parses theme data from JSON strings that follow the Zed theme schema.
- **`gpui` Integration:** Leverages the `gpui` library for the user interface and application state management.

## Project Structure

- `src/main.rs`: Contains the application's entry point, state management, and the primary UI view.
- `src/scheduler.rs`: Houses the `ThemeScheduler` for time-based theme switching.
- `assets/`: Contains theme definition files (e.g., `one.json`, `ayu.json`).
- `sandbox/gpui-source/gpui/`: Local clone of the `gpui` framework source code, available for reference during development.
- `sandbox/zed-source/zed/`: Local clone of the `zed` source code, available for reference during development.
- `Cargo.toml`: Defines project dependencies.
- `GEMINI.md`: This file, providing a high-level overview of the project.

## Development Notes

- **GPUI Source:** The source code for the version of GPUI used in this project has been cloned from Zed Industries' repository into `sandbox/gpui-source/gpui/`. Reference this source code for understanding the `gpui` framework's internals when developing new features or debugging compiler issues.
- **Compilation:** The immediate focus is on achieving a successful compilation. Use `cargo build` to check for errors
  - Never call `cargo run` directly. Instead instruct the user to when you feel it is appropriate.

---

## Project Management: The Roadmap is the Plan

To ensure clarity and alignment, the `ROADMAP.md` file will serve as the single source of truth for all planned work.

- **Planning First:** Before implementing any new feature, bug fix, or significant refactoring, a detailed plan must be added to `ROADMAP.md`. This plan should follow the established format, breaking the work down into clear, actionable tasks.
- **Executing the Plan:** All implementation work should directly follow the plan laid out in the roadmap.
- **Keeping it Current:** As tasks are completed, their corresponding checkboxes (`- [ ]`) must be marked as done (`- [x]`). The "Last Updated" date should also be updated periodically.

This process ensures that we always have a clear, up-to-date view of the project's status and direction.

---

## Zed Integration Notes: Changing the Theme

Investigation into the Zed source code (`sandbox/zed-source/zed/`) has revealed the primary mechanism for programmatically changing the active theme. This is the core integration point for our theme scheduler extension.

- **No Direct Action:** There is no simple, global `Action` to dispatch for setting a theme (e.g., `actions::SetTheme("MyTheme")`). The theme is changed by modifying the user's settings file.

- **The Key Function:** The central function for persisting a settings change is `settings::update_settings_file`.
    - **Location:** `crates/settings/src/settings_file.rs`
    - **Signature:** `pub fn update_settings_file(fs: Arc<dyn Fs>, cx: &App, update: impl FnOnce(&mut SettingsContent, &App))`

- **Mechanism:** To change the theme, our extension must call `update_settings_file`. This function takes a closure that receives a mutable `SettingsContent` object. Our code will run inside this closure.

- **Implementation Steps:**
    1. Within the `update` closure, our code will receive `&mut SettingsContent`.
    2. We will then call the `theme::settings::set_theme` helper function, passing it the `SettingsContent` object and the name of the new theme.
    3. The `update_settings_file` function will then handle the process of saving the modified `SettingsContent` to the user's `settings.json` file, triggering a theme change across the application.

This is the canonical way to make persistent setting changes and is the path our extension must follow to apply a new theme as part of its scheduling logic.

---

## Architectural Pattern: Convergent Method for UI Events

To ensure stability and maintainability, all new UI features that can be triggered by multiple sources (e.g., a button click and a keyboard shortcut) will follow the "Convergent Method" pattern. This pattern avoids subtle context errors (like `gpui: window not found`) and creates a single source of truth for business logic.

The core principle is: **All event paths converge on a single, public logic method in `AppView`.**

Here is the step-by-step implementation guide:

### Step 1: Create the Logic Method (The Source of Truth)

First, implement the core logic as a public method on `AppView`. This method contains all the actual work and is the single source of truth for the feature.

**Example (`src/main.rs`):**

```rust
// In `impl AppView`
pub fn archive_selected_item(&mut self, cx: &mut Context<Self>) {
    // All logic to modify state or perform side effects lives here.
    cx.update_global::<AppState, _>(|app_state, _| {
        if let Some(selected_id) = app_state.selected_item_id {
            println!("Archiving item with ID: {}", selected_id);
            // ... logic to remove the item, etc. ...
        }
    });
}
```

### Step 2: Wire the Click Handler (The Direct-Listener Pattern)

The `on_click` handler for a UI element should **always** use `cx.listener` to call the logic method from Step 1 directly. It must **never** dispatch an action.

**Example (`src/ui.rs`):**

```rust
// In the render function...
div()
    .id("archive-button")
    .child("Archive Selected Item")
    .on_click(cx.listener(|view, _, _, cx_for_method| {
        // `view` is a mutable reference to our AppView instance.
        // We call the logic method from Step 1 directly on it.
        view.archive_selected_item(cx_for_method);
    }))
```

**Note:** The `cx.listener` call requires access to a `Context` (`cx`). If you are building UI in a helper function (e.g., `render_header(cx)`), ensure that you pass the `cx` from the parent `render` method down into your helper function.

### Step 3: Wire the Keyboard Shortcut (The Action-Shim Pattern)

To connect a keyboard shortcut, we must use an `Action` as a bridge between the global keybinding and the specific `AppView` instance.

**3a. Define the Action Struct:** In `src/main.rs`, define a simple struct for the action.

```rust
#[derive(Clone, PartialEq, Action)]
pub struct ArchiveSelectedItem;
```

**3b. Define the Key Binding:** In the `main` function, bind the key to the action.

```rust
// In `main()`
cx.bind_keys([
    KeyBinding::new("cmd-e", ArchiveSelectedItem, Some("AppView")),
]);
```

**3c. Create the "Shim" Handler:** In `impl AppView`, create a private `on_action` handler. This handler's **only job** is to call the main logic method from Step 1.

```rust
// In `impl AppView`
fn on_archive_selected_item(
    &mut self,
    _action: &ArchiveSelectedItem,
    _window: &mut Window,
    cx: &mut Context<Self>
) {
    // This handler is just a "shim" that calls the real logic method.
    self.archive_selected_item(cx);
}
```

**3d. Register the Action Listener:** In `src/ui.rs`, tell the main view to listen for the action and connect it to the shim handler.

```rust
// In `ui.rs` on the root div...
div()
    .key_context("AppView")
    .on_action(cx.listener(AppView::on_archive_selected_item))
    // ... other listeners
```

This ensures that both clicks and key presses execute the exact same code path, providing consistency and stability.

---

## Core UI Update Rule: `cx.notify()` vs. `cx.refresh()`

To prevent repeated compilation errors, the following rule must be strictly followed when updating UI components.

- **Problem:** There has been a recurring mistake of trying to call `cx.refresh()` on a view's context (`&mut Context<V>`), where this method does not exist.
- **Rule:** When you modify the state of a view _without_ calling `cx.update_global(...)` and need to trigger a re-render, the correct method is `cx.notify()`. This is common in keyboard action handlers that may not automatically trigger a redraw.
- **Automatic Refresh:** Remember that `cx.update_global(...)` automatically triggers a refresh. A manual call to `cx.notify()` is generally not needed after it.

### Correct Usage

```rust
// CORRECT: Inside a method on `impl AppView` called from a keyboard action.
pub fn my_method(&mut self, cx: &mut Context<Self>) {
    // This method might change global state, but the keyboard-derived
    // context doesn't auto-refresh the UI.
    cx.update_global::<AppState, _>(|state, _| {
        state.some_value += 1;
    });
    // We must manually notify the view to redraw.
    cx.notify();
}
```

### Incorrect Usage

```rust
// INCORRECT: This will not compile.
pub fn my_method(&mut self, cx: &mut Context<Self>) {
    // ...
    cx.refresh(); // COMPILE ERROR: `refresh` not found on `Context<Self>`
}
```

---

## GPUI API Notes

This section documents specific, recurring GPUI API issues and their correct solutions as discovered during development.

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
