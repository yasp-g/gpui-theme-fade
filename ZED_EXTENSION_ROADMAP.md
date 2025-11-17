# Zed Extension: Project Roadmap

This document outlines the plan for developing the standalone demo into a fully-fledged Zed extension.

---

## Phase 1: Core Feature Development

Once the architecture is solidified, we can begin adding new, core library functionality.

### 1. Theme Integration with Zed Settings

- **Description:** Integrate the theme scheduler with Zed's settings system to allow programmatic theme changes. This is the primary mechanism for the final Zed extension to function.
- **Status:** [ ] Not Started
- **Priority:** High
- **Tasks:**
  - [ ] Adapt the core scheduling logic to call `settings::update_settings_file` when a theme change is due.
  - [ ] Inside the `update_settings_file` closure, use the `theme::settings::set_theme` helper to modify the `SettingsContent` with the new theme name.
  - [ ] Ensure the scheduler has access to the necessary `Fs` and `AppContext` handles to perform this operation.

### 2. Zed Extension Structure and Theme Access

- **Description:** Define the project's structure as a Zed extension and adapt its theme loading mechanism to use Zed's internal theme registry.
- **Status:** [ ] Not Started
- **Priority:** High
- **Tasks:**
  - [ ] Investigate Zed's extension loading mechanism and define the `Cargo.toml` and entry point for our extension.
  - [ ] Understand how Zed extensions access the available themes from Zed's `ThemeRegistry`.
  - [ ] Adapt our theme loading mechanism to use Zed's API instead of reading theme files directly from the `assets/` directory.

### 3. Configuration Persistence

- **Description:** Save and load the user's settings to provide a consistent experience between sessions. This will be designed as a core feature that the Zed extension can leverage.
- **Status:** [ ] Not Started
- **Priority:** Low
- **Tasks:**
  - [ ] **Define Config Struct:** Create a new struct that can be serialized/deserialized (e.g., with `serde`) to hold settings like `start_theme_index`, `end_theme_index`, `sleep_duration`, and `fade_duration`.
  - [ ] **Implement Save/Load Logic:** On application startup and shutdown, serialize/deserialize the config struct to a file (e.g., `config.json`).

---

## Phase 2 (Optional): Test Harness Polish

These tasks improve the standalone test application but do not contribute directly to the Zed extension. They should only be worked on if deemed necessary for improving the development experience.

### 1. UX Enhancements

- **Description:** Implement several small but high-impact UX improvements to make the test application feel more polished.
- **Status:** [ ] Not Started
- **Priority:** Very Low
- **Tasks:**
  - [ ] **Implement "Click-Away-to-Close" for Dropdowns.**
  - [ ] **Add Visual Cues for Disabled Dropdown Items.**
  - [ ] **Add Standard Window Management Keybindings (`cmd-q`, `cmd-w`, `cmd-m`).**
