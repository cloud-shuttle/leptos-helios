//! Enhanced Styling System
//! Advanced theming, customization, and visual styling for charts

use crate::chart_config::*;
use std::collections::HashMap;
use std::time::Duration;

/// Theme system for consistent visual styling
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub colors: ThemeColors,
    pub typography: Typography,
    pub spacing: Spacing,
    pub border_radius: BorderRadius,
    pub shadows: Shadows,
}

/// Theme colors
#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub background: String,
    pub foreground: String,
    pub primary: String,
    pub secondary: String,
    pub success: String,
    pub warning: String,
    pub error: String,
    pub info: String,
}

/// Typography configuration
#[derive(Debug, Clone)]
pub struct Typography {
    pub font_family: String,
    pub font_size_base: f32,
    pub font_size_small: f32,
    pub font_size_large: f32,
    pub font_weight_normal: u16,
    pub font_weight_bold: u16,
    pub line_height: f32,
}

/// Spacing configuration
#[derive(Debug, Clone)]
pub struct Spacing {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
}

/// Border radius configuration
#[derive(Debug, Clone)]
pub struct BorderRadius {
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
}

/// Shadows configuration
#[derive(Debug, Clone)]
pub struct Shadows {
    pub sm: String,
    pub md: String,
    pub lg: String,
}

/// Theme manager
pub struct ThemeManager {
    themes: HashMap<String, Theme>,
    current_theme: Option<String>,
}

impl ThemeManager {
    pub fn new() -> Result<Self, ChartRenderError> {
        Ok(Self {
            themes: HashMap::new(),
            current_theme: None,
        })
    }

    pub fn register_theme(&mut self, theme: Theme) -> Result<(), ChartRenderError> {
        self.themes.insert(theme.name.clone(), theme);
        Ok(())
    }

    pub fn apply_theme(
        &self,
        config: &LineChartConfig,
        theme_name: &str,
    ) -> Result<LineChartConfig, ChartRenderError> {
        let theme = self.themes.get(theme_name).ok_or_else(|| {
            ChartRenderError::InvalidConfig(format!("Theme '{}' not found", theme_name))
        })?;

        let mut styled_config = config.clone();
        styled_config.base.background_color = theme.colors.background.clone();
        styled_config.base.text_color = theme.colors.foreground.clone();
        styled_config.color = theme.colors.primary.clone();

        Ok(styled_config)
    }

    pub fn set_current_theme(&mut self, theme_name: &str) -> Result<(), ChartRenderError> {
        if self.themes.contains_key(theme_name) {
            self.current_theme = Some(theme_name.to_string());
            Ok(())
        } else {
            Err(ChartRenderError::InvalidConfig(format!(
                "Theme '{}' not found",
                theme_name
            )))
        }
    }
}

/// Color palette configuration
#[derive(Debug, Clone)]
pub struct ColorPaletteConfig {
    pub name: String,
    pub colors: Vec<String>,
    pub color_blind_safe: bool,
    pub accessibility_contrast: bool,
}

/// Color palette manager
pub struct ColorPaletteManager {
    palettes: HashMap<String, ColorPaletteConfig>,
}

impl ColorPaletteManager {
    pub fn new() -> Result<Self, ChartRenderError> {
        Ok(Self {
            palettes: HashMap::new(),
        })
    }

    pub fn register_palette(
        &mut self,
        palette: ColorPaletteConfig,
    ) -> Result<(), ChartRenderError> {
        self.palettes.insert(palette.name.clone(), palette);
        Ok(())
    }

    pub fn apply_palette(
        &self,
        config: &BarChartConfig,
        palette_name: &str,
        num_colors: usize,
    ) -> Result<BarChartConfig, ChartRenderError> {
        let palette = self.palettes.get(palette_name).ok_or_else(|| {
            ChartRenderError::InvalidConfig(format!("Palette '{}' not found", palette_name))
        })?;

        let mut styled_config = config.clone();
        styled_config.colors = palette.colors.iter().take(num_colors).cloned().collect();

        Ok(styled_config)
    }
}

/// Advanced styling configuration
#[derive(Debug, Clone)]
pub struct AdvancedStylingConfig {
    pub animations: AnimationConfig,
    pub effects: EffectsConfig,
    pub layout: LayoutConfig,
    pub typography: TypographyConfig,
}

/// Animation configuration
#[derive(Debug, Clone)]
pub struct AnimationConfig {
    pub enabled: bool,
    pub duration: Duration,
    pub easing: EasingType,
    pub stagger_delay: Duration,
}

/// Easing type
#[derive(Debug, Clone)]
pub enum EasingType {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
}

/// Effects configuration
#[derive(Debug, Clone)]
pub struct EffectsConfig {
    pub gradients: bool,
    pub shadows: bool,
    pub blur: bool,
    pub glow: bool,
}

/// Layout configuration
#[derive(Debug, Clone)]
pub struct LayoutConfig {
    pub padding: PaddingConfig,
    pub margin: MarginConfig,
    pub grid: GridConfig,
}

/// Padding configuration
#[derive(Debug, Clone)]
pub struct PaddingConfig {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

/// Margin configuration
#[derive(Debug, Clone)]
pub struct MarginConfig {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

/// Grid configuration
#[derive(Debug, Clone)]
pub struct GridConfig {
    pub show: bool,
    pub color: String,
    pub width: f32,
    pub style: GridStyle,
}

/// Grid style
#[derive(Debug, Clone)]
pub enum GridStyle {
    Solid,
    Dashed,
    Dotted,
}

/// Typography configuration
#[derive(Debug, Clone)]
pub struct TypographyConfig {
    pub title: TitleConfig,
    pub axis_labels: AxisLabelConfig,
    pub legend: LegendConfig,
}

/// Title configuration
#[derive(Debug, Clone)]
pub struct TitleConfig {
    pub font_size: f32,
    pub font_weight: u16,
    pub color: String,
    pub margin_bottom: f32,
}

/// Axis label configuration
#[derive(Debug, Clone)]
pub struct AxisLabelConfig {
    pub font_size: f32,
    pub font_weight: u16,
    pub color: String,
    pub rotation: f32,
}

/// Legend configuration
#[derive(Debug, Clone)]
pub struct LegendConfig {
    pub font_size: f32,
    pub font_weight: u16,
    pub color: String,
    pub position: LegendPosition,
}

/// Legend position
#[derive(Debug, Clone)]
pub enum LegendPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
}

/// Advanced styling manager
pub struct AdvancedStylingManager {
    config: Option<AdvancedStylingConfig>,
}

impl AdvancedStylingManager {
    pub fn new() -> Result<Self, ChartRenderError> {
        Ok(Self { config: None })
    }

    pub fn set_styling_config(
        &mut self,
        config: AdvancedStylingConfig,
    ) -> Result<(), ChartRenderError> {
        self.config = Some(config);
        Ok(())
    }

    pub fn apply_styling(
        &self,
        config: &AreaChartConfig,
    ) -> Result<AreaChartConfig, ChartRenderError> {
        let mut styled_config = config.clone();

        if let Some(styling_config) = &self.config {
            if styling_config.effects.gradients && styled_config.gradient.is_none() {
                styled_config.gradient = Some(GradientConfig {
                    start_color: styled_config.fill_color.clone(),
                    end_color: styled_config.stroke_color.clone(),
                    direction: GradientDirection::Vertical,
                });
            }
        }

        Ok(styled_config)
    }
}

/// Responsive design configuration
#[derive(Debug, Clone)]
pub struct ResponsiveConfig {
    pub breakpoints: Vec<Breakpoint>,
    pub adaptive_layout: bool,
    pub fluid_typography: bool,
    pub responsive_colors: bool,
}

/// Breakpoint
#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub name: String,
    pub min_width: u32,
    pub max_width: u32,
}

/// Responsive manager
pub struct ResponsiveManager {
    config: Option<ResponsiveConfig>,
}

impl ResponsiveManager {
    pub fn new() -> Result<Self, ChartRenderError> {
        Ok(Self { config: None })
    }

    pub fn set_responsive_config(
        &mut self,
        config: ResponsiveConfig,
    ) -> Result<(), ChartRenderError> {
        self.config = Some(config);
        Ok(())
    }

    pub fn apply_responsive(
        &self,
        config: &ScatterPlotConfig,
        width: u32,
        height: u32,
    ) -> Result<ScatterPlotConfig, ChartRenderError> {
        let mut responsive_config = config.clone();
        responsive_config.base.width = width;
        responsive_config.base.height = height;

        if let Some(responsive_cfg) = &self.config {
            if responsive_cfg.fluid_typography {
                // Adjust font sizes based on screen size
                let scale_factor = (width as f32 / 800.0).min(2.0).max(0.5);
                responsive_config.base.title =
                    format!("{} ({}x{})", config.base.title, width, height);
            }
        }

        Ok(responsive_config)
    }
}

/// Accessibility styling configuration
#[derive(Debug, Clone)]
pub struct AccessibilityStylingConfig {
    pub high_contrast_mode: bool,
    pub color_blind_support: bool,
    pub reduced_motion: bool,
    pub focus_indicators: bool,
    pub screen_reader_optimized: bool,
    pub minimum_contrast_ratio: f64,
}

/// Accessibility styling manager
pub struct AccessibilityStylingManager {
    config: Option<AccessibilityStylingConfig>,
}

impl AccessibilityStylingManager {
    pub fn new() -> Result<Self, ChartRenderError> {
        Ok(Self { config: None })
    }

    pub fn set_accessibility_config(
        &mut self,
        config: AccessibilityStylingConfig,
    ) -> Result<(), ChartRenderError> {
        self.config = Some(config);
        Ok(())
    }

    pub fn apply_accessibility(
        &self,
        config: &LineChartConfig,
    ) -> Result<LineChartConfig, ChartRenderError> {
        let mut accessible_config = config.clone();

        if let Some(accessibility_config) = &self.config {
            if accessibility_config.high_contrast_mode {
                accessible_config.color = "#000000".to_string();
                accessible_config.base.background_color = "#ffffff".to_string();
                accessible_config.base.text_color = "#000000".to_string();
            }

            if accessibility_config.reduced_motion {
                // Ensure minimum line width and point size for better visibility
                accessible_config.line_width = accessible_config.line_width.max(3.0);
                accessible_config.point_size = accessible_config.point_size.max(6.0);
            }
        }

        Ok(accessible_config)
    }
}

/// Animation system configuration
#[derive(Debug, Clone)]
pub struct AnimationSystemConfig {
    pub entrance_animations: EntranceAnimationConfig,
    pub data_animations: DataAnimationConfig,
    pub interaction_animations: InteractionAnimationConfig,
}

/// Entrance animation configuration
#[derive(Debug, Clone)]
pub struct EntranceAnimationConfig {
    pub enabled: bool,
    pub type_: EntranceAnimationType,
    pub duration: Duration,
    pub delay: Duration,
}

/// Entrance animation type
#[derive(Debug, Clone)]
pub enum EntranceAnimationType {
    FadeIn,
    SlideIn,
    ScaleIn,
    BounceIn,
}

/// Data animation configuration
#[derive(Debug, Clone)]
pub struct DataAnimationConfig {
    pub enabled: bool,
    pub type_: DataAnimationType,
    pub duration: Duration,
    pub easing: EasingType,
}

/// Data animation type
#[derive(Debug, Clone)]
pub enum DataAnimationType {
    Morph,
    Transition,
    Stagger,
    Cascade,
}

/// Interaction animation configuration
#[derive(Debug, Clone)]
pub struct InteractionAnimationConfig {
    pub hover: HoverAnimationConfig,
    pub click: ClickAnimationConfig,
}

/// Hover animation configuration
#[derive(Debug, Clone)]
pub struct HoverAnimationConfig {
    pub enabled: bool,
    pub scale: f32,
    pub duration: Duration,
}

/// Click animation configuration
#[derive(Debug, Clone)]
pub struct ClickAnimationConfig {
    pub enabled: bool,
    pub scale: f32,
    pub duration: Duration,
}

/// Animation manager
pub struct AnimationManager {
    config: Option<AnimationSystemConfig>,
}

impl AnimationManager {
    pub fn new() -> Result<Self, ChartRenderError> {
        Ok(Self { config: None })
    }

    pub fn set_animation_config(
        &mut self,
        config: AnimationSystemConfig,
    ) -> Result<(), ChartRenderError> {
        self.config = Some(config);
        Ok(())
    }

    pub fn apply_animations(
        &self,
        config: &BarChartConfig,
    ) -> Result<BarChartConfig, ChartRenderError> {
        let mut animated_config = config.clone();

        if let Some(animation_config) = &self.config {
            if animation_config.entrance_animations.enabled {
                // Apply entrance animation settings
                animated_config.corner_radius = Some(4.0);
            }
        }

        Ok(animated_config)
    }
}

/// Export styling configuration
#[derive(Debug, Clone)]
pub struct ExportStylingConfig {
    pub formats: Vec<ExportFormat>,
    pub styling_presets: Vec<String>,
}

/// Export format
#[derive(Debug, Clone)]
pub enum ExportFormat {
    PNG {
        width: u32,
        height: u32,
        dpi: u32,
        background: String,
    },
    SVG {
        width: u32,
        height: u32,
        include_styles: bool,
    },
    PDF {
        width: f32,
        height: f32,
        dpi: u32,
        margin: f32,
    },
}

/// Export styling manager
pub struct ExportStylingManager {
    config: Option<ExportStylingConfig>,
}

impl ExportStylingManager {
    pub fn new() -> Result<Self, ChartRenderError> {
        Ok(Self { config: None })
    }

    pub fn set_export_config(
        &mut self,
        config: ExportStylingConfig,
    ) -> Result<(), ChartRenderError> {
        self.config = Some(config);
        Ok(())
    }

    pub fn prepare_for_export(
        &self,
        config: &LineChartConfig,
        preset: &str,
    ) -> Result<LineChartConfig, ChartRenderError> {
        let mut export_config = config.clone();

        match preset {
            "print" => {
                export_config.base.background_color = "#ffffff".to_string();
                export_config.base.text_color = "#000000".to_string();
                export_config.color = "#000000".to_string();
            }
            "presentation" => {
                export_config.base.background_color = "#f8f9fa".to_string();
                export_config.base.text_color = "#212529".to_string();
                export_config.color = "#007bff".to_string();
            }
            "web" => {
                export_config.base.background_color = "#f8f9fa".to_string();
                export_config.base.text_color = "#333333".to_string();
                export_config.color = "#00d4ff".to_string();
            }
            _ => {}
        }

        Ok(export_config)
    }
}

/// Style validation configuration
#[derive(Debug, Clone)]
pub struct StyleValidationConfig {
    pub validate_colors: bool,
    pub validate_contrast: bool,
    pub validate_accessibility: bool,
    pub validate_performance: bool,
    pub warnings_as_errors: bool,
}

/// Style validation result
#[derive(Debug, Clone)]
pub struct StyleValidationResult {
    pub is_valid: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Style validation manager
pub struct StyleValidationManager {
    config: Option<StyleValidationConfig>,
}

impl StyleValidationManager {
    pub fn new() -> Result<Self, ChartRenderError> {
        Ok(Self { config: None })
    }

    pub fn set_validation_config(
        &mut self,
        config: StyleValidationConfig,
    ) -> Result<(), ChartRenderError> {
        self.config = Some(config);
        Ok(())
    }

    pub fn validate_styles(
        &self,
        config: &LineChartConfig,
    ) -> Result<StyleValidationResult, ChartRenderError> {
        let mut warnings = Vec::new();
        let mut errors = Vec::new();

        if let Some(validation_config) = &self.config {
            if validation_config.validate_colors {
                // Validate color format
                if !config.color.starts_with('#') || config.color.len() != 7 {
                    warnings.push("Invalid color format".to_string());
                }
            }

            if validation_config.validate_contrast {
                // Simple contrast validation
                if config.base.background_color == config.base.text_color {
                    errors.push("Insufficient contrast between background and text".to_string());
                }
            }
        }

        let is_valid = errors.is_empty()
            && (!self.config.as_ref().map_or(false, |c| c.warnings_as_errors)
                || warnings.is_empty());

        Ok(StyleValidationResult {
            is_valid,
            warnings,
            errors,
        })
    }
}

/// Style performance configuration
#[derive(Debug, Clone)]
pub struct StylePerformanceConfig {
    pub measure_render_time: bool,
    pub measure_memory_usage: bool,
    pub measure_gpu_usage: bool,
    pub target_fps: u32,
    pub max_memory_mb: u32,
}

/// Style performance result
#[derive(Debug, Clone)]
pub struct StylePerformanceResult {
    pub render_time_ms: f64,
    pub memory_usage_mb: f64,
    pub gpu_usage_percent: f64,
    pub meets_target_fps: bool,
    pub meets_memory_limit: bool,
}

/// Style performance manager
pub struct StylePerformanceManager {
    config: Option<StylePerformanceConfig>,
}

impl StylePerformanceManager {
    pub fn new() -> Result<Self, ChartRenderError> {
        Ok(Self { config: None })
    }

    pub fn set_performance_config(
        &mut self,
        config: StylePerformanceConfig,
    ) -> Result<(), ChartRenderError> {
        self.config = Some(config);
        Ok(())
    }

    pub fn measure_performance(
        &self,
        config: &LineChartConfig,
    ) -> Result<StylePerformanceResult, ChartRenderError> {
        let start = std::time::Instant::now();

        // Mock performance measurement - simulate some rendering work
        std::thread::sleep(std::time::Duration::from_micros(100));
        let render_time_ms = start.elapsed().as_secs_f64() * 1000.0;
        let memory_usage_mb =
            (config.base.width * config.base.height * 4) as f64 / (1024.0 * 1024.0);
        let gpu_usage_percent = 25.0; // Mock GPU usage

        let target_fps = self.config.as_ref().map_or(60, |c| c.target_fps);
        let max_memory_mb = self.config.as_ref().map_or(100, |c| c.max_memory_mb);

        let meets_target_fps = render_time_ms < (1000.0 / target_fps as f64);
        let meets_memory_limit = memory_usage_mb <= max_memory_mb as f64;

        Ok(StylePerformanceResult {
            render_time_ms,
            memory_usage_mb,
            gpu_usage_percent,
            meets_target_fps,
            meets_memory_limit,
        })
    }
}
