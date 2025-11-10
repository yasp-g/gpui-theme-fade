use anyhow::{anyhow, Result};
use gpui::{hsla, Hsla, Rgba};
use once_cell::sync::Lazy;
use regex::Regex;
use schemars::JsonSchema;
use serde::Deserialize;
use std::{collections::HashMap, str::FromStr};

// --- THEME & COLOR DEFINITIONS (from scheduler.rs) ---

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub rgba: Rgba,
    pub hsla: Hsla,
}

impl Default for Color {
    fn default() -> Self {
        Color {
            rgba: gpui::rgba(0xff00ff),
            hsla: hsla(0.83, 1.0, 0.5, 1.0),
        }
    }
}

static COLOR_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^#([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})?$").unwrap());

impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let caps = COLOR_REGEX
            .captures(s)
            .ok_or_else(|| anyhow!("Invalid hex color: {}", s))?;

        let r = u8::from_str_radix(caps.get(1).unwrap().as_str(), 16)?;
        let g = u8::from_str_radix(caps.get(2).unwrap().as_str(), 16)?;
        let b = u8::from_str_radix(caps.get(3).unwrap().as_str(), 16)?;
        let a = caps
            .get(4)
            .map_or(Ok(255), |m| u8::from_str_radix(m.as_str(), 16))?;

        let rgba = Rgba {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        };
        let hsla = Hsla::from(rgba);
        Ok(Color { rgba, hsla })
    }
}

pub fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    let hsla = Hsla {
        h: a.hsla.h + (b.hsla.h - a.hsla.h) * t,
        s: a.hsla.s + (b.hsla.s - a.hsla.s) * t,
        l: a.hsla.l + (b.hsla.l - a.hsla.l) * t,
        a: a.hsla.a + (b.hsla.a - a.hsla.a) * t,
    };
    let rgba = Rgba::from(hsla);
    Color { rgba, hsla }
}

#[derive(Clone, Default, Debug)]
pub struct InterpolatableTheme(pub HashMap<String, Color>);

pub fn lerp_theme(a: &InterpolatableTheme, b: &InterpolatableTheme, t: f32) -> InterpolatableTheme {
    let mut new_theme = InterpolatableTheme::default();

    for (key, color_a) in &a.0 {
        if let Some(color_b) = b.0.get(key) {
            new_theme
                .0
                .insert(key.clone(), lerp_color(*color_a, *color_b, t));
        } else {
            new_theme.0.insert(key.clone(), *color_a);
        }
    }
    new_theme
}

// --- THEME PARSING (from main.rs) ---

#[derive(Clone, Debug)]
pub struct Theme {
    pub name: String,
    pub interpolatable_theme: InterpolatableTheme,
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct ZedThemeFile {
    pub themes: Vec<ThemeDefinition>,
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct ThemeDefinition {
    pub name: String,
    pub style: ThemeStyle,
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct ThemeStyle {
    #[serde(flatten)]
    pub colors: HashMap<String, serde_json::Value>,
}

pub fn flatten_colors(
    colors: &HashMap<String, serde_json::Value>,
    interpolatable_theme: &mut InterpolatableTheme,
    prefix: &str,
) {
    for (key, value) in colors {
        let new_key = if prefix.is_empty() {
            key.clone()
        } else {
            format!("{}.{}", prefix, key)
        };

        if let Some(hex_string) = value.as_str() {
            match Color::from_str(hex_string) {
                Ok(color) => {
                    interpolatable_theme.0.insert(new_key, color);
                }
                Err(e) => {
                    tracing::warn!(
                        "Failed to parse color for key '{}': {} (value: '{}')",
                        new_key,
                        e,
                        hex_string
                    );
                }
            }
        } else if let Some(nested_obj) = value.as_object() {
            let nested_map: HashMap<String, serde_json::Value> =
                nested_obj.clone().into_iter().collect();
            flatten_colors(&nested_map, interpolatable_theme, &new_key);
        }
    }
}
