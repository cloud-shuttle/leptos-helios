//! Accessibility Module
//!
//! This module provides WCAG 2.1 AA compliance, screen reader support,
//! keyboard navigation, and performance optimization capabilities.

pub mod alt_text;
pub mod errors;
pub mod keyboard;
pub mod performance;
pub mod screen_reader;
pub mod wcag;

// Re-export main types and functions
pub use alt_text::*;
pub use errors::*;
pub use keyboard::*;
pub use performance::*;
pub use screen_reader::*;
pub use wcag::*;

// Common types used across all accessibility features
// use crate::chart::{ChartSpec, MarkType}; // Currently unused
// use polars::prelude::DataFrame; // Currently unused
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Accessibility configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityConfig {
    pub wcag_level: WCAGLevel,
    pub screen_reader: ScreenReaderSupport,
    pub keyboard_nav: KeyboardNavigation,
    pub color_vision: ColorVisionSupport,
    pub motion: MotionPreferences,
    pub focus_management: FocusManagement,
    pub alternative_formats: AlternativeFormats,
}

impl Default for AccessibilityConfig {
    fn default() -> Self {
        Self {
            wcag_level: WCAGLevel::AA,
            screen_reader: ScreenReaderSupport::default(),
            keyboard_nav: KeyboardNavigation::default(),
            color_vision: ColorVisionSupport::default(),
            motion: MotionPreferences::default(),
            focus_management: FocusManagement::default(),
            alternative_formats: AlternativeFormats::default(),
        }
    }
}

/// Screen reader support configuration for enhanced accessibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenReaderSupport {
    pub enabled: bool,
    pub generate_alt_text: bool,
    pub create_data_tables: bool,
    pub provide_summaries: bool,
    pub announce_updates: bool,
    pub aria_labels: bool,
    pub structured_navigation: bool,
}

impl Default for ScreenReaderSupport {
    fn default() -> Self {
        Self {
            enabled: true,
            generate_alt_text: true,
            create_data_tables: true,
            provide_summaries: true,
            announce_updates: true,
            aria_labels: true,
            structured_navigation: true,
        }
    }
}

/// Keyboard navigation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardNavigation {
    pub enabled: bool,
    pub tab_order: bool,
    pub arrow_key_navigation: bool,
    pub skip_links: bool,
    pub focus_indicators: bool,
    pub escape_handling: bool,
    pub custom_shortcuts: HashMap<String, String>,
}

impl Default for KeyboardNavigation {
    fn default() -> Self {
        Self {
            enabled: true,
            tab_order: true,
            arrow_key_navigation: true,
            skip_links: true,
            focus_indicators: true,
            escape_handling: true,
            custom_shortcuts: HashMap::new(),
        }
    }
}

/// Color vision support configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorVisionSupport {
    pub enabled: bool,
    pub colorblind_friendly: bool,
    pub high_contrast: bool,
    pub pattern_fills: bool,
    pub text_labels: bool,
    pub color_alternatives: Vec<String>,
}

impl Default for ColorVisionSupport {
    fn default() -> Self {
        Self {
            enabled: true,
            colorblind_friendly: true,
            high_contrast: true,
            pattern_fills: true,
            text_labels: true,
            color_alternatives: vec![
                "#FF6B6B".to_string(), // Red
                "#4ECDC4".to_string(), // Teal
                "#45B7D1".to_string(), // Blue
                "#96CEB4".to_string(), // Green
                "#FFEAA7".to_string(), // Yellow
                "#DDA0DD".to_string(), // Purple
            ],
        }
    }
}

/// Motion preferences for accessibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionPreferences {
    pub respect_reduced_motion: bool,
    pub animation_duration_ms: u32,
    pub transition_duration_ms: u32,
    pub disable_animations: bool,
    pub reduce_parallax: bool,
}

impl Default for MotionPreferences {
    fn default() -> Self {
        Self {
            respect_reduced_motion: true,
            animation_duration_ms: 300,
            transition_duration_ms: 150,
            disable_animations: false,
            reduce_parallax: true,
        }
    }
}

/// Focus management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusManagement {
    pub enabled: bool,
    pub focus_trap: bool,
    pub focus_restoration: bool,
    pub focus_indicators: bool,
    pub tab_index_management: bool,
    pub aria_live_regions: bool,
}

impl Default for FocusManagement {
    fn default() -> Self {
        Self {
            enabled: true,
            focus_trap: true,
            focus_restoration: true,
            focus_indicators: true,
            tab_index_management: true,
            aria_live_regions: true,
        }
    }
}

/// Alternative formats configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeFormats {
    pub data_tables: bool,
    pub text_descriptions: bool,
    pub sonification: bool,
    pub tactile_graphics: bool,
    pub high_contrast_version: bool,
}

impl Default for AlternativeFormats {
    fn default() -> Self {
        Self {
            data_tables: true,
            text_descriptions: true,
            sonification: false,     // Advanced feature
            tactile_graphics: false, // Advanced feature
            high_contrast_version: true,
        }
    }
}

/// Alternative data table representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTable {
    pub title: String,
    pub summary: String,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub caption: Option<String>,
    pub scope_attributes: HashMap<String, String>,
}
