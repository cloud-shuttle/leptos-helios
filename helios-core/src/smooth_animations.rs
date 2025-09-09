//! Smooth Animations and Transitions for leptos-helios
//!
//! This module provides comprehensive animation features including
//! tweening, easing functions, state transitions, and animation orchestration.

use std::collections::HashMap;
use std::time::{Duration, Instant};

// ============================================================================
// Easing Functions
// ============================================================================

/// Comprehensive easing function library for smooth animations
#[derive(Debug, Clone)]
pub struct EasingFunctions {
    pub initialized: bool,
    pub cache: HashMap<String, f64>,
}

impl EasingFunctions {
    /// Create a new easing functions instance
    pub fn new() -> Self {
        Self {
            initialized: true,
            cache: HashMap::new(),
        }
    }

    /// Check if the easing system is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get list of available easing functions
    pub fn get_available_functions(&self) -> Vec<String> {
        vec![
            "linear".to_string(),
            "ease_in_cubic".to_string(),
            "ease_out_cubic".to_string(),
            "ease_in_out_cubic".to_string(),
            "ease_out_elastic".to_string(),
            "ease_in_quad".to_string(),
            "ease_out_quad".to_string(),
            "ease_in_out_quad".to_string(),
            "ease_in_back".to_string(),
            "ease_out_back".to_string(),
            "ease_in_out_back".to_string(),
            "ease_in_bounce".to_string(),
            "ease_out_bounce".to_string(),
            "ease_in_out_bounce".to_string(),
        ]
    }

    /// Linear easing (no easing)
    pub fn linear(&self, t: f64) -> f64 {
        t
    }

    /// Cubic ease in
    pub fn ease_in_cubic(&self, t: f64) -> f64 {
        t * t * t
    }

    /// Cubic ease out
    pub fn ease_out_cubic(&self, t: f64) -> f64 {
        let t = t - 1.0;
        1.0 + t * t * t
    }

    /// Cubic ease in-out
    pub fn ease_in_out_cubic(&self, t: f64) -> f64 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            let t = 2.0 * t - 2.0;
            1.0 + t * t * t / 2.0
        }
    }

    /// Quadratic ease in
    pub fn ease_in_quad(&self, t: f64) -> f64 {
        t * t
    }

    /// Quadratic ease out
    pub fn ease_out_quad(&self, t: f64) -> f64 {
        1.0 - (1.0 - t) * (1.0 - t)
    }

    /// Quadratic ease in-out
    pub fn ease_in_out_quad(&self, t: f64) -> f64 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            1.0 - 2.0 * (1.0 - t) * (1.0 - t)
        }
    }

    /// Elastic ease out with overshoot
    pub fn ease_out_elastic(&self, t: f64) -> f64 {
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else {
            let c4 = (2.0 * std::f64::consts::PI) / 3.0;
            2.0_f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
        }
    }

    /// Elastic ease in
    pub fn ease_in_elastic(&self, t: f64) -> f64 {
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else {
            let c4 = (2.0 * std::f64::consts::PI) / 3.0;
            -2.0_f64.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * c4).sin()
        }
    }

    /// Back ease in with overshoot
    pub fn ease_in_back(&self, t: f64) -> f64 {
        const C1: f64 = 1.70158;
        const C3: f64 = C1 + 1.0;
        C3 * t * t * t - C1 * t * t
    }

    /// Back ease out with overshoot
    pub fn ease_out_back(&self, t: f64) -> f64 {
        const C1: f64 = 1.70158;
        const C3: f64 = C1 + 1.0;
        1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0).powi(2)
    }

    /// Back ease in-out
    pub fn ease_in_out_back(&self, t: f64) -> f64 {
        const C1: f64 = 1.70158;
        const C2: f64 = C1 * 1.525;

        if t < 0.5 {
            (2.0 * t).powi(2) * ((C2 + 1.0) * 2.0 * t - C2) / 2.0
        } else {
            ((2.0 * t - 2.0).powi(2) * ((C2 + 1.0) * (2.0 * t - 2.0) + C2) + 2.0) / 2.0
        }
    }

    /// Bounce ease in
    pub fn ease_in_bounce(&self, t: f64) -> f64 {
        1.0 - self.ease_out_bounce(1.0 - t)
    }

    /// Bounce ease out
    pub fn ease_out_bounce(&self, t: f64) -> f64 {
        const N1: f64 = 7.5625;
        const D1: f64 = 2.75;

        if t < 1.0 / D1 {
            N1 * t * t
        } else if t < 2.0 / D1 {
            let t = t - 1.5 / D1;
            N1 * t * t + 0.75
        } else if t < 2.5 / D1 {
            let t = t - 2.25 / D1;
            N1 * t * t + 0.9375
        } else {
            let t = t - 2.625 / D1;
            N1 * t * t + 0.984375
        }
    }

    /// Bounce ease in-out
    pub fn ease_in_out_bounce(&self, t: f64) -> f64 {
        if t < 0.5 {
            (1.0 - self.ease_out_bounce(1.0 - 2.0 * t)) / 2.0
        } else {
            (1.0 + self.ease_out_bounce(2.0 * t - 1.0)) / 2.0
        }
    }

    /// Apply easing function by name
    pub fn apply_easing(&self, name: &str, t: f64) -> f64 {
        match name {
            "linear" => self.linear(t),
            "ease_in_cubic" => self.ease_in_cubic(t),
            "ease_out_cubic" => self.ease_out_cubic(t),
            "ease_in_out_cubic" => self.ease_in_out_cubic(t),
            "ease_in_quad" => self.ease_in_quad(t),
            "ease_out_quad" => self.ease_out_quad(t),
            "ease_in_out_quad" => self.ease_in_out_quad(t),
            "ease_out_elastic" => self.ease_out_elastic(t),
            "ease_in_elastic" => self.ease_in_elastic(t),
            "ease_in_back" => self.ease_in_back(t),
            "ease_out_back" => self.ease_out_back(t),
            "ease_in_out_back" => self.ease_in_out_back(t),
            "ease_in_bounce" => self.ease_in_bounce(t),
            "ease_out_bounce" => self.ease_out_bounce(t),
            "ease_in_out_bounce" => self.ease_in_out_bounce(t),
            _ => self.linear(t),
        }
    }
}

// ============================================================================
// Tween Animation
// ============================================================================

/// Tween animation for smooth value transitions
#[derive(Debug, Clone)]
pub struct TweenAnimation {
    pub start_value: f64,
    pub end_value: f64,
    pub duration: Duration,
    pub current_value: f64,
    pub elapsed: Duration,
    pub easing: EasingType,
    pub delay: Duration,
    pub repeat_count: i32,
    pub repeat_delay: Duration,
    pub yoyo: bool,
    pub is_paused: bool,
}

/// Available easing types
#[derive(Debug, Clone, PartialEq)]
pub enum EasingType {
    Linear,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseOutElastic,
    EaseInElastic,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,
    Custom(String),
}

impl TweenAnimation {
    /// Create a new tween animation
    pub fn new(start: f64, end: f64, duration: Duration) -> Self {
        Self {
            start_value: start,
            end_value: end,
            duration,
            current_value: start,
            elapsed: Duration::ZERO,
            easing: EasingType::Linear,
            delay: Duration::ZERO,
            repeat_count: 0,
            repeat_delay: Duration::ZERO,
            yoyo: false,
            is_paused: false,
        }
    }

    /// Create a tween animation with specific easing
    pub fn with_easing(start: f64, end: f64, duration: Duration, easing: EasingType) -> Self {
        Self {
            start_value: start,
            end_value: end,
            duration,
            current_value: start,
            elapsed: Duration::ZERO,
            easing,
            delay: Duration::ZERO,
            repeat_count: 0,
            repeat_delay: Duration::ZERO,
            yoyo: false,
            is_paused: false,
        }
    }

    /// Set animation delay
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// Set repeat count (-1 for infinite)
    pub fn with_repeat(mut self, count: i32) -> Self {
        self.repeat_count = count;
        self
    }

    /// Set yoyo effect (reverse animation on repeat)
    pub fn with_yoyo(mut self, yoyo: bool) -> Self {
        self.yoyo = yoyo;
        self
    }

    /// Update the animation
    pub fn update(&mut self, delta: Duration) {
        if self.is_paused {
            return;
        }

        // Handle delay
        if self.delay > Duration::ZERO {
            self.delay = self.delay.saturating_sub(delta);
            return;
        }

        self.elapsed += delta;

        let progress = if self.duration.as_nanos() > 0 {
            self.elapsed.as_nanos() as f64 / self.duration.as_nanos() as f64
        } else {
            1.0
        };

        let eased_progress = self.apply_easing(progress.min(1.0));

        // Apply yoyo effect
        let final_progress =
            if self.yoyo && (self.elapsed.as_nanos() / self.duration.as_nanos()) % 2 == 1 {
                1.0 - eased_progress
            } else {
                eased_progress
            };

        self.current_value =
            self.start_value + (self.end_value - self.start_value) * final_progress;
    }

    /// Apply easing function
    fn apply_easing(&self, t: f64) -> f64 {
        let easing_functions = EasingFunctions::new();

        match &self.easing {
            EasingType::Linear => easing_functions.linear(t),
            EasingType::EaseInCubic => easing_functions.ease_in_cubic(t),
            EasingType::EaseOutCubic => easing_functions.ease_out_cubic(t),
            EasingType::EaseInOutCubic => easing_functions.ease_in_out_cubic(t),
            EasingType::EaseInQuad => easing_functions.ease_in_quad(t),
            EasingType::EaseOutQuad => easing_functions.ease_out_quad(t),
            EasingType::EaseInOutQuad => easing_functions.ease_in_out_quad(t),
            EasingType::EaseOutElastic => easing_functions.ease_out_elastic(t),
            EasingType::EaseInElastic => easing_functions.ease_in_elastic(t),
            EasingType::EaseInBack => easing_functions.ease_in_back(t),
            EasingType::EaseOutBack => easing_functions.ease_out_back(t),
            EasingType::EaseInOutBack => easing_functions.ease_in_out_back(t),
            EasingType::EaseInBounce => easing_functions.ease_in_bounce(t),
            EasingType::EaseOutBounce => easing_functions.ease_out_bounce(t),
            EasingType::EaseInOutBounce => easing_functions.ease_in_out_bounce(t),
            EasingType::Custom(name) => easing_functions.apply_easing(name, t),
        }
    }

    /// Check if animation is completed
    pub fn is_completed(&self) -> bool {
        if self.repeat_count == -1 {
            false // Infinite repeat
        } else if self.repeat_count > 0 {
            self.elapsed >= self.duration * (self.repeat_count as u32 + 1)
        } else {
            self.elapsed >= self.duration
        }
    }

    /// Pause the animation
    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    /// Resume the animation
    pub fn resume(&mut self) {
        self.is_paused = false;
    }

    /// Reset the animation
    pub fn reset(&mut self) {
        self.elapsed = Duration::ZERO;
        self.current_value = self.start_value;
        self.is_paused = false;
    }

    /// Get current progress (0.0 to 1.0)
    pub fn get_progress(&self) -> f64 {
        if self.duration.as_nanos() > 0 {
            (self.elapsed.as_nanos() as f64 / self.duration.as_nanos() as f64).min(1.0)
        } else {
            1.0
        }
    }
}

// ============================================================================
// State Transitions
// ============================================================================

/// State transition system for smooth state changes
#[derive(Debug, Clone)]
pub struct StateTransition {
    pub from_state: String,
    pub to_state: String,
    pub progress: f64,
    pub duration: Duration,
    pub elapsed: Duration,
    pub easing: EasingType,
    pub is_completed: bool,
}

impl StateTransition {
    /// Create a new state transition
    pub fn new(from: &str, to: &str) -> Self {
        Self {
            from_state: from.to_string(),
            to_state: to.to_string(),
            progress: 0.0,
            duration: Duration::from_millis(500),
            elapsed: Duration::ZERO,
            easing: EasingType::EaseInOutCubic,
            is_completed: false,
        }
    }

    /// Create state transition with duration
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Create state transition with easing
    pub fn with_easing(mut self, easing: EasingType) -> Self {
        self.easing = easing;
        self
    }

    /// Update transition progress
    pub fn update(&mut self, delta: Duration) {
        if self.is_completed {
            return;
        }

        self.elapsed += delta;

        let raw_progress = if self.duration.as_nanos() > 0 {
            self.elapsed.as_nanos() as f64 / self.duration.as_nanos() as f64
        } else {
            1.0
        };

        let progress = raw_progress.min(1.0);

        // Apply easing
        let easing_functions = EasingFunctions::new();
        self.progress = match &self.easing {
            EasingType::Linear => easing_functions.linear(progress),
            EasingType::EaseInCubic => easing_functions.ease_in_cubic(progress),
            EasingType::EaseOutCubic => easing_functions.ease_out_cubic(progress),
            EasingType::EaseInOutCubic => easing_functions.ease_in_out_cubic(progress),
            EasingType::EaseInQuad => easing_functions.ease_in_quad(progress),
            EasingType::EaseOutQuad => easing_functions.ease_out_quad(progress),
            EasingType::EaseInOutQuad => easing_functions.ease_in_out_quad(progress),
            EasingType::EaseOutElastic => easing_functions.ease_out_elastic(progress),
            EasingType::EaseInElastic => easing_functions.ease_in_elastic(progress),
            EasingType::EaseInBack => easing_functions.ease_in_back(progress),
            EasingType::EaseOutBack => easing_functions.ease_out_back(progress),
            EasingType::EaseInOutBack => easing_functions.ease_in_out_back(progress),
            EasingType::EaseInBounce => easing_functions.ease_in_bounce(progress),
            EasingType::EaseOutBounce => easing_functions.ease_out_bounce(progress),
            EasingType::EaseInOutBounce => easing_functions.ease_in_out_bounce(progress),
            EasingType::Custom(name) => easing_functions.apply_easing(name, progress),
        };

        if progress >= 1.0 {
            self.is_completed = true;
        }
    }

    /// Check if transition is completed
    pub fn is_completed(&self) -> bool {
        self.is_completed
    }

    /// Interpolate between two colors
    pub fn interpolate_color(&self, start: &Color, end: &Color) -> Color {
        Color {
            r: start.r + (end.r - start.r) * self.progress as f32,
            g: start.g + (end.g - start.g) * self.progress as f32,
            b: start.b + (end.b - start.b) * self.progress as f32,
            a: start.a + (end.a - start.a) * self.progress as f32,
        }
    }

    /// Interpolate between two positions
    pub fn interpolate_position(&self, start: &Point2D, end: &Point2D) -> Point2D {
        Point2D {
            x: start.x + (end.x - start.x) * self.progress,
            y: start.y + (end.y - start.y) * self.progress,
        }
    }

    /// Interpolate between two values
    pub fn interpolate_value(&self, start: f64, end: f64) -> f64 {
        start + (end - start) * self.progress
    }

    /// Interpolate between two sizes
    pub fn interpolate_size(&self, start: &Size, end: &Size) -> Size {
        Size {
            width: start.width + (end.width - start.width) * self.progress,
            height: start.height + (end.height - start.height) * self.progress,
        }
    }
}

// ============================================================================
// Animation Orchestration
// ============================================================================

/// Animation orchestrator for managing multiple animations
#[derive(Debug, Clone)]
pub struct AnimationOrchestrator {
    pub active_animations: HashMap<String, TweenAnimation>,
    pub animation_queue: Vec<String>,
    pub sequences: HashMap<String, Vec<String>>,
    pub parallel_groups: HashMap<String, Vec<String>>,
    pub next_id: usize,
    pub is_running: bool,
}

impl AnimationOrchestrator {
    /// Create a new animation orchestrator
    pub fn new() -> Self {
        Self {
            active_animations: HashMap::new(),
            animation_queue: Vec::new(),
            sequences: HashMap::new(),
            parallel_groups: HashMap::new(),
            next_id: 0,
            is_running: false,
        }
    }

    /// Add an animation to the orchestrator
    pub fn add_animation(&mut self, animation: TweenAnimation) -> String {
        let id = format!("anim_{}", self.next_id);
        self.next_id += 1;
        self.active_animations.insert(id.clone(), animation);
        self.is_running = true;
        id
    }

    /// Check if orchestrator is running
    pub fn is_running(&self) -> bool {
        self.is_running && !self.active_animations.is_empty()
    }

    /// Check if specific animation is running
    pub fn is_animation_running(&self, id: &str) -> bool {
        self.active_animations.contains_key(id)
    }

    /// Get animation by ID
    pub fn get_animation(&self, id: &str) -> Option<&TweenAnimation> {
        self.active_animations.get(id)
    }

    /// Get mutable animation by ID
    pub fn get_animation_mut(&mut self, id: &str) -> Option<&mut TweenAnimation> {
        self.active_animations.get_mut(id)
    }

    /// Create a sequence of animations
    pub fn sequence_animations(&mut self, animation_ids: Vec<String>) -> String {
        let sequence_id = format!("seq_{}", self.next_id);
        self.next_id += 1;
        self.sequences.insert(sequence_id.clone(), animation_ids);
        sequence_id
    }

    /// Create a parallel group of animations
    pub fn parallel_animations(&mut self, animation_ids: Vec<String>) -> String {
        let group_id = format!("parallel_{}", self.next_id);
        self.next_id += 1;
        self.parallel_groups.insert(group_id.clone(), animation_ids);
        group_id
    }

    /// Update all animations
    pub fn update(&mut self, delta: Duration) {
        let mut completed_animations = Vec::new();

        for (id, animation) in &mut self.active_animations {
            animation.update(delta);
            if animation.is_completed() {
                completed_animations.push(id.clone());
            }
        }

        // Remove completed animations
        for id in completed_animations {
            self.active_animations.remove(&id);
        }

        // Update running state
        self.is_running = !self.active_animations.is_empty();
    }

    /// Pause all animations
    pub fn pause_all(&mut self) {
        for animation in self.active_animations.values_mut() {
            animation.pause();
        }
    }

    /// Resume all animations
    pub fn resume_all(&mut self) {
        for animation in self.active_animations.values_mut() {
            animation.resume();
        }
    }

    /// Stop all animations
    pub fn stop_all(&mut self) {
        self.active_animations.clear();
        self.is_running = false;
    }

    /// Get animation count
    pub fn animation_count(&self) -> usize {
        self.active_animations.len()
    }
}

// ============================================================================
// Performance-Optimized Rendering
// ============================================================================

/// High-performance animation renderer
#[derive(Debug, Clone)]
pub struct AnimationRenderer {
    pub width: u32,
    pub height: u32,
    pub frame_count: u32,
    pub fps: f64,
    pub last_frame_time: Instant,
    pub frame_times: Vec<Duration>,
    pub target_fps: f64,
    pub vsync_enabled: bool,
}

impl AnimationRenderer {
    /// Create a new animation renderer
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            frame_count: 0,
            fps: 60.0,
            last_frame_time: Instant::now(),
            frame_times: Vec::new(),
            target_fps: 60.0,
            vsync_enabled: true,
        }
    }

    /// Set target FPS
    pub fn set_target_fps(&mut self, fps: f64) {
        self.target_fps = fps;
    }

    /// Enable or disable VSync
    pub fn set_vsync(&mut self, enabled: bool) {
        self.vsync_enabled = enabled;
    }

    /// Render a frame
    pub fn render_frame(&mut self) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame_time);

        self.frame_times.push(frame_time);
        if self.frame_times.len() > 60 {
            self.frame_times.remove(0);
        }

        // Calculate FPS
        if self.frame_times.len() > 1 {
            let total_time: Duration = self.frame_times.iter().sum();
            self.fps = (self.frame_times.len() as f64) / total_time.as_secs_f64();
        }

        self.frame_count += 1;
        self.last_frame_time = now;
    }

    /// Interpolate position with easing
    pub fn interpolate_position(
        &self,
        start: &Point2D,
        end: &Point2D,
        progress: f64,
        easing: &EasingType,
    ) -> Point2D {
        let easing_functions = EasingFunctions::new();
        let eased_progress = match easing {
            EasingType::Linear => easing_functions.linear(progress),
            EasingType::EaseInCubic => easing_functions.ease_in_cubic(progress),
            EasingType::EaseOutCubic => easing_functions.ease_out_cubic(progress),
            EasingType::EaseInOutCubic => easing_functions.ease_in_out_cubic(progress),
            EasingType::EaseInQuad => easing_functions.ease_in_quad(progress),
            EasingType::EaseOutQuad => easing_functions.ease_out_quad(progress),
            EasingType::EaseInOutQuad => easing_functions.ease_in_out_quad(progress),
            EasingType::EaseOutElastic => easing_functions.ease_out_elastic(progress),
            EasingType::EaseInElastic => easing_functions.ease_in_elastic(progress),
            EasingType::EaseInBack => easing_functions.ease_in_back(progress),
            EasingType::EaseOutBack => easing_functions.ease_out_back(progress),
            EasingType::EaseInOutBack => easing_functions.ease_in_out_back(progress),
            EasingType::EaseInBounce => easing_functions.ease_in_bounce(progress),
            EasingType::EaseOutBounce => easing_functions.ease_out_bounce(progress),
            EasingType::EaseInOutBounce => easing_functions.ease_in_out_bounce(progress),
            EasingType::Custom(name) => easing_functions.apply_easing(name, progress),
        };

        Point2D {
            x: start.x + (end.x - start.x) * eased_progress,
            y: start.y + (end.y - start.y) * eased_progress,
        }
    }

    /// Render animation batch
    pub fn render_batch(&mut self, batch: &AnimationBatch) {
        // Simulate rendering all animations in batch
        for animation in &batch.animations {
            // Render animation based on current value
            let _current_value = animation.current_value;
        }
        self.render_frame();
    }

    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        PerformanceMetrics {
            fps: self.fps,
            frame_count: self.frame_count,
            average_frame_time: if !self.frame_times.is_empty() {
                self.frame_times.iter().sum::<Duration>() / self.frame_times.len() as u32
            } else {
                Duration::ZERO
            },
            target_fps: self.target_fps,
        }
    }
}

/// Animation batch for efficient rendering
#[derive(Debug, Clone)]
pub struct AnimationBatch {
    pub animations: Vec<TweenAnimation>,
    pub render_order: Vec<usize>,
}

impl AnimationBatch {
    /// Create a new animation batch
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
            render_order: Vec::new(),
        }
    }

    /// Add animation to batch
    pub fn add_animation(&mut self, animation: TweenAnimation) {
        self.animations.push(animation);
        self.render_order.push(self.animations.len() - 1);
    }

    /// Sort animations by render order
    pub fn sort_by_render_order(&mut self) {
        // Sort by z-index or other criteria
        self.render_order.sort();
    }

    /// Get animation count
    pub fn animation_count(&self) -> usize {
        self.animations.len()
    }
}

// ============================================================================
// Supporting Types
// ============================================================================

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

    /// Create color from RGB values (0-255)
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }

    /// Create color from RGBA values (0-255)
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }
}

/// 2D point
#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    /// Create a new 2D point
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Calculate distance to another point
    pub fn distance_to(&self, other: &Point2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Size with width and height
#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    /// Create a new size
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

/// Performance metrics for animation rendering
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub fps: f64,
    pub frame_count: u32,
    pub average_frame_time: Duration,
    pub target_fps: f64,
}
