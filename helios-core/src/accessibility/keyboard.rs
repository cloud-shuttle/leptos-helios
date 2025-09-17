//! Keyboard Navigation System
//!
//! This module provides keyboard navigation capabilities for accessibility.

use super::{AccessibilityError, KeyboardNavigation};
use crate::chart::{ChartSpec, MarkType};
use std::collections::HashMap;

/// Keyboard navigation manager
pub struct KeyboardManager {
    config: KeyboardNavigation,
    current_focus: Option<String>,
    focus_history: Vec<String>,
    shortcuts: HashMap<String, String>,
}

impl KeyboardManager {
    /// Create a new keyboard manager
    pub fn new(config: KeyboardNavigation) -> Self {
        Self {
            config,
            current_focus: None,
            focus_history: Vec::new(),
            shortcuts: HashMap::new(),
        }
    }

    /// Initialize keyboard shortcuts for a chart
    pub fn initialize_shortcuts(&mut self, spec: &ChartSpec) {
        self.shortcuts = self.generate_keyboard_map(spec);
    }

    /// Handle keyboard event
    pub fn handle_key_event(
        &mut self,
        key: &str,
        modifiers: &[String],
    ) -> Result<KeyboardAction, AccessibilityError> {
        if !self.config.enabled {
            return Ok(KeyboardAction::NoAction);
        }

        let key_combo = self.format_key_combo(key, modifiers);

        if let Some(action) = self.shortcuts.get(&key_combo) {
            self.execute_action(action)
        } else {
            // Handle default keyboard navigation
            self.handle_default_navigation(key, modifiers)
        }
    }

    /// Set focus to a specific element
    pub fn set_focus(&mut self, element_id: &str) -> Result<(), AccessibilityError> {
        if !self.config.enabled {
            return Ok(());
        }

        // Update focus history
        if let Some(current) = &self.current_focus {
            self.focus_history.push(current.clone());
        }

        self.current_focus = Some(element_id.to_string());
        Ok(())
    }

    /// Move focus to next element
    pub fn focus_next(&mut self) -> Result<Option<String>, AccessibilityError> {
        if !self.config.tab_order {
            return Ok(None);
        }

        // Implementation would depend on the specific chart structure
        // This is a simplified version
        Ok(None)
    }

    /// Move focus to previous element
    pub fn focus_previous(&mut self) -> Result<Option<String>, AccessibilityError> {
        if !self.config.tab_order {
            return Ok(None);
        }

        // Implementation would depend on the specific chart structure
        // This is a simplified version
        Ok(None)
    }

    /// Handle escape key
    pub fn handle_escape(&mut self) -> Result<KeyboardAction, AccessibilityError> {
        if !self.config.escape_handling {
            return Ok(KeyboardAction::NoAction);
        }

        // Clear focus or close modal
        self.current_focus = None;
        Ok(KeyboardAction::ClearFocus)
    }

    /// Generate keyboard map for chart type
    fn generate_keyboard_map(&self, spec: &ChartSpec) -> HashMap<String, String> {
        let mut keyboard_map = HashMap::new();

        // Add common shortcuts
        keyboard_map.insert("tab".to_string(), "focus_next".to_string());
        keyboard_map.insert("shift+tab".to_string(), "focus_previous".to_string());
        keyboard_map.insert("escape".to_string(), "clear_focus".to_string());
        keyboard_map.insert("enter".to_string(), "activate".to_string());
        keyboard_map.insert("space".to_string(), "activate".to_string());

        // Add chart-specific shortcuts
        match spec.mark {
            MarkType::Line { .. } | MarkType::Area { .. } => {
                keyboard_map.insert("right".to_string(), "next_point".to_string());
                keyboard_map.insert("left".to_string(), "prev_point".to_string());
                keyboard_map.insert("up".to_string(), "next_series".to_string());
                keyboard_map.insert("down".to_string(), "prev_series".to_string());
            }
            MarkType::Bar { .. } => {
                keyboard_map.insert("right".to_string(), "next_bar".to_string());
                keyboard_map.insert("left".to_string(), "prev_bar".to_string());
                keyboard_map.insert("up".to_string(), "next_category".to_string());
                keyboard_map.insert("down".to_string(), "prev_category".to_string());
            }
            MarkType::Point { .. } => {
                keyboard_map.insert("tab".to_string(), "next_point".to_string());
                keyboard_map.insert("shift+tab".to_string(), "prev_point".to_string());
            }
            _ => {
                keyboard_map.insert("arrow_keys".to_string(), "navigate".to_string());
            }
        }

        // Add custom shortcuts from config
        for (action, key) in &self.config.custom_shortcuts {
            keyboard_map.insert(key.clone(), action.clone());
        }

        keyboard_map
    }

    /// Format key combination
    fn format_key_combo(&self, key: &str, modifiers: &[String]) -> String {
        if modifiers.is_empty() {
            key.to_string()
        } else {
            format!("{}+{}", modifiers.join("+"), key)
        }
    }

    /// Execute keyboard action
    fn execute_action(&self, action: &str) -> Result<KeyboardAction, AccessibilityError> {
        match action {
            "focus_next" => Ok(KeyboardAction::FocusNext),
            "focus_previous" => Ok(KeyboardAction::FocusPrevious),
            "clear_focus" => Ok(KeyboardAction::ClearFocus),
            "activate" => Ok(KeyboardAction::Activate),
            "next_point" => Ok(KeyboardAction::NavigateNext),
            "prev_point" => Ok(KeyboardAction::NavigatePrevious),
            "next_series" => Ok(KeyboardAction::NextSeries),
            "prev_series" => Ok(KeyboardAction::PreviousSeries),
            "next_bar" => Ok(KeyboardAction::NextBar),
            "prev_bar" => Ok(KeyboardAction::PreviousBar),
            "next_category" => Ok(KeyboardAction::NextCategory),
            "prev_category" => Ok(KeyboardAction::PreviousCategory),
            _ => Ok(KeyboardAction::NoAction),
        }
    }

    /// Handle default navigation
    fn handle_default_navigation(
        &self,
        key: &str,
        _modifiers: &[String],
    ) -> Result<KeyboardAction, AccessibilityError> {
        match key {
            "Tab" => Ok(KeyboardAction::FocusNext),
            "Shift+Tab" => Ok(KeyboardAction::FocusPrevious),
            "Escape" => Ok(KeyboardAction::ClearFocus),
            "Enter" | " " => Ok(KeyboardAction::Activate),
            "ArrowRight" => Ok(KeyboardAction::NavigateNext),
            "ArrowLeft" => Ok(KeyboardAction::NavigatePrevious),
            "ArrowUp" => Ok(KeyboardAction::NavigateUp),
            "ArrowDown" => Ok(KeyboardAction::NavigateDown),
            _ => Ok(KeyboardAction::NoAction),
        }
    }
}

/// Keyboard actions that can be performed
#[derive(Debug, Clone, PartialEq)]
pub enum KeyboardAction {
    NoAction,
    FocusNext,
    FocusPrevious,
    ClearFocus,
    Activate,
    NavigateNext,
    NavigatePrevious,
    NavigateUp,
    NavigateDown,
    NextSeries,
    PreviousSeries,
    NextBar,
    PreviousBar,
    NextCategory,
    PreviousCategory,
}
