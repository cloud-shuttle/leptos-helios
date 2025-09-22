//! Query parsing and normalization logic

use super::types::{NLError, QueryMatch, NLConfig};
use crate::chart::MarkType;
use std::collections::HashMap;

/// Query parser for natural language processing
pub struct QueryParser {
    config: NLConfig,
    chart_patterns: Vec<ChartPattern>,
    field_synonyms: HashMap<String, String>,
}

/// Pattern for matching chart types in queries
#[derive(Debug, Clone)]
struct ChartPattern {
    keywords: Vec<String>,
    chart_type: MarkType,
    confidence_boost: f64,
}

impl QueryParser {
    /// Create a new query parser
    pub fn new(config: NLConfig) -> Self {
        let mut parser = Self {
            config,
            chart_patterns: Vec::new(),
            field_synonyms: HashMap::new(),
        };
        
        parser.initialize_patterns();
        parser.initialize_field_synonyms();
        parser
    }

    /// Normalize a query for processing
    pub fn normalize_query(&self, query: &str) -> String {
        query
            .to_lowercase()
            .trim()
            .replace(['.', ',', '!', '?'], "")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Match query against known patterns
    pub fn match_query_patterns(&self, query: &str) -> Result<QueryMatch, NLError> {
        let mut best_match: Option<QueryMatch> = None;
        let mut highest_confidence = 0.0;

        for pattern in &self.chart_patterns {
            let confidence = self.calculate_pattern_confidence(query, pattern);
            println!("Pattern {:?} confidence: {}", pattern.keywords, confidence); // Added for debugging
            
            if confidence > highest_confidence && confidence >= self.config.confidence_threshold {
                highest_confidence = confidence;
                best_match = Some(QueryMatch {
                    chart_type: pattern.chart_type.clone(),
                    fields: self.extract_field_references(query),
                    confidence,
                    intelligence: None, // Will be set by intelligence extractor
                });
            }
        }

        // If no chart pattern matched, check if it's an intelligence-only query
        if best_match.is_none() && self.is_intelligence_only_query(query) {
            println!("Detected intelligence-only query: {}", query);
            best_match = Some(QueryMatch {
                chart_type: MarkType::Line { interpolate: None, stroke_width: None, stroke_dash: None }, // Default to line chart
                fields: self.extract_field_references(query),
                confidence: 0.5, // Lower confidence for intelligence-only queries
                intelligence: None, // Will be set by intelligence extractor
            });
        }

        best_match.ok_or_else(|| {
            NLError::ParseError(format!(
                "No matching pattern found for query: '{}' (threshold: {})",
                query, self.config.confidence_threshold
            ))
        })
    }

    /// Extract field references from query
    pub fn extract_field_references(&self, query: &str) -> Vec<String> {
        let indicators = ["column", "field", "by", "of", "using", "with"];
        let mut fields = Vec::new();

        for indicator in &indicators {
            if let Some(field) = self.extract_field_reference(query, &[indicator]) {
                fields.push(field);
            }
        }

        fields
    }

    /// Extract the next word after an indicator
    pub fn extract_next_word(&self, text: &str) -> Option<String> {
        let words: Vec<&str> = text.split_whitespace().collect();
        for (i, _word) in words.iter().enumerate() {
            if i + 1 < words.len() {
                return Some(words[i + 1].to_string());
            }
        }
        None
    }

    /// Initialize chart patterns
    fn initialize_patterns(&mut self) {
        self.chart_patterns = vec![
            ChartPattern {
                keywords: vec!["line", "trend", "over time", "time series"].iter().map(|s| s.to_string()).collect(),
                chart_type: MarkType::Line { interpolate: None, stroke_width: None, stroke_dash: None },
                confidence_boost: 0.9,
            },
            ChartPattern {
                keywords: vec!["bar", "column", "compare", "comparison"].iter().map(|s| s.to_string()).collect(),
                chart_type: MarkType::Bar { width: None, corner_radius: None },
                confidence_boost: 0.9,
            },
            ChartPattern {
                keywords: vec!["scatter", "correlation", "relationship"].iter().map(|s| s.to_string()).collect(),
                chart_type: MarkType::Point { size: None, shape: None, opacity: None },
                confidence_boost: 0.8,
            },
            ChartPattern {
                keywords: vec!["pie", "proportion", "percentage", "share"].iter().map(|s| s.to_string()).collect(),
                chart_type: MarkType::Bar { width: None, corner_radius: None },
                confidence_boost: 0.8,
            },
            ChartPattern {
                keywords: vec!["area", "filled", "stacked"].iter().map(|s| s.to_string()).collect(),
                chart_type: MarkType::Area { interpolate: None, opacity: None },
                confidence_boost: 0.7,
            },
        ];
    }

    /// Initialize field synonyms
    fn initialize_field_synonyms(&mut self) {
        self.field_synonyms.insert("date".to_string(), "date".to_string());
        self.field_synonyms.insert("time".to_string(), "date".to_string());
        self.field_synonyms.insert("amount".to_string(), "value".to_string());
        self.field_synonyms.insert("count".to_string(), "value".to_string());
        self.field_synonyms.insert("total".to_string(), "value".to_string());
        self.field_synonyms.insert("sum".to_string(), "value".to_string());
        self.field_synonyms.insert("category".to_string(), "category".to_string());
        self.field_synonyms.insert("type".to_string(), "category".to_string());
        self.field_synonyms.insert("group".to_string(), "category".to_string());
    }

    /// Calculate confidence score for a pattern match
    fn calculate_pattern_confidence(&self, query: &str, pattern: &ChartPattern) -> f64 {
        let mut confidence: f64 = 0.0;
        let query_lower = query.to_lowercase();
        let mut matched_keywords = 0;

        for keyword in &pattern.keywords {
            if query_lower.contains(keyword) {
                matched_keywords += 1;
                confidence += 0.7; // Increased from 0.6 to 0.7
            } else {
                // Check if any word in the query contains this keyword (for cases like "comparing" matching "compare")
                let words: Vec<&str> = query_lower.split_whitespace().collect();
                for word in words {
                    if word.contains(keyword) || word.starts_with(keyword) {
                        matched_keywords += 1;
                        confidence += 0.7; // Increased from 0.6 to 0.7
                        break;
                    }
                }
            }
        }

        // Boost confidence for multiple keyword matches
        if matched_keywords > 1 {
            confidence += 0.3; // Increased from 0.2 to 0.3
        }

        // Boost confidence for exact matches
        if pattern.keywords.iter().any(|kw| query_lower == *kw) {
            confidence += 0.2; // Increased from 0.1 to 0.2
        }

        confidence.min(1.0_f64)
    }

    /// Extract field reference after an indicator
    fn extract_field_reference(&self, query: &str, indicators: &[&str]) -> Option<String> {
        let query_lower = query.to_lowercase();
        
        for indicator in indicators {
            if let Some(pos) = query_lower.find(indicator) {
                let after_indicator = &query[pos + indicator.len()..];
                if let Some(field) = self.extract_next_word(after_indicator) {
                    return Some(field);
                }
            }
        }
        
        None
    }

    /// Check if query contains intelligence keywords but no chart type keywords
    fn is_intelligence_only_query(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        
        // Check for intelligence keywords
        let intelligence_keywords = [
            "forecast", "forecasts", "predict", "predicts", "future", "next", "upcoming", "projection", "projections", "estimate", "estimates", "anticipate", "anticipates",
            "anomaly", "anomalies", "outlier", "outliers", "unusual", "abnormal", "exception", "exceptions", "spike", "spikes", "drop", "drops", "deviation", "deviations", "irregular", "irregulars",
            "cluster", "clusters", "group", "groups", "segment", "segments", "classify", "classifies", "categorize", "categorizes", "similar", "pattern", "patterns", "grouping", "groupings",
            "trend", "trends", "direction", "directions", "slope", "slopes", "increase", "increases", "decrease", "decreases", "growth", "decline", "declines", "change over time"
        ];
        
        let has_intelligence_keywords = intelligence_keywords.iter().any(|keyword| query_lower.contains(keyword));
        println!("Query: '{}', has_intelligence_keywords: {}", query_lower, has_intelligence_keywords);
        
        // Check for chart type keywords
        let chart_keywords = [
            "line", "trend", "over time", "time series",
            "bar", "column", "compare", "comparison", 
            "scatter", "correlation", "relationship",
            "pie", "proportion", "percentage", "share",
            "area", "filled", "stacked"
        ];
        
        let has_chart_keywords = chart_keywords.iter().any(|keyword| query_lower.contains(keyword));
        println!("Query: '{}', has_chart_keywords: {}", query_lower, has_chart_keywords);
        
        let result = has_intelligence_keywords && !has_chart_keywords;
        println!("Query: '{}', is_intelligence_only: {}", query_lower, result);
        result
    }
}
