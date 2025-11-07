The overarching purpose of the project is to help me learn Rust and broaden my ability as a developer. I have strong Python experience and also have some experience with other languages like C++ and Javascript. But, I've never worked with Rust before this project, so please keep my experience level and background that i bring in mind and please be sure to take every opportunity to help me learn Rust as best and fast as possible. Although your main goal is to help me develop the app explained below, you will also take on the roll of teacher and mentor me as i learn Rust.

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
- `Cargo.toml`: Defines project dependencies.
- `GEMINI.md`: This file, providing a high-level overview of the project.

## Development Notes

- **GPUI Source:** The source code for the version of GPUI used in this project has been cloned from Zed Industries' repository into `sandbox/gpui-source/gpui/`. Reference this source code for understanding the `gpui` framework's internals when developing new features or debugging compiler issues.
- **Compilation:** The immediate focus is on achieving a successful compilation. Use `cargo build` to check for errors
  - Never call `cargo run` directly. Instead instruct the user to when you feel it is appropriate.

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
- **Rule:** When you modify the state of a view *without* calling `cx.update_global(...)` and need to trigger a re-render, the correct method is `cx.notify()`. This is common in keyboard action handlers that may not automatically trigger a redraw.
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
