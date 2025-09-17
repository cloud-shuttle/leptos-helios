//! Chart Types and Enums
//!
//! This module defines the core types and enums used in chart specifications.

use serde::{Deserialize, Serialize};

/// Data reference for chart data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataReference {
    pub source: String,
    pub format: DataFormat,
    pub schema: Option<DataSchema>,
}

/// Data format types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataFormat {
    JSON,
    CSV,
    Parquet,
    Arrow,
    Inline,
}

/// Data schema definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSchema {
    pub fields: Vec<FieldDef>,
    pub primary_key: Option<String>,
}

/// Field definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldDef {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
    pub description: Option<String>,
}

/// Data types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    String,
    Number,
    Integer,
    Boolean,
    Date,
    DateTime,
    Time,
    // Additional types for compatibility
    Quantitative,
    Nominal,
    Ordinal,
    Temporal,
}

/// Chart encoding specification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Encoding {
    pub x: Option<EncodingDef>,
    pub y: Option<EncodingDef>,
    pub color: Option<EncodingDef>,
    pub size: Option<EncodingDef>,
    pub shape: Option<EncodingDef>,
    pub opacity: Option<EncodingDef>,
    pub text: Option<EncodingDef>,
    pub tooltip: Option<EncodingDef>,
    pub detail: Option<EncodingDef>,
    pub order: Option<EncodingDef>,
    pub row: Option<EncodingDef>,
    pub column: Option<EncodingDef>,
}

/// Encoding definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncodingDef {
    pub field: String,
    pub data_type: DataType,
    pub scale: Option<Scale>,
    pub axis: Option<Axis>,
    pub legend: Option<Legend>,
    pub bin: Option<Bin>,
    pub aggregate: Option<Aggregate>,
    pub sort: Option<Sort>,
}

/// Position encoding for x and y axes (alias for EncodingDef)
pub type PositionEncoding = EncodingDef;

/// Color encoding (alias for EncodingDef)
pub type ColorEncoding = EncodingDef;

/// Size encoding (alias for EncodingDef)
pub type SizeEncoding = EncodingDef;

/// Scale configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scale {
    pub domain: Option<ScaleDomain>,
    pub range: Option<ScaleRange>,
    pub type_: Option<ScaleType>,
    pub zero: Option<bool>,
    pub nice: Option<bool>,
    pub padding: Option<f32>,
}

/// Scale domain
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScaleDomain {
    Data,
    Independent,
    Values(Vec<serde_json::Value>),
}

/// Scale range
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScaleRange {
    Values(Vec<serde_json::Value>),
    Scheme(String),
}

/// Scale types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScaleType {
    Linear,
    Log,
    Pow,
    Sqrt,
    Symlog,
    Time,
    Utc,
    Ordinal,
    Band,
    Point,
}

/// Axis configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Axis {
    pub title: Option<String>,
    pub labels: Option<bool>,
    pub grid: Option<bool>,
    pub ticks: Option<bool>,
    pub domain: Option<bool>,
    pub orient: Option<AxisOrient>,
    pub format: Option<String>,
    pub angle: Option<f32>,
}

/// Axis orientation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AxisOrient {
    Top,
    Bottom,
    Left,
    Right,
}

/// Legend configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Legend {
    pub title: Option<String>,
    pub orient: Option<LegendOrient>,
    pub offset: Option<f32>,
    pub padding: Option<f32>,
    pub margin: Option<f32>,
    pub gradient_length: Option<f32>,
    pub gradient_thickness: Option<f32>,
    pub gradient_stroke_color: Option<String>,
    pub gradient_stroke_width: Option<f32>,
    pub label_color: Option<String>,
    pub label_font: Option<String>,
    pub label_font_size: Option<f32>,
    pub label_limit: Option<f32>,
    pub symbol_color: Option<String>,
    pub symbol_size: Option<f32>,
    pub symbol_stroke_width: Option<f32>,
    pub symbol_type: Option<String>,
    pub title_color: Option<String>,
    pub title_font: Option<String>,
    pub title_font_size: Option<f32>,
    pub title_font_weight: Option<String>,
    pub title_limit: Option<f32>,
    pub title_padding: Option<f32>,
}

/// Legend orientation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LegendOrient {
    Left,
    Right,
    Top,
    Bottom,
    None,
}

/// Bin configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bin {
    pub maxbins: Option<u32>,
    pub step: Option<f32>,
    pub steps: Option<Vec<f32>>,
    pub minstep: Option<f32>,
    pub divide: Option<Vec<f32>>,
    pub extent: Option<Vec<f32>>,
    pub nice: Option<bool>,
}

/// Aggregate operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Aggregate {
    Count,
    Valid,
    Missing,
    Distinct,
    Sum,
    Mean,
    Average,
    Variance,
    Variancep,
    Stdev,
    Stdevp,
    Median,
    Q1,
    Q3,
    Modeskew,
    Min,
    Max,
    Argmin,
    Argmax,
    Values,
    Uniq,
    UniqBy,
}

/// Sort configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sort {
    pub field: Option<String>,
    pub order: Option<SortOrder>,
}

/// Sort order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

/// Chart configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChartConfig {
    pub title: String,
    pub description: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub padding: Option<Padding>,
    pub background: Option<String>,
    pub viewport: Option<Viewport>,
    pub legend: Option<Legend>,
    pub axis: Option<AxisConfig>,
    pub mark: Option<MarkConfig>,
    pub selection: Option<SelectionConfig>,
    pub scale: Option<ScaleConfig>,
    pub range: Option<RangeConfig>,
    pub facet: Option<FacetConfig>,
    pub header: Option<HeaderConfig>,
    pub overlay: Option<OverlayConfig>,
    pub style: Option<StyleConfig>,
    pub signals: Option<Vec<SignalConfig>>,
    pub data: Option<Vec<DataConfig>>,
    pub layout: Option<LayoutConfig>,
    pub projection: Option<ProjectionConfig>,
    pub encoding: Option<EncodingConfig>,
    pub resolve: Option<ResolveConfig>,
    pub autosize: Option<AutosizeConfig>,
    pub usermeta: Option<serde_json::Value>,
}

/// Padding configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Padding {
    pub top: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
    pub right: Option<f32>,
}

/// Viewport configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Viewport {
    pub stroke: Option<String>,
    pub stroke_width: Option<f32>,
    pub fill: Option<String>,
    pub fill_opacity: Option<f32>,
}

/// Axis configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AxisConfig {
    pub band_position: Option<f32>,
    pub domain: Option<bool>,
    pub domain_color: Option<String>,
    pub domain_opacity: Option<f32>,
    pub domain_width: Option<f32>,
    pub grid: Option<bool>,
    pub grid_color: Option<String>,
    pub grid_dash: Option<Vec<f32>>,
    pub grid_opacity: Option<f32>,
    pub grid_width: Option<f32>,
    pub label_angle: Option<f32>,
    pub label_color: Option<String>,
    pub label_flush: Option<bool>,
    pub label_flush_offset: Option<f32>,
    pub label_font: Option<String>,
    pub label_font_size: Option<f32>,
    pub label_font_style: Option<String>,
    pub label_font_weight: Option<String>,
    pub label_limit: Option<f32>,
    pub label_padding: Option<f32>,
    pub label_separation: Option<f32>,
    pub max_extent: Option<f32>,
    pub min_extent: Option<f32>,
    pub short_time_labels: Option<bool>,
    pub tick_color: Option<String>,
    pub tick_dash: Option<Vec<f32>>,
    pub tick_opacity: Option<f32>,
    pub tick_size: Option<f32>,
    pub tick_width: Option<f32>,
    pub ticks: Option<bool>,
    pub title_align: Option<String>,
    pub title_anchor: Option<String>,
    pub title_angle: Option<f32>,
    pub title_baseline: Option<String>,
    pub title_color: Option<String>,
    pub title_font: Option<String>,
    pub title_font_size: Option<f32>,
    pub title_font_style: Option<String>,
    pub title_font_weight: Option<String>,
    pub title_limit: Option<f32>,
    pub title_padding: Option<f32>,
    pub title_x: Option<f32>,
    pub title_y: Option<f32>,
}

/// Mark configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkConfig {
    pub color: Option<String>,
    pub fill: Option<String>,
    pub fill_opacity: Option<f32>,
    pub stroke: Option<String>,
    pub stroke_cap: Option<String>,
    pub stroke_dash: Option<Vec<f32>>,
    pub stroke_dash_offset: Option<f32>,
    pub stroke_join: Option<String>,
    pub stroke_miter_limit: Option<f32>,
    pub stroke_opacity: Option<f32>,
    pub stroke_width: Option<f32>,
    pub opacity: Option<f32>,
    pub blend: Option<String>,
    pub fill_rule: Option<String>,
    pub stroke_foreground: Option<bool>,
    pub stroke_offset: Option<f32>,
    pub aria: Option<bool>,
    pub description: Option<String>,
    pub cursor: Option<String>,
    pub href: Option<String>,
    pub tooltip: Option<serde_json::Value>,
    pub zindex: Option<f32>,
}

/// Selection configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectionConfig {
    pub interval: Option<IntervalConfig>,
    pub single: Option<SingleConfig>,
    pub multi: Option<MultiConfig>,
}

/// Interval selection configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntervalConfig {
    pub bind: Option<String>,
    pub clear: Option<String>,
    pub empty: Option<String>,
    pub encodings: Option<Vec<String>>,
    pub fields: Option<Vec<String>>,
    pub init: Option<serde_json::Value>,
    pub mark: Option<MarkConfig>,
    pub on: Option<String>,
    pub resolve: Option<String>,
    pub translate: Option<String>,
    pub zoom: Option<String>,
}

/// Single selection configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleConfig {
    pub bind: Option<String>,
    pub clear: Option<String>,
    pub empty: Option<String>,
    pub encodings: Option<Vec<String>>,
    pub fields: Option<Vec<String>>,
    pub init: Option<serde_json::Value>,
    pub on: Option<String>,
    pub resolve: Option<String>,
    pub toggle: Option<String>,
    pub nearest: Option<bool>,
}

/// Multi selection configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiConfig {
    pub bind: Option<String>,
    pub clear: Option<String>,
    pub empty: Option<String>,
    pub encodings: Option<Vec<String>>,
    pub fields: Option<Vec<String>>,
    pub init: Option<serde_json::Value>,
    pub on: Option<String>,
    pub resolve: Option<String>,
    pub toggle: Option<String>,
    pub nearest: Option<bool>,
}

/// Scale configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScaleConfig {
    pub band_padding_inner: Option<f32>,
    pub band_padding_outer: Option<f32>,
    pub bar_size: Option<f32>,
    pub continuous_padding: Option<f32>,
    pub max_font_size: Option<f32>,
    pub max_opacity: Option<f32>,
    pub max_size: Option<f32>,
    pub max_stroke_width: Option<f32>,
    pub min_font_size: Option<f32>,
    pub min_opacity: Option<f32>,
    pub min_size: Option<f32>,
    pub min_stroke_width: Option<f32>,
    pub point_padding: Option<f32>,
    pub range_step: Option<f32>,
    pub round: Option<bool>,
    pub text_x_range_step: Option<f32>,
    pub use_unaggregated_domain: Option<bool>,
}

/// Range configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RangeConfig {
    pub category: Option<Vec<String>>,
    pub diverging: Option<Vec<String>>,
    pub heatmap: Option<Vec<String>>,
    pub ordinal: Option<Vec<String>>,
    pub ramp: Option<Vec<String>>,
    pub symbol: Option<Vec<String>>,
}

/// Facet configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FacetConfig {
    pub spacing: Option<f32>,
}

/// Header configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeaderConfig {
    pub label_angle: Option<f32>,
    pub label_color: Option<String>,
    pub label_font: Option<String>,
    pub label_font_size: Option<f32>,
    pub label_font_style: Option<String>,
    pub label_font_weight: Option<String>,
    pub label_limit: Option<f32>,
    pub label_padding: Option<f32>,
    pub title_align: Option<String>,
    pub title_anchor: Option<String>,
    pub title_angle: Option<f32>,
    pub title_baseline: Option<String>,
    pub title_color: Option<String>,
    pub title_font: Option<String>,
    pub title_font_size: Option<f32>,
    pub title_font_style: Option<String>,
    pub title_font_weight: Option<String>,
    pub title_limit: Option<f32>,
    pub title_padding: Option<f32>,
}

/// Overlay configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverlayConfig {
    pub line: Option<bool>,
    pub point_style_fill_opacity: Option<f32>,
    pub point_style_stroke: Option<String>,
    pub point_style_stroke_width: Option<f32>,
    pub point_style_fill: Option<String>,
    pub point_style_stroke_opacity: Option<f32>,
    pub point_style_stroke_dash: Option<Vec<f32>>,
    pub point_style_stroke_dash_offset: Option<f32>,
    pub point_style_stroke_cap: Option<String>,
    pub point_style_stroke_join: Option<String>,
    pub point_style_stroke_miter_limit: Option<f32>,
    pub point_style_stroke_foreground: Option<bool>,
    pub point_style_stroke_offset: Option<f32>,
    pub point_style_opacity: Option<f32>,
    pub point_style_blend: Option<String>,
    pub point_style_fill_rule: Option<String>,
    pub point_style_aria: Option<bool>,
    pub point_style_description: Option<String>,
    pub point_style_cursor: Option<String>,
    pub point_style_href: Option<String>,
    pub point_style_tooltip: Option<serde_json::Value>,
    pub point_style_zindex: Option<f32>,
}

/// Style configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleConfig {
    pub group: Option<serde_json::Value>,
    pub cell: Option<serde_json::Value>,
    pub facet: Option<serde_json::Value>,
}

/// Signal configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignalConfig {
    pub name: String,
    pub value: Option<serde_json::Value>,
    pub bind: Option<serde_json::Value>,
    pub on: Option<Vec<serde_json::Value>>,
    pub update: Option<String>,
    pub init: Option<String>,
    pub react: Option<bool>,
    pub force: Option<bool>,
}

/// Data configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataConfig {
    pub name: String,
    pub source: Option<String>,
    pub url: Option<String>,
    pub format: Option<serde_json::Value>,
    pub values: Option<serde_json::Value>,
    pub transform: Option<Vec<serde_json::Value>>,
}

/// Layout configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutConfig {
    pub padding: Option<f32>,
    pub columns: Option<u32>,
    pub bounds: Option<String>,
    pub spacing: Option<f32>,
    pub header_row: Option<f32>,
    pub header_column: Option<f32>,
    pub header_facet: Option<f32>,
    pub footer_row: Option<f32>,
    pub footer_column: Option<f32>,
    pub footer_facet: Option<f32>,
    pub title_row: Option<f32>,
    pub title_column: Option<f32>,
    pub title_facet: Option<f32>,
    pub label_row: Option<f32>,
    pub label_column: Option<f32>,
    pub label_facet: Option<f32>,
    pub center_row: Option<bool>,
    pub center_column: Option<bool>,
    pub center_facet: Option<bool>,
    pub align: Option<String>,
    pub columns_align: Option<String>,
    pub rows_align: Option<String>,
    pub facet_align: Option<String>,
    pub columns_columns: Option<u32>,
    pub rows_columns: Option<u32>,
    pub facet_columns: Option<u32>,
    pub columns_rows: Option<u32>,
    pub rows_rows: Option<u32>,
    pub facet_rows: Option<u32>,
    pub columns_spacing: Option<f32>,
    pub rows_spacing: Option<f32>,
    pub facet_spacing: Option<f32>,
    pub columns_padding: Option<f32>,
    pub rows_padding: Option<f32>,
    pub facet_padding: Option<f32>,
    pub columns_bounds: Option<String>,
    pub rows_bounds: Option<String>,
    pub facet_bounds: Option<String>,
    pub columns_center: Option<bool>,
    pub rows_center: Option<bool>,
    pub facet_center: Option<bool>,
    pub columns_align_all: Option<String>,
    pub rows_align_all: Option<String>,
    pub facet_align_all: Option<String>,
    pub columns_columns_all: Option<u32>,
    pub rows_columns_all: Option<u32>,
    pub facet_columns_all: Option<u32>,
    pub columns_rows_all: Option<u32>,
    pub rows_rows_all: Option<u32>,
    pub facet_rows_all: Option<u32>,
    pub columns_spacing_all: Option<f32>,
    pub rows_spacing_all: Option<f32>,
    pub facet_spacing_all: Option<f32>,
    pub columns_padding_all: Option<f32>,
    pub rows_padding_all: Option<f32>,
    pub facet_padding_all: Option<f32>,
    pub columns_bounds_all: Option<String>,
    pub rows_bounds_all: Option<String>,
    pub facet_bounds_all: Option<String>,
    pub columns_center_all: Option<bool>,
    pub rows_center_all: Option<bool>,
    pub facet_center_all: Option<bool>,
}

/// Projection configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectionConfig {
    pub type_: Option<String>,
    pub clip_angle: Option<f32>,
    pub clip_extent: Option<Vec<Vec<f32>>>,
    pub center: Option<Vec<f32>>,
    pub rotate: Option<Vec<f32>>,
    pub precision: Option<f32>,
    pub scale: Option<f32>,
    pub translate: Option<Vec<f32>>,
    pub coefficient: Option<f32>,
    pub distance: Option<f32>,
    pub fraction: Option<f32>,
    pub lobes: Option<f32>,
    pub parallel: Option<f32>,
    pub radius: Option<f32>,
    pub ratio: Option<f32>,
    pub spacing: Option<f32>,
    pub tilt: Option<f32>,
}

/// Encoding configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncodingConfig {
    pub x: Option<EncodingDef>,
    pub y: Option<EncodingDef>,
    pub color: Option<EncodingDef>,
    pub size: Option<EncodingDef>,
    pub shape: Option<EncodingDef>,
    pub opacity: Option<EncodingDef>,
    pub text: Option<EncodingDef>,
    pub tooltip: Option<EncodingDef>,
    pub detail: Option<EncodingDef>,
    pub order: Option<EncodingDef>,
    pub row: Option<EncodingDef>,
    pub column: Option<EncodingDef>,
}

/// Resolve configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolveConfig {
    pub axis: Option<serde_json::Value>,
    pub legend: Option<serde_json::Value>,
    pub scale: Option<serde_json::Value>,
}

/// Autosize configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutosizeConfig {
    pub type_: Option<String>,
    pub contains: Option<String>,
    pub resize: Option<bool>,
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            width: None,
            height: None,
            padding: None,
            background: None,
            viewport: None,
            legend: None,
            axis: None,
            mark: None,
            selection: None,
            scale: None,
            range: None,
            facet: None,
            header: None,
            overlay: None,
            style: None,
            signals: None,
            data: None,
            layout: None,
            projection: None,
            encoding: None,
            resolve: None,
            autosize: None,
            usermeta: None,
        }
    }
}

/// Mark types for charts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MarkType {
    /// Point marks for scatter plots
    Point {
        size: Option<f32>,
        shape: Option<PointShape>,
        opacity: Option<f32>,
    },
    /// Line marks for line charts
    Line {
        interpolate: Option<Interpolation>,
        stroke_width: Option<f32>,
        stroke_dash: Option<Vec<f32>>,
    },
    /// Bar marks for bar charts
    Bar {
        width: Option<BarWidth>,
        corner_radius: Option<f32>,
    },
    /// Area marks for area charts
    Area {
        interpolate: Option<Interpolation>,
        opacity: Option<f32>,
    },
    /// Text marks for annotations
    Text {
        font_size: Option<f32>,
        font_family: Option<String>,
        align: Option<TextAlign>,
    },
    /// Rectangle marks for heatmaps
    Rect {
        stroke: Option<crate::Color>,
        stroke_width: Option<f32>,
    },
    /// Scatter plot with enhanced features
    Scatter {
        size: Option<f32>,
        shape: Option<PointShape>,
        opacity: Option<f32>,
        jitter: Option<f32>,
        trend_line: Option<bool>,
    },
    /// Box plot for statistical visualization
    BoxPlot {
        width: Option<f32>,
        outlier_detection: Option<bool>,
        whisker_type: Option<WhiskerType>,
    },
    /// Violin plot for distribution visualization
    Violin {
        width: Option<f32>,
        bandwidth: Option<f32>,
        kernel: Option<KernelType>,
    },
    /// Heatmap with color mapping
    Heatmap {
        color_scheme: Option<String>,
        interpolation: Option<Interpolation>,
        stroke: Option<crate::Color>,
    },
    /// Histogram for distribution analysis
    Histogram {
        bin_width: Option<f32>,
        bin_count: Option<u32>,
        density: Option<bool>,
    },
    /// Density plot for smooth distributions
    Density {
        bandwidth: Option<f32>,
        kernel: Option<KernelType>,
        fill: Option<bool>,
    },
    /// Contour plot for 2D density
    Contour {
        levels: Option<u32>,
        color_scheme: Option<String>,
        stroke_width: Option<f32>,
    },
    /// Radar chart for multivariate data
    Radar {
        radius: Option<f32>,
        stroke_width: Option<f32>,
        fill_opacity: Option<f32>,
    },
    /// Sankey diagram for flow visualization
    Sankey {
        node_width: Option<f32>,
        node_padding: Option<f32>,
        link_opacity: Option<f32>,
    },
    /// Treemap for hierarchical data
    Treemap {
        padding: Option<f32>,
        round: Option<bool>,
        method: Option<TreemapMethod>,
    },
    /// Composite chart with multiple marks
    Composite(Vec<MarkType>),
    /// 3D Point marks
    Point3D {
        size: Option<f32>,
        shape: Option<PointShape>,
        opacity: Option<f32>,
    },
    /// 3D Surface
    Surface3D {
        wireframe: Option<bool>,
        opacity: Option<f32>,
    },
    /// Choropleth map
    Choropleth {
        projection: Option<String>,
        stroke: Option<crate::Color>,
        stroke_width: Option<f32>,
    },
    /// Network graph
    NetworkGraph {
        layout: Option<String>,
        node_size: Option<f32>,
        edge_width: Option<f32>,
    },
    /// Dot map
    DotMap {
        projection: Option<String>,
        dot_size: Option<f32>,
        dot_opacity: Option<f32>,
    },
    /// Flow map
    FlowMap {
        projection: Option<String>,
        flow_width: Option<f32>,
        flow_opacity: Option<f32>,
    },
}

/// Point shapes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PointShape {
    Circle,
    Square,
    Diamond,
    Triangle,
    Cross,
    Plus,
    Star,
}

/// Interpolation types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Interpolation {
    Linear,
    Step,
    StepBefore,
    StepAfter,
    Basis,
    BasisOpen,
    BasisClosed,
    Bundle,
    Cardinal,
    CardinalOpen,
    CardinalClosed,
    Monotone,
    CatmullRom,
    CatmullRomOpen,
    CatmullRomClosed,
}

/// Bar width specification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BarWidth {
    Band,
    Point,
    Value(f32),
}

/// Text alignment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

/// Whisker types for box plots
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WhiskerType {
    IQR,
    MinMax,
    StdDev,
}

/// Kernel types for density plots
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KernelType {
    Gaussian,
    Epanechnikov,
    Uniform,
    Triangular,
    Biweight,
    Triweight,
    Cosine,
}

/// Treemap methods
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TreemapMethod {
    Squarify,
    Resquarify,
    Binary,
    Dice,
    Slice,
    SliceDice,
}

/// Transform operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub operation: TransformOperation,
    pub parameters: std::collections::HashMap<String, serde_json::Value>,
}

/// Transform operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransformOperation {
    Filter,
    Aggregate,
    Bin,
    Calculate,
    Density,
    Extent,
    Flatten,
    Fold,
    Impute,
    JoinAggregate,
    Loess,
    Lookup,
    Pivot,
    Quantile,
    Regression,
    Sample,
    Sequence,
    Stack,
    TimeUnit,
    Window,
}

/// Selection definitions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Selection {
    pub name: String,
    pub selection_type: SelectionType,
    pub parameters: std::collections::HashMap<String, serde_json::Value>,
}

/// Selection types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SelectionType {
    Single,
    Multi,
    Interval,
}

/// Intelligence features
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Intelligence {
    pub features: Vec<IntelligenceFeature>,
    pub parameters: std::collections::HashMap<String, serde_json::Value>,
    pub forecast: Option<ForecastConfig>,
    pub anomaly_detection: Option<AnomalyConfig>,
    pub trend_analysis: Option<bool>,
    pub clustering: Option<ClusterConfig>,
}

/// Forecast configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForecastConfig {
    pub enabled: bool,
    pub horizon: Option<u32>,
    pub confidence_interval: Option<f64>,
}

/// Anomaly detection configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnomalyConfig {
    pub enabled: bool,
    pub threshold: Option<f64>,
    pub method: Option<String>,
}

/// Clustering configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterConfig {
    pub enabled: bool,
    pub algorithm: Option<String>,
    pub num_clusters: Option<u32>,
}

/// Intelligence features
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IntelligenceFeature {
    AnomalyDetection,
    TrendAnalysis,
    Clustering,
    Classification,
    Regression,
    Forecasting,
    OutlierDetection,
    PatternRecognition,
}
