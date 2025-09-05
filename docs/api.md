# Helios API Reference

> Complete API documentation for the Helios visualization library

## Core Components

### HeliosChart

The primary visualization component that integrates with Leptos's reactive system.

```rust
#[component]
pub fn HeliosChart(
    /// Chart specification - can be static or reactive
    #[prop(into)] spec: MaybeSignal<ChartSpec>,

    /// Canvas width in pixels (optional, defaults to 800)
    #[prop(optional)] width: Option<u32>,

    /// Canvas height in pixels (optional, defaults to 600)
    #[prop(optional)] height: Option<u32>,

    /// Additional CSS classes (optional)
    #[prop(optional, into)] class: Option<AttributeValue>,

    /// Interaction configuration (optional)
    #[prop(optional)] interactions: Option<InteractionConfig>,

    /// Performance configuration (optional)
    #[prop(optional)] performance: Option<PerformanceConfig>,

    /// Debug mode for development (optional, defaults to false)
    #[prop(optional)] debug: Option<bool>,
) -> impl IntoView
```

#### Usage Examples

```rust
// Basic static chart
view! {
    <HeliosChart spec=my_chart_spec />
}

// Reactive chart with custom dimensions
let (data, set_data) = create_signal(DataFrame::empty());
let chart_spec = create_memo(move |_| create_line_chart(data.get()));

view! {
    <HeliosChart
        spec=chart_spec
        width=1200
        height=800
        class="my-chart-class"
    />
}

// Interactive chart with performance tuning
view! {
    <HeliosChart
        spec=chart_spec
        interactions=InteractionConfig::new()
            .enable_pan()
            .enable_zoom()
            .enable_brush_select()
        performance=PerformanceConfig::new()
            .target_fps(60)
            .quality_mode(QualityMode::Adaptive)
        debug=true
    />
}
```

### DataLoader

Component for loading and processing data with reactive updates.

```rust
#[component]
pub fn DataLoader<T, F, E>(
    /// Data source configuration
    source: DataSource,

    /// Transform function applied to loaded data
    #[prop(optional)] transform: Option<T>,

    /// Error handler function
    #[prop(optional)] on_error: Option<F>,

    /// Loading state handler
    #[prop(optional)] on_loading: Option<fn() -> impl IntoView>,

    /// Children components that receive the data
    children: Children,
) -> impl IntoView
where
    T: Fn(DataFrame) -> Result<DataFrame, E> + 'static,
    F: Fn(E) -> impl IntoView + 'static,
    E: Error + 'static,
```

#### Usage Examples

```rust
// Basic data loading
view! {
    <DataLoader source=DataSource::csv("data.csv")>
        {move |data: DataFrame| view! {
            <HeliosChart spec=create_chart(data) />
        }}
    </DataLoader>
}

// With error handling and transforms
view! {
    <DataLoader
        source=DataSource::url("https://api.example.com/data")
        transform=|df| df.lazy().filter(col("value").gt(0)).collect()
        on_error=|e| view! { <div class="error">{e.to_string()}</div> }
        on_loading=|| view! { <div class="spinner">"Loading..."</div> }
    >
        {move |data: DataFrame| view! {
            <HeliosChart spec=create_chart(data) />
        }}
    </DataLoader>
}
```

### VisualizationDashboard

Container component for multiple coordinated visualizations.

```rust
#[component]
pub fn VisualizationDashboard(
    /// Dashboard layout configuration
    #[prop(into)] layout: DashboardLayout,

    /// Chart specifications for each panel
    #[prop(into)] charts: MaybeSignal<Vec<ChartSpec>>,

    /// Inter-chart interactions (optional)
    #[prop(optional)] interactions: Option<Vec<ChartInteraction>>,

    /// Global dashboard state (optional)
    #[prop(optional)] state: Option<DashboardState>,
) -> impl IntoView
```

## Chart Specifications

### ChartSpec

Core chart specification structure with compile-time validation.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ChartSpec {
    pub data: DataReference,
    pub mark: MarkType,
    pub encoding: Encoding,
    pub transform: Vec<Transform>,
    pub selection: Vec<Selection>,
    pub intelligence: Option<Intelligence>,
    pub config: ChartConfig,
}

impl ChartSpec {
    /// Create a new chart specification
    pub fn new() -> ChartSpecBuilder;

    /// Validate the specification
    pub fn validate(&self) -> Result<(), ValidationError>;

    /// Optimize for performance
    pub fn optimize(self) -> Self;

    /// Estimate render complexity
    pub fn complexity(&self) -> f64;
}
```

### Chart Macro

Declarative chart creation with compile-time validation.

```rust
// Basic chart specification
let chart = helios::chart! {
    data: my_dataframe,
    mark: Line,
    encoding: {
        x: { field: "date", type: Temporal },
        y: { field: "value", type: Quantitative, scale: { zero: false } },
        color: { field: "category", type: Nominal }
    }
};

// Chart with transformations
let chart = helios::chart! {
    data: raw_data,
    transform: [
        { filter: "datum.value > 0" },
        { aggregate: [{ op: "mean", field: "value", as: "avg_value" }], groupby: ["category"] }
    ],
    mark: {
        type: Bar,
        tooltip: true,
        opacity: 0.8
    },
    encoding: {
        x: { field: "category", type: Ordinal },
        y: { field: "avg_value", type: Quantitative },
        color: { value: "#1f77b4" }
    }
};

// Chart with intelligence features
let chart = helios::chart! {
    data: time_series_data,
    mark: Line,
    encoding: {
        x: { field: "timestamp", type: Temporal },
        y: { field: "metric", type: Quantitative }
    },
    intelligence: {
        forecast: { periods: 30, confidence: 0.95 },
        anomaly_detection: { method: "isolation_forest", threshold: 0.1 },
        trend_analysis: true
    }
};
```

## Data Types

### DataReference

Reference to data for visualization.

```rust
#[derive(Debug, Clone)]
pub enum DataReference {
    /// Direct DataFrame reference
    DataFrame(DataFrame),

    /// URL to data source
    Url { url: String, format: DataFormat },

    /// SQL query against registered dataset
    Query { sql: String, dataset: String },

    /// Server function for data processing
    ServerFunction { function_name: String, params: Value },

    /// Stream of real-time data
    Stream { stream: Box<dyn Stream<Item = DataFrame>> },
}

impl DataReference {
    pub async fn resolve(&self) -> Result<DataFrame, DataError>;
    pub fn is_streaming(&self) -> bool;
    pub fn estimated_size(&self) -> Option<usize>;
}
```

### Mark Types

Comprehensive visual mark types for rendering with advanced statistical and specialized chart types.

```rust
#[derive(Debug, Clone, PartialEq)]
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
        stroke: Option<Color>,
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
        stroke: Option<Color>,
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
        link_opacity: Option<f32>,
        color_scheme: Option<String>,
    },

    /// Treemap for hierarchical data
    Treemap {
        padding: Option<f32>,
        stroke: Option<Color>,
        stroke_width: Option<f32>,
    },

    /// Composite marks
    Composite(Vec<MarkType>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WhiskerType {
    MinMax,
    IQR,
    StandardDeviation,
    Percentile(f32),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KernelType {
    Gaussian,
    Epanechnikov,
    Uniform,
    Triangular,
    Cosine,
}
```

#### Chart Type Examples

```rust
// Scatter plot with trend line
let scatter_chart = helios::chart! {
    data: sales_data,
    mark: {
        type: Scatter,
        size: 5.0,
        jitter: 0.1,
        trend_line: true
    },
    encoding: {
        x: { field: "price", type: Quantitative },
        y: { field: "sales", type: Quantitative },
        color: { field: "category", type: Nominal }
    }
};

// Box plot for statistical analysis
let box_plot = helios::chart! {
    data: performance_data,
    mark: {
        type: BoxPlot,
        width: 0.5,
        outlier_detection: true,
        whisker_type: IQR
    },
    encoding: {
        x: { field: "algorithm", type: Nominal },
        y: { field: "execution_time", type: Quantitative }
    }
};

// Violin plot for distribution comparison
let violin_plot = helios::chart! {
    data: distribution_data,
    mark: {
        type: Violin,
        width: 0.8,
        bandwidth: 0.1,
        kernel: Gaussian
    },
    encoding: {
        x: { field: "group", type: Nominal },
        y: { field: "value", type: Quantitative }
    }
};

// Heatmap for correlation analysis
let heatmap = helios::chart! {
    data: correlation_data,
    mark: {
        type: Heatmap,
        color_scheme: "viridis",
        interpolation: Bilinear
    },
    encoding: {
        x: { field: "variable1", type: Nominal },
        y: { field: "variable2", type: Nominal },
        color: { field: "correlation", type: Quantitative }
    }
};

// Histogram with density overlay
let histogram = helios::chart! {
    data: measurement_data,
    mark: {
        type: Histogram,
        bin_width: 0.5,
        density: true
    },
    encoding: {
        x: { field: "measurement", type: Quantitative },
        y: { field: "density", type: Quantitative }
    }
};

// Density plot for smooth distribution
let density_plot = helios::chart! {
    data: sample_data,
    mark: {
        type: Density,
        bandwidth: 0.2,
        kernel: Gaussian,
        fill: true
    },
    encoding: {
        x: { field: "value", type: Quantitative },
        y: { field: "density", type: Quantitative }
    }
};

// Contour plot for 2D data
let contour_plot = helios::chart! {
    data: spatial_data,
    mark: {
        type: Contour,
        levels: 10,
        color_scheme: "plasma",
        stroke_width: 1.0
    },
    encoding: {
        x: { field: "x", type: Quantitative },
        y: { field: "y", type: Quantitative },
        color: { field: "density", type: Quantitative }
    }
};

// Radar chart for multivariate comparison
let radar_chart = helios::chart! {
    data: multi_dim_data,
    mark: {
        type: Radar,
        radius: 100.0,
        stroke_width: 2.0,
        fill_opacity: 0.3
    },
    encoding: {
        x: { field: "dimension", type: Nominal },
        y: { field: "value", type: Quantitative },
        color: { field: "category", type: Nominal }
    }
};

// Sankey diagram for flow analysis
let sankey_diagram = helios::chart! {
    data: flow_data,
    mark: {
        type: Sankey,
        node_width: 20.0,
        link_opacity: 0.6,
        color_scheme: "category20"
    },
    encoding: {
        source: { field: "source", type: Nominal },
        target: { field: "target", type: Nominal },
        value: { field: "flow", type: Quantitative }
    }
};

// Treemap for hierarchical data
let treemap = helios::chart! {
    data: hierarchical_data,
    mark: {
        type: Treemap,
        padding: 2.0,
        stroke: [0.0, 0.0, 0.0, 1.0],
        stroke_width: 1.0
    },
    encoding: {
        x: { field: "x", type: Quantitative },
        y: { field: "y", type: Quantitative },
        size: { field: "value", type: Quantitative },
        color: { field: "category", type: Nominal }
    }
};
```

### Encoding

Maps data fields to visual properties.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Encoding {
    pub x: Option<PositionEncoding>,
    pub y: Option<PositionEncoding>,
    pub x2: Option<PositionEncoding>,
    pub y2: Option<PositionEncoding>,
    pub color: Option<ColorEncoding>,
    pub opacity: Option<OpacityEncoding>,
    pub size: Option<SizeEncoding>,
    pub shape: Option<ShapeEncoding>,
    pub text: Option<TextEncoding>,
    pub detail: Option<DetailEncoding>,
    pub order: Option<OrderEncoding>,
    pub facet: Option<FacetEncoding>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PositionEncoding {
    pub field: String,
    pub data_type: DataType,
    pub scale: Option<Scale>,
    pub axis: Option<Axis>,
    pub bin: Option<Bin>,
    pub sort: Option<Sort>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Quantitative,
    Ordinal,
    Nominal,
    Temporal,
    Geographic,
}
```

## Enhanced Data Processing

### DataProcessor

Advanced data processing engine with SIMD optimization, intelligent caching, and parallel processing.

```rust
pub struct DataProcessor {
    strategy_selector: StrategySelector,
    cache: HashMap<u64, ProcessedData>,
    stream_buffers: HashMap<String, StreamBuffer>,
    performance_manager: Arc<PerformanceManager>,
}

impl DataProcessor {
    /// Create new data processor with performance optimization
    pub fn new() -> Result<Self, DataError>;

    /// Process data with automatic strategy selection
    pub async fn process(&mut self, spec: &DataSpec) -> Result<ProcessedData, DataError>;

    /// Process data on CPU with SIMD optimization
    pub async fn process_cpu(&self, spec: &DataSpec, config: &RayonConfig) -> Result<ProcessedData, DataError>;

    /// Process data on GPU with WebGPU acceleration
    pub async fn process_gpu(&self, spec: &DataSpec, config: &ComputeConfig) -> Result<ProcessedData, DataError>;

    /// Process streaming data with buffering
    pub async fn process_stream(&mut self, stream_id: &str, data: DataFrame) -> Result<ProcessedData, DataError>;

    /// Apply data transformations with optimization
    fn apply_transform_optimized(&self, df: DataFrame, transform: &DataTransform) -> Result<DataFrame, DataError>;

    /// Apply filters with SIMD acceleration
    fn apply_filter_optimized(&self, df: DataFrame, filter: &Filter) -> Result<DataFrame, DataError>;

    /// Apply aggregations with parallel processing
    fn apply_aggregation_optimized(&self, df: DataFrame, aggregation: &Aggregation) -> Result<DataFrame, DataError>;

    /// Get device capabilities for strategy selection
    pub fn device_capabilities(&self) -> &DeviceCapabilities;
}

#[derive(Debug, Clone)]
pub struct DataSpec {
    pub source: DataSource,
    pub filters: Vec<Filter>,
    pub transforms: Vec<DataTransform>,
    pub aggregations: Vec<Aggregation>,
    pub streaming: bool,
}

#[derive(Debug, Clone)]
pub enum DataTransform {
    /// Select specific columns
    Select { columns: Vec<String> },

    /// Rename columns
    Rename { mappings: Vec<(String, String)> },

    /// Cast columns to different types
    Cast { column: String, data_type: DataType },

    /// Fill null values with strategies
    FillNull { column: String, value: FillValue },

    /// Drop null values
    DropNulls { columns: Option<Vec<String>> },
}

#[derive(Debug, Clone)]
pub enum Filter {
    /// Expression-based filtering
    Expression { expr: String },

    /// Range filtering
    Range { column: String, min: Option<f64>, max: Option<f64> },

    /// Value-based filtering
    Values { column: String, values: Vec<serde_json::Value> },

    /// Null value filtering
    Null { column: String, keep_nulls: bool },
}

#[derive(Debug, Clone)]
pub enum Aggregation {
    /// Simple aggregations
    Aggregate { operations: Vec<AggOp> },

    /// Group by aggregations
    GroupBy { columns: Vec<String> },

    /// Window functions
    Window { operations: Vec<WindowOp> },

    /// Pivot operations
    Pivot { index: String, columns: String, values: String },
}

#[derive(Debug, Clone)]
pub enum AggOp {
    Sum { column: String, alias: Option<String> },
    Mean { column: String, alias: Option<String> },
    Count { column: String, alias: Option<String> },
    Min { column: String, alias: Option<String> },
    Max { column: String, alias: Option<String> },
    Std { column: String, alias: Option<String> },
    Var { column: String, alias: Option<String> },
}

#[derive(Debug, Clone)]
pub enum WindowOp {
    RowNumber { alias: String },
    Rank { column: String, alias: String },
    Lag { column: String, offset: i64, alias: String },
    Lead { column: String, offset: i64, alias: String },
    RollingMean { column: String, window: usize, alias: String },
    RollingSum { column: String, window: usize, alias: String },
}

#[derive(Debug, Clone)]
pub struct ProcessedData {
    pub data: DataFrame,
    pub metadata: DataMetadata,
    pub processing_time: Duration,
}

#[derive(Debug, Clone)]
pub struct DataMetadata {
    pub row_count: usize,
    pub column_count: usize,
    pub memory_usage: usize,
    pub processing_time: Duration,
    pub strategy: ProcessingStrategy,
}

#[derive(Debug, Clone)]
pub enum ProcessingStrategy {
    CPU(RayonConfig),
    GPU(ComputeConfig),
    Stream(StreamConfig),
}

#[derive(Debug, Clone)]
pub struct RayonConfig {
    pub num_threads: Option<usize>,
    pub chunk_size: Option<usize>,
    pub enable_simd: bool,
}

#[derive(Debug, Clone)]
pub struct ComputeConfig {
    pub device_type: DeviceType,
    pub memory_limit: Option<usize>,
    pub enable_shared_memory: bool,
}

#[derive(Debug, Clone)]
pub struct StreamConfig {
    pub buffer_size: usize,
    pub batch_size: usize,
    pub compression: bool,
}

#[derive(Debug, Clone)]
pub struct DeviceCapabilities {
    pub gpu_available: bool,
    pub cpu_cores: usize,
    pub memory_gb: f64,
    pub simd_available: bool,
    pub cache_size: usize,
}

impl DeviceCapabilities {
    /// Detect device capabilities automatically
    pub fn detect() -> Self;
}
```

#### Data Processing Examples

```rust
// Create data processor with performance optimization
let mut processor = DataProcessor::new()?;

// Define data specification with filters and transforms
let spec = DataSpec {
    source: DataSource::csv("sales_data.csv"),
    filters: vec![
        Filter::Range {
            column: "price".to_string(),
            min: Some(10.0),
            max: Some(1000.0)
        },
        Filter::Expression {
            expr: "category == 'Electronics'".to_string()
        },
    ],
    transforms: vec![
        DataTransform::Select {
            columns: vec!["date".to_string(), "price".to_string(), "category".to_string()]
        },
        DataTransform::Cast {
            column: "date".to_string(),
            data_type: DataType::Temporal
        },
        DataTransform::FillNull {
            column: "price".to_string(),
            value: FillValue::Mean
        },
    ],
    aggregations: vec![
        Aggregation::GroupBy {
            columns: vec!["category".to_string()]
        },
        Aggregation::Aggregate {
            operations: vec![
                AggOp::Sum {
                    column: "price".to_string(),
                    alias: Some("total_sales".to_string())
                },
                AggOp::Mean {
                    column: "price".to_string(),
                    alias: Some("avg_price".to_string())
                },
                AggOp::Count {
                    column: "price".to_string(),
                    alias: Some("transaction_count".to_string())
                },
            ]
        },
    ],
    streaming: false,
};

// Process data with automatic strategy selection
let result = processor.process(&spec).await?;

// Process with specific CPU configuration and SIMD
let cpu_config = RayonConfig {
    num_threads: Some(8),
    chunk_size: Some(10000),
    enable_simd: true,
};
let cpu_result = processor.process_cpu(&spec, &cpu_config).await?;

// Process with GPU acceleration
let gpu_config = ComputeConfig {
    device_type: DeviceType::Discrete,
    memory_limit: Some(1024 * 1024 * 1024), // 1GB
    enable_shared_memory: true,
};
let gpu_result = processor.process_gpu(&spec, &gpu_config).await?;

// Streaming data processing
let stream_config = StreamConfig {
    buffer_size: 1000,
    batch_size: 100,
    compression: true,
};

// Process streaming data
let stream_result = processor.process_stream("sales_stream", new_data).await?;

// Get device capabilities
let capabilities = processor.device_capabilities();
println!("GPU available: {}", capabilities.gpu_available);
println!("CPU cores: {}", capabilities.cpu_cores);
println!("SIMD available: {}", capabilities.simd_available);
```

## Server Functions

### Data Processing

Server functions for heavy data processing with performance optimization.

```rust
#[server(ProcessLargeDataset, "/api")]
pub async fn process_large_dataset(
    query: String,
    params: ProcessingParams,
) -> Result<DataFrame, ServerFnError> {
    // Server-side processing with DataFusion and performance optimization
    let ctx = SessionContext::new();
    ctx.register_csv("data", &params.file_path, CsvReadOptions::new()).await?;
    let df = ctx.sql(&query).await?.collect().await?;
    Ok(arrow_to_polars(df)?)
}

#[server(StreamingQuery, "/api")]
pub async fn streaming_query(
    query: String,
) -> Result<impl Stream<Item = DataFrame>, ServerFnError> {
    // Streaming query results with buffering
    let stream = create_streaming_query(&query).await?;
    Ok(stream)
}

#[server(MLForecast, "/api")]
pub async fn ml_forecast(
    data: DataFrame,
    periods: u32,
    config: ForecastConfig,
) -> Result<ForecastResult, ServerFnError> {
    // Machine learning on server with GPU acceleration
    let model = load_forecast_model(&config).await?;
    let forecast = model.predict(&data, periods).await?;
    Ok(forecast)
}

#[server(OptimizedAggregation, "/api")]
pub async fn optimized_aggregation(
    data: DataFrame,
    operations: Vec<AggOp>,
    config: PerformanceConfig,
) -> Result<DataFrame, ServerFnError> {
    // Server-side aggregation with SIMD and parallel processing
    let processor = DataProcessor::new()?;
    let spec = DataSpec {
        source: DataSource::dataframe(data),
        filters: vec![],
        transforms: vec![],
        aggregations: vec![Aggregation::Aggregate { operations }],
        streaming: false,
    };
    let result = processor.process(&spec).await?;
    Ok(result.data)
}
```

## Performance Optimization

### PerformanceConfig

Comprehensive configuration for performance optimization with SIMD, caching, memory pooling, and parallel processing.

```rust
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// Enable SIMD vectorization for data processing
    pub simd_enabled: bool,

    /// Enable intelligent caching system
    pub cache_enabled: bool,

    /// Enable memory pooling for efficient buffer management
    pub memory_pool_enabled: bool,

    /// Enable parallel processing with work-stealing
    pub parallel_processing: bool,

    /// Maximum cache size in bytes (default: 100MB)
    pub max_cache_size: usize,

    /// Memory pool size in bytes (default: 50MB)
    pub memory_pool_size: usize,

    /// Enable work-stealing for load balancing
    pub work_stealing_enabled: bool,

    /// Enable performance profiling and metrics collection
    pub profiling_enabled: bool,

    /// Target FPS for rendering
    pub target_fps: Option<u32>,

    /// Quality mode for adaptive rendering
    pub quality_mode: QualityMode,

    /// Memory limit for operations
    pub memory_limit: Option<usize>,

    /// GPU preference for rendering
    pub gpu_preference: GpuPreference,
}

impl PerformanceConfig {
    /// Create default performance configuration
    pub fn default() -> Self;

    /// Create high-performance configuration
    pub fn high_performance() -> Self;

    /// Create memory-optimized configuration
    pub fn memory_optimized() -> Self;

    /// Create balanced configuration
    pub fn balanced() -> Self;
}
```

### PerformanceManager

Central performance optimization manager coordinating all optimization systems.

```rust
pub struct PerformanceManager {
    config: PerformanceConfig,
    simd_processor: SimdProcessor,
    cache_manager: CacheManager,
    memory_pool: MemoryPool,
    profiler: PerformanceProfiler,
    parallel_processor: ParallelProcessor,
}

impl PerformanceManager {
    /// Create new performance manager with configuration
    pub fn new(config: PerformanceConfig) -> Self;

    /// Get SIMD processor for vectorized operations
    pub fn simd_processor(&self) -> &SimdProcessor;

    /// Get cache manager for intelligent caching
    pub fn cache_manager(&self) -> &CacheManager;

    /// Get memory pool for efficient buffer management
    pub fn memory_pool(&self) -> &MemoryPool;

    /// Get profiler for performance metrics
    pub fn profiler(&self) -> &PerformanceProfiler;

    /// Get parallel processor for work-stealing
    pub fn parallel_processor(&self) -> &ParallelProcessor;

    /// Cleanup all resources
    pub fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>>;
}
```

### SIMD Processor

Vectorized data processing with automatic CPU capability detection.

```rust
pub struct SimdProcessor {
    config: PerformanceConfig,
    capabilities: SimdCapabilities,
}

impl SimdProcessor {
    /// Create SIMD processor with capability detection
    pub fn new(config: PerformanceConfig) -> Self;

    /// Vectorized sum operation with SSE2/NEON acceleration
    pub fn vectorized_sum(&self, data: &[f32]) -> f32;

    /// Vectorized mean calculation
    pub fn vectorized_mean(&self, data: &[f32]) -> f32;

    /// Vectorized standard deviation calculation
    pub fn vectorized_std(&self, data: &[f32]) -> f32;

    /// Vectorized variance calculation
    pub fn vectorized_variance(&self, data: &[f32], mean: f32) -> f32;

    /// Vectorized filtering operation
    pub fn vectorized_filter(&self, data: &[f32], threshold: f32) -> Vec<f32>;
}

#[derive(Debug, Clone)]
pub struct SimdCapabilities {
    pub sse2_available: bool,
    pub sse3_available: bool,
    pub sse4_1_available: bool,
    pub sse4_2_available: bool,
    pub avx_available: bool,
    pub avx2_available: bool,
    pub neon_available: bool,
}

impl SimdCapabilities {
    /// Detect available SIMD capabilities
    pub fn detect() -> Self;
}
```

### Cache Manager

Intelligent caching system with LRU eviction and hit/miss tracking.

```rust
pub struct CacheManager {
    cache: Arc<RwLock<HashMap<CacheKey, CacheEntry>>>,
    config: PerformanceConfig,
    stats: Arc<Mutex<CacheStats>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey {
    pub operation_type: String,
    pub data_hash: u64,
    pub parameters_hash: u64,
}

#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub data: Vec<u8>,
    pub timestamp: SystemTime,
    pub access_count: u64,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub total_size: usize,
}

impl CacheManager {
    /// Create new cache manager
    pub fn new(config: PerformanceConfig) -> Self;

    /// Get cached value
    pub fn get<T>(&self, key: &CacheKey) -> Option<T>
    where T: serde::de::DeserializeOwned;

    /// Put value in cache
    pub fn put<T>(&self, key: CacheKey, value: &T) -> Result<(), CacheError>
    where T: serde::Serialize;

    /// Get cache statistics
    pub fn get_stats(&self) -> Result<CacheStats, CacheError>;

    /// Clear all cached entries
    pub fn clear(&self) -> Result<(), CacheError>;
}
```

### Memory Pool

Efficient buffer management with pre-allocated memory pools.

```rust
pub struct MemoryPool {
    pools: Arc<Mutex<HashMap<usize, Vec<NonNull<u8>>>>>,
    config: PerformanceConfig,
    allocator: Arc<dyn GlobalAlloc + Send + Sync>,
}

impl MemoryPool {
    /// Create new memory pool
    pub fn new(config: PerformanceConfig) -> Self;

    /// Allocate memory from pool
    pub fn allocate(&self, size: usize) -> Result<NonNull<u8>, MemoryPoolError>;

    /// Deallocate memory back to pool
    pub fn deallocate(&self, ptr: NonNull<u8>, size: usize) -> Result<(), MemoryPoolError>;

    /// Cleanup all pooled memory
    pub fn cleanup(&self) -> Result<(), MemoryPoolError>;
}
```

### Parallel Processor

Work-stealing parallel processing with load balancing.

```rust
pub struct ParallelProcessor {
    config: PerformanceConfig,
    thread_pool: Arc<rayon::ThreadPool>,
}

impl ParallelProcessor {
    /// Create parallel processor with thread pool
    pub fn new(config: PerformanceConfig) -> Self;

    /// Parallel map operation with work-stealing
    pub fn parallel_map<T, U, F>(&self, data: &[T], f: F) -> Vec<U>
    where
        T: Send + Sync,
        U: Send,
        F: Fn(&T) -> U + Send + Sync;

    /// Parallel reduce operation
    pub fn parallel_reduce<T, F>(&self, data: &[T], identity: T, f: F) -> T
    where
        T: Send + Sync + Clone,
        F: Fn(T, &T) -> T + Send + Sync;

    /// Parallel filter operation
    pub fn parallel_filter<T, F>(&self, data: &[T], f: F) -> Vec<T>
    where
        T: Send + Sync + Clone,
        F: Fn(&T) -> bool + Send + Sync;
}
```

### Performance Profiler

Comprehensive performance metrics collection and analysis.

```rust
pub struct PerformanceProfiler {
    metrics: Arc<Mutex<HashMap<String, PerformanceMetric>>>,
    config: PerformanceConfig,
}

#[derive(Debug, Clone)]
pub struct PerformanceMetric {
    pub name: String,
    pub total_time: Duration,
    pub call_count: u64,
    pub min_time: Duration,
    pub max_time: Duration,
    pub avg_time: Duration,
}

pub struct PerformanceTimer {
    name: String,
    start_time: Instant,
    profiler: PerformanceProfiler,
}

impl PerformanceProfiler {
    /// Create new profiler
    pub fn new(config: PerformanceConfig) -> Self;

    /// Start timing an operation
    pub fn start_timer(&self, name: String) -> PerformanceTimer;

    /// Record performance metric
    pub fn record_metric(&self, name: String, duration: Duration) -> Result<(), ProfilerError>;

    /// Get all performance metrics
    pub fn get_metrics(&self) -> Result<Vec<PerformanceMetric>, ProfilerError>;

    /// Clear all metrics
    pub fn clear_metrics(&self) -> Result<(), ProfilerError>;
}

impl Drop for PerformanceTimer {
    fn drop(&mut self) {
        let duration = self.start_time.elapsed();
        let _ = self.profiler.record_metric(self.name.clone(), duration);
    }
}
```

#### Usage Examples

```rust
// Create performance manager with custom configuration
let config = PerformanceConfig {
    simd_enabled: true,
    cache_enabled: true,
    memory_pool_enabled: true,
    parallel_processing: true,
    max_cache_size: 200 * 1024 * 1024, // 200MB
    memory_pool_size: 100 * 1024 * 1024, // 100MB
    work_stealing_enabled: true,
    profiling_enabled: true,
    ..Default::default()
};

let perf_manager = PerformanceManager::new(config);

// SIMD-accelerated data processing
let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let sum = perf_manager.simd_processor().vectorized_sum(&data);
let mean = perf_manager.simd_processor().vectorized_mean(&data);
let std_dev = perf_manager.simd_processor().vectorized_std(&data);

// Intelligent caching
let cache_key = CacheKey {
    operation_type: "aggregation".to_string(),
    data_hash: 12345,
    parameters_hash: 67890,
};

if let Some(cached_result) = perf_manager.cache_manager().get::<DataFrame>(&cache_key) {
    // Use cached result
} else {
    // Compute result and cache it
    let result = expensive_computation();
    perf_manager.cache_manager().put(cache_key, &result)?;
}

// Memory pool allocation
let ptr = perf_manager.memory_pool().allocate(1024)?;
// Use allocated memory
perf_manager.memory_pool().deallocate(ptr, 1024)?;

// Parallel processing
let processed_data = perf_manager.parallel_processor().parallel_map(&data, |x| x * 2);
let sum = perf_manager.parallel_processor().parallel_reduce(&data, 0, |acc, x| acc + x);

// Performance profiling
let _timer = perf_manager.profiler().start_timer("data_processing".to_string());
// ... perform operations ...
// Timer automatically records duration when dropped

// Get performance metrics
let metrics = perf_manager.profiler().get_metrics()?;
for metric in metrics {
    println!("{}: {} calls, avg: {:?}", metric.name, metric.call_count, metric.avg_time);
}
```

## WebGPU Rendering Pipeline

### Renderer

High-performance rendering engine with WebGPU acceleration and fallback support.

```rust
pub struct Renderer {
    backend: RenderBackend,
    quality_manager: QualityManager,
    pipeline_cache: HashMap<ChartType, RenderPipeline>,
    performance_manager: Arc<PerformanceManager>,
}

impl Renderer {
    /// Create new renderer with automatic backend selection
    pub async fn new() -> Result<Self, RenderError>;

    /// Render chart with performance optimization
    pub async fn render(&self, spec: &ChartSpec, target: &RenderTarget) -> Result<RenderStats, RenderError>;

    /// Get or create render pipeline for chart type
    fn get_or_create_pipeline(&self, chart_type: &ChartType) -> Result<RenderPipeline, RenderError>;

    /// Execute render pass with WebGPU acceleration
    fn execute_render_pass(&self, pipeline: &RenderPipeline, data: &RenderData) -> Result<RenderStats, RenderError>;
}

#[derive(Debug, Clone)]
pub enum RenderBackend {
    /// WebGPU backend with device and queue
    WebGPU {
        device: Option<Arc<Device>>,
        queue: Option<Arc<Queue>>,
        surface: Option<Arc<Surface<'static>>>,
        adapter_info: AdapterInfo,
    },

    /// WebGL2 backend with context
    WebGL2 {
        context: Option<String>, // WebGl2RenderingContext placeholder
        capabilities: WebGL2Capabilities,
    },

    /// Canvas2D backend with context
    Canvas2D {
        context: Option<String>, // CanvasRenderingContext2d placeholder
    },
}

#[derive(Debug, Clone)]
pub struct AdapterInfo {
    pub name: String,
    pub vendor: String,
    pub device_type: DeviceType,
    pub backend: Backend,
}

#[derive(Debug, Clone)]
pub struct WebGL2Capabilities {
    pub max_texture_size: u32,
    pub max_vertex_attribs: u32,
    pub max_varying_vectors: u32,
    pub max_fragment_uniforms: u32,
    pub max_vertex_uniforms: u32,
}

#[derive(Debug, Clone)]
pub enum DeviceType {
    Integrated,
    Discrete,
    Virtual,
    Cpu,
}

#[derive(Debug, Clone)]
pub enum Backend {
    Vulkan,
    Metal,
    Dx12,
    Dx11,
    Gl,
    BrowserWebGpu,
}

impl RenderBackend {
    /// Create WebGPU backend with device selection
    pub async fn webgpu_backend() -> Result<Self, RenderError>;

    /// Create WebGL2 backend with context
    pub async fn webgl2_backend() -> Result<Self, RenderError>;

    /// Create Canvas2D backend with context
    pub async fn canvas2d_backend() -> Result<Self, RenderError>;

    /// Get performance characteristics of backend
    pub fn performance_characteristics(&self) -> BackendCharacteristics;
}

#[derive(Debug, Clone)]
pub struct BackendCharacteristics {
    pub max_triangles_per_frame: u32,
    pub max_textures: u32,
    pub memory_bandwidth_gbps: f32,
    pub compute_capability: f32,
}

#[derive(Debug, Clone)]
pub struct RenderPipeline {
    chart_type: ChartType,
    webgpu_pipeline: Option<Arc<wgpu::RenderPipeline>>,
    bind_group_layout: Option<Arc<BindGroupLayout>>,
    vertex_buffer: Option<Arc<Buffer>>,
    index_buffer: Option<Arc<Buffer>>,
    uniform_buffer: Option<Arc<Buffer>>,
    shader_module: Option<Arc<ShaderModule>>,
}

impl RenderPipeline {
    /// Create new render pipeline for chart type
    pub fn new(chart_type: ChartType, backend: &RenderBackend) -> Result<Self, RenderError>;

    /// Create WebGPU pipeline with shaders
    fn create_webgpu_pipeline(&self, device: &Device, chart_type: &ChartType) -> Result<wgpu::RenderPipeline, RenderError>;

    /// Get shader source for chart type
    fn get_shader_source(&self, chart_type: &ChartType) -> &str;

    /// Get vertex buffer layout for chart type
    fn get_vertex_buffer_layout(&self, chart_type: &ChartType) -> VertexBufferLayout;

    /// Get primitive topology for chart type
    fn get_primitive_topology(&self, chart_type: &ChartType) -> PrimitiveTopology;
}

#[derive(Debug, Clone)]
pub enum ChartType {
    Point,
    Line,
    Bar,
    Area,
    Text,
    Rect,
    Scatter,
    BoxPlot,
    Violin,
    Heatmap,
    Histogram,
    Density,
    Contour,
    Radar,
    Sankey,
    Treemap,
    Composite,
}

impl ChartType {
    /// Create chart type from mark type
    pub fn from_spec(mark_type: &MarkType) -> Self;

    /// Get chart type name
    pub fn name(&self) -> &'static str;
}

#[derive(Debug, Clone)]
pub struct RenderTarget {
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub sample_count: u32,
}

#[derive(Debug, Clone)]
pub struct RenderData {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub uniforms: Vec<f32>,
    pub textures: Vec<TextureData>,
}

#[derive(Debug, Clone)]
pub struct TextureData {
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct RenderStats {
    pub frame_time: Duration,
    pub triangles_rendered: u32,
    pub draw_calls: u32,
    pub memory_used: usize,
    pub gpu_utilization: f32,
    pub cache_hit_rate: f32,
    pub backend_type: String,
    pub shader_compilation_time: Duration,
}

impl RenderStats {
    /// Calculate FPS from frame time
    pub fn fps(&self) -> f64;

    /// Check if within performance budget
    pub fn is_within_budget(&self, budget: &PerformanceBudget) -> bool;

    /// Suggest performance optimizations
    pub fn suggest_optimizations(&self) -> Vec<OptimizationSuggestion>;
}
```

#### WebGPU Shader Examples

```rust
// Point shader (point.wgsl)
const POINT_SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 0.0, 1.0);
    out.color = model.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
"#;

// Scatter plot shader (scatter.wgsl)
const SCATTER_SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec2<f32>, // Using vec2 for simplicity, will be vec4
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 0.0, 1.0);
    out.color = vec4<f32>(model.color.x, model.color.y, 0.0, 1.0); // Placeholder
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
"#;

// Box plot shader (boxplot.wgsl)
const BOXPLOT_SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) stats: vec4<f32>, // min, q1, median, q3, max
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 0.0, 1.0);
    out.color = vec4<f32>(0.0, 0.5, 0.5, 1.0); // Teal color
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
"#;
```

#### Rendering Examples

```rust
// Create renderer with WebGPU backend
let renderer = Renderer::new().await?;

// Define render target
let target = RenderTarget {
    width: 1200,
    height: 800,
    format: TextureFormat::Bgra8Unorm,
    sample_count: 4,
};

// Create chart specification
let chart_spec = ChartSpec {
    data: DataReference::DataFrame(sales_data),
    mark: MarkType::Scatter {
        size: Some(5.0),
        shape: Some(PointShape::Circle),
        opacity: Some(0.8),
        jitter: Some(0.1),
        trend_line: Some(true),
    },
    encoding: Encoding {
        x: Some(PositionEncoding {
            field: "price".to_string(),
            data_type: DataType::Quantitative,
            scale: None,
            axis: None,
            bin: None,
            sort: None,
        }),
        y: Some(PositionEncoding {
            field: "sales".to_string(),
            data_type: DataType::Quantitative,
            scale: None,
            axis: None,
            bin: None,
            sort: None,
        }),
        color: Some(ColorEncoding {
            field: Some("category".to_string()),
            data_type: Some(DataType::Nominal),
            scale: None,
            legend: None,
        }),
        ..Default::default()
    },
    transform: vec![],
    selection: vec![],
    intelligence: None,
    config: ChartConfig::default(),
};

// Render chart with performance optimization
let stats = renderer.render(&chart_spec, &target).await?;

// Check performance metrics
println!("Frame time: {:?}", stats.frame_time);
println!("FPS: {:.2}", stats.fps());
println!("Triangles rendered: {}", stats.triangles_rendered);
println!("GPU utilization: {:.1}%", stats.gpu_utilization * 100.0);

// Get optimization suggestions
let suggestions = stats.suggest_optimizations();
for suggestion in suggestions {
    println!("Optimization: {}", suggestion.description);
}
```

## Intelligence Features

### ML Pipeline

Machine learning integration for advanced analytics.

```rust
pub struct MLPipeline {
    device: Device,
    models: HashMap<MLTask, LoadedModel>,
    cache: ModelCache,
}

impl MLPipeline {
    /// Create forecasts for time series data
    pub async fn forecast(
        &self,
        data: &DataFrame,
        periods: u32,
        config: ForecastConfig
    ) -> Result<ForecastResult, MLError>;

    /// Detect anomalies in data
    pub async fn detect_anomalies(
        &self,
        data: &DataFrame,
        config: AnomalyConfig
    ) -> Result<Vec<Anomaly>, MLError>;

    /// Cluster data points
    pub async fn cluster(
        &self,
        data: &DataFrame,
        k: u32,
        config: ClusterConfig
    ) -> Result<ClusterResult, MLError>;

    /// Classify data points
    pub async fn classify(
        &self,
        data: &DataFrame,
        model: &str,
        config: ClassificationConfig
    ) -> Result<Vec<Classification>, MLError>;
}
```

### Natural Language Processing

Convert natural language queries to chart specifications.

```rust
pub struct NLProcessor {
    parser: QueryParser,
    validator: SchemaValidator,
    suggester: RecommendationEngine,
}

impl NLProcessor {
    /// Parse natural language query into chart specification
    pub fn parse_query(&self, text: &str, schema: &Schema) -> Result<ChartSpec, ParseError>;

    /// Suggest optimal visualizations for data
    pub fn suggest_visualizations(&self, data: &DataFrame) -> Vec<ChartRecommendation>;

    /// Generate explanation for chart
    pub fn explain_chart(&self, spec: &ChartSpec) -> String;

    /// Convert chart to accessible description
    pub fn generate_alt_text(&self, chart: &RenderedChart) -> String;
}

#[derive(Debug, Clone)]
pub struct ChartRecommendation {
    pub spec: ChartSpec,
    pub confidence: f64,
    pub reasoning: String,
    pub best_practices: Vec<String>,
}
```

## Error Handling

### Error Types

Comprehensive error handling throughout the API.

```rust
#[derive(Debug, thiserror::Error)]
pub enum HeliosError {
    #[error("Data processing error: {0}")]
    DataProcessing(#[from] DataError),

    #[error("Rendering error: {0}")]
    Rendering(#[from] RenderError),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("ML error: {0}")]
    MachineLearning(#[from] MLError),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Performance budget exceeded: {details}")]
    PerformanceBudget { details: String },
}

impl HeliosError {
    pub fn is_recoverable(&self) -> bool;
    pub fn user_message(&self) -> String;
    pub fn suggested_actions(&self) -> Vec<String>;
}
```

## Accessibility

### AccessibilityConfig

Configuration for accessibility compliance.

```rust
#[derive(Debug, Clone)]
pub struct AccessibilityConfig {
    pub screen_reader: ScreenReaderConfig,
    pub keyboard_nav: KeyboardConfig,
    pub color_vision: ColorVisionConfig,
    pub motion: MotionConfig,
    pub text_scaling: TextScalingConfig,
}

impl AccessibilityConfig {
    pub fn wcag_aa() -> Self;
    pub fn wcag_aaa() -> Self;
    pub fn from_user_preferences(prefs: &UserPreferences) -> Self;

    pub fn validate_compliance(&self, chart: &Chart) -> ComplianceReport;
    pub fn generate_data_table(&self, chart: &Chart) -> DataTable;
    pub fn create_sonification(&self, data: &DataFrame) -> AudioRepresentation;
}
```

## Type Definitions

### Common Types

```rust
pub type DataFrame = polars::DataFrame;
pub type LazyFrame = polars::LazyFrame;
pub type Color = [f32; 4]; // RGBA
pub type Point2D = [f32; 2];
pub type Point3D = [f32; 3];
pub type Rect = [f32; 4]; // x, y, width, height
pub type Transform2D = [[f32; 3]; 2]; // 2D affine transform matrix

pub type Result<T> = std::result::Result<T, HeliosError>;
pub type AsyncResult<T> = Pin<Box<dyn Future<Output = Result<T>>>>;
```

This API provides a comprehensive, type-safe interface for creating high-performance visualizations while maintaining integration with Leptos's reactive system and leveraging Rust's compile-time guarantees.
