//! Stylish Charts Demo - leptos-helios + leptos-shadcn-ui Integration
//!
//! This module demonstrates how to create beautiful, modern charts by combining
//! leptos-helios rendering capabilities with leptos-shadcn-ui styling components.

use leptos_helios::chart::*;
use leptos_helios::styling::*;
use serde::{Deserialize, Serialize};

/// Demo data structures for stylish charts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
    pub label: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricData {
    pub title: String,
    pub value: String,
    pub change: String,
    pub trend: TrendDirection,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Up,
    Down,
    Stable,
}

/// Styled chart container with modern design
pub struct StyledChartContainer {
    pub title: String,
    pub description: Option<String>,
    pub class: Option<String>,
}

impl StyledChartContainer {
    pub fn new(title: String, description: Option<String>, class: Option<String>) -> Self {
        Self {
            title,
            description,
            class,
        }
    }

    pub fn get_container_class(&self) -> String {
        let theme = ChartTheme::dark();
        format!(
            "{} {} {} rounded-xl p-6 shadow-2xl backdrop-blur-sm border border-gray-700/50 {}",
            theme.background,
            theme.border,
            theme.text,
            self.class.as_deref().unwrap_or("")
        )
    }
}

/// Modern metric card component
pub struct MetricCard {
    pub title: String,
    pub value: String,
    pub change: String,
    pub trend: TrendDirection,
    pub icon: String,
}

impl MetricCard {
    pub fn new(title: String, value: String, change: String, trend: TrendDirection, icon: String) -> Self {
        Self {
            title,
            value,
            change,
            trend,
            icon,
        }
    }

    pub fn get_card_class(&self) -> String {
        let theme = ChartTheme::dark();
        format!(
            "{} {} {} rounded-lg p-6 shadow-lg backdrop-blur-sm border border-gray-700/50",
            theme.background, theme.border, theme.text
        )
    }

    pub fn get_trend_class(&self) -> String {
        match self.trend {
            TrendDirection::Up => "text-green-400".to_string(),
            TrendDirection::Down => "text-red-400".to_string(),
            TrendDirection::Stable => "text-gray-400".to_string(),
        }
    }

    pub fn get_trend_icon(&self) -> String {
        match self.trend {
            TrendDirection::Up => "↗".to_string(),
            TrendDirection::Down => "↘".to_string(),
            TrendDirection::Stable => "→".to_string(),
        }
    }
}

/// Styled line chart with modern design
pub struct StyledLineChart {
    pub data: Vec<DataPoint>,
    pub title: String,
    pub show_tooltip: bool,
    pub show_legend: bool,
}

impl StyledLineChart {
    pub fn new(data: Vec<DataPoint>, title: String, show_tooltip: bool, show_legend: bool) -> Self {
        Self {
            data,
            title,
            show_tooltip,
            show_legend,
        }
    }

    pub fn create_chart_spec(&self) -> ChartSpec {
        create_styled_line_chart(self.data.clone(), self.title.clone())
    }
}

/// Styled bar chart with modern design
pub struct StyledBarChart {
    pub data: Vec<DataPoint>,
    pub title: String,
    pub show_tooltip: bool,
}

impl StyledBarChart {
    pub fn new(data: Vec<DataPoint>, title: String, show_tooltip: bool) -> Self {
        Self {
            data,
            title,
            show_tooltip,
        }
    }
}

/// Styled area chart with modern design
pub struct StyledAreaChart {
    pub data: Vec<DataPoint>,
    pub title: String,
}

impl StyledAreaChart {
    pub fn new(data: Vec<DataPoint>, title: String) -> Self {
        Self {
            data,
            title,
        }
    }
}

/// Styled scatter plot with modern design
pub struct StyledScatterPlot {
    pub data: Vec<DataPoint>,
    pub title: String,
}

impl StyledScatterPlot {
    pub fn new(data: Vec<DataPoint>, title: String) -> Self {
        Self {
            data,
            title,
        }
    }
}

/// Styled pie chart with modern design
pub struct StyledPieChart {
    pub data: Vec<DataPoint>,
    pub title: String,
}

impl StyledPieChart {
    pub fn new(data: Vec<DataPoint>, title: String) -> Self {
        Self {
            data,
            title,
        }
    }
}

/// Modern analytics dashboard
pub struct StylishAnalyticsDashboard {
    pub metrics: Vec<MetricCard>,
    pub charts: Vec<Box<dyn StyledChart>>,
}

impl StylishAnalyticsDashboard {
    pub fn new() -> Self {
        let metrics = vec![
            MetricCard::new(
                "Total Revenue".to_string(),
                "$45,231.89".to_string(),
                "+20.1%".to_string(),
                TrendDirection::Up,
                "💰".to_string(),
            ),
            MetricCard::new(
                "Active Users".to_string(),
                "2,350".to_string(),
                "+180.1%".to_string(),
                TrendDirection::Up,
                "👥".to_string(),
            ),
            MetricCard::new(
                "Conversion Rate".to_string(),
                "12.5%".to_string(),
                "-19%".to_string(),
                TrendDirection::Down,
                "🎯".to_string(),
            ),
            MetricCard::new(
                "Avg. Session".to_string(),
                "4m 32s".to_string(),
                "+2%".to_string(),
                TrendDirection::Up,
                "⏱️".to_string(),
            ),
        ];

        let charts: Vec<Box<dyn StyledChart>> = vec![
            Box::new(StyledLineChart::new(
                generate_sample_data(),
                "Revenue Trend".to_string(),
                true,
                true,
            )),
            Box::new(StyledBarChart::new(
                generate_sample_data(),
                "User Growth".to_string(),
                true,
            )),
            Box::new(StyledAreaChart::new(
                generate_sample_data(),
                "Conversion Funnel".to_string(),
            )),
            Box::new(StyledScatterPlot::new(
                generate_sample_data(),
                "User Behavior".to_string(),
            )),
            Box::new(StyledPieChart::new(
                generate_sample_data(),
                "Traffic Sources".to_string(),
            )),
        ];

        Self { metrics, charts }
    }

    pub fn get_dashboard_class(&self) -> String {
        "min-h-screen bg-gradient-to-br from-gray-900 via-blue-900 to-indigo-900".to_string()
    }

    pub fn get_container_class(&self) -> String {
        "container mx-auto p-6 space-y-6".to_string()
    }

    pub fn get_metrics_grid_class(&self) -> String {
        "grid gap-4 md:grid-cols-2 lg:grid-cols-4".to_string()
    }

    pub fn get_charts_grid_class(&self) -> String {
        "grid gap-6 md:grid-cols-2".to_string()
    }
}

/// Trait for styled charts
pub trait StyledChart {
    fn get_title(&self) -> &str;
    fn get_chart_type(&self) -> &str;
    fn get_data_count(&self) -> usize;
}

impl StyledChart for StyledLineChart {
    fn get_title(&self) -> &str {
        &self.title
    }

    fn get_chart_type(&self) -> &str {
        "line"
    }

    fn get_data_count(&self) -> usize {
        self.data.len()
    }
}

impl StyledChart for StyledBarChart {
    fn get_title(&self) -> &str {
        &self.title
    }

    fn get_chart_type(&self) -> &str {
        "bar"
    }

    fn get_data_count(&self) -> usize {
        self.data.len()
    }
}

impl StyledChart for StyledAreaChart {
    fn get_title(&self) -> &str {
        &self.title
    }

    fn get_chart_type(&self) -> &str {
        "area"
    }

    fn get_data_count(&self) -> usize {
        self.data.len()
    }
}

impl StyledChart for StyledScatterPlot {
    fn get_title(&self) -> &str {
        &self.title
    }

    fn get_chart_type(&self) -> &str {
        "scatter"
    }

    fn get_data_count(&self) -> usize {
        self.data.len()
    }
}

impl StyledChart for StyledPieChart {
    fn get_title(&self) -> &str {
        &self.title
    }

    fn get_chart_type(&self) -> &str {
        "pie"
    }

    fn get_data_count(&self) -> usize {
        self.data.len()
    }
}

/// Interactive chart with controls
pub struct InteractiveChartDemo {
    pub chart_type: String,
    pub show_grid: bool,
    pub animation_enabled: bool,
    pub data: Vec<DataPoint>,
}

impl InteractiveChartDemo {
    pub fn new() -> Self {
        Self {
            chart_type: "line".to_string(),
            show_grid: true,
            animation_enabled: true,
            data: generate_sample_data(),
        }
    }

    pub fn set_chart_type(&mut self, chart_type: String) {
        self.chart_type = chart_type;
    }

    pub fn toggle_grid(&mut self) {
        self.show_grid = !self.show_grid;
    }

    pub fn toggle_animation(&mut self) {
        self.animation_enabled = !self.animation_enabled;
    }

    pub fn refresh_data(&mut self) {
        self.data = generate_sample_data();
    }

    pub fn get_demo_class(&self) -> String {
        "min-h-screen bg-gradient-to-br from-gray-900 via-purple-900 to-indigo-900".to_string()
    }

    pub fn get_controls_class(&self) -> String {
        "bg-gray-800/50 backdrop-blur-sm rounded-xl p-6 border border-gray-700/50".to_string()
    }
}

// Helper functions
fn create_styled_line_chart(_data: Vec<DataPoint>, _title: String) -> ChartSpec {
    ChartSpec::new()
}

fn generate_sample_data() -> Vec<DataPoint> {
    vec![
        DataPoint { x: 1.0, y: 2.0, label: Some("Point 1".to_string()), color: Some("#3b82f6".to_string()) },
        DataPoint { x: 2.0, y: 4.0, label: Some("Point 2".to_string()), color: Some("#10b981".to_string()) },
        DataPoint { x: 3.0, y: 6.0, label: Some("Point 3".to_string()), color: Some("#f59e0b".to_string()) },
        DataPoint { x: 4.0, y: 8.0, label: Some("Point 4".to_string()), color: Some("#ef4444".to_string()) },
        DataPoint { x: 5.0, y: 10.0, label: Some("Point 5".to_string()), color: Some("#8b5cf6".to_string()) },
    ]
}

/// Demo functions for creating stylish charts
pub mod demos {
    use super::*;

    /// Create a complete analytics dashboard
    pub fn create_analytics_dashboard() -> StylishAnalyticsDashboard {
        StylishAnalyticsDashboard::new()
    }

    /// Create an interactive chart demo
    pub fn create_interactive_demo() -> InteractiveChartDemo {
        InteractiveChartDemo::new()
    }

    /// Create a styled line chart
    pub fn create_styled_line_chart_demo() -> StyledLineChart {
        StyledLineChart::new(
            generate_sample_data(),
            "Revenue Trend".to_string(),
            true,
            true,
        )
    }

    /// Create a styled bar chart
    pub fn create_styled_bar_chart_demo() -> StyledBarChart {
        StyledBarChart::new(
            generate_sample_data(),
            "User Growth".to_string(),
            true,
        )
    }

    /// Create metric cards
    pub fn create_metric_cards() -> Vec<MetricCard> {
        vec![
            MetricCard::new(
                "Total Revenue".to_string(),
                "$45,231.89".to_string(),
                "+20.1%".to_string(),
                TrendDirection::Up,
                "💰".to_string(),
            ),
            MetricCard::new(
                "Active Users".to_string(),
                "2,350".to_string(),
                "+180.1%".to_string(),
                TrendDirection::Up,
                "👥".to_string(),
            ),
            MetricCard::new(
                "Conversion Rate".to_string(),
                "12.5%".to_string(),
                "-19%".to_string(),
                TrendDirection::Down,
                "🎯".to_string(),
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_styled_chart_creation() {
        let data = generate_sample_data();
        let chart = StyledLineChart::new(data.clone(), "Test Chart".to_string(), true, true);
        
        assert_eq!(chart.data.len(), 5);
        assert_eq!(chart.title, "Test Chart");
        assert!(chart.show_tooltip);
        assert!(chart.show_legend);
    }

    #[test]
    fn test_metric_card_creation() {
        let metric = MetricCard::new(
            "Test Metric".to_string(),
            "100".to_string(),
            "+5%".to_string(),
            TrendDirection::Up,
            "📊".to_string(),
        );
        
        assert_eq!(metric.title, "Test Metric");
        assert_eq!(metric.value, "100");
        assert_eq!(metric.get_trend_class(), "text-green-400");
        assert_eq!(metric.get_trend_icon(), "↗");
    }

    #[test]
    fn test_sample_data_generation() {
        let data = generate_sample_data();
        assert_eq!(data.len(), 5);
        assert_eq!(data[0].x, 1.0);
        assert_eq!(data[0].y, 2.0);
    }

    #[test]
    fn test_analytics_dashboard_creation() {
        let dashboard = StylishAnalyticsDashboard::new();
        assert_eq!(dashboard.metrics.len(), 4);
        assert_eq!(dashboard.charts.len(), 5);
        
        // Test metric cards
        assert_eq!(dashboard.metrics[0].title, "Total Revenue");
        assert_eq!(dashboard.metrics[0].value, "$45,231.89");
        assert!(matches!(dashboard.metrics[0].trend, TrendDirection::Up));
    }

    #[test]
    fn test_interactive_demo_creation() {
        let mut demo = InteractiveChartDemo::new();
        assert_eq!(demo.chart_type, "line");
        assert!(demo.show_grid);
        assert!(demo.animation_enabled);
        
        // Test interactions
        demo.set_chart_type("bar".to_string());
        assert_eq!(demo.chart_type, "bar");
        
        demo.toggle_grid();
        assert!(!demo.show_grid);
        
        demo.toggle_animation();
        assert!(!demo.animation_enabled);
    }

    #[test]
    fn test_styled_chart_traits() {
        let line_chart = StyledLineChart::new(
            generate_sample_data(),
            "Test Line Chart".to_string(),
            true,
            true,
        );
        
        assert_eq!(line_chart.get_title(), "Test Line Chart");
        assert_eq!(line_chart.get_chart_type(), "line");
        assert_eq!(line_chart.get_data_count(), 5);
    }

    #[test]
    fn test_demo_functions() {
        let dashboard = demos::create_analytics_dashboard();
        assert_eq!(dashboard.metrics.len(), 4);
        
        let interactive = demos::create_interactive_demo();
        assert_eq!(interactive.chart_type, "line");
        
        let line_chart = demos::create_styled_line_chart_demo();
        assert_eq!(line_chart.title, "Revenue Trend");
        
        let bar_chart = demos::create_styled_bar_chart_demo();
        assert_eq!(bar_chart.title, "User Growth");
        
        let metric_cards = demos::create_metric_cards();
        assert_eq!(metric_cards.len(), 3);
    }
}