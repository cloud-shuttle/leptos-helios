//! Accessibility Error Types
//!
//! This module defines error types for accessibility operations.

use thiserror::Error;

/// Accessibility system errors
#[derive(Debug, thiserror::Error)]
pub enum AccessibilityError {
    #[error("WCAG compliance violation: {0}")]
    ComplianceViolation(String),

    #[error("Screen reader generation failed: {0}")]
    ScreenReaderError(String),

    #[error("Keyboard navigation error: {0}")]
    KeyboardNavigationError(String),

    #[error("Performance budget exceeded: {0}")]
    PerformanceBudgetExceeded(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}
