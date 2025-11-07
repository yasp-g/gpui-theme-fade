# Components Subfolder Roadmap

This document outlines the plan for extracting reusable UI components from `src/ui.rs` into this `src/components/` directory. The goal is to significantly clean up `src/ui.rs`, improve modularity, and enhance maintainability.

## Current Status

*   `src/ui.rs` contains the `render_interactive_ui` function, which is currently quite large and contains several distinct UI blocks that can be generalized.
*   `src/components/popover.rs` already exists as an initial component.

## Refactoring Plan

The following components have been identified for extraction from `src/ui.rs`. They are listed in a suggested order of implementation, prioritizing those that offer the most significant immediate cleanup and reusability benefits.

### 1. Styled Button (`button.rs`)

*   **Description:** A generic, styled button component with focus tracking, hover effects, and a customizable `on_click` action. This will replace the "Run Simulation" button.
*   **Implementation Details:** Will be a public function (`render_button`) taking an `id`, `label`, `focus_handle`, and an `on_click_callback` closure.
*   **Priority:** High (promotes consistent button styling and behavior, and is relatively simple to implement).

### 2. Dropdown (`dropdown.rs`)

*   **Description:** A generic component for a clickable trigger that toggles the visibility of arbitrary dropdown content. It will handle common styling, focus management, and keyboard navigation actions. This will replace the duplicated "Start Theme Selector" and "End Theme Selector" blocks, which will then utilize this generic dropdown.
*   **Implementation Details:** Will be a public function (`render_dropdown`) that takes parameters such as `id_prefix`, `trigger_content` (an `impl IntoElement` for the clickable part), `dropdown_content` (an `impl IntoElement` for the content shown when open), `is_open`, `focus_handle`, and various `on_...` callback closures for toggling and keyboard actions.
*   **Priority:** Medium (addresses significant code duplication and provides a highly reusable component).

### 3. Validated Text Input (`validated_text_input.rs`)

*   **Description:** A component for displaying a label alongside a `TextInput` view, with a border that visually indicates validity (e.g., green for valid, red for invalid). This will replace the "Sleep Duration (s):" and "Fade Duration (s):" input blocks.
*   **Implementation Details:** Will likely be a public function (`render_validated_text_input`) taking a label, the `TextInput` view, and a boolean `is_valid` flag.
*   **Priority:** Medium (addresses minor code duplication and consistent styling).

### 4. Panel Container (`panel.rs`)

*   **Description:** A reusable container component for grouping related UI elements, featuring consistent styling such as borders, padding, and rounded corners. This will replace the "Left Panel" and "Right Panel" containers.
*   **Implementation Details:** Will likely be a public function (`render_panel`) that accepts an `id` and a collection of child elements (`impl IntoElement` or a closure returning children).
*   **Priority:** Low (primarily for structural organization and consistent container styling).

### 5. Header (`header.rs`)

*   **Description:** A simple component for displaying the main title of the application.
*   **Implementation Details:** Will likely be a public function (`render_header`) taking the title string.
*   **Priority:** Low (minimal code reduction, but good for semantic structure).

## Next Steps

We will proceed with extracting these components one by one, starting with the `Theme Selector`, and updating `src/ui.rs` to use the new components after each extraction.
