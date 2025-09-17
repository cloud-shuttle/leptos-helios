//! Performance profiling and optimization for rendering
//!
//! This module provides performance monitoring, frame timing, and adaptive
//! quality management for the rendering system.

use std::time::{Duration, Instant};

/// Performance profile configuration
#[derive(Debug, Clone)]
pub struct PerformanceProfile {
    pub max_points: u32,
    pub target_fps: u32,
    pub memory_efficiency: f32,
    pub compute_shaders: bool,
}

/// Frame timing and performance monitoring
pub struct FrameTimer {
    frame_times: Vec<Duration>,
    last_frame_time: Instant,
    target_frame_time: Duration,
}

impl FrameTimer {
    pub fn new() -> Self {
        Self {
            frame_times: Vec::new(),
            last_frame_time: Instant::now(),
            target_frame_time: Duration::from_millis(16), // 60 FPS
        }
    }

    pub fn start_frame(&mut self) {
        self.last_frame_time = Instant::now();
    }

    pub fn end_frame(&mut self) -> Duration {
        let frame_time = self.last_frame_time.elapsed();
        self.frame_times.push(frame_time);

        // Keep only last 60 frames
        if self.frame_times.len() > 60 {
            self.frame_times.remove(0);
        }

        frame_time
    }

    pub fn get_average_frame_time(&self) -> Duration {
        if self.frame_times.is_empty() {
            return Duration::from_millis(16);
        }

        let total: Duration = self.frame_times.iter().sum();
        total / self.frame_times.len() as u32
    }

    pub fn get_fps(&self) -> f32 {
        let avg_frame_time = self.get_average_frame_time();
        if avg_frame_time.as_secs_f32() > 0.0 {
            1.0 / avg_frame_time.as_secs_f32()
        } else {
            60.0
        }
    }

    pub fn suggest_quality(&self) -> QualityLevel {
        let avg_frame_time = self.get_average_frame_time();
        let target_frame_time = self.target_frame_time;

        if avg_frame_time > target_frame_time * 2 {
            QualityLevel::Low
        } else if avg_frame_time > target_frame_time + (target_frame_time / 2) {
            QualityLevel::Medium
        } else {
            QualityLevel::High
        }
    }
}

/// Quality levels for adaptive rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QualityLevel {
    Low,
    Medium,
    High,
}

/// Adaptive quality management
pub struct AdaptiveQualityManager {
    current_quality: QualityLevel,
    quality_configs: std::collections::HashMap<QualityLevel, QualityConfig>,
}

impl AdaptiveQualityManager {
    pub fn new() -> Self {
        let mut quality_configs = std::collections::HashMap::new();

        quality_configs.insert(
            QualityLevel::Low,
            QualityConfig {
                max_points: 1000,
                antialiasing: false,
                shadows: false,
                reflections: false,
            },
        );

        quality_configs.insert(
            QualityLevel::Medium,
            QualityConfig {
                max_points: 5000,
                antialiasing: true,
                shadows: false,
                reflections: false,
            },
        );

        quality_configs.insert(
            QualityLevel::High,
            QualityConfig {
                max_points: 10000,
                antialiasing: true,
                shadows: true,
                reflections: true,
            },
        );

        Self {
            current_quality: QualityLevel::High,
            quality_configs,
        }
    }

    pub fn get_render_config(&self, quality_level: QualityLevel) -> QualityConfig {
        self.quality_configs
            .get(&quality_level)
            .cloned()
            .unwrap_or_else(|| QualityConfig {
                max_points: 1000,
                antialiasing: false,
                shadows: false,
                reflections: false,
            })
    }

    pub fn set_quality(&mut self, quality: QualityLevel) {
        self.current_quality = quality;
    }

    pub fn get_current_quality(&self) -> QualityLevel {
        self.current_quality
    }
}

/// Quality configuration for rendering
#[derive(Debug, Clone)]
pub struct QualityConfig {
    pub max_points: u32,
    pub antialiasing: bool,
    pub shadows: bool,
    pub reflections: bool,
}

/// Render statistics
#[derive(Debug, Clone)]
pub struct RenderStats {
    pub frame_time: Duration,
    pub triangles_rendered: u32,
    pub draw_calls: u32,
    pub memory_used: usize,
    pub gpu_utilization: f32,
    pub cache_hit_rate: f32,
}

/// Performance budget for rendering
#[derive(Debug, Clone)]
pub struct PerformanceBudget {
    pub max_frame_time: Duration,
    pub max_memory_usage: usize,
    pub target_fps: u32,
}

/// Optimization suggestions
#[derive(Debug, Clone)]
pub enum OptimizationSuggestion {
    ReduceQuality,
    ReducePoints,
    EnableCaching,
    UseComputeShaders,
    OptimizeShaders,
}
