//! Natural Language Query Processing
//!
//! This module provides natural language to chart specification conversion,
//! enabling users to create visualizations using natural language queries.

use crate::chart::{
    ChartSpec, ChartSpecBuilder, ColorEncoding, DataType, Encoding, Intelligence, MarkType,
    PositionEncoding, SizeEncoding,
};
use chrono::NaiveDate;
use polars::prelude::*;
use std::collections::HashMap;

/// Natural Language Query Processing errors
#[derive(Debug, thiserror::Error)]
pub enum NLError {
    #[error("Failed to parse query: {0}")]
    ParseError(String),

    #[error("Unsupported chart type: {0}")]
    UnsupportedChartType(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid data reference: {0}")]
    InvalidDataReference(String),

    #[error("Ambiguous query: {0}")]
    AmbiguousQuery(String),
}

/// Configuration for natural language processing
#[derive(Debug, Clone)]
pub struct NLConfig {
    pub fuzzy_matching: bool,
    pub auto_suggestions: bool,
    pub confidence_threshold: f64,
    pub max_suggestions: usize,
}

impl Default for NLConfig {
    fn default() -> Self {
        Self {
            fuzzy_matching: true,
            auto_suggestions: true,
            confidence_threshold: 0.3,
            max_suggestions: 5,
        }
    }
}

/// Intelligence configuration extracted from natural language
#[derive(Debug, Clone)]
pub struct IntelligenceConfig {
    pub forecast: Option<ForecastConfig>,
    pub anomaly_detection: Option<AnomalyConfig>,
    pub trend_analysis: bool,
    pub clustering: Option<ClusteringConfig>,
}

#[derive(Debug, Clone)]
pub struct ForecastConfig {
    pub periods: u32,
    pub confidence: f64,
    pub method: ForecastMethod,
}

#[derive(Debug, Clone)]
pub enum ForecastMethod {
    Auto,
    Linear,
    Seasonal,
    ARIMA { p: u32, d: u32, q: u32 },
}

#[derive(Debug, Clone)]
pub struct AnomalyConfig {
    pub method: String,
    pub threshold: f64,
    pub sensitivity: f64,
}

#[derive(Debug, Clone)]
pub struct ClusteringConfig {
    pub method: String,
    pub n_clusters: Option<usize>,
    pub auto_clusters: bool,
}

/// Query pattern matching result
#[derive(Debug, Clone)]
pub struct QueryMatch {
    pub confidence: f64,
    pub chart_type: MarkType,
    pub x_field: Option<String>,
    pub y_field: Option<String>,
    pub color_field: Option<String>,
    pub size_field: Option<String>,
    pub intelligence: Option<IntelligenceConfig>,
}

/// Natural Language Query Processor
pub struct NLProcessor {
    config: NLConfig,
    chart_patterns: Vec<ChartPattern>,
    field_synonyms: HashMap<String, Vec<String>>,
    intelligence_patterns: Vec<IntelligencePattern>,
}

/// Pattern for matching chart types in queries
#[derive(Debug, Clone)]
struct ChartPattern {
    keywords: Vec<String>,
    chart_type: MarkType,
    confidence_boost: f64,
}

/// Pattern for matching intelligence features in queries
#[derive(Debug, Clone)]
struct IntelligencePattern {
    keywords: Vec<String>,
    intelligence_type: IntelligenceType,
    confidence_boost: f64,
}

#[derive(Debug, Clone)]
enum IntelligenceType {
    Forecast,
    AnomalyDetection,
    TrendAnalysis,
    Clustering,
}

impl NLProcessor {
    /// Create a new natural language processor
    pub fn new() -> Self {
        Self::with_config(NLConfig::default())
    }

    /// Create a processor with custom configuration
    pub fn with_config(config: NLConfig) -> Self {
        let mut processor = Self {
            config,
            chart_patterns: Vec::new(),
            field_synonyms: HashMap::new(),
            intelligence_patterns: Vec::new(),
        };

        processor.initialize_patterns();
        processor.initialize_field_synonyms();
        processor
    }

    /// Parse natural language query into chart specification
    pub fn parse_query(&self, query: &str) -> Result<ChartSpec, NLError> {
        let normalized_query = self.normalize_query(query);

        // Extract chart type and fields
        let query_match = self.match_query_patterns(&normalized_query)?;

        // Build chart specification
        let mut spec = ChartSpec::new();
        spec.mark = query_match.chart_type;

        // Add encodings
        let mut encoding = Encoding::default();

        if let Some(x_field) = query_match.x_field {
            encoding.x = Some(PositionEncoding {
                field: x_field,
                data_type: DataType::Quantitative,
                scale: None,
                axis: None,
                bin: None,
                sort: None,
            });
        }

        if let Some(y_field) = query_match.y_field {
            encoding.y = Some(PositionEncoding {
                field: y_field,
                data_type: DataType::Quantitative,
                scale: None,
                axis: None,
                bin: None,
                sort: None,
            });
        }

        // For now, let's simplify the color and size encodings by not implementing them
        // since they require different encoding types

        spec.encoding = encoding;

        // Add intelligence configuration
        if let Some(intelligence_config) = query_match.intelligence {
            spec.intelligence = Some(Intelligence {
                forecast: intelligence_config
                    .forecast
                    .map(|fc| crate::chart::ForecastConfig {
                        periods: fc.periods,
                        confidence: Some(fc.confidence as f32),
                        method: Some(format!("{:?}", fc.method)),
                    }),
                anomaly_detection: intelligence_config.anomaly_detection.map(|ac| {
                    crate::chart::AnomalyConfig {
                        method: ac.method,
                        threshold: ac.threshold as f32,
                        sensitivity: Some(ac.sensitivity as f32),
                    }
                }),
                trend_analysis: Some(intelligence_config.trend_analysis),
                clustering: intelligence_config
                    .clustering
                    .map(|cc| crate::chart::ClusterConfig {
                        k: cc.n_clusters.unwrap_or(3) as u32,
                        method: cc.method,
                        features: vec![],
                    }),
            });
        }

        Ok(spec)
    }

    /// Suggest optimal visualizations based on data characteristics
    pub fn suggest_visualizations(&self, data: &DataFrame) -> Vec<ChartSpec> {
        let mut suggestions = Vec::new();
        let schema = data.schema();

        // Analyze data characteristics
        let analysis = self.analyze_data_characteristics(data);

        // Generate suggestions based on data types and patterns
        if analysis.has_time_series {
            suggestions.push(self.create_time_series_suggestion(&schema));
        }

        if analysis.has_categorical && analysis.has_numerical {
            suggestions.push(self.create_categorical_comparison_suggestion(&schema));
        }

        if analysis.has_multiple_numerical {
            suggestions.push(self.create_scatter_plot_suggestion(&schema));
        }

        if analysis.has_hierarchical {
            suggestions.push(self.create_treemap_suggestion(&schema));
        }

        // Limit suggestions based on configuration
        suggestions.truncate(self.config.max_suggestions);
        suggestions
    }

    /// Explain a chart specification in natural language
    pub fn explain_chart(&self, spec: &ChartSpec) -> String {
        let mut explanation = String::new();

        // Explain chart type
        let chart_type_desc = match spec.mark {
            MarkType::Line { .. } => "line chart",
            MarkType::Bar { .. } => "bar chart",
            MarkType::Point { .. } => "scatter plot",
            MarkType::Area { .. } => "area chart",
            MarkType::Text { .. } => "text chart",
            MarkType::Rect { .. } => "rectangle chart",
            MarkType::Scatter { .. } => "scatter plot",
            _ => "visualization",
        };

        explanation.push_str(&format!("This is a {} that shows", chart_type_desc));

        // Explain encodings
        if let Some(y_field) = &spec.encoding.y {
            explanation.push_str(&format!(" {} values", y_field.field));
        }

        if let Some(x_field) = &spec.encoding.x {
            explanation.push_str(&format!(" over {}", x_field.field));
        }

        if let Some(color_field) = &spec.encoding.color {
            if let Some(field) = &color_field.field {
                explanation.push_str(&format!(", colored by {}", field));
            }
        }

        if let Some(size_field) = &spec.encoding.size {
            if let Some(field) = &size_field.field {
                explanation.push_str(&format!(", with size representing {}", field));
            }
        }

        // Explain intelligence features
        if let Some(intelligence) = &spec.intelligence {
            if intelligence.forecast.is_some() {
                explanation.push_str(", including future predictions");
            }

            if intelligence.anomaly_detection.is_some() {
                explanation.push_str(", with anomaly detection");
            }

            if intelligence.trend_analysis.unwrap_or(false) {
                explanation.push_str(", with trend analysis");
            }
        }

        explanation.push('.');
        explanation
    }

    fn initialize_patterns(&mut self) {
        // Line chart patterns
        self.chart_patterns.push(ChartPattern {
            keywords: vec!["line", "time series", "over time", "trend", "temporal"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            chart_type: MarkType::Line {
                interpolate: None,
                stroke_width: None,
                stroke_dash: None,
            },
            confidence_boost: 0.8,
        });

        // Bar chart patterns
        self.chart_patterns.push(ChartPattern {
            keywords: vec!["bar", "column", "comparison", "category", "categorical"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            chart_type: MarkType::Bar {
                width: None,
                corner_radius: None,
            },
            confidence_boost: 0.8,
        });

        // Scatter plot patterns
        self.chart_patterns.push(ChartPattern {
            keywords: vec![
                "scatter",
                "scatter plot",
                "correlation",
                "relationship",
                "point",
                "dot",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            chart_type: MarkType::Point {
                size: None,
                shape: None,
                opacity: None,
            },
            confidence_boost: 0.8,
        });

        // Area chart patterns
        self.chart_patterns.push(ChartPattern {
            keywords: vec!["area", "filled", "stacked", "cumulative"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            chart_type: MarkType::Area {
                interpolate: None,
                opacity: None,
            },
            confidence_boost: 0.7,
        });

        // Intelligence patterns
        self.intelligence_patterns.push(IntelligencePattern {
            keywords: vec!["forecast", "predict", "future", "projection", "estimate"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            intelligence_type: IntelligenceType::Forecast,
            confidence_boost: 0.9,
        });

        self.intelligence_patterns.push(IntelligencePattern {
            keywords: vec![
                "anomaly",
                "anomalies",
                "outlier",
                "outliers",
                "unusual",
                "abnormal",
                "deviation",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            intelligence_type: IntelligenceType::AnomalyDetection,
            confidence_boost: 0.9,
        });

        self.intelligence_patterns.push(IntelligencePattern {
            keywords: vec!["trend", "pattern", "direction", "slope", "growth"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            intelligence_type: IntelligenceType::TrendAnalysis,
            confidence_boost: 0.8,
        });

        self.intelligence_patterns.push(IntelligencePattern {
            keywords: vec!["cluster", "group", "segment", "category", "classification"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            intelligence_type: IntelligenceType::Clustering,
            confidence_boost: 0.8,
        });
    }

    fn initialize_field_synonyms(&mut self) {
        // Time-related synonyms
        self.field_synonyms.insert(
            "time".to_string(),
            vec![
                "date".to_string(),
                "timestamp".to_string(),
                "when".to_string(),
                "period".to_string(),
                "month".to_string(),
                "year".to_string(),
                "day".to_string(),
                "hour".to_string(),
                "temporal".to_string(),
            ],
        );

        // Value-related synonyms
        self.field_synonyms.insert(
            "value".to_string(),
            vec![
                "amount".to_string(),
                "total".to_string(),
                "count".to_string(),
                "sum".to_string(),
                "revenue".to_string(),
                "sales".to_string(),
                "price".to_string(),
                "cost".to_string(),
                "number".to_string(),
            ],
        );

        // Category-related synonyms
        self.field_synonyms.insert(
            "category".to_string(),
            vec![
                "type".to_string(),
                "group".to_string(),
                "class".to_string(),
                "segment".to_string(),
                "region".to_string(),
                "department".to_string(),
                "product".to_string(),
                "brand".to_string(),
                "label".to_string(),
            ],
        );
    }

    fn normalize_query(&self, query: &str) -> String {
        query
            .to_lowercase()
            .replace("create", "show")
            .replace("make", "show")
            .replace("generate", "show")
            .replace("build", "show")
    }

    fn match_query_patterns(&self, query: &str) -> Result<QueryMatch, NLError> {
        let mut best_match = QueryMatch {
            confidence: 0.0,
            chart_type: MarkType::Point {
                size: None,
                shape: None,
                opacity: None,
            },
            x_field: None,
            y_field: None,
            color_field: None,
            size_field: None,
            intelligence: None,
        };

        // Match chart type patterns
        for pattern in &self.chart_patterns {
            let mut confidence = 0.0;
            let mut matched_keywords = 0;
            for keyword in &pattern.keywords {
                if query.contains(keyword) {
                    matched_keywords += 1;
                }
            }

            if matched_keywords > 0 {
                // Calculate confidence based on percentage of keywords matched
                confidence = (matched_keywords as f64 / pattern.keywords.len() as f64)
                    * pattern.confidence_boost;
            }

            if confidence > best_match.confidence {
                best_match.confidence = confidence;
                best_match.chart_type = pattern.chart_type.clone();
            }
        }

        // Extract field references
        best_match.x_field =
            self.extract_field_reference(query, &["over", "by", "x", "horizontal"]);
        best_match.y_field =
            self.extract_field_reference(query, &["of", "y", "vertical", "values"]);
        best_match.color_field =
            self.extract_field_reference(query, &["colored by", "color", "category"]);
        best_match.size_field = self.extract_field_reference(query, &["sized by", "size"]);

        // Match intelligence patterns
        best_match.intelligence = self.extract_intelligence_config(query)?;

        // Boost confidence if intelligence features are detected
        if best_match.intelligence.is_some() {
            best_match.confidence += 0.1; // Add 0.1 confidence boost for intelligence features
        }

        if best_match.confidence < self.config.confidence_threshold {
            return Err(NLError::ParseError(format!(
                "Could not parse query with sufficient confidence: {} < {}",
                best_match.confidence, self.config.confidence_threshold
            )));
        }

        Ok(best_match)
    }

    fn extract_field_reference(&self, query: &str, indicators: &[&str]) -> Option<String> {
        for indicator in indicators {
            if let Some(pos) = query.find(indicator) {
                let after_indicator = &query[pos + indicator.len()..];
                if let Some(field) = self.extract_next_word(after_indicator) {
                    return Some(field);
                }
            }
        }
        None
    }

    fn extract_next_word(&self, text: &str) -> Option<String> {
        text.trim()
            .split_whitespace()
            .next()
            .map(|word| word.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|word| !word.is_empty())
            .map(|word| word.to_string())
    }

    fn extract_intelligence_config(
        &self,
        query: &str,
    ) -> Result<Option<IntelligenceConfig>, NLError> {
        let mut intelligence = IntelligenceConfig {
            forecast: None,
            anomaly_detection: None,
            trend_analysis: false,
            clustering: None,
        };

        let mut has_intelligence = false;

        // Check for forecasting
        if query.contains("forecast") || query.contains("predict") || query.contains("future") {
            let periods = self.extract_forecast_periods(query).unwrap_or(30);
            intelligence.forecast = Some(ForecastConfig {
                periods,
                confidence: 0.95,
                method: ForecastMethod::Auto,
            });
            has_intelligence = true;
        }

        // Check for anomaly detection
        if query.contains("anomaly")
            || query.contains("anomalies")
            || query.contains("outlier")
            || query.contains("outliers")
            || query.contains("unusual")
        {
            intelligence.anomaly_detection = Some(AnomalyConfig {
                method: "isolation_forest".to_string(),
                threshold: 0.1,
                sensitivity: 0.1,
            });
            has_intelligence = true;
        }

        // Check for trend analysis
        if query.contains("trend") || query.contains("pattern") || query.contains("direction") {
            intelligence.trend_analysis = true;
            has_intelligence = true;
        }

        // Check for clustering
        if query.contains("cluster") || query.contains("group") || query.contains("segment") {
            intelligence.clustering = Some(ClusteringConfig {
                method: "kmeans".to_string(),
                n_clusters: None,
                auto_clusters: true,
            });
            has_intelligence = true;
        }

        if has_intelligence {
            Ok(Some(intelligence))
        } else {
            Ok(None)
        }
    }

    fn extract_forecast_periods(&self, query: &str) -> Option<u32> {
        // Look for patterns like "30 days", "6 months", "12 periods", "6-month", etc.
        let words: Vec<&str> = query.split_whitespace().collect();

        for (i, word) in words.iter().enumerate() {
            // Check for hyphenated patterns like "6-month"
            if word.contains('-') {
                let parts: Vec<&str> = word.split('-').collect();
                if parts.len() == 2 {
                    if let Ok(number) = parts[0].parse::<u32>() {
                        match parts[1] {
                            "day" | "days" => return Some(number),
                            "week" | "weeks" => return Some(number * 7),
                            "month" | "months" => return Some(number),
                            "year" | "years" => return Some(number * 12),
                            "period" | "periods" => return Some(number),
                            _ => continue,
                        }
                    }
                }
            }

            // Check for separate number and unit
            if let Ok(number) = word.parse::<u32>() {
                if i + 1 < words.len() {
                    let unit = words[i + 1];
                    match unit {
                        "day" | "days" => return Some(number),
                        "week" | "weeks" => return Some(number * 7),
                        "month" | "months" => return Some(number),
                        "year" | "years" => return Some(number * 12),
                        "period" | "periods" => return Some(number),
                        _ => continue,
                    }
                }
            }
        }

        None
    }

    fn analyze_data_characteristics(&self, data: &DataFrame) -> DataAnalysis {
        let schema = data.schema();
        let mut analysis = DataAnalysis::default();

        for (name, dtype) in schema.iter() {
            match dtype {
                polars::prelude::DataType::Datetime(_, _) | polars::prelude::DataType::Date => {
                    analysis.has_time_series = true;
                    analysis.time_fields.push(name.to_string());
                }
                polars::prelude::DataType::String
                | polars::prelude::DataType::Categorical(_, _) => {
                    analysis.has_categorical = true;
                    analysis.categorical_fields.push(name.to_string());
                }
                polars::prelude::DataType::Int64
                | polars::prelude::DataType::Float64
                | polars::prelude::DataType::Int32
                | polars::prelude::DataType::Float32 => {
                    analysis.has_numerical = true;
                    analysis.numerical_fields.push(name.to_string());
                }
                _ => {}
            }
        }

        analysis.has_multiple_numerical = analysis.numerical_fields.len() >= 2;
        analysis.has_hierarchical = analysis.categorical_fields.len() >= 2;

        analysis
    }

    fn create_time_series_suggestion(&self, schema: &Schema) -> ChartSpec {
        let mut encoding = Encoding::default();

        // Find time field
        for (name, dtype) in schema.iter() {
            if matches!(
                dtype,
                polars::prelude::DataType::Datetime(_, _) | polars::prelude::DataType::Date
            ) {
                encoding.x = Some(PositionEncoding {
                    field: name.to_string(),
                    data_type: crate::chart::DataType::Temporal,
                    scale: None,
                    axis: None,
                    bin: None,
                    sort: None,
                });
                break;
            }
        }

        // Find first numerical field
        for (name, dtype) in schema.iter() {
            if matches!(
                dtype,
                polars::prelude::DataType::Int64
                    | polars::prelude::DataType::Float64
                    | polars::prelude::DataType::Int32
                    | polars::prelude::DataType::Float32
            ) {
                encoding.y = Some(PositionEncoding {
                    field: name.to_string(),
                    data_type: crate::chart::DataType::Quantitative,
                    scale: None,
                    axis: None,
                    bin: None,
                    sort: None,
                });
                break;
            }
        }

        ChartSpecBuilder::new()
            .mark(MarkType::Line {
                interpolate: None,
                stroke_width: None,
                stroke_dash: None,
            })
            .encoding(encoding)
            .build()
            .unwrap_or_else(|_| ChartSpec::new())
    }

    fn create_categorical_comparison_suggestion(&self, schema: &Schema) -> ChartSpec {
        let mut encoding = Encoding::default();

        // Find categorical field
        for (name, dtype) in schema.iter() {
            if matches!(
                dtype,
                polars::prelude::DataType::String | polars::prelude::DataType::Categorical(_, _)
            ) {
                // For bar charts, categorical data goes on x-axis
                encoding.x = Some(PositionEncoding {
                    field: name.to_string(),
                    data_type: crate::chart::DataType::Nominal,
                    scale: None,
                    axis: None,
                    bin: None,
                    sort: None,
                });
                break;
            }
        }

        // Find numerical field
        for (name, dtype) in schema.iter() {
            if matches!(
                dtype,
                polars::prelude::DataType::Int64
                    | polars::prelude::DataType::Float64
                    | polars::prelude::DataType::Int32
                    | polars::prelude::DataType::Float32
            ) {
                // For bar charts, numerical data goes on y-axis
                encoding.y = Some(PositionEncoding {
                    field: name.to_string(),
                    data_type: crate::chart::DataType::Quantitative,
                    scale: None,
                    axis: None,
                    bin: None,
                    sort: None,
                });
                break;
            }
        }

        ChartSpecBuilder::new()
            .mark(MarkType::Bar {
                width: None,
                corner_radius: None,
            })
            .encoding(encoding)
            .build()
            .unwrap_or_else(|_| ChartSpec::new())
    }

    fn create_scatter_plot_suggestion(&self, schema: &Schema) -> ChartSpec {
        let mut encoding = Encoding::default();

        let numerical_fields: Vec<_> = schema
            .iter()
            .filter(|(_, dtype)| {
                matches!(
                    dtype,
                    polars::prelude::DataType::Int64
                        | polars::prelude::DataType::Float64
                        | polars::prelude::DataType::Int32
                        | polars::prelude::DataType::Float32
                )
            })
            .map(|(name, _)| name.to_string())
            .collect();

        if numerical_fields.len() >= 2 {
            encoding.x = Some(PositionEncoding {
                field: numerical_fields[0].clone(),
                data_type: crate::chart::DataType::Quantitative,
                scale: None,
                axis: None,
                bin: None,
                sort: None,
            });
            encoding.y = Some(PositionEncoding {
                field: numerical_fields[1].clone(),
                data_type: crate::chart::DataType::Quantitative,
                scale: None,
                axis: None,
                bin: None,
                sort: None,
            });
        }

        ChartSpecBuilder::new()
            .mark(MarkType::Point {
                size: None,
                shape: None,
                opacity: None,
            })
            .encoding(encoding)
            .build()
            .unwrap_or_else(|_| ChartSpec::new())
    }

    fn create_treemap_suggestion(&self, schema: &Schema) -> ChartSpec {
        let mut encoding = Encoding::default();

        let categorical_fields: Vec<_> = schema
            .iter()
            .filter(|(_, dtype)| {
                matches!(
                    dtype,
                    polars::prelude::DataType::String
                        | polars::prelude::DataType::Categorical(_, _)
                )
            })
            .map(|(name, _)| name.to_string())
            .collect();

        if let Some(first_cat) = categorical_fields.first() {
            encoding.color = Some(ColorEncoding {
                field: Some(first_cat.clone()),
                data_type: Some(crate::chart::DataType::Nominal),
                scale: None,
                condition: None,
            });
        }

        // Find numerical field for size
        for (name, dtype) in schema.iter() {
            if matches!(
                dtype,
                polars::prelude::DataType::Int64
                    | polars::prelude::DataType::Float64
                    | polars::prelude::DataType::Int32
                    | polars::prelude::DataType::Float32
            ) {
                encoding.size = Some(SizeEncoding {
                    field: Some(name.to_string()),
                    data_type: Some(crate::chart::DataType::Quantitative),
                    scale: None,
                });
                break;
            }
        }

        ChartSpecBuilder::new()
            .mark(MarkType::Rect {
                stroke: None,
                stroke_width: None,
            })
            .encoding(encoding)
            .build()
            .unwrap_or_else(|_| ChartSpec::new())
    }
}

#[derive(Debug, Default)]
struct DataAnalysis {
    has_time_series: bool,
    has_categorical: bool,
    has_numerical: bool,
    has_multiple_numerical: bool,
    has_hierarchical: bool,
    time_fields: Vec<String>,
    categorical_fields: Vec<String>,
    numerical_fields: Vec<String>,
}

impl Default for NLProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::prelude::*;

    #[test]
    fn test_nl_processor_creation() {
        let processor = NLProcessor::new();
        assert_eq!(processor.config.confidence_threshold, 0.3);
        assert!(processor.config.fuzzy_matching);
    }

    #[test]
    fn test_basic_line_chart_parsing() {
        let processor = NLProcessor::new();
        let query = "Show me a line chart of sales over time";

        let result = processor.parse_query(query);
        assert!(result.is_ok());

        let spec = result.unwrap();
        assert!(matches!(spec.mark, MarkType::Line { .. }));
        assert!(spec.encoding.y.is_some());
        assert_eq!(spec.encoding.y.as_ref().unwrap().field, "sales");
        assert!(spec.encoding.x.is_some());
        assert_eq!(spec.encoding.x.as_ref().unwrap().field, "time");
    }

    #[test]
    fn test_bar_chart_parsing() {
        let processor = NLProcessor::new();
        let query = "Create a bar chart comparing revenue by category";

        let result = processor.parse_query(query);
        assert!(result.is_ok());

        let spec = result.unwrap();
        assert!(matches!(spec.mark, MarkType::Bar { .. }));
    }

    #[test]
    fn test_forecast_intelligence_parsing() {
        let processor = NLProcessor::new();
        let query = "Show me a line chart of revenue over time with a 6-month forecast";

        let result = processor.parse_query(query);
        assert!(result.is_ok());

        let spec = result.unwrap();
        assert!(spec.intelligence.is_some());

        let intelligence = spec.intelligence.unwrap();
        assert!(intelligence.forecast.is_some());
        assert_eq!(intelligence.forecast.unwrap().periods, 6);
    }

    #[test]
    fn test_anomaly_detection_parsing() {
        let processor = NLProcessor::new();
        let query = "Show me a scatter plot of user activity and highlight anomalies";

        let result = processor.parse_query(query);
        assert!(result.is_ok());

        let spec = result.unwrap();
        assert!(matches!(spec.mark, MarkType::Point { .. }));
        assert!(spec.intelligence.is_some());

        let intelligence = spec.intelligence.unwrap();
        assert!(intelligence.anomaly_detection.is_some());
    }

    #[test]
    fn test_data_visualization_suggestions() {
        let processor = NLProcessor::new();

        // Create time series data with proper date types
        let dates = vec![
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 3).unwrap(),
        ];
        let values = vec![100.0, 150.0, 120.0];

        let date_series = Series::new("date".into(), &dates);
        let value_series = Series::new("revenue".into(), &values);
        let data = DataFrame::new(vec![date_series.into(), value_series.into()]).unwrap();

        let suggestions = processor.suggest_visualizations(&data);
        assert!(!suggestions.is_empty());

        // First suggestion should be a line chart for time series data
        assert!(matches!(suggestions[0].mark, MarkType::Line { .. }));
    }

    #[test]
    fn test_chart_explanation() {
        let processor = NLProcessor::new();

        let spec = ChartSpecBuilder::new()
            .mark(MarkType::Line {
                interpolate: None,
                stroke_width: None,
                stroke_dash: None,
            })
            .encoding(Encoding {
                x: Some(PositionEncoding {
                    field: "date".to_string(),
                    data_type: crate::chart::DataType::Temporal,
                    scale: None,
                    axis: None,
                    bin: None,
                    sort: None,
                }),
                y: Some(PositionEncoding {
                    field: "sales".to_string(),
                    data_type: crate::chart::DataType::Quantitative,
                    scale: None,
                    axis: None,
                    bin: None,
                    sort: None,
                }),
                color: Some(ColorEncoding {
                    field: Some("region".to_string()),
                    data_type: Some(crate::chart::DataType::Nominal),
                    scale: None,
                    condition: None,
                }),
                ..Default::default()
            })
            .build();

        let explanation = processor.explain_chart(&spec.unwrap());
        assert!(explanation.contains("line chart"));
        assert!(explanation.contains("sales values"));
        assert!(explanation.contains("over date"));
        assert!(explanation.contains("colored by region"));
    }

    #[test]
    fn test_forecast_period_extraction() {
        let processor = NLProcessor::new();

        assert_eq!(
            processor.extract_forecast_periods("predict for 30 days"),
            Some(30)
        );
        assert_eq!(
            processor.extract_forecast_periods("forecast 6 months"),
            Some(6)
        );
        assert_eq!(
            processor.extract_forecast_periods("12 periods ahead"),
            Some(12)
        );
        assert_eq!(processor.extract_forecast_periods("no numbers here"), None);
    }

    #[test]
    fn test_low_confidence_query_rejection() {
        let processor = NLProcessor::new();
        let query = "xyzabc random gibberish query";

        let result = processor.parse_query(query);
        assert!(result.is_err());

        if let Err(NLError::ParseError(msg)) = result {
            assert!(msg.contains("confidence"));
        } else {
            panic!("Expected ParseError");
        }
    }
}
