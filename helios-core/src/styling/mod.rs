//! Styling utilities for leptos-helios
//!
//! This module provides Tailwind CSS integration and styling utilities
//! for creating modern, responsive visualizations.

use serde::{Deserialize, Serialize};

/// Tailwind CSS class generator for chart components
pub struct ChartStyler {
    // TODO: Implement Tailwind integration when available
}

/// Chart theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartTheme {
    pub background: String,
    pub text: String,
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub grid: String,
    pub border: String,
}

/// Predefined themes
impl ChartTheme {
    pub fn dark() -> Self {
        Self {
            background: "bg-gray-900".to_string(),
            text: "text-white".to_string(),
            primary: "text-blue-400".to_string(),
            secondary: "text-gray-300".to_string(),
            accent: "text-purple-400".to_string(),
            grid: "text-gray-600".to_string(),
            border: "border-gray-700".to_string(),
        }
    }

    pub fn light() -> Self {
        Self {
            background: "bg-white".to_string(),
            text: "text-gray-900".to_string(),
            primary: "text-blue-600".to_string(),
            secondary: "text-gray-600".to_string(),
            accent: "text-purple-600".to_string(),
            grid: "text-gray-300".to_string(),
            border: "border-gray-200".to_string(),
        }
    }

    pub fn neon() -> Self {
        Self {
            background: "bg-black".to_string(),
            text: "text-green-400".to_string(),
            primary: "text-cyan-400".to_string(),
            secondary: "text-green-300".to_string(),
            accent: "text-pink-400".to_string(),
            grid: "text-green-800".to_string(),
            border: "border-green-600".to_string(),
        }
    }
}

impl ChartStyler {
    /// Create a new chart styler
    pub fn new() -> Self {
        Self {
            // TODO: Initialize Tailwind when available
        }
    }

    /// Generate container classes for chart components
    pub fn chart_container(&self, theme: &ChartTheme) -> String {
        format!(
            "{} {} {} rounded-lg p-4 shadow-lg backdrop-blur-sm",
            theme.background, theme.border, theme.text
        )
    }

    /// Generate button classes for controls
    pub fn control_button(&self, variant: ButtonVariant) -> String {
        match variant {
            ButtonVariant::Primary => "bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-md font-medium transition-colors duration-200".to_string(),
            ButtonVariant::Secondary => "bg-gray-600 hover:bg-gray-700 text-white px-4 py-2 rounded-md font-medium transition-colors duration-200".to_string(),
            ButtonVariant::Success => "bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-md font-medium transition-colors duration-200".to_string(),
            ButtonVariant::Danger => "bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded-md font-medium transition-colors duration-200".to_string(),
            ButtonVariant::Warning => "bg-yellow-600 hover:bg-yellow-700 text-white px-4 py-2 rounded-md font-medium transition-colors duration-200".to_string(),
        }
    }

    /// Generate input field classes
    pub fn input_field(&self, theme: &ChartTheme) -> String {
        format!(
            "{} {} bg-transparent border rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500",
            theme.background,
            theme.border
        )
    }

    /// Generate select dropdown classes
    pub fn select_field(&self, theme: &ChartTheme) -> String {
        format!(
            "{} {} bg-transparent border rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500",
            theme.background,
            theme.border
        )
    }

    /// Generate metric card classes
    pub fn metric_card(&self, theme: &ChartTheme) -> String {
        format!(
            "{} {} {} rounded-lg p-4 text-center shadow-md",
            theme.background, theme.border, theme.text
        )
    }

    /// Generate status indicator classes
    pub fn status_indicator(&self, status: StatusType) -> String {
        match status {
            StatusType::Connected => {
                "bg-green-500 text-white px-2 py-1 rounded-full text-xs font-semibold".to_string()
            }
            StatusType::Disconnected => {
                "bg-red-500 text-white px-2 py-1 rounded-full text-xs font-semibold".to_string()
            }
            StatusType::Streaming => {
                "bg-yellow-500 text-white px-2 py-1 rounded-full text-xs font-semibold".to_string()
            }
            StatusType::Error => {
                "bg-red-600 text-white px-2 py-1 rounded-full text-xs font-semibold".to_string()
            }
        }
    }

    /// Generate grid layout classes
    pub fn grid_layout(&self, columns: usize) -> String {
        match columns {
            1 => "grid grid-cols-1 gap-4".to_string(),
            2 => "grid grid-cols-1 md:grid-cols-2 gap-4".to_string(),
            3 => "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4".to_string(),
            4 => "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4".to_string(),
            _ => "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4".to_string(),
        }
    }

    /// Generate responsive chart container classes
    pub fn responsive_chart(&self) -> String {
        "w-full h-64 md:h-80 lg:h-96 bg-gray-100 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700".to_string()
    }

    /// Generate console/log classes
    pub fn console_output(&self, theme: &ChartTheme) -> String {
        format!(
            "{} {} {} font-mono text-sm rounded-lg p-4 max-h-48 overflow-y-auto",
            theme.background, theme.border, theme.text
        )
    }

    /// Generate log line classes based on log level
    pub fn log_line(&self, level: LogLevel) -> String {
        match level {
            LogLevel::Info => "text-blue-400".to_string(),
            LogLevel::Success => "text-green-400".to_string(),
            LogLevel::Warning => "text-yellow-400".to_string(),
            LogLevel::Error => "text-red-400".to_string(),
        }
    }
}

/// Button variants for styling
#[derive(Debug, Clone)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
}

/// Status types for indicators
#[derive(Debug, Clone)]
pub enum StatusType {
    Connected,
    Disconnected,
    Streaming,
    Error,
}

/// Log levels for console output
#[derive(Debug, Clone)]
pub enum LogLevel {
    Info,
    Success,
    Warning,
    Error,
}

/// Generate a complete Tailwind CSS class string for a component
pub fn generate_component_classes(component: ComponentType, theme: &ChartTheme) -> String {
    let styler = ChartStyler::new();

    match component {
        ComponentType::MainContainer => {
            format!(
                "min-h-screen {} {} bg-gradient-to-br from-blue-900 via-purple-900 to-indigo-900",
                theme.background, theme.text
            )
        }
        ComponentType::Header => {
            format!("text-center mb-8 {}", theme.text)
        }
        ComponentType::ControlsPanel => {
            format!(
                "{} {} {} rounded-xl p-6 mb-6 shadow-xl backdrop-blur-md",
                theme.background, theme.border, theme.text
            )
        }
        ComponentType::ChartGrid => "grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8".to_string(),
        ComponentType::MetricsGrid => "grid grid-cols-2 md:grid-cols-4 gap-4 mb-8".to_string(),
    }
}

/// Component types for styling
#[derive(Debug, Clone)]
pub enum ComponentType {
    MainContainer,
    Header,
    ControlsPanel,
    ChartGrid,
    MetricsGrid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_styler_creation() {
        let styler = ChartStyler::new();
        assert!(styler.tailwind.is_ok());
    }

    #[test]
    fn test_theme_generation() {
        let dark_theme = ChartTheme::dark();
        assert!(dark_theme.background.contains("gray-900"));

        let light_theme = ChartTheme::light();
        assert!(light_theme.background.contains("white"));

        let neon_theme = ChartTheme::neon();
        assert!(neon_theme.background.contains("black"));
    }

    #[test]
    fn test_button_styling() {
        let styler = ChartStyler::new();
        let primary_class = styler.control_button(ButtonVariant::Primary);
        assert!(primary_class.contains("bg-blue-600"));

        let danger_class = styler.control_button(ButtonVariant::Danger);
        assert!(danger_class.contains("bg-red-600"));
    }

    #[test]
    fn test_status_styling() {
        let styler = ChartStyler::new();
        let connected_class = styler.status_indicator(StatusType::Connected);
        assert!(connected_class.contains("bg-green-500"));

        let error_class = styler.status_indicator(StatusType::Error);
        assert!(error_class.contains("bg-red-600"));
    }
}
