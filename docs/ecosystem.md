# Helios Ecosystem Integration

> Comprehensive guide to integrating Helios with the Rust web ecosystem

## Overview

Helios is designed to seamlessly integrate with the modern Rust web development ecosystem. This guide covers integration patterns, best practices, and real-world examples for combining Helios with popular frameworks, libraries, and tools.

## Core Ecosystem Integrations

### Leptos v0.8 Integration

Helios is built specifically for Leptos v0.8, leveraging its fine-grained reactivity and modern web capabilities.

#### Server-Side Rendering (SSR)

```rust
// Server configuration
#[cfg(feature = "ssr")]
use leptos_axum::{generate_route_list, LeptosRoutes};

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{Router, routing::get};
    use leptos::*;

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|| view! { <App/> });

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, || view! { <App/> })
        .route("/api/*fn_name", get(leptos_axum::handle_server_fns))
        // Add Helios-specific routes
        .route("/api/data/*path", get(helios_data_handler))
        .with_state(leptos_options);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Server function for data processing
#[server(ProcessVisualizationData, "/api")]
pub async fn process_visualization_data(
    query: String,
    format: DataFormat,
) -> Result<ProcessedDataSet, ServerFnError> {
    use datafusion::prelude::*;

    let ctx = SessionContext::new();

    // Register data sources
    match format {
        DataFormat::CSV => {
            ctx.register_csv("data", &query, CsvReadOptions::new()).await?;
        },
        DataFormat::Parquet => {
            ctx.register_parquet("data", &query, ParquetReadOptions::default()).await?;
        },
        DataFormat::JSON => {
            ctx.register_json("data", &query, NdJsonReadOptions::default()).await?;
        }
    }

    // Execute optimized query
    let df = ctx.sql("SELECT * FROM data LIMIT 100000").await?.collect().await?;

    Ok(ProcessedDataSet {
        data: arrow_to_polars(df)?,
        metadata: DataMetadata::from_arrow(&df),
    })
}
```

#### Islands Architecture

```rust
// Use islands for selective hydration
#[island]
pub fn InteractiveChart() -> impl IntoView {
    let (data, set_data) = create_signal(DataFrame::empty());
    let (config, set_config) = create_signal(ChartConfig::default());

    // This component is hydrated on the client
    let chart_spec = create_memo(move |_| {
        helios::chart! {
            data: data.get(),
            mark: config.get().mark_type,
            encoding: config.get().encoding.clone(),
            intelligence: {
                forecast: { periods: 30 }
            }
        }
    });

    view! {
        <div class="interactive-chart">
            <HeliosChart spec=chart_spec />
            <ChartControls
                config=config
                on_config_change=set_config
            />
        </div>
    }
}

// Server-only component for static content
#[component]
pub fn StaticDashboard() -> impl IntoView {
    // This renders on the server only
    let server_data = expect_context::<ServerData>();

    view! {
        <div class="dashboard">
            <h1>"Analytics Dashboard"</h1>

            // Static charts rendered server-side
            <div class="static-charts">
                <StaticChart data=server_data.revenue />
                <StaticChart data=server_data.users />
            </div>

            // Interactive island
            <InteractiveChart />
        </div>
    }
}
```

## Data Processing Ecosystem

### Polars Integration

Helios uses Polars as its primary data processing engine, providing seamless interoperability.

```rust
use polars::prelude::*;
use helios::prelude::*;

// Direct DataFrame integration
fn create_analysis_dashboard(df: DataFrame) -> impl IntoView {
    // Polars lazy evaluation for optimal performance
    let processed_data = df
        .lazy()
        .select([
            col("timestamp"),
            col("value"),
            col("category"),
            col("value").rolling_mean(RollingOptions::default()).alias("moving_avg"),
        ])
        .filter(col("value").gt(0))
        .group_by([col("category")])
        .agg([
            col("value").mean().alias("avg_value"),
            col("value").std(1).alias("std_value"),
            col("value").count().alias("count"),
        ])
        .collect()
        .unwrap();

    let charts = vec![
        // Time series chart
        helios::chart! {
            data: df.clone(),
            mark: Line,
            encoding: {
                x: { field: "timestamp", type: Temporal },
                y: { field: "value", type: Quantitative },
                color: { field: "category", type: Nominal }
            }
        },
        // Statistical summary
        helios::chart! {
            data: processed_data,
            mark: Bar,
            encoding: {
                x: { field: "category", type: Ordinal },
                y: { field: "avg_value", type: Quantitative },
                color: { field: "std_value", type: Quantitative }
            }
        }
    ];

    view! {
        <VisualizationDashboard
            layout=DashboardLayout::Grid(2, 1)
            charts=charts
        />
    }
}
```

### DataFusion Query Engine

For complex analytical queries, Helios integrates with Apache DataFusion.

```rust
use datafusion::prelude::*;
use datafusion::arrow::record_batch::RecordBatch;

// Advanced SQL analytics server function
#[server(AdvancedAnalytics, "/api")]
pub async fn advanced_analytics(
    sql_query: String,
    data_sources: Vec<DataSource>,
) -> Result<AnalyticsResult, ServerFnError> {
    let ctx = SessionContext::new();

    // Register multiple data sources
    for source in data_sources {
        match source {
            DataSource::CSV { path, name } => {
                ctx.register_csv(&name, &path, CsvReadOptions::new()).await?;
            },
            DataSource::Parquet { path, name } => {
                ctx.register_parquet(&name, &path, ParquetReadOptions::default()).await?;
            },
            DataSource::Database { connection_string, name, query } => {
                // Custom database integration
                let table = create_database_table(&connection_string, &query).await?;
                ctx.register_table(&name, table)?;
            }
        }
    }

    // Execute complex analytical query
    let logical_plan = ctx.sql(&sql_query).await?;

    // DataFusion optimizations:
    // - Predicate pushdown
    // - Projection pushdown
    // - Join reordering
    // - Constant folding
    let optimized_plan = ctx.optimize(&logical_plan)?;

    let result_batches = ctx.collect(optimized_plan).await?;

    // Convert to visualization-ready format
    Ok(AnalyticsResult {
        data: arrow_batches_to_polars(result_batches)?,
        query_stats: QueryStats {
            execution_time: start_time.elapsed(),
            rows_processed: result_batches.iter().map(|b| b.num_rows()).sum(),
            bytes_processed: calculate_batch_size(&result_batches),
        }
    })
}
```

### Apache Arrow Integration

Helios leverages Arrow's columnar format for efficient data interchange.

```rust
use arrow::array::{Float64Array, StringArray};
use arrow::record_batch::RecordBatch;

// Zero-copy Arrow to Polars conversion
pub fn arrow_to_polars_efficient(batches: Vec<RecordBatch>) -> Result<DataFrame, PolarsError> {
    // Convert Arrow schema to Polars
    let arrow_schema = batches[0].schema();
    let polars_schema = Schema::from_arrow_schema(&arrow_schema)?;

    // Convert data with zero-copy where possible
    let mut columns = Vec::new();

    for (i, field) in arrow_schema.fields().iter().enumerate() {
        let arrow_arrays: Vec<_> = batches
            .iter()
            .map(|batch| batch.column(i).clone())
            .collect();

        // Zero-copy conversion for supported types
        let polars_series = match field.data_type() {
            DataType::Float64 => {
                let arrays: Vec<_> = arrow_arrays
                    .into_iter()
                    .map(|arr| arr.as_any().downcast_ref::<Float64Array>().unwrap())
                    .collect();
                Series::new(&field.name(), arrays)
            },
            DataType::Utf8 => {
                let arrays: Vec<_> = arrow_arrays
                    .into_iter()
                    .map(|arr| arr.as_any().downcast_ref::<StringArray>().unwrap())
                    .collect();
                Series::new(&field.name(), arrays)
            },
            _ => {
                // Fallback for other types
                Series::from_arrow(&field.name(), &arrow_arrays[0])?
            }
        };

        columns.push(polars_series);
    }

    DataFrame::new(columns)
}
```

## Web Framework Integrations

### Axum Integration

Helios integrates seamlessly with Axum for server-side functionality.

```rust
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};

#[derive(serde::Deserialize)]
pub struct DataQuery {
    table: String,
    filters: Option<String>,
    limit: Option<usize>,
}

// RESTful API for data access
async fn get_visualization_data(
    Query(params): Query<DataQuery>,
    State(app_state): State<AppState>,
) -> Result<Json<DataFrame>, StatusCode> {
    let mut query = format!("SELECT * FROM {}", params.table);

    if let Some(filters) = params.filters {
        query.push_str(&format!(" WHERE {}", filters));
    }

    if let Some(limit) = params.limit {
        query.push_str(&format!(" LIMIT {}", limit));
    }

    match app_state.query_engine.execute_sql(&query).await {
        Ok(df) => Ok(Json(df)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// WebSocket for real-time data
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(app_state): State<AppState>,
) -> axum::response::Response {
    ws.on_upgrade(|socket| handle_websocket(socket, app_state))
}

async fn handle_websocket(mut socket: WebSocket, state: AppState) {
    let mut data_stream = state.create_realtime_stream().await;

    while let Some(data_batch) = data_stream.next().await {
        let message = serde_json::to_string(&data_batch).unwrap();

        if socket.send(Message::Text(message)).await.is_err() {
            break;
        }
    }
}

// Build the router
pub fn create_api_router() -> Router<AppState> {
    Router::new()
        .route("/api/data", get(get_visualization_data))
        .route("/api/stream", get(websocket_handler))
        .route("/api/export/:format", post(export_visualization))
}
```

### Tauri Desktop Integration

For desktop applications, Helios integrates with Tauri.

```rust
// Tauri command for file system access
#[tauri::command]
async fn load_data_file(path: String) -> Result<DataFrame, String> {
    use polars::prelude::*;

    let df = match std::path::Path::new(&path).extension().and_then(|s| s.to_str()) {
        Some("csv") => {
            LazyFrame::scan_csv(&path, ScanArgsIo::default())
                .collect()
                .map_err(|e| e.to_string())?
        },
        Some("parquet") => {
            LazyFrame::scan_parquet(&path, ScanArgsParquet::default())
                .collect()
                .map_err(|e| e.to_string())?
        },
        Some("json") => {
            LazyFrame::scan_ndjson(&path, ScanArgsNdJson::default())
                .collect()
                .map_err(|e| e.to_string())?
        },
        _ => return Err("Unsupported file format".to_string()),
    };

    Ok(df)
}

// Tauri main function
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_data_file,
            export_chart,
            get_system_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Frontend integration
#[component]
pub fn DesktopVisualizationApp() -> impl IntoView {
    let (data, set_data) = create_signal(DataFrame::empty());

    let load_file = move |file_path: String| {
        spawn_local(async move {
            // Call Tauri command
            let result = invoke("load_data_file", serde_json::json!({
                "path": file_path
            })).await;

            match result {
                Ok(df) => set_data(df),
                Err(e) => console::error_1(&format!("Error loading file: {}", e).into()),
            }
        });
    };

    view! {
        <div class="desktop-app">
            <FileSelector on_file_selected=load_file />
            <HeliosChart spec=create_chart_spec(data.get()) />
        </div>
    }
}
```

## Database Integrations

### PostgreSQL Integration

```rust
use sqlx::{postgres::PgPoolOptions, PgPool, Row};

pub struct PostgresDataSource {
    pool: PgPool,
}

impl PostgresDataSource {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn query_to_dataframe(&self, query: &str) -> Result<DataFrame, DataError> {
        let rows = sqlx::query(query)
            .fetch_all(&self.pool)
            .await?;

        // Convert PostgreSQL rows to Polars DataFrame
        self.rows_to_dataframe(rows)
    }

    fn rows_to_dataframe(&self, rows: Vec<PgRow>) -> Result<DataFrame, DataError> {
        if rows.is_empty() {
            return Ok(DataFrame::empty());
        }

        let columns = rows[0].columns();
        let mut series_map: HashMap<String, Vec<AnyValue>> = HashMap::new();

        for column in columns {
            series_map.insert(column.name().to_string(), Vec::new());
        }

        for row in rows {
            for (i, column) in columns.iter().enumerate() {
                let value = match column.type_info().name() {
                    "INT4" => {
                        let val: Option<i32> = row.try_get(i)?;
                        val.map(AnyValue::Int32).unwrap_or(AnyValue::Null)
                    },
                    "INT8" => {
                        let val: Option<i64> = row.try_get(i)?;
                        val.map(AnyValue::Int64).unwrap_or(AnyValue::Null)
                    },
                    "FLOAT8" => {
                        let val: Option<f64> = row.try_get(i)?;
                        val.map(AnyValue::Float64).unwrap_or(AnyValue::Null)
                    },
                    "TEXT" | "VARCHAR" => {
                        let val: Option<String> = row.try_get(i)?;
                        val.map(|s| AnyValue::Utf8(&s)).unwrap_or(AnyValue::Null)
                    },
                    _ => AnyValue::Null,
                };

                series_map.get_mut(column.name()).unwrap().push(value);
            }
        }

        let series: Vec<Series> = series_map
            .into_iter()
            .map(|(name, values)| Series::new(&name, values))
            .collect();

        Ok(DataFrame::new(series)?)
    }
}

// Usage in server function
#[server(QueryDatabase, "/api")]
pub async fn query_database(sql: String) -> Result<DataFrame, ServerFnError> {
    let db = PostgresDataSource::new(&std::env::var("DATABASE_URL")?).await?;
    let df = db.query_to_dataframe(&sql).await?;
    Ok(df)
}
```

### ClickHouse Integration

```rust
use clickhouse::{Client, Row};

pub struct ClickHouseDataSource {
    client: Client,
}

impl ClickHouseDataSource {
    pub fn new(url: &str) -> Self {
        let client = Client::default().with_url(url);
        Self { client }
    }

    pub async fn query_to_dataframe(&self, query: &str) -> Result<DataFrame, DataError> {
        // ClickHouse is optimized for analytical queries
        let mut cursor = self.client.query(query).fetch()?;

        let mut rows = Vec::new();
        while let Some(row) = cursor.next().await? {
            rows.push(row);
        }

        self.rows_to_dataframe(rows)
    }

    // Optimized for analytical workloads
    pub async fn streaming_query(&self, query: &str) -> Result<impl Stream<Item = DataFrame>, DataError> {
        let cursor = self.client.query(query).fetch()?;

        // Convert cursor to DataFrame stream
        Ok(cursor.map(|batch_result| {
            match batch_result {
                Ok(rows) => self.rows_to_dataframe(rows),
                Err(e) => Err(DataError::from(e)),
            }
        }))
    }
}
```

## Machine Learning Ecosystem

### Candle Integration

```rust
use candle_core::{Device, Tensor, Result as CandleResult};
use candle_nn::{Linear, Module, VarBuilder};

pub struct ForecastingModel {
    device: Device,
    model: LinearRegression,
    scaler: StandardScaler,
}

impl ForecastingModel {
    pub fn new(device: Device) -> CandleResult<Self> {
        let model = LinearRegression::new(&device, 1, 1)?;
        let scaler = StandardScaler::new();

        Ok(Self { device, model, scaler })
    }

    pub fn train(&mut self, data: &DataFrame) -> CandleResult<TrainingStats> {
        // Convert DataFrame to tensors
        let features = self.dataframe_to_tensor(data, &["feature"])?;
        let targets = self.dataframe_to_tensor(data, &["target"])?;

        // Scale features
        let scaled_features = self.scaler.fit_transform(&features)?;

        // Training loop
        let mut optimizer = candle_nn::AdamW::new(self.model.parameters(), 0.001)?;
        let mut losses = Vec::new();

        for epoch in 0..1000 {
            let predictions = self.model.forward(&scaled_features)?;
            let loss = mse_loss(&predictions, &targets)?;

            optimizer.backward_step(&loss)?;

            if epoch % 100 == 0 {
                losses.push(loss.to_scalar::<f32>()?);
            }
        }

        Ok(TrainingStats { losses })
    }

    pub fn predict(&self, data: &DataFrame, periods: usize) -> CandleResult<DataFrame> {
        let features = self.dataframe_to_tensor(data, &["feature"])?;
        let scaled_features = self.scaler.transform(&features)?;

        let predictions = self.model.forward(&scaled_features)?;

        // Convert predictions back to DataFrame
        self.tensor_to_dataframe(&predictions, &["forecast"])
    }

    fn dataframe_to_tensor(&self, df: &DataFrame, columns: &[&str]) -> CandleResult<Tensor> {
        let mut data = Vec::new();

        for col_name in columns {
            let series = df.column(col_name).map_err(|e| {
                candle_core::Error::Msg(format!("Column not found: {}", e))
            })?;

            let values: Vec<f32> = series
                .f64()?
                .into_iter()
                .map(|v| v.unwrap_or(0.0) as f32)
                .collect();

            data.extend(values);
        }

        let shape = (df.height(), columns.len());
        Tensor::from_vec(data, shape, &self.device)
    }

    fn tensor_to_dataframe(&self, tensor: &Tensor, columns: &[&str]) -> CandleResult<DataFrame> {
        let data = tensor.to_vec2::<f32>()?;
        let mut series_vec = Vec::new();

        for (i, col_name) in columns.iter().enumerate() {
            let values: Vec<f64> = data.iter().map(|row| row[i] as f64).collect();
            let series = Series::new(col_name, values);
            series_vec.push(series);
        }

        DataFrame::new(series_vec).map_err(|e| {
            candle_core::Error::Msg(format!("DataFrame creation failed: {}", e))
        })
    }
}

// Integration with Helios
#[component]
pub fn ForecastingChart() -> impl IntoView {
    let (historical_data, set_historical_data) = create_signal(DataFrame::empty());
    let (forecast_data, set_forecast_data) = create_signal(None::<DataFrame>);

    let generate_forecast = create_action(move |periods: &u32| {
        let periods = *periods;
        let data = historical_data.get();

        async move {
            // Run ML model on server
            match run_forecast_model(data, periods).await {
                Ok(forecast) => set_forecast_data(Some(forecast)),
                Err(e) => console::error_1(&format!("Forecast failed: {}", e).into()),
            }
        }
    });

    let combined_data = create_memo(move |_| {
        let historical = historical_data.get();
        let forecast = forecast_data.get();

        if let Some(forecast_df) = forecast {
            combine_dataframes(historical, forecast_df)
        } else {
            historical
        }
    });

    let chart_spec = create_memo(move |_| {
        helios::chart! {
            data: combined_data.get(),
            layer: [
                {
                    mark: Line,
                    encoding: {
                        x: { field: "date", type: Temporal },
                        y: { field: "value", type: Quantitative },
                        color: { value: "blue" }
                    },
                    transform: [{ filter: "datum.type === 'historical'" }]
                },
                {
                    mark: Line { stroke_dash: Some(vec![5.0, 5.0]) },
                    encoding: {
                        x: { field: "date", type: Temporal },
                        y: { field: "value", type: Quantitative },
                        color: { value: "red" }
                    },
                    transform: [{ filter: "datum.type === 'forecast'" }]
                }
            ]
        }
    });

    view! {
        <div class="forecasting-dashboard">
            <div class="controls">
                <button on:click=move |_| generate_forecast.dispatch(30)>
                    "30-day Forecast"
                </button>
                <button on:click=move |_| generate_forecast.dispatch(90)>
                    "90-day Forecast"
                </button>
            </div>
            <HeliosChart spec=chart_spec />
        </div>
    }
}
```

## Deployment Integrations

### Docker Integration

```dockerfile
# Multi-stage Dockerfile for Helios applications
FROM rust:1.79-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js for WASM tools
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs

# Install Rust tools
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-opt

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

# Build dependencies (cached layer)
RUN cargo build --release --bin server

# Copy source code
COPY . .

# Build WASM frontend
RUN trunk build --release

# Build server
RUN cargo build --release --bin server

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary and static files
COPY --from=builder /app/target/release/server /app/
COPY --from=builder /app/dist /app/static/

# Environment variables
ENV RUST_LOG=info
ENV HELIOS_STATIC_DIR=/app/static

EXPOSE 3000

CMD ["./server"]
```

### Kubernetes Deployment

```yaml
# kubernetes/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: helios-viz-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: helios-viz-app
  template:
    metadata:
      labels:
        app: helios-viz-app
    spec:
      containers:
      - name: app
        image: helios-viz-app:latest
        ports:
        - containerPort: 3000
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: db-secret
              key: url
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "1Gi"
            cpu: "1000m"
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5

---
apiVersion: v1
kind: Service
metadata:
  name: helios-viz-service
spec:
  selector:
    app: helios-viz-app
  ports:
  - port: 80
    targetPort: 3000
  type: LoadBalancer

---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: helios-viz-ingress
  annotations:
    kubernetes.io/ingress.class: nginx
    cert-manager.io/cluster-issuer: letsencrypt-prod
spec:
  tls:
  - hosts:
    - viz.example.com
    secretName: helios-viz-tls
  rules:
  - host: viz.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: helios-viz-service
            port:
              number: 80
```

### CDN Integration

```rust
// CDN asset optimization
use axum::{
    response::{IntoResponse, Response},
    http::{header, StatusCode},
};

// Serve optimized static assets
async fn serve_wasm_asset(
    uri: Uri,
) -> Result<Response, StatusCode> {
    let path = uri.path();

    if path.ends_with(".wasm") {
        // Set optimal headers for WASM files
        let headers = [
            (header::CONTENT_TYPE, "application/wasm"),
            (header::CACHE_CONTROL, "public, max-age=31536000"), // 1 year
            (header::CONTENT_ENCODING, "gzip"),
        ];

        let content = tokio::fs::read(format!("./static{}", path))
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        Ok((headers, content).into_response())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// Progressive loading for large applications
#[component]
pub fn ProgressiveApp() -> impl IntoView {
    view! {
        <div class="app">
            // Core shell loads immediately
            <AppShell />

            // Charts load progressively
            <Suspense fallback=|| view! { <ChartSkeleton /> }>
                <LazyLoadedCharts />
            </Suspense>
        </div>
    }
}
```

## Testing Ecosystem

### Integration Testing

```rust
// Browser testing with wasm-bindgen-test
#[cfg(test)]
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_chart_rendering() {
    use web_sys::window;

    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    // Create test container
    let container = document.create_element("div").unwrap();
    body.append_child(&container).unwrap();

    // Mount Helios chart
    let chart_spec = helios::chart! {
        data: create_test_dataframe(),
        mark: Point,
        encoding: {
            x: { field: "x", type: Quantitative },
            y: { field: "y", type: Quantitative }
        }
    };

    // Test rendering performance
    let start = web_sys::js_sys::Date::now();
    render_chart_to_element(&chart_spec, &container).await;
    let render_time = web_sys::js_sys::Date::now() - start;

    assert!(render_time < 100.0, "Rendering took too long: {}ms", render_time);

    // Test DOM structure
    let canvas = container.query_selector("canvas").unwrap().unwrap();
    assert!(canvas.client_width() > 0);
    assert!(canvas.client_height() > 0);

    // Cleanup
    body.remove_child(&container).unwrap();
}

// Performance benchmarking
#[cfg(test)]
mod bench {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    fn benchmark_data_processing(c: &mut Criterion) {
        let large_df = create_large_test_dataframe(1_000_000);

        c.bench_function("process 1M rows", |b| {
            b.iter(|| {
                process_dataframe_for_visualization(black_box(&large_df))
            })
        });
    }

    fn benchmark_webgpu_rendering(c: &mut Criterion) {
        // Note: This would run in a separate GPU benchmark suite
        c.bench_function("render 100K points", |b| {
            b.iter(|| {
                render_point_cloud(black_box(&create_point_data(100_000)))
            })
        });
    }

    criterion_group!(benches, benchmark_data_processing, benchmark_webgpu_rendering);
    criterion_main!(benches);
}
```

## Monitoring and Observability

### Telemetry Integration

```rust
use tracing::{info, instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[instrument(skip(data))]
pub async fn process_large_dataset(data: DataFrame) -> Result<ProcessedData, ProcessingError> {
    let span = Span::current();
    span.set_attribute("data.rows", data.height() as i64);
    span.set_attribute("data.columns", data.width() as i64);

    info!("Starting data processing for {} rows", data.height());

    let start = std::time::Instant::now();

    let result = data
        .lazy()
        .select([col("*")])
        .filter(col("value").gt(0))
        .collect();

    let processing_time = start.elapsed();
    span.set_attribute("processing.duration_ms", processing_time.as_millis() as i64);

    match result {
        Ok(processed) => {
            info!("Data processing completed in {:?}", processing_time);
            span.set_attribute("processing.success", true);
            Ok(ProcessedData::new(processed))
        },
        Err(e) => {
            span.set_attribute("processing.success", false);
            span.set_attribute("error.message", e.to_string());
            Err(ProcessingError::from(e))
        }
    }
}

// Performance monitoring
#[component]
pub fn MonitoredChart() -> impl IntoView {
    let (metrics, set_metrics) = create_signal(PerformanceMetrics::default());

    let chart_spec = helios::chart! {
        data: get_chart_data(),
        mark: Point,
        encoding: {
            x: { field: "x", type: Quantitative },
            y: { field: "y", type: Quantitative }
        },
        config: {
            on_render_complete: Box::new(move |stats| {
                // Send metrics to monitoring system
                send_performance_metrics(&stats);
                set_metrics(stats.into());
            })
        }
    };

    view! {
        <div class="monitored-chart">
            <HeliosChart spec=chart_spec />
            <PerformanceIndicator metrics=metrics />
        </div>
    }
}
```

This ecosystem integration guide demonstrates how Helios seamlessly connects with the broader Rust web development ecosystem, providing a complete solution for high-performance data visualization applications.
