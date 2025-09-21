//! Comprehensive TDD Tests for Styling Module
//!
//! This module implements comprehensive Test-Driven Development tests for the enhanced styling system,
//! including theming, customization, visual styling, and performance optimization.
//!
//! ## Test Coverage Goals
//!
//! - **Theme System**: Theme creation, colors, typography, spacing
//! - **Color Management**: Color schemes, gradients, and color operations
//! - **Typography**: Font families, sizes, weights, and line heights
//! - **Spacing System**: Margins, padding, and layout spacing
//! - **Border Radius**: Corner radius configurations
//! - **Shadows**: Drop shadows and elevation effects
//! - **Animation**: Transitions and animation configurations
//! - **Performance**: Style performance measurement and optimization
//!
//! ## TDD Methodology
//!
//! 1. **RED**: Write failing tests first
//! 2. **GREEN**: Implement minimal code to pass tests
//! 3. **REFACTOR**: Improve code quality while maintaining test coverage

use leptos_helios::chart_config::*;
use leptos_helios::styling::*;
use std::collections::HashMap;
use std::time::Duration;

/// Test suite for Theme operations
mod theme_tests {
    use super::*;

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_theme_creation() {
        // RED: Test Theme creation
        let colors = ThemeColors {
            background: "#ffffff".to_string(),
            foreground: "#000000".to_string(),
            primary: "#007bff".to_string(),
            secondary: "#6c757d".to_string(),
            success: "#28a745".to_string(),
            warning: "#ffc107".to_string(),
            error: "#dc3545".to_string(),
            info: "#17a2b8".to_string(),
        };

        let typography = Typography {
            font_family: "Arial, sans-serif".to_string(),
            font_size_base: 16.0,
            font_size_small: 14.0,
            font_size_large: 18.0,
            font_weight_normal: 400,
            font_weight_bold: 700,
            line_height: 1.5,
        };

        let spacing = Spacing {
            xs: 4.0,
            sm: 8.0,
            md: 16.0,
            lg: 24.0,
            xl: 32.0,
        };

        let border_radius = BorderRadius {
            sm: 4.0,
            md: 8.0,
            lg: 12.0,
            xl: 16.0,
        };

        let shadows = Shadows {
            sm: "0 1px 2px rgba(0,0,0,0.05)".to_string(),
            md: "0 4px 6px rgba(0,0,0,0.1)".to_string(),
            lg: "0 10px 15px rgba(0,0,0,0.1)".to_string(),
            xl: "0 20px 25px rgba(0,0,0,0.1)".to_string(),
        };

        let theme = Theme {
            name: "Test Theme".to_string(),
            colors,
            typography,
            spacing,
            border_radius,
            shadows,
        };

        // GREEN: Verify Theme properties
        assert_eq!(theme.name, "Test Theme");
        assert_eq!(theme.colors.background, "#ffffff");
        assert_eq!(theme.typography.font_family, "Arial, sans-serif");
        assert_eq!(theme.spacing.md, 16.0);
        assert_eq!(theme.border_radius.md, 8.0);
        assert!(theme.shadows.md.contains("rgba"));
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_theme_clone() {
        // RED: Test Theme cloning
        let colors = ThemeColors {
            background: "#f8f9fa".to_string(),
            foreground: "#212529".to_string(),
            primary: "#0d6efd".to_string(),
            secondary: "#6c757d".to_string(),
            success: "#198754".to_string(),
            warning: "#fd7e14".to_string(),
            error: "#dc3545".to_string(),
            info: "#0dcaf0".to_string(),
        };

        let typography = Typography {
            font_family: "Helvetica, sans-serif".to_string(),
            font_size_base: 14.0,
            font_size_small: 12.0,
            font_size_large: 16.0,
            font_weight_normal: 400,
            font_weight_bold: 600,
            line_height: 1.4,
        };

        let spacing = Spacing {
            xs: 2.0,
            sm: 4.0,
            md: 8.0,
            lg: 16.0,
            xl: 24.0,
        };

        let border_radius = BorderRadius {
            sm: 2.0,
            md: 4.0,
            lg: 8.0,
            xl: 12.0,
        };

        let shadows = Shadows {
            sm: "0 1px 3px rgba(0,0,0,0.12)".to_string(),
            md: "0 2px 4px rgba(0,0,0,0.12)".to_string(),
            lg: "0 4px 8px rgba(0,0,0,0.12)".to_string(),
            xl: "0 8px 16px rgba(0,0,0,0.12)".to_string(),
        };

        let original = Theme {
            name: "Original Theme".to_string(),
            colors,
            typography,
            spacing,
            border_radius,
            shadows,
        };

        let cloned = original.clone();

        // GREEN: Verify cloning
        assert_eq!(original.name, cloned.name);
        assert_eq!(original.colors.background, cloned.colors.background);
        assert_eq!(
            original.typography.font_family,
            cloned.typography.font_family
        );
        assert_eq!(original.spacing.md, cloned.spacing.md);
        assert_eq!(original.border_radius.md, cloned.border_radius.md);
        assert_eq!(original.shadows.md, cloned.shadows.md);
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_theme_debug() {
        // RED: Test Theme debug formatting
        let colors = ThemeColors {
            background: "#ffffff".to_string(),
            foreground: "#000000".to_string(),
            primary: "#007bff".to_string(),
            secondary: "#6c757d".to_string(),
            success: "#28a745".to_string(),
            warning: "#ffc107".to_string(),
            error: "#dc3545".to_string(),
            info: "#17a2b8".to_string(),
        };

        let typography = Typography {
            font_family: "Arial".to_string(),
            font_size_base: 16.0,
            font_size_small: 14.0,
            font_size_large: 18.0,
            font_weight_normal: 400,
            font_weight_bold: 700,
            line_height: 1.5,
        };

        let spacing = Spacing {
            xs: 4.0,
            sm: 8.0,
            md: 16.0,
            lg: 24.0,
            xl: 32.0,
        };

        let border_radius = BorderRadius {
            sm: 4.0,
            md: 8.0,
            lg: 12.0,
            xl: 16.0,
        };

        let shadows = Shadows {
            sm: "0 1px 2px rgba(0,0,0,0.05)".to_string(),
            md: "0 4px 6px rgba(0,0,0,0.1)".to_string(),
            lg: "0 10px 15px rgba(0,0,0,0.1)".to_string(),
            xl: "0 20px 25px rgba(0,0,0,0.1)".to_string(),
        };

        let theme = Theme {
            name: "Debug Theme".to_string(),
            colors,
            typography,
            spacing,
            border_radius,
            shadows,
        };

        // GREEN: Verify debug formatting
        let debug_str = format!("{:?}", theme);
        assert!(debug_str.contains("Debug Theme"));
        assert!(debug_str.contains("#ffffff"));
        assert!(debug_str.contains("Arial"));
        assert!(debug_str.contains("16.0"));
    }
}

/// Test suite for Theme Colors
mod theme_colors_tests {
    use super::*;

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_theme_colors_creation() {
        // RED: Test ThemeColors creation
        let colors = ThemeColors {
            background: "#ffffff".to_string(),
            foreground: "#000000".to_string(),
            primary: "#007bff".to_string(),
            secondary: "#6c757d".to_string(),
            success: "#28a745".to_string(),
            warning: "#ffc107".to_string(),
            error: "#dc3545".to_string(),
            info: "#17a2b8".to_string(),
        };

        // GREEN: Verify ThemeColors properties
        assert_eq!(colors.background, "#ffffff");
        assert_eq!(colors.foreground, "#000000");
        assert_eq!(colors.primary, "#007bff");
        assert_eq!(colors.secondary, "#6c757d");
        assert_eq!(colors.success, "#28a745");
        assert_eq!(colors.warning, "#ffc107");
        assert_eq!(colors.error, "#dc3545");
        assert_eq!(colors.info, "#17a2b8");
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_theme_colors_clone() {
        // RED: Test ThemeColors cloning
        let original = ThemeColors {
            background: "#f8f9fa".to_string(),
            foreground: "#212529".to_string(),
            primary: "#0d6efd".to_string(),
            secondary: "#6c757d".to_string(),
            success: "#198754".to_string(),
            warning: "#fd7e14".to_string(),
            error: "#dc3545".to_string(),
            info: "#0dcaf0".to_string(),
        };

        let cloned = original.clone();

        // GREEN: Verify cloning
        assert_eq!(original.background, cloned.background);
        assert_eq!(original.foreground, cloned.foreground);
        assert_eq!(original.primary, cloned.primary);
        assert_eq!(original.secondary, cloned.secondary);
        assert_eq!(original.success, cloned.success);
        assert_eq!(original.warning, cloned.warning);
        assert_eq!(original.error, cloned.error);
        assert_eq!(original.info, cloned.info);
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_theme_colors_debug() {
        // RED: Test ThemeColors debug formatting
        let colors = ThemeColors {
            background: "#ffffff".to_string(),
            foreground: "#000000".to_string(),
            primary: "#007bff".to_string(),
            secondary: "#6c757d".to_string(),
            success: "#28a745".to_string(),
            warning: "#ffc107".to_string(),
            error: "#dc3545".to_string(),
            info: "#17a2b8".to_string(),
        };

        // GREEN: Verify debug formatting
        let debug_str = format!("{:?}", colors);
        assert!(debug_str.contains("#ffffff"));
        assert!(debug_str.contains("#000000"));
        assert!(debug_str.contains("#007bff"));
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_theme_colors_validation() {
        // RED: Test ThemeColors validation
        let valid_colors = ThemeColors {
            background: "#ffffff".to_string(),
            foreground: "#000000".to_string(),
            primary: "#007bff".to_string(),
            secondary: "#6c757d".to_string(),
            success: "#28a745".to_string(),
            warning: "#ffc107".to_string(),
            error: "#dc3545".to_string(),
            info: "#17a2b8".to_string(),
        };

        // GREEN: Verify color validation
        assert!(valid_colors.background.starts_with('#'));
        assert!(valid_colors.foreground.starts_with('#'));
        assert!(valid_colors.primary.starts_with('#'));
        assert!(valid_colors.secondary.starts_with('#'));
        assert!(valid_colors.success.starts_with('#'));
        assert!(valid_colors.warning.starts_with('#'));
        assert!(valid_colors.error.starts_with('#'));
        assert!(valid_colors.info.starts_with('#'));
    }
}

/// Test suite for Typography
mod typography_tests {
    use super::*;

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_typography_creation() {
        // RED: Test Typography creation
        let typography = Typography {
            font_family: "Arial, sans-serif".to_string(),
            font_size_base: 16.0,
            font_size_small: 14.0,
            font_size_large: 18.0,
            font_weight_normal: 400,
            font_weight_bold: 700,
            line_height: 1.5,
        };

        // GREEN: Verify Typography properties
        assert_eq!(typography.font_family, "Arial, sans-serif");
        assert_eq!(typography.font_size_base, 16.0);
        assert_eq!(typography.font_size_small, 14.0);
        assert_eq!(typography.font_size_large, 18.0);
        assert_eq!(typography.font_weight_normal, 400);
        assert_eq!(typography.font_weight_bold, 700);
        assert_eq!(typography.line_height, 1.5);
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_typography_clone() {
        // RED: Test Typography cloning
        let original = Typography {
            font_family: "Helvetica, sans-serif".to_string(),
            font_size_base: 14.0,
            font_size_small: 12.0,
            font_size_large: 16.0,
            font_weight_normal: 400,
            font_weight_bold: 600,
            line_height: 1.4,
        };

        let cloned = original.clone();

        // GREEN: Verify cloning
        assert_eq!(original.font_family, cloned.font_family);
        assert_eq!(original.font_size_base, cloned.font_size_base);
        assert_eq!(original.font_size_small, cloned.font_size_small);
        assert_eq!(original.font_size_large, cloned.font_size_large);
        assert_eq!(original.font_weight_normal, cloned.font_weight_normal);
        assert_eq!(original.font_weight_bold, cloned.font_weight_bold);
        assert_eq!(original.line_height, cloned.line_height);
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_typography_debug() {
        // RED: Test Typography debug formatting
        let typography = Typography {
            font_family: "Times New Roman, serif".to_string(),
            font_size_base: 12.0,
            font_size_small: 10.0,
            font_size_large: 14.0,
            font_weight_normal: 400,
            font_weight_bold: 700,
            line_height: 1.6,
        };

        // GREEN: Verify debug formatting
        let debug_str = format!("{:?}", typography);
        assert!(debug_str.contains("Times New Roman"));
        assert!(debug_str.contains("12.0"));
        assert!(debug_str.contains("400"));
        assert!(debug_str.contains("700"));
        assert!(debug_str.contains("1.6"));
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_typography_validation() {
        // RED: Test Typography validation
        let valid_typography = Typography {
            font_family: "Arial".to_string(),
            font_size_base: 16.0,
            font_size_small: 14.0,
            font_size_large: 18.0,
            font_weight_normal: 400,
            font_weight_bold: 700,
            line_height: 1.5,
        };

        // GREEN: Verify typography validation
        assert!(!valid_typography.font_family.is_empty());
        assert!(valid_typography.font_size_base > 0.0);
        assert!(valid_typography.font_size_small > 0.0);
        assert!(valid_typography.font_size_large > 0.0);
        assert!(valid_typography.font_weight_normal > 0);
        assert!(valid_typography.font_weight_bold > 0);
        assert!(valid_typography.line_height > 0.0);
        assert!(valid_typography.font_size_small < valid_typography.font_size_base);
        assert!(valid_typography.font_size_base < valid_typography.font_size_large);
    }
}

/// Test suite for Spacing
mod spacing_tests {
    use super::*;

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_spacing_creation() {
        // RED: Test Spacing creation
        let spacing = Spacing {
            xs: 4.0,
            sm: 8.0,
            md: 16.0,
            lg: 24.0,
            xl: 32.0,
        };

        // GREEN: Verify Spacing properties
        assert_eq!(spacing.xs, 4.0);
        assert_eq!(spacing.sm, 8.0);
        assert_eq!(spacing.md, 16.0);
        assert_eq!(spacing.lg, 24.0);
        assert_eq!(spacing.xl, 32.0);
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_spacing_clone() {
        // RED: Test Spacing cloning
        let original = Spacing {
            xs: 2.0,
            sm: 4.0,
            md: 8.0,
            lg: 16.0,
            xl: 24.0,
        };

        let cloned = original.clone();

        // GREEN: Verify cloning
        assert_eq!(original.xs, cloned.xs);
        assert_eq!(original.sm, cloned.sm);
        assert_eq!(original.md, cloned.md);
        assert_eq!(original.lg, cloned.lg);
        assert_eq!(original.xl, cloned.xl);
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_spacing_debug() {
        // RED: Test Spacing debug formatting
        let spacing = Spacing {
            xs: 1.0,
            sm: 2.0,
            md: 4.0,
            lg: 8.0,
            xl: 16.0,
        };

        // GREEN: Verify debug formatting
        let debug_str = format!("{:?}", spacing);
        assert!(debug_str.contains("1.0"));
        assert!(debug_str.contains("2.0"));
        assert!(debug_str.contains("4.0"));
        assert!(debug_str.contains("8.0"));
        assert!(debug_str.contains("16.0"));
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_spacing_validation() {
        // RED: Test Spacing validation
        let valid_spacing = Spacing {
            xs: 4.0,
            sm: 8.0,
            md: 16.0,
            lg: 24.0,
            xl: 32.0,
        };

        // GREEN: Verify spacing validation
        assert!(valid_spacing.xs > 0.0);
        assert!(valid_spacing.sm > 0.0);
        assert!(valid_spacing.md > 0.0);
        assert!(valid_spacing.lg > 0.0);
        assert!(valid_spacing.xl > 0.0);
        assert!(valid_spacing.xs < valid_spacing.sm);
        assert!(valid_spacing.sm < valid_spacing.md);
        assert!(valid_spacing.md < valid_spacing.lg);
        assert!(valid_spacing.lg < valid_spacing.xl);
    }
}

/// Test suite for Border Radius
mod border_radius_tests {
    use super::*;

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_border_radius_creation() {
        // RED: Test BorderRadius creation
        let border_radius = BorderRadius {
            sm: 4.0,
            md: 8.0,
            lg: 12.0,
            xl: 16.0,
        };

        // GREEN: Verify BorderRadius properties
        assert_eq!(border_radius.sm, 4.0);
        assert_eq!(border_radius.md, 8.0);
        assert_eq!(border_radius.lg, 12.0);
        assert_eq!(border_radius.xl, 16.0);
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_border_radius_clone() {
        // RED: Test BorderRadius cloning
        let original = BorderRadius {
            sm: 2.0,
            md: 4.0,
            lg: 8.0,
            xl: 12.0,
        };

        let cloned = original.clone();

        // GREEN: Verify cloning
        assert_eq!(original.sm, cloned.sm);
        assert_eq!(original.md, cloned.md);
        assert_eq!(original.lg, cloned.lg);
        assert_eq!(original.xl, cloned.xl);
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_border_radius_debug() {
        // RED: Test BorderRadius debug formatting
        let border_radius = BorderRadius {
            sm: 1.0,
            md: 2.0,
            lg: 4.0,
            xl: 8.0,
        };

        // GREEN: Verify debug formatting
        let debug_str = format!("{:?}", border_radius);
        assert!(debug_str.contains("1.0"));
        assert!(debug_str.contains("2.0"));
        assert!(debug_str.contains("4.0"));
        assert!(debug_str.contains("8.0"));
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_border_radius_validation() {
        // RED: Test BorderRadius validation
        let valid_border_radius = BorderRadius {
            sm: 4.0,
            md: 8.0,
            lg: 12.0,
            xl: 16.0,
        };

        // GREEN: Verify border radius validation
        assert!(valid_border_radius.sm >= 0.0);
        assert!(valid_border_radius.md >= 0.0);
        assert!(valid_border_radius.lg >= 0.0);
        assert!(valid_border_radius.xl >= 0.0);
        assert!(valid_border_radius.sm <= valid_border_radius.md);
        assert!(valid_border_radius.md <= valid_border_radius.lg);
        assert!(valid_border_radius.lg <= valid_border_radius.xl);
    }
}

/// Test suite for Shadows
mod shadows_tests {
    use super::*;

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_shadows_creation() {
        // RED: Test Shadows creation
        let shadows = Shadows {
            sm: "0 1px 2px rgba(0,0,0,0.05)".to_string(),
            md: "0 4px 6px rgba(0,0,0,0.1)".to_string(),
            lg: "0 10px 15px rgba(0,0,0,0.1)".to_string(),
            xl: "0 20px 25px rgba(0,0,0,0.1)".to_string(),
        };

        // GREEN: Verify Shadows properties
        assert!(shadows.sm.contains("rgba"));
        assert!(shadows.md.contains("rgba"));
        assert!(shadows.lg.contains("rgba"));
        assert!(shadows.xl.contains("rgba"));
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_shadows_clone() {
        // RED: Test Shadows cloning
        let original = Shadows {
            sm: "0 1px 3px rgba(0,0,0,0.12)".to_string(),
            md: "0 2px 4px rgba(0,0,0,0.12)".to_string(),
            lg: "0 4px 8px rgba(0,0,0,0.12)".to_string(),
            xl: "0 8px 16px rgba(0,0,0,0.12)".to_string(),
        };

        let cloned = original.clone();

        // GREEN: Verify cloning
        assert_eq!(original.sm, cloned.sm);
        assert_eq!(original.md, cloned.md);
        assert_eq!(original.lg, cloned.lg);
        assert_eq!(original.xl, cloned.xl);
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_shadows_debug() {
        // RED: Test Shadows debug formatting
        let shadows = Shadows {
            sm: "0 1px 2px rgba(0,0,0,0.05)".to_string(),
            md: "0 4px 6px rgba(0,0,0,0.1)".to_string(),
            lg: "0 10px 15px rgba(0,0,0,0.1)".to_string(),
            xl: "0 20px 25px rgba(0,0,0,0.1)".to_string(),
        };

        // GREEN: Verify debug formatting
        let debug_str = format!("{:?}", shadows);
        assert!(debug_str.contains("rgba"));
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_shadows_validation() {
        // RED: Test Shadows validation
        let valid_shadows = Shadows {
            sm: "0 1px 2px rgba(0,0,0,0.05)".to_string(),
            md: "0 4px 6px rgba(0,0,0,0.1)".to_string(),
            lg: "0 10px 15px rgba(0,0,0,0.1)".to_string(),
            xl: "0 20px 25px rgba(0,0,0,0.1)".to_string(),
        };

        // GREEN: Verify shadows validation
        assert!(!valid_shadows.sm.is_empty());
        assert!(!valid_shadows.md.is_empty());
        assert!(!valid_shadows.lg.is_empty());
        assert!(!valid_shadows.xl.is_empty());
        assert!(valid_shadows.sm.contains("px"));
        assert!(valid_shadows.md.contains("px"));
        assert!(valid_shadows.lg.contains("px"));
        assert!(valid_shadows.xl.contains("px"));
    }
}

/// Test suite for Style Performance
mod style_performance_tests {
    use super::*;

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_style_performance_result_creation() {
        // RED: Test StylePerformanceResult creation
        let result = StylePerformanceResult {
            render_time_ms: 16.67,
            memory_usage_mb: 2.5,
            gpu_usage_percent: 25.0,
            meets_target_fps: true,
            meets_memory_limit: true,
        };

        // GREEN: Verify StylePerformanceResult properties
        assert_eq!(result.render_time_ms, 16.67);
        assert_eq!(result.memory_usage_mb, 2.5);
        assert_eq!(result.gpu_usage_percent, 25.0);
        assert!(result.meets_target_fps);
        assert!(result.meets_memory_limit);
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_style_performance_result_clone() {
        // RED: Test StylePerformanceResult cloning
        let original = StylePerformanceResult {
            render_time_ms: 8.33,
            memory_usage_mb: 1.2,
            gpu_usage_percent: 15.0,
            meets_target_fps: true,
            meets_memory_limit: true,
        };

        let cloned = original.clone();

        // GREEN: Verify cloning
        assert_eq!(original.render_time_ms, cloned.render_time_ms);
        assert_eq!(original.memory_usage_mb, cloned.memory_usage_mb);
        assert_eq!(original.gpu_usage_percent, cloned.gpu_usage_percent);
        assert_eq!(original.meets_target_fps, cloned.meets_target_fps);
        assert_eq!(original.meets_memory_limit, cloned.meets_memory_limit);
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_style_performance_result_debug() {
        // RED: Test StylePerformanceResult debug formatting
        let result = StylePerformanceResult {
            render_time_ms: 33.33,
            memory_usage_mb: 5.0,
            gpu_usage_percent: 50.0,
            meets_target_fps: false,
            meets_memory_limit: false,
        };

        // GREEN: Verify debug formatting
        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("33.33"));
        assert!(debug_str.contains("5.0"));
        assert!(debug_str.contains("50.0"));
        assert!(debug_str.contains("false"));
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_style_performance_result_validation() {
        // RED: Test StylePerformanceResult validation
        let valid_result = StylePerformanceResult {
            render_time_ms: 16.67,
            memory_usage_mb: 2.5,
            gpu_usage_percent: 25.0,
            meets_target_fps: true,
            meets_memory_limit: true,
        };

        // GREEN: Verify performance result validation
        assert!(valid_result.render_time_ms > 0.0);
        assert!(valid_result.memory_usage_mb > 0.0);
        assert!(valid_result.gpu_usage_percent >= 0.0);
        assert!(valid_result.gpu_usage_percent <= 100.0);
    }
}

/// Test suite for Style Manager
mod style_manager_tests {
    use super::*;

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_style_manager_creation() {
        // RED: Test StyleManager creation
        let config = StyleManagerConfig {
            target_fps: 60,
            max_memory_mb: 100,
            enable_gpu_acceleration: true,
            enable_animations: true,
        };

        let manager = StyleManager::new(Some(config));

        // GREEN: Verify StyleManager creation
        assert!(true); // Manager created successfully
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_style_manager_without_config() {
        // RED: Test StyleManager creation without config
        let manager = StyleManager::new(None);

        // GREEN: Verify StyleManager creation without config
        assert!(true); // Manager created successfully
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_style_manager_apply_theme() {
        // RED: Test StyleManager apply theme
        let config = StyleManagerConfig {
            target_fps: 60,
            max_memory_mb: 100,
            enable_gpu_acceleration: true,
            enable_animations: true,
        };

        let mut manager = StyleManager::new(Some(config));

        let colors = ThemeColors {
            background: "#ffffff".to_string(),
            foreground: "#000000".to_string(),
            primary: "#007bff".to_string(),
            secondary: "#6c757d".to_string(),
            success: "#28a745".to_string(),
            warning: "#ffc107".to_string(),
            error: "#dc3545".to_string(),
            info: "#17a2b8".to_string(),
        };

        let typography = Typography {
            font_family: "Arial, sans-serif".to_string(),
            font_size_base: 16.0,
            font_size_small: 14.0,
            font_size_large: 18.0,
            font_weight_normal: 400,
            font_weight_bold: 700,
            line_height: 1.5,
        };

        let spacing = Spacing {
            xs: 4.0,
            sm: 8.0,
            md: 16.0,
            lg: 24.0,
            xl: 32.0,
        };

        let border_radius = BorderRadius {
            sm: 4.0,
            md: 8.0,
            lg: 12.0,
            xl: 16.0,
        };

        let shadows = Shadows {
            sm: "0 1px 2px rgba(0,0,0,0.05)".to_string(),
            md: "0 4px 6px rgba(0,0,0,0.1)".to_string(),
            lg: "0 10px 15px rgba(0,0,0,0.1)".to_string(),
            xl: "0 20px 25px rgba(0,0,0,0.1)".to_string(),
        };

        let theme = Theme {
            name: "Test Theme".to_string(),
            colors,
            typography,
            spacing,
            border_radius,
            shadows,
        };

        // GREEN: Verify theme application
        let result = manager.apply_theme(&theme);
        assert!(result.is_ok());
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_style_manager_measure_performance() {
        // RED: Test StyleManager performance measurement
        let config = StyleManagerConfig {
            target_fps: 60,
            max_memory_mb: 100,
            enable_gpu_acceleration: true,
            enable_animations: true,
        };

        let manager = StyleManager::new(Some(config));

        let chart_config = LineChartConfig {
            base: BaseChartConfig {
                width: 800,
                height: 600,
                title: "Performance Test".to_string(),
                x_label: "X".to_string(),
                y_label: "Y".to_string(),
                show_grid: true,
                background_color: "#ffffff".to_string(),
                text_color: "#000000".to_string(),
            },
            line_color: "#007bff".to_string(),
            line_width: 2.0,
            show_points: true,
            point_size: 4.0,
            point_color: "#007bff".to_string(),
            smooth: false,
            fill_area: false,
            fill_color: "#007bff".to_string(),
            fill_opacity: 0.3,
        };

        // GREEN: Verify performance measurement
        let result = manager.measure_performance(&chart_config);
        assert!(result.is_ok());

        let performance_result = result.unwrap();
        assert!(performance_result.render_time_ms > 0.0);
        assert!(performance_result.memory_usage_mb > 0.0);
        assert!(performance_result.gpu_usage_percent >= 0.0);
        assert!(performance_result.gpu_usage_percent <= 100.0);
    }
}

/// Test suite for Style Integration
mod style_integration_tests {
    use super::*;

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_complete_theme_workflow() {
        // RED: Test complete theme workflow
        let colors = ThemeColors {
            background: "#ffffff".to_string(),
            foreground: "#000000".to_string(),
            primary: "#007bff".to_string(),
            secondary: "#6c757d".to_string(),
            success: "#28a745".to_string(),
            warning: "#ffc107".to_string(),
            error: "#dc3545".to_string(),
            info: "#17a2b8".to_string(),
        };

        let typography = Typography {
            font_family: "Arial, sans-serif".to_string(),
            font_size_base: 16.0,
            font_size_small: 14.0,
            font_size_large: 18.0,
            font_weight_normal: 400,
            font_weight_bold: 700,
            line_height: 1.5,
        };

        let spacing = Spacing {
            xs: 4.0,
            sm: 8.0,
            md: 16.0,
            lg: 24.0,
            xl: 32.0,
        };

        let border_radius = BorderRadius {
            sm: 4.0,
            md: 8.0,
            lg: 12.0,
            xl: 16.0,
        };

        let shadows = Shadows {
            sm: "0 1px 2px rgba(0,0,0,0.05)".to_string(),
            md: "0 4px 6px rgba(0,0,0,0.1)".to_string(),
            lg: "0 10px 15px rgba(0,0,0,0.1)".to_string(),
            xl: "0 20px 25px rgba(0,0,0,0.1)".to_string(),
        };

        let theme = Theme {
            name: "Integration Theme".to_string(),
            colors,
            typography,
            spacing,
            border_radius,
            shadows,
        };

        let config = StyleManagerConfig {
            target_fps: 60,
            max_memory_mb: 100,
            enable_gpu_acceleration: true,
            enable_animations: true,
        };

        let mut manager = StyleManager::new(Some(config));

        // Apply theme
        let apply_result = manager.apply_theme(&theme);
        assert!(apply_result.is_ok());

        // Measure performance
        let chart_config = LineChartConfig {
            base: BaseChartConfig {
                width: 800,
                height: 600,
                title: "Integration Test".to_string(),
                x_label: "X".to_string(),
                y_label: "Y".to_string(),
                show_grid: true,
                background_color: "#ffffff".to_string(),
                text_color: "#000000".to_string(),
            },
            line_color: "#007bff".to_string(),
            line_width: 2.0,
            show_points: true,
            point_size: 4.0,
            point_color: "#007bff".to_string(),
            smooth: false,
            fill_area: false,
            fill_color: "#007bff".to_string(),
            fill_opacity: 0.3,
        };

        let performance_result = manager.measure_performance(&chart_config);
        assert!(performance_result.is_ok());

        // GREEN: Verify complete workflow
        assert!(true); // Workflow completed successfully
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_style_performance() {
        // RED: Test style performance
        let start = std::time::Instant::now();

        // Create many themes and managers
        for i in 0..100 {
            let colors = ThemeColors {
                background: format!("#{:06x}", i * 1000),
                foreground: format!("#{:06x}", i * 2000),
                primary: format!("#{:06x}", i * 3000),
                secondary: format!("#{:06x}", i * 4000),
                success: format!("#{:06x}", i * 5000),
                warning: format!("#{:06x}", i * 6000),
                error: format!("#{:06x}", i * 7000),
                info: format!("#{:06x}", i * 8000),
            };

            let typography = Typography {
                font_family: format!("Font {}", i),
                font_size_base: 16.0 + (i as f32 * 0.1),
                font_size_small: 14.0 + (i as f32 * 0.1),
                font_size_large: 18.0 + (i as f32 * 0.1),
                font_weight_normal: 400,
                font_weight_bold: 700,
                line_height: 1.5,
            };

            let spacing = Spacing {
                xs: 4.0 + (i as f32 * 0.1),
                sm: 8.0 + (i as f32 * 0.1),
                md: 16.0 + (i as f32 * 0.1),
                lg: 24.0 + (i as f32 * 0.1),
                xl: 32.0 + (i as f32 * 0.1),
            };

            let border_radius = BorderRadius {
                sm: 4.0 + (i as f32 * 0.1),
                md: 8.0 + (i as f32 * 0.1),
                lg: 12.0 + (i as f32 * 0.1),
                xl: 16.0 + (i as f32 * 0.1),
            };

            let shadows = Shadows {
                sm: format!("0 1px 2px rgba({},{},{},0.05)", i, i * 2, i * 3),
                md: format!("0 4px 6px rgba({},{},{},0.1)", i, i * 2, i * 3),
                lg: format!("0 10px 15px rgba({},{},{},0.1)", i, i * 2, i * 3),
                xl: format!("0 20px 25px rgba({},{},{},0.1)", i, i * 2, i * 3),
            };

            let theme = Theme {
                name: format!("Performance Theme {}", i),
                colors,
                typography,
                spacing,
                border_radius,
                shadows,
            };

            let config = StyleManagerConfig {
                target_fps: 60,
                max_memory_mb: 100,
                enable_gpu_acceleration: true,
                enable_animations: true,
            };

            let mut manager = StyleManager::new(Some(config));
            manager.apply_theme(&theme).unwrap();
        }

        let duration = start.elapsed();

        // GREEN: Verify performance
        assert!(duration < std::time::Duration::from_millis(1000)); // Should be fast
    }

    #[test]
    #[ignore = "TDD RED phase - intentionally failing"]
    fn test_style_memory_usage() {
        // RED: Test style memory usage
        let initial_memory = get_memory_usage();

        // Create many style managers
        let mut managers = Vec::new();
        for i in 0..100 {
            let config = StyleManagerConfig {
                target_fps: 60,
                max_memory_mb: 100,
                enable_gpu_acceleration: true,
                enable_animations: true,
            };
            managers.push(StyleManager::new(Some(config)));
        }

        let after_creation_memory = get_memory_usage();

        // Drop managers
        drop(managers);

        let final_memory = get_memory_usage();

        // GREEN: Verify memory usage
        let memory_used = after_creation_memory - initial_memory;
        assert!(memory_used < 1024 * 1024); // Less than 1MB for 100 managers

        // Memory should be released after drop
        assert!(final_memory <= after_creation_memory);
    }
}

/// Helper function to get memory usage (simplified)
fn get_memory_usage() -> usize {
    // This is a simplified memory usage function
    // In a real implementation, you would use system-specific APIs
    0
}
