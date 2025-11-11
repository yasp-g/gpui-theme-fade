# Components Subfolder Roadmap

This document outlines the plan for extracting reusable UI components from `src/ui.rs` into this `src/components/` directory. The goal is to significantly clean up `src/ui.rs`, improve modularity, and enhance maintainability.

## Current Status

*   `src/ui.rs` contains the `render_interactive_ui` function, which is being refactored by extracting distinct UI blocks into reusable components.
*   `src/components/popover.rs` exists and has been updated to align with the current GPUI API.
*   `src/components/button.rs` has been created and integrated.
*   `src/components/dropdown.rs` has been created and integrated, replacing the theme selectors in the main UI.

## Refactoring Plan

The following components have been identified for extraction from `src/ui.rs`. They are listed in a suggested order of implementation, prioritizing those that offer the most significant immediate cleanup and reusability benefits.

### 1. Styled Button (`button.rs`)

*   **Description:** A generic, styled button component with focus tracking, hover effects, and a customizable `on_click` action. This will replace the "Run Simulation" button.
*   **Implementation Details:** Will be a public function (`render_button`) taking an `id`, `label`, `focus_handle`, and an `on_click_callback` closure.
*   **Priority:** High (promotes consistent button styling and behavior, and is relatively simple to implement).

### 2. Dropdown (`dropdown.rs`)

*   **Description:** A generic component for a clickable trigger that toggles the visibility of arbitrary dropdown content. It will handle common styling, focus management, and keyboard navigation actions. This has replaced the duplicated "Start Theme Selector" and "End Theme Selector" blocks. **Note:** The event-capturing bug will be fixed by applying the `.occlude()` method to the popover's root `div`.
*   **Implementation Details:** A public function (`render_dropdown`) that takes parameters such as static IDs, `is_open`, `focus_handle`, a slice of `disabled_indices`, and various `on_...` callback closures for toggling and selection. It now includes auto-scrolling for keyboard navigation to keep the highlighted item in view, and "Escape" key handling to close the dropdown.
*   **Priority:** Complete.

### 3. Gradient Bar (`gradient_bar.rs`)

*   **Description:** A custom-drawn component that renders a smooth horizontal gradient between a start and end color. This will be used to preview theme transitions for key colors.
*   **Implementation Details:** Will be a public function (`render_gradient_bar`) that returns a `canvas` element. The canvas's `paint` closure will manually draw the gradient by interpolating between the colors and drawing 1-pixel-wide vertical lines.
*   **Priority:** High (this is the next feature to be implemented).

### 4. Form Field (`form_field.rs`)

*   **Description:** A generic wrapper component that provides consistent styling for a label paired with an input control (like a `TextInput` or `Dropdown`). It renders a border around its child that visually indicates a validation state (e.g., green for valid, red for invalid).
*   **Implementation Details:** Will be a public function (`render_form_field`) that takes a `label`, an `is_valid` flag, and accepts any `impl IntoElement` as a child. This allows it to wrap any kind of control.
*   **Priority:** Medium (promotes consistency and reusability for all input fields).

### 5. Panel Container (`panel.rs`)

*   **Description:** A reusable container component for grouping related UI elements, featuring consistent styling such as borders, padding, and rounded corners. This will replace the "Left Panel" and "Right Panel" containers.
*   **Implementation Details:** Will likely be a public function (`render_panel`) that accepts an `id` and a collection of child elements (`impl IntoElement` or a closure returning children).
*   **Priority:** Low (primarily for structural organization and consistent container styling).

### 6. Header (`header.rs`)

*   **Description:** A simple component for displaying the main title of the application.
*   **Implementation Details:** Will likely be a public function (`render_header`) taking the title string.
*   **Priority:** Low (minimal code reduction, but good for semantic structure).

## Next Steps

The immediate next step is to fix the critical bug where mouse clicks "pass through" the dropdown menu. This will be done by adding the `.occlude()` method to the root `div` of the `popover` component. The `gradient_bar` implementation will be postponed until this is complete.
