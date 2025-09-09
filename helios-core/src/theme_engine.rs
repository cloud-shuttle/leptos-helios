//! Advanced Theme Engine
//!
//! This module provides a comprehensive theming system for Helios visualizations,
//! including CSS-in-Rust styling, custom components, animations, and responsive design.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

/// Theme engine errors
#[derive(Debug, Error)]
pub enum ThemeError {
    #[error("Theme not found: {id}")]
    ThemeNotFound { id: String },

    #[error("Invalid theme format: {message}")]
    InvalidThemeFormat { message: String },

    #[error("Style compilation failed: {message}")]
    StyleCompilationFailed { message: String },

    #[error("Component validation failed: {message}")]
    ComponentValidationFailed { message: String },

    #[error("Animation configuration error: {message}")]
    AnimationError { message: String },

    #[error("Responsive design error: {message}")]
    ResponsiveError { message: String },
}

/// Theme identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThemeId(pub String);

/// Theme value types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThemeValue {
    Color(String),
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<ThemeValue>),
    Object(HashMap<String, ThemeValue>),
}

/// Color palette for themes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub background: String,
    pub surface: String,
    pub text: String,
    pub text_secondary: String,
    pub border: String,
    pub error: String,
    pub warning: String,
    pub success: String,
    pub info: String,
}

/// Typography configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographySet {
    pub font_family: String,
    pub font_size_base: f64,
    pub font_size_small: f64,
    pub font_size_large: f64,
    pub font_weight_normal: u16,
    pub font_weight_bold: u16,
    pub line_height: f64,
    pub letter_spacing: f64,
}

/// Spacing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingSet {
    pub xs: f64,
    pub sm: f64,
    pub md: f64,
    pub lg: f64,
    pub xl: f64,
    pub xxl: f64,
}

/// Shadow configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowSet {
    pub small: String,
    pub medium: String,
    pub large: String,
    pub xl: String,
}

/// Border configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderSet {
    pub width: f64,
    pub radius: f64,
    pub style: String,
}

/// Theme variables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeVariables {
    pub colors: ColorPalette,
    pub typography: TypographySet,
    pub spacing: SpacingSet,
    pub shadows: ShadowSet,
    pub borders: BorderSet,
}

/// Component type identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComponentType {
    Chart,
    Axis,
    Legend,
    Tooltip,
    Grid,
    DataPoint,
    Line,
    Bar,
    Area,
    Scatter,
    Pie,
    Custom(String),
}

/// Component style configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStyle {
    pub component_type: ComponentType,
    pub styles: HashMap<String, ThemeValue>,
    pub variants: HashMap<String, HashMap<String, ThemeValue>>,
    pub states: HashMap<String, HashMap<String, ThemeValue>>,
}

/// Animation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub name: String,
    pub duration: f64,
    pub easing: String,
    pub delay: f64,
    pub iteration_count: String,
    pub direction: String,
    pub fill_mode: String,
}

/// Animation set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationSet {
    pub animations: Vec<Animation>,
    pub transitions: HashMap<String, String>,
}

/// Breakpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breakpoint {
    pub name: String,
    pub min_width: Option<f64>,
    pub max_width: Option<f64>,
    pub styles: HashMap<String, ThemeValue>,
}

/// Breakpoint set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakpointSet {
    pub breakpoints: Vec<Breakpoint>,
    pub default: HashMap<String, ThemeValue>,
}

/// Complete theme definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub id: ThemeId,
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub variables: ThemeVariables,
    pub components: HashMap<ComponentType, ComponentStyle>,
    pub animations: AnimationSet,
    pub breakpoints: BreakpointSet,
    pub metadata: HashMap<String, ThemeValue>,
}

/// Compiled style information
#[derive(Debug, Clone)]
pub struct CompiledStyle {
    pub css: String,
    pub variables: HashMap<String, String>,
    pub media_queries: Vec<String>,
    pub animations: Vec<String>,
}

/// Style cache entry
#[derive(Debug, Clone)]
pub struct StyleCacheEntry {
    pub style: CompiledStyle,
    pub timestamp: u64,
    pub size: usize,
}

/// Theme registry
#[derive(Debug)]
pub struct ThemeRegistry {
    themes: HashMap<ThemeId, Theme>,
    active_theme: Option<ThemeId>,
    default_theme: ThemeId,
}

impl ThemeRegistry {
    /// Create a new theme registry
    pub fn new() -> Self {
        Self {
            themes: HashMap::new(),
            active_theme: None,
            default_theme: ThemeId("default".to_string()),
        }
    }

    /// Register a theme
    pub fn register_theme(&mut self, theme: Theme) -> Result<(), ThemeError> {
        self.validate_theme(&theme)?;
        self.themes.insert(theme.id.clone(), theme);
        Ok(())
    }

    /// Get a theme by ID
    pub fn get_theme(&self, id: &ThemeId) -> Result<&Theme, ThemeError> {
        self.themes
            .get(id)
            .ok_or_else(|| ThemeError::ThemeNotFound { id: id.0.clone() })
    }

    /// Set the active theme
    pub fn set_active_theme(&mut self, id: &ThemeId) -> Result<(), ThemeError> {
        if self.themes.contains_key(id) {
            self.active_theme = Some(id.clone());
            Ok(())
        } else {
            Err(ThemeError::ThemeNotFound { id: id.0.clone() })
        }
    }

    /// Get the active theme
    pub fn get_active_theme(&self) -> Result<&Theme, ThemeError> {
        if let Some(ref id) = self.active_theme {
            self.get_theme(id)
        } else {
            self.get_theme(&self.default_theme)
        }
    }

    /// List all registered themes
    pub fn list_themes(&self) -> Vec<&Theme> {
        self.themes.values().collect()
    }

    /// Validate a theme
    fn validate_theme(&self, theme: &Theme) -> Result<(), ThemeError> {
        // Validate theme ID
        if theme.id.0.is_empty() {
            return Err(ThemeError::InvalidThemeFormat {
                message: "Theme ID cannot be empty".to_string(),
            });
        }

        // Validate theme name
        if theme.name.is_empty() {
            return Err(ThemeError::InvalidThemeFormat {
                message: "Theme name cannot be empty".to_string(),
            });
        }

        // Validate version
        if theme.version.is_empty() {
            return Err(ThemeError::InvalidThemeFormat {
                message: "Theme version cannot be empty".to_string(),
            });
        }

        // Validate color palette
        self.validate_color_palette(&theme.variables.colors)?;

        // Validate typography
        self.validate_typography(&theme.variables.typography)?;

        // Validate spacing
        self.validate_spacing(&theme.variables.spacing)?;

        // Validate animations
        self.validate_animations(&theme.animations)?;

        // Validate breakpoints
        self.validate_breakpoints(&theme.breakpoints)?;

        Ok(())
    }

    /// Validate color palette
    fn validate_color_palette(&self, palette: &ColorPalette) -> Result<(), ThemeError> {
        let colors = vec![
            &palette.primary,
            &palette.secondary,
            &palette.accent,
            &palette.background,
            &palette.surface,
            &palette.text,
            &palette.text_secondary,
            &palette.border,
            &palette.error,
            &palette.warning,
            &palette.success,
            &palette.info,
        ];

        for color in colors {
            if !self.is_valid_color(color) {
                return Err(ThemeError::InvalidThemeFormat {
                    message: format!("Invalid color format: {}", color),
                });
            }
        }

        Ok(())
    }

    /// Validate typography
    fn validate_typography(&self, typography: &TypographySet) -> Result<(), ThemeError> {
        if typography.font_family.is_empty() {
            return Err(ThemeError::InvalidThemeFormat {
                message: "Font family cannot be empty".to_string(),
            });
        }

        if typography.font_size_base <= 0.0 {
            return Err(ThemeError::InvalidThemeFormat {
                message: "Font size base must be positive".to_string(),
            });
        }

        if typography.line_height <= 0.0 {
            return Err(ThemeError::InvalidThemeFormat {
                message: "Line height must be positive".to_string(),
            });
        }

        Ok(())
    }

    /// Validate spacing
    fn validate_spacing(&self, spacing: &SpacingSet) -> Result<(), ThemeError> {
        let spacing_values = vec![
            spacing.xs,
            spacing.sm,
            spacing.md,
            spacing.lg,
            spacing.xl,
            spacing.xxl,
        ];

        for value in spacing_values {
            if value < 0.0 {
                return Err(ThemeError::InvalidThemeFormat {
                    message: "Spacing values cannot be negative".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Validate animations
    fn validate_animations(&self, animations: &AnimationSet) -> Result<(), ThemeError> {
        for animation in &animations.animations {
            if animation.name.is_empty() {
                return Err(ThemeError::AnimationError {
                    message: "Animation name cannot be empty".to_string(),
                });
            }

            if animation.duration < 0.0 {
                return Err(ThemeError::AnimationError {
                    message: "Animation duration cannot be negative".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Validate breakpoints
    fn validate_breakpoints(&self, breakpoints: &BreakpointSet) -> Result<(), ThemeError> {
        for breakpoint in &breakpoints.breakpoints {
            if breakpoint.name.is_empty() {
                return Err(ThemeError::ResponsiveError {
                    message: "Breakpoint name cannot be empty".to_string(),
                });
            }

            if let (Some(min), Some(max)) = (breakpoint.min_width, breakpoint.max_width) {
                if min >= max {
                    return Err(ThemeError::ResponsiveError {
                        message: "Breakpoint min_width must be less than max_width".to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Check if a color string is valid
    fn is_valid_color(&self, color: &str) -> bool {
        // Basic color validation - can be enhanced
        color.starts_with('#') && color.len() == 7
            || color.starts_with("rgb(")
            || color.starts_with("rgba(")
            || color.starts_with("hsl(")
            || color.starts_with("hsla(")
            || ["red", "blue", "green", "black", "white", "transparent"].contains(&color)
    }
}

/// Style compiler for CSS-in-Rust
#[derive(Debug)]
pub struct StyleCompiler {
    cache: HashMap<String, CompiledStyle>,
    optimization_level: OptimizationLevel,
}

#[derive(Debug, Clone, Copy)]
pub enum OptimizationLevel {
    None,
    Basic,
    Aggressive,
}

impl StyleCompiler {
    /// Create a new style compiler
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            optimization_level: OptimizationLevel::Basic,
        }
    }

    /// Compile a theme to CSS
    pub fn compile_theme(&mut self, theme: &Theme) -> Result<CompiledStyle, ThemeError> {
        let cache_key = format!("theme_{}", theme.id.0);

        if let Some(cached) = self.cache.get(&cache_key) {
            return Ok(cached.clone());
        }

        let compiled = self.compile_theme_internal(theme)?;
        self.cache.insert(cache_key, compiled.clone());

        Ok(compiled)
    }

    /// Internal theme compilation
    fn compile_theme_internal(&self, theme: &Theme) -> Result<CompiledStyle, ThemeError> {
        let mut css = String::new();
        let mut variables = HashMap::new();
        let mut media_queries = Vec::new();
        let mut animations = Vec::new();

        // Generate CSS variables
        self.generate_css_variables(&theme.variables, &mut variables);

        // Generate component styles
        self.generate_component_styles(&theme.components, &mut css);

        // Generate animations
        self.generate_animations(&theme.animations, &mut animations);

        // Generate responsive styles
        self.generate_responsive_styles(&theme.breakpoints, &mut media_queries);

        // Combine all CSS
        let mut full_css = String::new();

        // Add CSS variables
        full_css.push_str(":root {\n");
        for (name, value) in &variables {
            full_css.push_str(&format!("  --{}: {};\n", name, value));
        }
        full_css.push_str("}\n\n");

        // Add component styles
        full_css.push_str(&css);
        full_css.push('\n');

        // Add animations
        for animation in &animations {
            full_css.push_str(animation);
            full_css.push('\n');
        }

        // Add media queries
        for media_query in &media_queries {
            full_css.push_str(media_query);
            full_css.push('\n');
        }

        Ok(CompiledStyle {
            css: full_css,
            variables,
            media_queries,
            animations,
        })
    }

    /// Generate CSS variables from theme variables
    fn generate_css_variables(
        &self,
        variables: &ThemeVariables,
        output: &mut HashMap<String, String>,
    ) {
        // Color variables
        output.insert(
            "color-primary".to_string(),
            variables.colors.primary.clone(),
        );
        output.insert(
            "color-secondary".to_string(),
            variables.colors.secondary.clone(),
        );
        output.insert("color-accent".to_string(), variables.colors.accent.clone());
        output.insert(
            "color-background".to_string(),
            variables.colors.background.clone(),
        );
        output.insert(
            "color-surface".to_string(),
            variables.colors.surface.clone(),
        );
        output.insert("color-text".to_string(), variables.colors.text.clone());
        output.insert(
            "color-text-secondary".to_string(),
            variables.colors.text_secondary.clone(),
        );
        output.insert("color-border".to_string(), variables.colors.border.clone());
        output.insert("color-error".to_string(), variables.colors.error.clone());
        output.insert(
            "color-warning".to_string(),
            variables.colors.warning.clone(),
        );
        output.insert(
            "color-success".to_string(),
            variables.colors.success.clone(),
        );
        output.insert("color-info".to_string(), variables.colors.info.clone());

        // Typography variables
        output.insert(
            "font-family".to_string(),
            variables.typography.font_family.clone(),
        );
        output.insert(
            "font-size-base".to_string(),
            format!("{}px", variables.typography.font_size_base),
        );
        output.insert(
            "font-size-small".to_string(),
            format!("{}px", variables.typography.font_size_small),
        );
        output.insert(
            "font-size-large".to_string(),
            format!("{}px", variables.typography.font_size_large),
        );
        output.insert(
            "font-weight-normal".to_string(),
            variables.typography.font_weight_normal.to_string(),
        );
        output.insert(
            "font-weight-bold".to_string(),
            variables.typography.font_weight_bold.to_string(),
        );
        output.insert(
            "line-height".to_string(),
            variables.typography.line_height.to_string(),
        );
        output.insert(
            "letter-spacing".to_string(),
            format!("{}px", variables.typography.letter_spacing),
        );

        // Spacing variables
        output.insert(
            "spacing-xs".to_string(),
            format!("{}px", variables.spacing.xs),
        );
        output.insert(
            "spacing-sm".to_string(),
            format!("{}px", variables.spacing.sm),
        );
        output.insert(
            "spacing-md".to_string(),
            format!("{}px", variables.spacing.md),
        );
        output.insert(
            "spacing-lg".to_string(),
            format!("{}px", variables.spacing.lg),
        );
        output.insert(
            "spacing-xl".to_string(),
            format!("{}px", variables.spacing.xl),
        );
        output.insert(
            "spacing-xxl".to_string(),
            format!("{}px", variables.spacing.xxl),
        );

        // Shadow variables
        output.insert("shadow-small".to_string(), variables.shadows.small.clone());
        output.insert(
            "shadow-medium".to_string(),
            variables.shadows.medium.clone(),
        );
        output.insert("shadow-large".to_string(), variables.shadows.large.clone());
        output.insert("shadow-xl".to_string(), variables.shadows.xl.clone());

        // Border variables
        output.insert(
            "border-width".to_string(),
            format!("{}px", variables.borders.width),
        );
        output.insert(
            "border-radius".to_string(),
            format!("{}px", variables.borders.radius),
        );
        output.insert("border-style".to_string(), variables.borders.style.clone());
    }

    /// Generate component styles
    fn generate_component_styles(
        &self,
        components: &HashMap<ComponentType, ComponentStyle>,
        output: &mut String,
    ) {
        for (component_type, style) in components {
            let selector = self.get_component_selector(component_type);
            output.push_str(&format!(".{} {{\n", selector));

            for (property, value) in &style.styles {
                if let Some(css_value) = self.theme_value_to_css(value) {
                    output.push_str(&format!("  {}: {};\n", property, css_value));
                }
            }

            output.push_str("}\n\n");

            // Generate variants
            for (variant_name, variant_styles) in &style.variants {
                output.push_str(&format!(".{}-{} {{\n", selector, variant_name));
                for (property, value) in variant_styles {
                    if let Some(css_value) = self.theme_value_to_css(value) {
                        output.push_str(&format!("  {}: {};\n", property, css_value));
                    }
                }
                output.push_str("}\n\n");
            }

            // Generate states
            for (state_name, state_styles) in &style.states {
                output.push_str(&format!(".{}:{} {{\n", selector, state_name));
                for (property, value) in state_styles {
                    if let Some(css_value) = self.theme_value_to_css(value) {
                        output.push_str(&format!("  {}: {};\n", property, css_value));
                    }
                }
                output.push_str("}\n\n");
            }
        }
    }

    /// Generate animations
    fn generate_animations(&self, animations: &AnimationSet, output: &mut Vec<String>) {
        for animation in &animations.animations {
            let keyframes = format!(
                "@keyframes {} {{\n  from {{ opacity: 0; }}\n  to {{ opacity: 1; }}\n}}",
                animation.name
            );
            output.push(keyframes);
        }

        for (property, transition) in &animations.transitions {
            let transition_css = format!(
                ".transition-{} {{\n  transition: {};\n}}",
                property, transition
            );
            output.push(transition_css);
        }
    }

    /// Generate responsive styles
    fn generate_responsive_styles(&self, breakpoints: &BreakpointSet, output: &mut Vec<String>) {
        for breakpoint in &breakpoints.breakpoints {
            let mut media_query = String::new();

            if let Some(min_width) = breakpoint.min_width {
                media_query.push_str(&format!("(min-width: {}px)", min_width));
            }

            if let Some(max_width) = breakpoint.max_width {
                if !media_query.is_empty() {
                    media_query.push_str(" and ");
                }
                media_query.push_str(&format!("(max-width: {}px)", max_width));
            }

            let mut styles = String::new();
            for (property, value) in &breakpoint.styles {
                if let Some(css_value) = self.theme_value_to_css(value) {
                    styles.push_str(&format!("  {}: {};\n", property, css_value));
                }
            }

            if !styles.is_empty() {
                output.push(format!("@media {} {{\n{}}}\n", media_query, styles));
            }
        }
    }

    /// Get component selector
    fn get_component_selector(&self, component_type: &ComponentType) -> String {
        match component_type {
            ComponentType::Chart => "helios-chart",
            ComponentType::Axis => "helios-axis",
            ComponentType::Legend => "helios-legend",
            ComponentType::Tooltip => "helios-tooltip",
            ComponentType::Grid => "helios-grid",
            ComponentType::DataPoint => "helios-data-point",
            ComponentType::Line => "helios-line",
            ComponentType::Bar => "helios-bar",
            ComponentType::Area => "helios-area",
            ComponentType::Scatter => "helios-scatter",
            ComponentType::Pie => "helios-pie",
            ComponentType::Custom(name) => &format!("helios-{}", name),
        }
        .to_string()
    }

    /// Convert theme value to CSS value
    fn theme_value_to_css(&self, value: &ThemeValue) -> Option<String> {
        match value {
            ThemeValue::Color(color) => Some(color.clone()),
            ThemeValue::Number(num) => Some(format!("{}px", num)),
            ThemeValue::String(s) => Some(s.clone()),
            ThemeValue::Boolean(b) => Some(b.to_string()),
            ThemeValue::Array(arr) => {
                let values: Vec<String> = arr
                    .iter()
                    .filter_map(|v| self.theme_value_to_css(v))
                    .collect();
                Some(values.join(" "))
            }
            ThemeValue::Object(_) => None, // Objects not supported in CSS
        }
    }
}

/// Main theme engine
#[derive(Debug)]
pub struct ThemeEngine {
    registry: ThemeRegistry,
    compiler: StyleCompiler,
    cache: HashMap<String, StyleCacheEntry>,
    cache_size_limit: usize,
}

impl ThemeEngine {
    /// Create a new theme engine
    pub fn new() -> Self {
        Self {
            registry: ThemeRegistry::new(),
            compiler: StyleCompiler::new(),
            cache: HashMap::new(),
            cache_size_limit: 100 * 1024 * 1024, // 100MB
        }
    }

    /// Register a theme
    pub fn register_theme(&mut self, theme: Theme) -> Result<(), ThemeError> {
        self.registry.register_theme(theme)
    }

    /// Set the active theme
    pub fn set_active_theme(&mut self, id: &ThemeId) -> Result<(), ThemeError> {
        self.registry.set_active_theme(id)
    }

    /// Get the active theme
    pub fn get_active_theme(&self) -> Result<&Theme, ThemeError> {
        self.registry.get_active_theme()
    }

    /// Compile the active theme
    pub fn compile_active_theme(&mut self) -> Result<CompiledStyle, ThemeError> {
        let theme = self.registry.get_active_theme()?;
        self.compiler.compile_theme(theme)
    }

    /// Get compiled style from cache
    pub fn get_cached_style(&self, theme_id: &ThemeId) -> Option<&CompiledStyle> {
        self.cache.get(&theme_id.0).map(|entry| &entry.style)
    }

    /// Clear the style cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
        self.compiler.cache.clear();
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> CacheStats {
        let total_size: usize = self.cache.values().map(|entry| entry.size).sum();
        let entry_count = self.cache.len();

        CacheStats {
            entry_count,
            total_size,
            size_limit: self.cache_size_limit,
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub entry_count: usize,
    pub total_size: usize,
    pub size_limit: usize,
}

impl Default for ThemeEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a default theme
pub fn create_default_theme() -> Theme {
    Theme {
        id: ThemeId("default".to_string()),
        name: "Default Theme".to_string(),
        description: Some("Default Helios theme with clean, modern styling".to_string()),
        version: "1.0.0".to_string(),
        variables: ThemeVariables {
            colors: ColorPalette {
                primary: "#3b82f6".to_string(),
                secondary: "#64748b".to_string(),
                accent: "#f59e0b".to_string(),
                background: "#ffffff".to_string(),
                surface: "#f8fafc".to_string(),
                text: "#1e293b".to_string(),
                text_secondary: "#64748b".to_string(),
                border: "#e2e8f0".to_string(),
                error: "#ef4444".to_string(),
                warning: "#f59e0b".to_string(),
                success: "#10b981".to_string(),
                info: "#3b82f6".to_string(),
            },
            typography: TypographySet {
                font_family: "Inter, system-ui, sans-serif".to_string(),
                font_size_base: 14.0,
                font_size_small: 12.0,
                font_size_large: 16.0,
                font_weight_normal: 400,
                font_weight_bold: 600,
                line_height: 1.5,
                letter_spacing: 0.0,
            },
            spacing: SpacingSet {
                xs: 4.0,
                sm: 8.0,
                md: 16.0,
                lg: 24.0,
                xl: 32.0,
                xxl: 48.0,
            },
            shadows: ShadowSet {
                small: "0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string(),
                medium: "0 4px 6px -1px rgb(0 0 0 / 0.1)".to_string(),
                large: "0 10px 15px -3px rgb(0 0 0 / 0.1)".to_string(),
                xl: "0 20px 25px -5px rgb(0 0 0 / 0.1)".to_string(),
            },
            borders: BorderSet {
                width: 1.0,
                radius: 6.0,
                style: "solid".to_string(),
            },
        },
        components: HashMap::new(),
        animations: AnimationSet {
            animations: vec![Animation {
                name: "fadeIn".to_string(),
                duration: 0.3,
                easing: "ease-in-out".to_string(),
                delay: 0.0,
                iteration_count: "1".to_string(),
                direction: "normal".to_string(),
                fill_mode: "forwards".to_string(),
            }],
            transitions: HashMap::new(),
        },
        breakpoints: BreakpointSet {
            breakpoints: vec![
                Breakpoint {
                    name: "mobile".to_string(),
                    min_width: None,
                    max_width: Some(768.0),
                    styles: HashMap::new(),
                },
                Breakpoint {
                    name: "tablet".to_string(),
                    min_width: Some(768.0),
                    max_width: Some(1024.0),
                    styles: HashMap::new(),
                },
                Breakpoint {
                    name: "desktop".to_string(),
                    min_width: Some(1024.0),
                    max_width: None,
                    styles: HashMap::new(),
                },
            ],
            default: HashMap::new(),
        },
        metadata: HashMap::new(),
    }
}
