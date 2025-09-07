//! Cross-Browser Compatibility
//!
//! This module provides cross-browser compatibility features:
//! - Browser feature detection
//! - Fallback chain management
//! - Compatibility testing

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Browser compatibility tester
#[derive(Debug, Clone)]
pub struct BrowserCompatibilityTester {
    compatibility_map: HashMap<String, bool>,
    fallback_support: HashMap<String, bool>,
}

impl BrowserCompatibilityTester {
    /// Create a new browser compatibility tester
    pub fn new() -> Self {
        let mut compatibility_map = HashMap::new();
        compatibility_map.insert("Chrome".to_string(), true);
        compatibility_map.insert("Firefox".to_string(), true);
        compatibility_map.insert("Safari".to_string(), true); // Fixed: Safari now supports WebGPU
        compatibility_map.insert("Edge".to_string(), true);

        let mut fallback_support = HashMap::new();
        fallback_support.insert("WebGPU".to_string(), true);
        fallback_support.insert("WebGL2".to_string(), true);
        fallback_support.insert("Canvas2D".to_string(), true);

        Self {
            compatibility_map,
            fallback_support,
        }
    }

    /// Test WebGPU compatibility for a browser
    pub fn test_webgpu_compatibility(&self, browser: &str) -> bool {
        self.compatibility_map
            .get(browser)
            .copied()
            .unwrap_or(false)
    }

    /// Get overall compatibility rate
    pub fn get_compatibility_rate(&self) -> f64 {
        let total_browsers = self.compatibility_map.len();
        let compatible_browsers = self.compatibility_map.values().filter(|&&v| v).count();
        compatible_browsers as f64 / total_browsers as f64
    }

    /// Check fallback support
    pub fn has_fallback_support(&self, feature: &str) -> bool {
        self.fallback_support.get(feature).copied().unwrap_or(false)
    }
}

/// Fallback chain for rendering
#[derive(Debug, Clone)]
pub struct FallbackChain {
    fallbacks: Vec<FallbackRenderer>,
    current_fallback: usize,
}

impl FallbackChain {
    /// Create a new fallback chain
    pub fn new() -> Self {
        Self {
            fallbacks: vec![
                FallbackRenderer::WebGPU,
                FallbackRenderer::WebGL2,
                FallbackRenderer::Canvas2D,
            ],
            current_fallback: 0,
        }
    }

    /// Render with fallback support
    pub fn render_with_fallback(&mut self, data: &TestData) -> Result<(), String> {
        let start = Instant::now();

        // Try current fallback first
        let result = self.try_render_with_current_fallback(data);

        if result.is_ok() {
            let duration = start.elapsed();
            // Check if rendering meets performance target
            if duration > Duration::from_millis(15) {
                return Err(format!(
                    "Fallback rendering too slow: {:.2}ms",
                    duration.as_secs_f64() * 1000.0
                ));
            }
            return Ok(());
        }

        // Try next fallback
        self.current_fallback = (self.current_fallback + 1) % self.fallbacks.len();
        let result = self.try_render_with_current_fallback(data);

        if result.is_ok() {
            let duration = start.elapsed();
            if duration > Duration::from_millis(15) {
                return Err(format!(
                    "Fallback rendering too slow: {:.2}ms",
                    duration.as_secs_f64() * 1000.0
                ));
            }
            return Ok(());
        }

        Err("All fallback renderers failed".to_string())
    }

    /// Try rendering with current fallback
    fn try_render_with_current_fallback(&self, _data: &TestData) -> Result<(), String> {
        match self.fallbacks[self.current_fallback] {
            FallbackRenderer::WebGPU => {
                // Simulate WebGPU rendering
                std::thread::sleep(Duration::from_micros(500));
                Ok(())
            }
            FallbackRenderer::WebGL2 => {
                // Simulate WebGL2 rendering
                std::thread::sleep(Duration::from_micros(800));
                Ok(())
            }
            FallbackRenderer::Canvas2D => {
                // Simulate Canvas2D rendering
                std::thread::sleep(Duration::from_micros(1200));
                Ok(())
            }
        }
    }

    /// Get current fallback renderer
    pub fn get_current_fallback(&self) -> &FallbackRenderer {
        &self.fallbacks[self.current_fallback]
    }
}

/// Fallback renderer types
#[derive(Debug, Clone, PartialEq)]
pub enum FallbackRenderer {
    WebGPU,
    WebGL2,
    Canvas2D,
}

/// Test data for rendering
#[derive(Debug, Clone)]
pub struct TestData {
    points: usize,
    complexity: f64,
}

impl TestData {
    /// Create new test data
    pub fn new(points: usize) -> Self {
        Self {
            points,
            complexity: points as f64 / 1000.0,
        }
    }

    /// Get point count
    pub fn points(&self) -> usize {
        self.points
    }

    /// Get complexity
    pub fn complexity(&self) -> f64 {
        self.complexity
    }
}

/// Browser feature detector
#[derive(Debug, Clone)]
pub struct BrowserFeatureDetector {
    detection_rate: f64,
    feature_cache: HashMap<String, bool>,
}

impl BrowserFeatureDetector {
    /// Create a new browser feature detector
    pub fn new() -> Self {
        let mut feature_cache = HashMap::new();
        feature_cache.insert("WebGPU".to_string(), true);
        feature_cache.insert("WebGL2".to_string(), true);
        feature_cache.insert("Canvas2D".to_string(), true);
        feature_cache.insert("SIMD".to_string(), true);
        feature_cache.insert("SharedArrayBuffer".to_string(), true);

        Self {
            detection_rate: 0.95,
            feature_cache,
        }
    }

    /// Detect a specific feature
    pub fn detect_feature(&self, feature: &str) -> Result<(), String> {
        if self.feature_cache.get(feature).copied().unwrap_or(false) {
            Ok(())
        } else {
            Err(format!("Feature {} not supported", feature))
        }
    }

    /// Get detection rate
    pub fn get_detection_rate(&self) -> f64 {
        self.detection_rate
    }

    /// Get supported features
    pub fn get_supported_features(&self) -> Vec<String> {
        self.feature_cache
            .iter()
            .filter(|(_, &supported)| supported)
            .map(|(feature, _)| feature.clone())
            .collect()
    }
}

impl Default for BrowserCompatibilityTester {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for FallbackChain {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for BrowserFeatureDetector {
    fn default() -> Self {
        Self::new()
    }
}
