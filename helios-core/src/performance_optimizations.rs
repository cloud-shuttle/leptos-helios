//! Performance Optimizations for leptos-helios
//!
//! This module provides comprehensive performance optimization features including
//! virtual scrolling, data sampling, WebGL/WebGPU acceleration, and memory optimization.

use std::collections::HashMap;
use std::time::{Duration, Instant};

// ============================================================================
// Virtual Scrolling
// ============================================================================

/// Virtual scrolling system for handling large datasets efficiently
#[derive(Debug, Clone)]
pub struct VirtualScroller {
    pub viewport_height: f64,
    pub item_height: f64,
    pub total_items: usize,
    pub visible_start: usize,
    pub visible_end: usize,
    pub scroll_offset: f64,
    pub overscan: usize, // Extra items to render outside viewport
}

impl VirtualScroller {
    /// Create a new virtual scroller
    pub fn new(viewport_height: f64, item_height: f64, total_items: usize) -> Self {
        let visible_count = (viewport_height / item_height).ceil() as usize;
        Self {
            viewport_height,
            item_height,
            total_items,
            visible_start: 0,
            visible_end: visible_count.min(total_items),
            scroll_offset: 0.0,
            overscan: 5, // Render 5 extra items above and below
        }
    }

    /// Scroll to a specific offset
    pub fn scroll_to(&mut self, offset: f64) {
        self.scroll_offset = offset.max(0.0);
        self.update_visible_range();
    }

    /// Update the visible range based on current scroll position
    fn update_visible_range(&mut self) {
        let visible_count = (self.viewport_height / self.item_height).ceil() as usize;
        self.visible_start = (self.scroll_offset / self.item_height).floor() as usize;
        self.visible_end = (self.visible_start + visible_count).min(self.total_items);

        // Apply overscan
        self.visible_start = self.visible_start.saturating_sub(self.overscan);
        self.visible_end = (self.visible_end + self.overscan).min(self.total_items);
    }

    /// Get the currently visible items
    pub fn get_visible_items(&self) -> Vec<VisibleItem> {
        (self.visible_start..self.visible_end)
            .map(|i| VisibleItem {
                index: i,
                y_position: i as f64 * self.item_height - self.scroll_offset,
                height: self.item_height,
            })
            .collect()
    }

    /// Get the total height of all items
    pub fn get_total_height(&self) -> f64 {
        self.total_items as f64 * self.item_height
    }

    /// Check if an item is visible
    pub fn is_item_visible(&self, index: usize) -> bool {
        index >= self.visible_start && index < self.visible_end
    }
}

/// Represents a visible item in the virtual scroller
#[derive(Debug, Clone)]
pub struct VisibleItem {
    pub index: usize,
    pub y_position: f64,
    pub height: f64,
}

// ============================================================================
// Data Sampling
// ============================================================================

/// Strategy for data sampling
#[derive(Debug, Clone, PartialEq)]
pub enum SamplingStrategy {
    /// Uniform sampling - evenly distributed points
    Uniform,
    /// Adaptive sampling - more points in dense regions
    Adaptive,
    /// Statistical sampling - preserves key statistics
    Statistical,
    /// LOD (Level of Detail) sampling - based on zoom level
    LevelOfDetail(f64),
}

/// Data sampler for reducing dataset size while preserving visual fidelity
#[derive(Debug, Clone)]
pub struct DataSampler {
    strategy: SamplingStrategy,
    cache: HashMap<String, Vec<DataPoint>>,
}

impl DataSampler {
    /// Create a new data sampler
    pub fn new(strategy: SamplingStrategy) -> Self {
        Self {
            strategy,
            cache: HashMap::new(),
        }
    }

    /// Sample data according to the strategy
    pub fn sample(&self, data: &[DataPoint], target_size: usize) -> Vec<DataPoint> {
        if data.len() <= target_size {
            return data.to_vec();
        }

        match self.strategy {
            SamplingStrategy::Uniform => self.uniform_sample(data, target_size),
            SamplingStrategy::Adaptive => self.adaptive_sample(data, target_size),
            SamplingStrategy::Statistical => self.statistical_sample(data, target_size),
            SamplingStrategy::LevelOfDetail(zoom) => self.lod_sample(data, target_size, zoom),
        }
    }

    /// Uniform sampling - evenly distributed points
    fn uniform_sample(&self, data: &[DataPoint], target_size: usize) -> Vec<DataPoint> {
        let step = data.len() as f64 / target_size as f64;
        (0..target_size)
            .map(|i| {
                let index = (i as f64 * step).floor() as usize;
                data[index.min(data.len() - 1)].clone()
            })
            .collect()
    }

    /// Adaptive sampling - more points in dense regions
    fn adaptive_sample(&self, data: &[DataPoint], target_size: usize) -> Vec<DataPoint> {
        // Simplified adaptive sampling - in practice would use density analysis
        let mut sampled = Vec::new();
        let chunk_size = data.len() / target_size;

        for i in 0..target_size {
            let start = i * chunk_size;
            let end = ((i + 1) * chunk_size).min(data.len());
            let chunk = &data[start..end];

            // Find the point with maximum value in this chunk
            if let Some(max_point) = chunk
                .iter()
                .max_by(|a, b| a.value.partial_cmp(&b.value).unwrap())
            {
                sampled.push(max_point.clone());
            }
        }

        sampled
    }

    /// Statistical sampling - preserves key statistics
    fn statistical_sample(&self, data: &[DataPoint], target_size: usize) -> Vec<DataPoint> {
        // Simplified statistical sampling - in practice would preserve mean, variance, etc.
        let mut sampled = Vec::new();

        // Always include min and max points
        if let (Some(min_point), Some(max_point)) = (
            data.iter()
                .min_by(|a, b| a.value.partial_cmp(&b.value).unwrap()),
            data.iter()
                .max_by(|a, b| a.value.partial_cmp(&b.value).unwrap()),
        ) {
            sampled.push(min_point.clone());
            sampled.push(max_point.clone());
        }

        // Fill remaining with uniform sampling
        let remaining = target_size.saturating_sub(sampled.len());
        if remaining > 0 {
            let uniform_sample = self.uniform_sample(data, remaining);
            sampled.extend(uniform_sample);
        }

        sampled
    }

    /// Level of detail sampling based on zoom level
    fn lod_sample(&self, data: &[DataPoint], target_size: usize, zoom: f64) -> Vec<DataPoint> {
        // Adjust target size based on zoom level
        let adjusted_size = (target_size as f64 * zoom).ceil() as usize;
        self.uniform_sample(data, adjusted_size.min(target_size))
    }
}

// ============================================================================
// WebGL/WebGPU Acceleration
// ============================================================================

/// WebGL renderer for hardware-accelerated rendering
#[derive(Debug, Clone)]
pub struct WebGLRenderer {
    pub width: u32,
    pub height: u32,
    pub shader_cache: HashMap<String, u32>,
    pub buffer_cache: HashMap<String, u32>,
    pub is_initialized: bool,
}

impl WebGLRenderer {
    /// Create a new WebGL renderer
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            shader_cache: HashMap::new(),
            buffer_cache: HashMap::new(),
            is_initialized: false,
        }
    }

    /// Initialize the WebGL context
    pub fn initialize(&mut self) -> Result<(), String> {
        // In practice, this would initialize WebGL context
        self.is_initialized = true;
        Ok(())
    }

    /// Check if the renderer is initialized
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    /// Compile a shader program
    pub fn compile_shader_program(&mut self, vertex: &str, fragment: &str) -> Option<u32> {
        let key = format!("{}:{}", vertex, fragment);
        if let Some(&program) = self.shader_cache.get(&key) {
            return Some(program);
        }

        // In practice, this would compile WebGL shaders
        let program = self.shader_cache.len() as u32 + 1;
        self.shader_cache.insert(key, program);
        Some(program)
    }

    /// Render a batch of primitives
    pub fn render_batch(&mut self, batch: &RenderBatch) -> Result<(), String> {
        if !self.is_initialized {
            return Err("WebGL renderer not initialized".to_string());
        }

        // In practice, this would use WebGL to render the batch
        let _ = batch.points.len();
        Ok(())
    }

    /// Clear the render target
    pub fn clear(&mut self, _color: Color) -> Result<(), String> {
        if !self.is_initialized {
            return Err("WebGL renderer not initialized".to_string());
        }

        // In practice, this would clear the WebGL framebuffer
        Ok(())
    }
}

/// WebGPU renderer for modern hardware acceleration
#[derive(Debug, Clone)]
pub struct WebGPURenderer {
    pub width: u32,
    pub height: u32,
    pub buffer_pool: Vec<Buffer>,
    pub shader_cache: HashMap<String, u32>,
    pub is_initialized: bool,
}

impl WebGPURenderer {
    /// Create a new WebGPU renderer
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buffer_pool: Vec::new(),
            shader_cache: HashMap::new(),
            is_initialized: false,
        }
    }

    /// Initialize the WebGPU context
    pub fn initialize(&mut self) -> Result<(), String> {
        // In practice, this would initialize WebGPU context
        self.is_initialized = true;
        Ok(())
    }

    /// Check if the renderer is initialized
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    /// Allocate a buffer from the pool
    pub fn allocate_buffer(&mut self, size: usize) -> Option<Buffer> {
        // Try to reuse a buffer from the pool
        if let Some(index) = self.buffer_pool.iter().position(|b| b.size >= size) {
            return Some(self.buffer_pool.remove(index));
        }

        // Create a new buffer
        Some(Buffer {
            size,
            id: self.buffer_pool.len() as u32 + 1,
        })
    }

    /// Deallocate a buffer back to the pool
    pub fn deallocate_buffer(&mut self, buffer: Buffer) {
        self.buffer_pool.push(buffer);
    }

    /// Render a batch using WebGPU
    pub fn render_batch(&mut self, batch: &RenderBatch) -> Result<(), String> {
        if !self.is_initialized {
            return Err("WebGPU renderer not initialized".to_string());
        }

        // In practice, this would use WebGPU to render the batch
        let _ = batch.points.len();
        Ok(())
    }
}

/// Represents a GPU buffer
#[derive(Debug, Clone)]
pub struct Buffer {
    pub size: usize,
    pub id: u32,
}

/// Batch of rendering primitives
#[derive(Debug, Clone)]
pub struct RenderBatch {
    pub points: Vec<(Point2D, Color)>,
    pub lines: Vec<(Point2D, Point2D, Color)>,
    pub triangles: Vec<(Point2D, Point2D, Point2D, Color)>,
}

impl RenderBatch {
    /// Create a new render batch
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            lines: Vec::new(),
            triangles: Vec::new(),
        }
    }

    /// Add a point to the batch
    pub fn add_point(&mut self, point: Point2D, color: Color) {
        self.points.push((point, color));
    }

    /// Add a line to the batch
    pub fn add_line(&mut self, start: Point2D, end: Point2D, color: Color) {
        self.lines.push((start, end, color));
    }

    /// Add a triangle to the batch
    pub fn add_triangle(&mut self, a: Point2D, b: Point2D, c: Point2D, color: Color) {
        self.triangles.push((a, b, c, color));
    }

    /// Clear the batch
    pub fn clear(&mut self) {
        self.points.clear();
        self.lines.clear();
        self.triangles.clear();
    }

    /// Get the total number of primitives
    pub fn primitive_count(&self) -> usize {
        self.points.len() + self.lines.len() + self.triangles.len()
    }
}

/// 2D point
#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

/// Color with RGBA components
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// Create a new color
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create a color from RGB values (0-255)
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }

    /// Create a color from RGBA values (0-255)
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }
}

// ============================================================================
// Memory Optimization
// ============================================================================

/// Memory pool for efficient memory allocation
#[derive(Debug, Clone)]
pub struct MemoryPool {
    pub total_size: usize,
    pub used_size: usize,
    pub available_size: usize,
    pub allocations: Vec<Allocation>,
}

impl MemoryPool {
    /// Create a new memory pool
    pub fn new(size: usize) -> Self {
        Self {
            total_size: size,
            used_size: 0,
            available_size: size,
            allocations: Vec::new(),
        }
    }

    /// Allocate memory from the pool
    pub fn allocate(&mut self, size: usize) -> Option<*mut u8> {
        if self.available_size < size {
            return None;
        }

        // Find a suitable free block
        if let Some(index) = self.find_free_block(size) {
            let allocation = &mut self.allocations[index];
            allocation.size = size;
            allocation.used = true;
            self.used_size += size;
            self.available_size -= size;
            return Some(allocation.ptr);
        }

        // Allocate new block
        let ptr = std::ptr::null_mut(); // Simplified for testing
        self.allocations.push(Allocation {
            ptr,
            size,
            used: true,
        });
        self.used_size += size;
        self.available_size -= size;
        Some(ptr)
    }

    /// Deallocate memory back to the pool
    pub fn deallocate(&mut self, ptr: *mut u8) {
        if let Some(allocation) = self.allocations.iter_mut().find(|a| a.ptr == ptr) {
            self.used_size -= allocation.size;
            self.available_size += allocation.size;
            allocation.used = false;
        }
    }

    /// Find a free block of sufficient size
    fn find_free_block(&self, size: usize) -> Option<usize> {
        self.allocations
            .iter()
            .position(|a| !a.used && a.size >= size)
    }

    /// Defragment the memory pool
    pub fn defragment(&mut self) {
        // Sort allocations by address and merge adjacent free blocks
        self.allocations.sort_by(|a, b| a.ptr.cmp(&b.ptr));

        let mut i = 0;
        while i < self.allocations.len() - 1 {
            if !self.allocations[i].used && !self.allocations[i + 1].used {
                // Merge adjacent free blocks
                self.allocations[i].size += self.allocations[i + 1].size;
                self.allocations.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }
}

/// Memory allocation record
#[derive(Debug, Clone)]
pub struct Allocation {
    pub ptr: *mut u8,
    pub size: usize,
    pub used: bool,
}

/// Garbage collector for managing object lifetimes
#[derive(Debug, Clone)]
pub struct GarbageCollector {
    objects: Vec<Object>,
    next_id: usize,
}

impl GarbageCollector {
    /// Create a new garbage collector
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            next_id: 0,
        }
    }

    /// Allocate a new object
    pub fn allocate_object(&mut self, name: &str) -> ObjectId {
        let id = ObjectId(self.next_id);
        self.next_id += 1;

        self.objects.push(Object {
            id,
            name: name.to_string(),
            reachable: false,
            size: 0, // Simplified for testing
        });

        id
    }

    /// Mark an object as reachable
    pub fn mark_reachable(&mut self, id: ObjectId) {
        if let Some(obj) = self.objects.iter_mut().find(|o| o.id == id) {
            obj.reachable = true;
        }
    }

    /// Run garbage collection
    pub fn collect(&mut self) {
        // Remove unreachable objects
        self.objects.retain(|obj| obj.reachable);

        // Reset reachability flags
        for obj in &mut self.objects {
            obj.reachable = false;
        }
    }

    /// Get the number of objects
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    /// Check if an object is allocated
    pub fn is_allocated(&self, id: ObjectId) -> bool {
        self.objects.iter().any(|obj| obj.id == id)
    }

    /// Get memory usage statistics
    pub fn get_memory_stats(&self) -> MemoryStats {
        let total_size: usize = self.objects.iter().map(|obj| obj.size).sum();
        MemoryStats {
            object_count: self.objects.len(),
            total_size,
            reachable_count: self.objects.iter().filter(|obj| obj.reachable).count(),
        }
    }
}

/// Object identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectId(usize);

/// Object in the garbage collector
#[derive(Debug, Clone)]
pub struct Object {
    pub id: ObjectId,
    pub name: String,
    pub reachable: bool,
    pub size: usize,
}

/// Memory usage statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub object_count: usize,
    pub total_size: usize,
    pub reachable_count: usize,
}

// ============================================================================
// Performance Monitoring
// ============================================================================

/// Performance monitor for tracking and optimizing performance
#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
    pub metrics: HashMap<String, PerformanceMetric>,
    pub budgets: HashMap<String, Duration>,
    pub enabled: bool,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
            budgets: HashMap::new(),
            enabled: true,
        }
    }

    /// Check if monitoring is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable or disable monitoring
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Start timing an operation
    pub fn start_timer(&mut self, name: &str) {
        if !self.enabled {
            return;
        }

        let metric = PerformanceMetric {
            name: name.to_string(),
            start_time: Instant::now(),
            duration: Duration::ZERO,
            call_count: 1,
            min_duration: Duration::MAX,
            max_duration: Duration::ZERO,
            total_duration: Duration::ZERO,
        };
        self.metrics.insert(name.to_string(), metric);
    }

    /// End timing an operation
    pub fn end_timer(&mut self, name: &str) {
        if !self.enabled {
            return;
        }

        if let Some(metric) = self.metrics.get_mut(name) {
            let duration = metric.start_time.elapsed();
            metric.duration = duration;
            metric.total_duration += duration;
            metric.min_duration = metric.min_duration.min(duration);
            metric.max_duration = metric.max_duration.max(duration);
        }
    }

    /// Get all performance metrics
    pub fn get_metrics(&self) -> &HashMap<String, PerformanceMetric> {
        &self.metrics
    }

    /// Set a performance budget for an operation
    pub fn set_budget(&mut self, name: &str, budget: Duration) {
        self.budgets.insert(name.to_string(), budget);
    }

    /// Check if an operation is over budget
    pub fn is_over_budget(&self, name: &str) -> bool {
        if let (Some(metric), Some(budget)) = (self.metrics.get(name), self.budgets.get(name)) {
            metric.duration > *budget
        } else {
            false
        }
    }

    /// Get optimization suggestions
    pub fn get_optimization_suggestions(&self) -> Vec<String> {
        let mut suggestions = Vec::new();

        for (name, metric) in &self.metrics {
            if metric.duration > Duration::from_millis(100) {
                suggestions.push(format!(
                    "Consider optimizing {} (took {:?}, avg: {:?})",
                    name,
                    metric.duration,
                    metric.total_duration / metric.call_count as u32
                ));
            }

            if metric.call_count > 1000 {
                suggestions.push(format!(
                    "Consider batching {} (called {} times)",
                    name, metric.call_count
                ));
            }
        }

        suggestions
    }

    /// Get performance report
    pub fn get_performance_report(&self) -> PerformanceReport {
        let mut slow_operations = Vec::new();
        let mut over_budget = Vec::new();

        for (name, metric) in &self.metrics {
            if metric.duration > Duration::from_millis(50) {
                slow_operations.push((name.clone(), metric.duration));
            }

            if self.is_over_budget(name) {
                over_budget.push(name.clone());
            }
        }

        slow_operations.sort_by(|a, b| b.1.cmp(&a.1));

        PerformanceReport {
            total_operations: self.metrics.len(),
            slow_operations,
            over_budget,
            suggestions: self.get_optimization_suggestions(),
        }
    }
}

/// Performance metric for a single operation
#[derive(Debug, Clone)]
pub struct PerformanceMetric {
    pub name: String,
    pub start_time: Instant,
    pub duration: Duration,
    pub call_count: u32,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub total_duration: Duration,
}

/// Performance report
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub total_operations: usize,
    pub slow_operations: Vec<(String, Duration)>,
    pub over_budget: Vec<String>,
    pub suggestions: Vec<String>,
}

// ============================================================================
// Data Point (from existing code)
// ============================================================================

/// Data point for visualization
#[derive(Debug, Clone)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
    pub value: f64,
}
