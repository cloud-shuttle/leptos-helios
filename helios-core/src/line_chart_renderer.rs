//! Line Chart Renderer
//!
//! This module provides line chart rendering functionality with data processing,
//! coordinate mapping, and styling.

use crate::chart_config::*;
use crate::webgpu_real::*;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LineChartError {
    #[error("Data processing failed: {0}")]
    DataProcessing(String),

    #[error("Coordinate mapping failed: {0}")]
    CoordinateMapping(String),

    #[error("Rendering failed: {0}")]
    Rendering(String),

    #[error("WebGPU error: {0}")]
    WebGpu(#[from] WebGpuRealError),
}

/// Data point for line charts
#[derive(Debug, Clone)]
pub struct DataPoint {
    pub x: f32,
    pub y: f32,
    pub label: Option<String>,
}

/// Line chart data
#[derive(Debug, Clone)]
pub struct LineChartData {
    pub points: Vec<DataPoint>,
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
}

impl LineChartData {
    /// Create new line chart data from raw points
    pub fn new(points: Vec<(f32, f32)>) -> Self {
        if points.is_empty() {
            return Self {
                points: vec![],
                x_min: 0.0,
                x_max: 0.0,
                y_min: 0.0,
                y_max: 0.0,
            };
        }

        let x_min = points.iter().map(|(x, _)| *x).fold(f32::INFINITY, f32::min);
        let x_max = points
            .iter()
            .map(|(x, _)| *x)
            .fold(f32::NEG_INFINITY, f32::max);
        let y_min = points.iter().map(|(_, y)| *y).fold(f32::INFINITY, f32::min);
        let y_max = points
            .iter()
            .map(|(_, y)| *y)
            .fold(f32::NEG_INFINITY, f32::max);

        let data_points: Vec<DataPoint> = points
            .into_iter()
            .map(|(x, y)| DataPoint { x, y, label: None })
            .collect();

        Self {
            points: data_points,
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    /// Normalize data to 0-1 range
    pub fn normalize(&self) -> Vec<[f32; 2]> {
        if self.points.is_empty() {
            return vec![];
        }

        let x_range = self.x_max - self.x_min;
        let y_range = self.y_max - self.y_min;

        self.points
            .iter()
            .map(|point| {
                let norm_x = if x_range > 0.0 {
                    (point.x - self.x_min) / x_range
                } else {
                    0.5
                };
                let norm_y = if y_range > 0.0 {
                    (point.y - self.y_min) / y_range
                } else {
                    0.5
                };
                [norm_x, norm_y]
            })
            .collect()
    }

    /// Map normalized data to screen coordinates
    pub fn map_to_screen(&self, width: u32, height: u32, margin: ChartMargin) -> Vec<[f32; 2]> {
        let normalized = self.normalize();
        if normalized.is_empty() {
            return vec![];
        }

        let chart_width = width - margin.left - margin.right;
        let chart_height = height - margin.top - margin.bottom;

        normalized
            .into_iter()
            .map(|[norm_x, norm_y]| {
                // Map to screen coordinates (flip Y axis)
                let screen_x = margin.left as f32 + (norm_x * chart_width as f32);
                let screen_y = margin.top as f32 + ((1.0 - norm_y) * chart_height as f32);
                [screen_x, screen_y]
            })
            .collect()
    }

    /// Map to WebGPU normalized coordinates (-1 to 1)
    pub fn map_to_webgpu(&self, width: u32, height: u32, margin: ChartMargin) -> Vec<[f32; 2]> {
        let screen_coords = self.map_to_screen(width, height, margin);
        if screen_coords.is_empty() {
            return vec![];
        }

        screen_coords
            .into_iter()
            .map(|[screen_x, screen_y]| {
                // Convert to WebGPU normalized coordinates
                let webgpu_x = (screen_x / width as f32) * 2.0 - 1.0;
                let webgpu_y = (screen_y / height as f32) * 2.0 - 1.0;
                [webgpu_x, webgpu_y]
            })
            .collect()
    }
}

/// Chart margin configuration
#[derive(Debug, Clone, Copy)]
pub struct ChartMargin {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

impl Default for ChartMargin {
    fn default() -> Self {
        Self {
            top: 20,
            right: 20,
            bottom: 40,
            left: 40,
        }
    }
}

/// Line chart renderer
pub struct LineChartRenderer {
    webgpu_renderer: Option<WebGpuRealRenderer>,
    data_cache: HashMap<String, LineChartData>,
    vertex_cache: HashMap<String, Vec<[f32; 2]>>,
}

impl LineChartRenderer {
    /// Create a new line chart renderer
    pub async fn new(canvas_id: Option<&str>) -> Result<Self, LineChartError> {
        let webgpu_renderer = WebGpuRealRenderer::new(canvas_id).await?;

        Ok(Self {
            webgpu_renderer: Some(webgpu_renderer),
            data_cache: HashMap::new(),
            vertex_cache: HashMap::new(),
        })
    }

    /// Create a new line chart renderer synchronously (for testing/simplified usage)
    pub fn new_sync() -> Result<Self, LineChartError> {
        // For now, create without WebGPU renderer
        Ok(Self {
            webgpu_renderer: None,
            data_cache: HashMap::new(),
            vertex_cache: HashMap::new(),
        })
    }

    /// Process and cache data
    pub fn process_data(
        &mut self,
        data_id: &str,
        raw_data: Vec<(f32, f32)>,
    ) -> Result<(), LineChartError> {
        let chart_data = LineChartData::new(raw_data);
        self.data_cache.insert(data_id.to_string(), chart_data);
        Ok(())
    }

    /// Generate vertices for rendering
    pub fn generate_vertices(
        &mut self,
        data_id: &str,
        config: &LineChartConfig,
    ) -> Result<Vec<[f32; 2]>, LineChartError> {
        let chart_data = self
            .data_cache
            .get(data_id)
            .ok_or_else(|| LineChartError::DataProcessing("Data not found".to_string()))?;

        let margin = ChartMargin::default();
        let vertices = chart_data.map_to_webgpu(config.base.width, config.base.height, margin);

        self.vertex_cache
            .insert(data_id.to_string(), vertices.clone());
        Ok(vertices)
    }

    /// Render a line chart
    pub fn render_line_chart(
        &mut self,
        data_id: &str,
        config: &LineChartConfig,
    ) -> Result<(), LineChartError> {
        // Generate vertices
        let vertices = self.generate_vertices(data_id, config)?;

        if vertices.is_empty() {
            return Ok(());
        }

        // Parse color from hex string
        let color = self.parse_color(&config.color)?;

        // Render using WebGPU if available
        if let Some(ref renderer) = self.webgpu_renderer {
            renderer.render_line_chart(&vertices, color)?;
        }

        Ok(())
    }

    /// Parse hex color string to RGBA array
    fn parse_color(&self, hex_color: &str) -> Result<[f32; 4], LineChartError> {
        let hex = hex_color.trim_start_matches('#');

        if hex.len() != 6 {
            return Err(LineChartError::Rendering(
                "Invalid hex color format".to_string(),
            ));
        }

        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| LineChartError::Rendering("Invalid hex color".to_string()))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| LineChartError::Rendering("Invalid hex color".to_string()))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| LineChartError::Rendering("Invalid hex color".to_string()))?;

        Ok([
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            1.0, // Alpha
        ])
    }

    /// Get cached data
    pub fn get_data(&self, data_id: &str) -> Option<&LineChartData> {
        self.data_cache.get(data_id)
    }

    /// Get cached vertices
    pub fn get_vertices(&self, data_id: &str) -> Option<&Vec<[f32; 2]>> {
        self.vertex_cache.get(data_id)
    }

    /// Clear data cache
    pub fn clear_cache(&mut self) {
        self.data_cache.clear();
        self.vertex_cache.clear();
    }

    /// Check if renderer is ready
    pub fn is_ready(&self) -> bool {
        self.webgpu_renderer
            .as_ref()
            .map(|r| r.is_ready())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_line_chart_data_creation() {
        let raw_data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 3.0), (3.0, 4.0), (4.0, 5.0)];

        let chart_data = LineChartData::new(raw_data);

        assert_eq!(chart_data.points.len(), 5);
        assert_eq!(chart_data.x_min, 0.0);
        assert_eq!(chart_data.x_max, 4.0);
        assert_eq!(chart_data.y_min, 1.0);
        assert_eq!(chart_data.y_max, 5.0);

        println!("✅ Line chart data creation test passed");
    }

    #[tokio::test]
    async fn test_data_normalization() {
        let raw_data = vec![(0.0, 1.0), (2.0, 3.0), (4.0, 5.0)];

        let chart_data = LineChartData::new(raw_data);
        let normalized = chart_data.normalize();

        assert_eq!(normalized.len(), 3);
        assert_eq!(normalized[0], [0.0, 0.0]); // First point
        assert_eq!(normalized[2], [1.0, 1.0]); // Last point

        println!("✅ Data normalization test passed");
    }

    #[tokio::test]
    async fn test_coordinate_mapping() {
        let raw_data = vec![(0.0, 1.0), (2.0, 3.0), (4.0, 5.0)];

        let chart_data = LineChartData::new(raw_data);
        let margin = ChartMargin::default();
        let webgpu_coords = chart_data.map_to_webgpu(800, 600, margin);

        assert_eq!(webgpu_coords.len(), 3);
        // First point should be at left edge
        assert!(webgpu_coords[0][0] < -0.5);
        // Last point should be at right edge
        assert!(webgpu_coords[2][0] > 0.5);

        println!("✅ Coordinate mapping test passed");
    }

    #[tokio::test]
    async fn test_line_chart_renderer() {
        let renderer_result = LineChartRenderer::new(None).await;

        match renderer_result {
            Ok(mut renderer) => {
                // Test data processing
                let raw_data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 3.0)];

                renderer.process_data("test", raw_data).unwrap();

                // Test vertex generation
                let config = LineChartConfig::default();
                let vertices = renderer.generate_vertices("test", &config).unwrap();

                assert_eq!(vertices.len(), 3);

                println!("✅ Line chart renderer test passed");
            }
            Err(LineChartError::WebGpu(WebGpuRealError::NotSupported(_))) => {
                println!("⚠️  WebGPU not supported, skipping test");
                assert!(true);
            }
            Err(e) => {
                println!("❌ Unexpected error: {}", e);
                panic!("Unexpected error: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_color_parsing() {
        let renderer_result = LineChartRenderer::new(None).await;

        match renderer_result {
            Ok(renderer) => {
                // Test valid colors
                let red = renderer.parse_color("#ff0000").unwrap();
                assert_eq!(red, [1.0, 0.0, 0.0, 1.0]);

                let green = renderer.parse_color("#00ff00").unwrap();
                assert_eq!(green, [0.0, 1.0, 0.0, 1.0]);

                let blue = renderer.parse_color("#0000ff").unwrap();
                assert_eq!(blue, [0.0, 0.0, 1.0, 1.0]);

                // Test invalid color
                let invalid = renderer.parse_color("invalid");
                assert!(invalid.is_err());

                println!("✅ Color parsing test passed");
            }
            Err(LineChartError::WebGpu(WebGpuRealError::NotSupported(_))) => {
                println!("⚠️  WebGPU not supported, skipping test");
                assert!(true);
            }
            Err(e) => {
                println!("❌ Unexpected error: {}", e);
                panic!("Unexpected error: {}", e);
            }
        }
    }
}
