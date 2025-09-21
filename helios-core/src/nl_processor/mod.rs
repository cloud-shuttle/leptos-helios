//! Natural Language Query Processing
//!
//! This module provides natural language to chart specification conversion,
//! enabling users to create visualizations using natural language queries.

mod types;
mod query_parser;
mod intelligence_extractor;
mod data_analyzer;

// Re-export public types
pub use types::*;
use query_parser::QueryParser;
use intelligence_extractor::IntelligenceExtractor;
use data_analyzer::DataAnalyzer;

use crate::chart::{ChartSpec, Intelligence, MarkType, Encoding, EncodingDef, DataType};
use polars::prelude::*;
use std::collections::HashMap;

/// Main natural language processor
pub struct NLProcessor {
    config: NLConfig,
    query_parser: QueryParser,
    intelligence_extractor: IntelligenceExtractor,
    data_analyzer: DataAnalyzer,
}

impl NLProcessor {
    /// Create a new natural language processor
    pub fn new() -> Self {
        Self::with_config(NLConfig::default())
    }

    /// Create a processor with custom configuration
    pub fn with_config(config: NLConfig) -> Self {
        let intelligence_config = IntelligenceConfig {
            forecasting: true,
            anomaly_detection: true,
            clustering: true,
            trend_analysis: true,
        };

        Self {
            config: config.clone(),
            query_parser: QueryParser::new(config),
            intelligence_extractor: IntelligenceExtractor::new(intelligence_config.clone()),
            data_analyzer: DataAnalyzer::new(intelligence_config),
        }
    }

    /// Parse natural language query into chart specification
    pub fn parse_query(&self, query: &str) -> Result<ChartSpec, NLError> {
        let normalized_query = self.query_parser.normalize_query(query);

        // Extract chart type and fields
        let mut query_match = self.query_parser.match_query_patterns(&normalized_query)?;

        // Extract intelligence features
        if let Some(intelligence) = self.intelligence_extractor.extract_intelligence_config(&normalized_query) {
            println!("Found intelligence features: {:?}", intelligence.features);
            query_match.intelligence = Some(intelligence);
        } else {
            println!("No intelligence features found for query: {}", normalized_query);
        }

        // Build chart specification
        let mut spec = ChartSpec::new();
        spec.mark = query_match.chart_type;
        
        // Set up basic encoding based on extracted fields
        if !query_match.fields.is_empty() {
            // Update x encoding with first field
            spec.encoding.x = Some(EncodingDef {
                field: query_match.fields[0].clone(),
                data_type: DataType::String,
                scale: None,
                axis: None,
                legend: None,
                bin: None,
                aggregate: None,
                sort: None,
            });
            
            // Update y encoding with second field if available
            if query_match.fields.len() > 1 {
                spec.encoding.y = Some(EncodingDef {
                    field: query_match.fields[1].clone(),
                    data_type: DataType::Number,
                    scale: None,
                    axis: None,
                    legend: None,
                    bin: None,
                    aggregate: None,
                    sort: None,
                });
            }
        }

        // Add intelligence if detected
        if let Some(intelligence) = query_match.intelligence {
            spec.intelligence = Some(intelligence);
        }

        Ok(spec)
    }

    /// Suggest visualizations based on data characteristics
    pub fn suggest_visualizations(&self, data: &DataFrame) -> Vec<ChartSpec> {
        let analysis = self.data_analyzer.analyze_data_characteristics(data);
        let mut suggestions = Vec::new();

        // Generate suggestions based on data characteristics
        for chart_type in &analysis.suggested_charts {
            let mut spec = ChartSpec::new();
            spec.mark = chart_type.clone();
            
            // Set up basic encoding based on schema
            let schema = data.schema();
            if let Some((name, dtype)) = schema.iter().next() {
                let data_type = match dtype {
                    polars::prelude::DataType::Date | polars::prelude::DataType::Datetime(_, _) => DataType::Date,
                    polars::prelude::DataType::String | polars::prelude::DataType::Categorical(_, _) => DataType::String,
                    _ => DataType::Number,
                };
                spec.encoding.x = Some(EncodingDef {
                    field: name.to_string(),
                    data_type,
                    scale: None,
                    axis: None,
                    legend: None,
                    bin: None,
                    aggregate: None,
                    sort: None,
                });
            }

            suggestions.push(spec);
        }

        suggestions
    }

    /// Explain a chart specification in natural language
    pub fn explain_chart(&self, spec: &ChartSpec) -> String {
        let mut explanation = String::new();

        // Explain chart type
        match &spec.mark {
            MarkType::Line { .. } => explanation.push_str("This is a line chart showing trends over time. "),
            MarkType::Bar { .. } => explanation.push_str("This is a bar chart comparing categories. "),
            MarkType::Point { .. } => explanation.push_str("This is a scatter plot showing relationships between variables. "),
            MarkType::Area { .. } => explanation.push_str("This is an area chart showing cumulative values. "),
            MarkType::Rect { .. } => explanation.push_str("This is a treemap showing hierarchical data. "),
            _ => explanation.push_str("This is a custom chart visualization. "),
        }

        // Explain encoding
        if let Some(x_encoding) = &spec.encoding.x {
            explanation.push_str(&format!("The X-axis represents {}. ", x_encoding.field));
        }

        if let Some(y_encoding) = &spec.encoding.y {
            explanation.push_str(&format!("The Y-axis represents {}. ", y_encoding.field));
        }

        if let Some(color_encoding) = &spec.encoding.color {
            explanation.push_str(&format!("Colors are used to distinguish {}. ", color_encoding.field));
        }

        // Explain intelligence features
        if let Some(intelligence) = &spec.intelligence {
            for feature in &intelligence.features {
                match feature {
                    crate::chart::IntelligenceFeature::Forecasting => {
                        if let Some(forecast) = &intelligence.forecast {
                            if let Some(horizon) = forecast.horizon {
                                explanation.push_str(&format!("This includes forecasting for {} periods. ", horizon));
                            }
                        }
                    }
                    crate::chart::IntelligenceFeature::AnomalyDetection => {
                        explanation.push_str("This includes anomaly detection to identify unusual patterns. ");
                    }
                    crate::chart::IntelligenceFeature::Clustering => {
                        if let Some(clustering) = &intelligence.clustering {
                            if let Some(num_clusters) = clustering.num_clusters {
                                explanation.push_str(&format!("This includes clustering analysis with {} groups. ", num_clusters));
                            }
                        }
                    }
                    crate::chart::IntelligenceFeature::TrendAnalysis => {
                        explanation.push_str("This includes trend analysis to identify patterns over time. ");
                    }
                    _ => {}
                }
            }
        }

        explanation.trim().to_string()
    }
}

impl Default for NLProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nl_processor_creation() {
        let processor = NLProcessor::new();
        assert!(processor.config.fuzzy_matching);
        assert!(processor.config.auto_suggestions);
    }

    #[test]
    fn test_basic_line_chart_parsing() {
        let processor = NLProcessor::new();
        let result = processor.parse_query("show line chart of sales over time");
        
        if let Err(e) = &result {
            println!("Error: {:?}", e);
        }
        assert!(result.is_ok());
        let spec = result.unwrap();
        assert!(matches!(spec.mark, MarkType::Line { .. }));
    }

    #[test]
    fn test_bar_chart_parsing() {
        let processor = NLProcessor::new();
        let result = processor.parse_query("create bar chart comparing revenue by category");
        
        if let Err(e) = &result {
            println!("Error: {:?}", e);
        }
        assert!(result.is_ok());
        let spec = result.unwrap();
        assert!(matches!(spec.mark, MarkType::Bar { .. }));
    }

    #[test]
    fn test_forecast_intelligence_parsing() {
        let processor = NLProcessor::new();
        let result = processor.parse_query("show sales forecast for next 6 months");
        
        assert!(result.is_ok());
        let spec = result.unwrap();
        assert!(spec.intelligence.is_some());
    }

    #[test]
    fn test_anomaly_detection_parsing() {
        let processor = NLProcessor::new();
        let result = processor.parse_query("detect anomalies in temperature data");
        
        assert!(result.is_ok());
        let spec = result.unwrap();
        assert!(spec.intelligence.is_some());
    }

    #[test]
    fn test_data_visualization_suggestions() {
        let processor = NLProcessor::new();
        
        // Create sample data
        let df = df! [
            "date" => ["2023-01-01", "2023-01-02", "2023-01-03"],
            "value" => [100, 120, 110]
        ].unwrap();

        let suggestions = processor.suggest_visualizations(&df);
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_chart_explanation() {
        let processor = NLProcessor::new();
        let mut spec = ChartSpec::new();
        spec.mark = MarkType::Line { interpolate: None, stroke_width: None, stroke_dash: None };
        
        let explanation = processor.explain_chart(&spec);
        assert!(explanation.contains("line chart"));
    }
}
