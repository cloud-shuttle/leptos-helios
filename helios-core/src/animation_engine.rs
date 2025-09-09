//! Animation Engine
//!
//! This module provides a comprehensive animation framework for Helios visualizations,
//! including keyframe animations, transitions, and performance-optimized animation scheduling.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use thiserror::Error;

/// Animation engine errors
#[derive(Debug, Error)]
pub enum AnimationError {
    #[error("Animation not found: {id}")]
    AnimationNotFound { id: String },

    #[error("Invalid animation configuration: {message}")]
    InvalidConfiguration { message: String },

    #[error("Animation execution failed: {message}")]
    ExecutionFailed { message: String },

    #[error("Animation scheduling failed: {message}")]
    SchedulingFailed { message: String },

    #[error("Performance limit exceeded: {message}")]
    PerformanceLimitExceeded { message: String },
}

/// Animation identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AnimationId(pub String);

/// Easing function types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
    EaseInQuint,
    EaseOutQuint,
    EaseInOutQuint,
    EaseInSine,
    EaseOutSine,
    EaseInOutSine,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,
    Custom(String),
}

/// Animation direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

/// Animation fill mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

/// Animation iteration count
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationIterationCount {
    Infinite,
    Count(u32),
}

/// Keyframe definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe {
    pub offset: f64, // 0.0 to 1.0
    pub properties: HashMap<String, String>,
}

/// Animation definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub id: AnimationId,
    pub name: String,
    pub duration: Duration,
    pub delay: Duration,
    pub iteration_count: AnimationIterationCount,
    pub direction: AnimationDirection,
    pub fill_mode: AnimationFillMode,
    pub easing: EasingFunction,
    pub keyframes: Vec<Keyframe>,
    pub target_element: Option<String>,
    pub auto_play: bool,
    pub paused: bool,
}

/// Animation state
#[derive(Debug, Clone)]
pub struct AnimationState {
    pub id: AnimationId,
    pub start_time: Instant,
    pub current_time: Duration,
    pub progress: f64, // 0.0 to 1.0
    pub is_playing: bool,
    pub is_paused: bool,
    pub is_finished: bool,
    pub current_iteration: u32,
}

/// Animation event
#[derive(Debug, Clone)]
pub enum AnimationEvent {
    Started {
        animation_id: AnimationId,
    },
    Paused {
        animation_id: AnimationId,
    },
    Resumed {
        animation_id: AnimationId,
    },
    Finished {
        animation_id: AnimationId,
    },
    IterationComplete {
        animation_id: AnimationId,
        iteration: u32,
    },
    Progress {
        animation_id: AnimationId,
        progress: f64,
    },
}

/// Animation event handler
pub type AnimationEventHandler = Box<dyn Fn(AnimationEvent) + Send + Sync>;

/// Animation scheduler
#[derive(Debug)]
pub struct AnimationScheduler {
    animations: HashMap<AnimationId, Animation>,
    states: HashMap<AnimationId, AnimationState>,
    event_handlers: Vec<AnimationEventHandler>,
    performance_config: PerformanceConfig,
    last_frame_time: Instant,
    frame_count: u64,
}

/// Performance configuration
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    pub max_animations: usize,
    pub target_fps: u32,
    pub max_frame_time: Duration,
    pub enable_performance_monitoring: bool,
    pub auto_pause_on_hidden: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_animations: 100,
            target_fps: 60,
            max_frame_time: Duration::from_millis(16), // ~60fps
            enable_performance_monitoring: true,
            auto_pause_on_hidden: true,
        }
    }
}

impl AnimationScheduler {
    /// Create a new animation scheduler
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            states: HashMap::new(),
            event_handlers: Vec::new(),
            performance_config: PerformanceConfig::default(),
            last_frame_time: Instant::now(),
            frame_count: 0,
        }
    }

    /// Create with custom performance configuration
    pub fn with_config(config: PerformanceConfig) -> Self {
        Self {
            animations: HashMap::new(),
            states: HashMap::new(),
            event_handlers: Vec::new(),
            performance_config: config,
            last_frame_time: Instant::now(),
            frame_count: 0,
        }
    }

    /// Register an animation
    pub fn register_animation(&mut self, animation: Animation) -> Result<(), AnimationError> {
        self.validate_animation(&animation)?;

        if self.animations.len() >= self.performance_config.max_animations {
            return Err(AnimationError::PerformanceLimitExceeded {
                message: format!(
                    "Maximum number of animations ({}) exceeded",
                    self.performance_config.max_animations
                ),
            });
        }

        self.animations.insert(animation.id.clone(), animation);
        Ok(())
    }

    /// Start an animation
    pub fn start_animation(&mut self, id: &AnimationId) -> Result<(), AnimationError> {
        let animation = self
            .animations
            .get(id)
            .ok_or_else(|| AnimationError::AnimationNotFound { id: id.0.clone() })?;

        let state = AnimationState {
            id: id.clone(),
            start_time: Instant::now(),
            current_time: Duration::ZERO,
            progress: 0.0,
            is_playing: true,
            is_paused: false,
            is_finished: false,
            current_iteration: 0,
        };

        self.states.insert(id.clone(), state);
        self.emit_event(AnimationEvent::Started {
            animation_id: id.clone(),
        });

        Ok(())
    }

    /// Pause an animation
    pub fn pause_animation(&mut self, id: &AnimationId) -> Result<(), AnimationError> {
        if let Some(state) = self.states.get_mut(id) {
            state.is_playing = false;
            state.is_paused = true;
            self.emit_event(AnimationEvent::Paused {
                animation_id: id.clone(),
            });
            Ok(())
        } else {
            Err(AnimationError::AnimationNotFound { id: id.0.clone() })
        }
    }

    /// Resume an animation
    pub fn resume_animation(&mut self, id: &AnimationId) -> Result<(), AnimationError> {
        if let Some(state) = self.states.get_mut(id) {
            state.is_playing = true;
            state.is_paused = false;
            self.emit_event(AnimationEvent::Resumed {
                animation_id: id.clone(),
            });
            Ok(())
        } else {
            Err(AnimationError::AnimationNotFound { id: id.0.clone() })
        }
    }

    /// Stop an animation
    pub fn stop_animation(&mut self, id: &AnimationId) -> Result<(), AnimationError> {
        if let Some(state) = self.states.get_mut(id) {
            state.is_playing = false;
            state.is_paused = false;
            state.is_finished = true;
            self.emit_event(AnimationEvent::Finished {
                animation_id: id.clone(),
            });
            Ok(())
        } else {
            Err(AnimationError::AnimationNotFound { id: id.0.clone() })
        }
    }

    /// Update all animations (call this in your render loop)
    pub fn update(&mut self, delta_time: Duration) -> Result<(), AnimationError> {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame_time);

        // Check performance limits
        if frame_time > self.performance_config.max_frame_time {
            if self.performance_config.enable_performance_monitoring {
                // Log performance warning
                eprintln!("Animation frame time exceeded limit: {:?}", frame_time);
            }
        }

        self.last_frame_time = now;
        self.frame_count += 1;

        // Update all active animations
        let mut finished_animations = Vec::new();

        for (id, state) in self.states.iter_mut() {
            if !state.is_playing || state.is_paused {
                continue;
            }

            let animation = self
                .animations
                .get(id)
                .ok_or_else(|| AnimationError::AnimationNotFound { id: id.0.clone() })?;

            // Update animation time
            state.current_time += delta_time;

            // Calculate progress
            let total_duration = animation.duration + animation.delay;
            let progress = if state.current_time >= total_duration {
                1.0
            } else if state.current_time < animation.delay {
                0.0
            } else {
                let animation_time = state.current_time - animation.delay;
                animation_time.as_secs_f64() / animation.duration.as_secs_f64()
            };

            state.progress = progress;

            // Apply easing
            let eased_progress = self.apply_easing(progress, &animation.easing);

            // Check for iteration completion
            if progress >= 1.0 {
                state.current_iteration += 1;
                self.emit_event(AnimationEvent::IterationComplete {
                    animation_id: id.clone(),
                    iteration: state.current_iteration,
                });

                // Check if animation should continue
                match animation.iteration_count {
                    AnimationIterationCount::Infinite => {
                        state.current_time = Duration::ZERO;
                        state.progress = 0.0;
                    }
                    AnimationIterationCount::Count(max_iterations) => {
                        if state.current_iteration >= max_iterations {
                            state.is_finished = true;
                            state.is_playing = false;
                            finished_animations.push(id.clone());
                        } else {
                            state.current_time = Duration::ZERO;
                            state.progress = 0.0;
                        }
                    }
                }
            }

            // Emit progress event
            self.emit_event(AnimationEvent::Progress {
                animation_id: id.clone(),
                progress: eased_progress,
            });
        }

        // Clean up finished animations
        for id in finished_animations {
            self.emit_event(AnimationEvent::Finished {
                animation_id: id.clone(),
            });
            self.states.remove(&id);
        }

        Ok(())
    }

    /// Get animation state
    pub fn get_animation_state(&self, id: &AnimationId) -> Option<&AnimationState> {
        self.states.get(id)
    }

    /// Get all active animations
    pub fn get_active_animations(&self) -> Vec<&AnimationState> {
        self.states.values().collect()
    }

    /// Add event handler
    pub fn add_event_handler(&mut self, handler: AnimationEventHandler) {
        self.event_handlers.push(handler);
    }

    /// Get performance statistics
    pub fn get_performance_stats(&self) -> PerformanceStats {
        PerformanceStats {
            active_animations: self.states.len(),
            total_animations: self.animations.len(),
            frame_count: self.frame_count,
            target_fps: self.performance_config.target_fps,
        }
    }

    /// Validate animation configuration
    fn validate_animation(&self, animation: &Animation) -> Result<(), AnimationError> {
        if animation.id.0.is_empty() {
            return Err(AnimationError::InvalidConfiguration {
                message: "Animation ID cannot be empty".to_string(),
            });
        }

        if animation.name.is_empty() {
            return Err(AnimationError::InvalidConfiguration {
                message: "Animation name cannot be empty".to_string(),
            });
        }

        if animation.duration.is_zero() {
            return Err(AnimationError::InvalidConfiguration {
                message: "Animation duration cannot be zero".to_string(),
            });
        }

        if animation.keyframes.is_empty() {
            return Err(AnimationError::InvalidConfiguration {
                message: "Animation must have at least one keyframe".to_string(),
            });
        }

        // Validate keyframes
        for (i, keyframe) in animation.keyframes.iter().enumerate() {
            if keyframe.offset < 0.0 || keyframe.offset > 1.0 {
                return Err(AnimationError::InvalidConfiguration {
                    message: format!("Keyframe {} offset must be between 0.0 and 1.0", i),
                });
            }
        }

        Ok(())
    }

    /// Apply easing function to progress
    fn apply_easing(&self, progress: f64, easing: &EasingFunction) -> f64 {
        match easing {
            EasingFunction::Linear => progress,
            EasingFunction::EaseIn => progress * progress,
            EasingFunction::EaseOut => 1.0 - (1.0 - progress) * (1.0 - progress),
            EasingFunction::EaseInOut => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                }
            }
            EasingFunction::EaseInQuad => progress * progress,
            EasingFunction::EaseOutQuad => 1.0 - (1.0 - progress) * (1.0 - progress),
            EasingFunction::EaseInOutQuad => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                }
            }
            EasingFunction::EaseInCubic => progress * progress * progress,
            EasingFunction::EaseOutCubic => 1.0 - (1.0 - progress).powi(3),
            EasingFunction::EaseInOutCubic => {
                if progress < 0.5 {
                    4.0 * progress * progress * progress
                } else {
                    1.0 - 4.0 * (1.0 - progress).powi(3)
                }
            }
            EasingFunction::EaseInQuart => progress * progress * progress * progress,
            EasingFunction::EaseOutQuart => 1.0 - (1.0 - progress).powi(4),
            EasingFunction::EaseInOutQuart => {
                if progress < 0.5 {
                    8.0 * progress * progress * progress * progress
                } else {
                    1.0 - 8.0 * (1.0 - progress).powi(4)
                }
            }
            EasingFunction::EaseInQuint => progress * progress * progress * progress * progress,
            EasingFunction::EaseOutQuint => 1.0 - (1.0 - progress).powi(5),
            EasingFunction::EaseInOutQuint => {
                if progress < 0.5 {
                    16.0 * progress * progress * progress * progress * progress
                } else {
                    1.0 - 16.0 * (1.0 - progress).powi(5)
                }
            }
            EasingFunction::EaseInSine => 1.0 - (progress * std::f64::consts::PI / 2.0).cos(),
            EasingFunction::EaseOutSine => (progress * std::f64::consts::PI / 2.0).sin(),
            EasingFunction::EaseInOutSine => {
                if progress < 0.5 {
                    0.5 * (1.0 - (progress * std::f64::consts::PI).cos())
                } else {
                    0.5 * (1.0 + ((progress - 0.5) * std::f64::consts::PI).sin())
                }
            }
            EasingFunction::EaseInExpo => {
                if progress == 0.0 {
                    0.0
                } else {
                    2.0_f64.powf(10.0 * (progress - 1.0))
                }
            }
            EasingFunction::EaseOutExpo => {
                if progress == 1.0 {
                    1.0
                } else {
                    1.0 - 2.0_f64.powf(-10.0 * progress)
                }
            }
            EasingFunction::EaseInOutExpo => {
                if progress == 0.0 {
                    0.0
                } else if progress == 1.0 {
                    1.0
                } else if progress < 0.5 {
                    0.5 * 2.0_f64.powf(20.0 * progress - 10.0)
                } else {
                    0.5 * (2.0 - 2.0_f64.powf(-20.0 * progress + 10.0))
                }
            }
            EasingFunction::EaseInCirc => 1.0 - (1.0 - progress * progress).sqrt(),
            EasingFunction::EaseOutCirc => ((progress - 1.0) * (progress - 1.0) - 1.0).sqrt(),
            EasingFunction::EaseInOutCirc => {
                if progress < 0.5 {
                    0.5 * (1.0 - (1.0 - 4.0 * progress * progress).sqrt())
                } else {
                    0.5 * ((2.0 * progress - 2.0) * (2.0 * progress - 2.0) - 1.0).sqrt() + 1.0
                }
            }
            EasingFunction::EaseInBack => {
                const C1: f64 = 1.70158;
                const C3: f64 = C1 + 1.0;
                C3 * progress * progress * progress - C1 * progress * progress
            }
            EasingFunction::EaseOutBack => {
                const C1: f64 = 1.70158;
                const C3: f64 = C1 + 1.0;
                1.0 + C3 * (progress - 1.0).powi(3) + C1 * (progress - 1.0).powi(2)
            }
            EasingFunction::EaseInOutBack => {
                const C1: f64 = 1.70158;
                const C2: f64 = C1 * 1.525;
                if progress < 0.5 {
                    0.5 * ((2.0 * progress).powi(2) * ((C2 + 1.0) * 2.0 * progress - C2))
                } else {
                    0.5 * ((2.0 * progress - 2.0).powi(2)
                        * ((C2 + 1.0) * (2.0 * progress - 2.0) + C2)
                        + 2.0)
                }
            }
            EasingFunction::EaseInElastic => {
                if progress == 0.0 {
                    0.0
                } else if progress == 1.0 {
                    1.0
                } else {
                    -2.0_f64.powf(10.0 * progress - 10.0)
                        * ((progress * 10.0 - 10.75) * (2.0 * std::f64::consts::PI / 3.0)).sin()
                }
            }
            EasingFunction::EaseOutElastic => {
                if progress == 0.0 {
                    0.0
                } else if progress == 1.0 {
                    1.0
                } else {
                    2.0_f64.powf(-10.0 * progress)
                        * ((progress * 10.0 - 0.75) * (2.0 * std::f64::consts::PI / 3.0)).sin()
                        + 1.0
                }
            }
            EasingFunction::EaseInOutElastic => {
                if progress == 0.0 {
                    0.0
                } else if progress == 1.0 {
                    1.0
                } else if progress < 0.5 {
                    -0.5 * 2.0_f64.powf(20.0 * progress - 10.0)
                        * ((20.0 * progress - 11.125) * (2.0 * std::f64::consts::PI / 4.5)).sin()
                } else {
                    0.5 * 2.0_f64.powf(-20.0 * progress + 10.0)
                        * ((20.0 * progress - 11.125) * (2.0 * std::f64::consts::PI / 4.5)).sin()
                        + 1.0
                }
            }
            EasingFunction::EaseInBounce => 1.0 - self.ease_out_bounce(1.0 - progress),
            EasingFunction::EaseOutBounce => self.ease_out_bounce(progress),
            EasingFunction::EaseInOutBounce => {
                if progress < 0.5 {
                    0.5 * (1.0 - self.ease_out_bounce(1.0 - 2.0 * progress))
                } else {
                    0.5 * self.ease_out_bounce(2.0 * progress - 1.0) + 0.5
                }
            }
            EasingFunction::Custom(_) => progress, // Custom easing not implemented yet
        }
    }

    /// Ease out bounce helper function
    fn ease_out_bounce(&self, progress: f64) -> f64 {
        const N1: f64 = 7.5625;
        const D1: f64 = 2.75;

        if progress < 1.0 / D1 {
            N1 * progress * progress
        } else if progress < 2.0 / D1 {
            let n = progress - 1.5 / D1;
            N1 * n * n + 0.75
        } else if progress < 2.5 / D1 {
            let n = progress - 2.25 / D1;
            N1 * n * n + 0.9375
        } else {
            let n = progress - 2.625 / D1;
            N1 * n * n + 0.984375
        }
    }

    /// Emit animation event
    fn emit_event(&self, event: AnimationEvent) {
        for handler in &self.event_handlers {
            handler(event.clone());
        }
    }
}

/// Performance statistics
#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub active_animations: usize,
    pub total_animations: usize,
    pub frame_count: u64,
    pub target_fps: u32,
}

impl Default for AnimationScheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a sample animation
pub fn create_sample_animation() -> Animation {
    Animation {
        id: AnimationId("fade-in".to_string()),
        name: "Fade In Animation".to_string(),
        duration: Duration::from_millis(500),
        delay: Duration::ZERO,
        iteration_count: AnimationIterationCount::Count(1),
        direction: AnimationDirection::Normal,
        fill_mode: AnimationFillMode::Forwards,
        easing: EasingFunction::EaseInOut,
        keyframes: vec![
            Keyframe {
                offset: 0.0,
                properties: HashMap::from([
                    ("opacity".to_string(), "0".to_string()),
                    ("transform".to_string(), "translateY(20px)".to_string()),
                ]),
            },
            Keyframe {
                offset: 1.0,
                properties: HashMap::from([
                    ("opacity".to_string(), "1".to_string()),
                    ("transform".to_string(), "translateY(0px)".to_string()),
                ]),
            },
        ],
        target_element: None,
        auto_play: true,
        paused: false,
    }
}
