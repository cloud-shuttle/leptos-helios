//! Interactive Debugger
//!
//! This module provides interactive debugging capabilities for Helios,
//! including breakpoints, step-through debugging, and state inspection.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::sync::{broadcast, oneshot};

/// Debugger errors
#[derive(Debug, thiserror::Error)]
pub enum DebuggerError {
    #[error("Debugger not started")]
    NotStarted,

    #[error("Breakpoint not found: {0}")]
    BreakpointNotFound(String),

    #[error("Invalid debug command: {0}")]
    InvalidCommand(String),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error("State inspection failed: {0}")]
    StateInspectionFailed(String),
}

/// Debug breakpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breakpoint {
    pub id: String,
    pub location: BreakpointLocation,
    pub condition: Option<String>,
    pub enabled: bool,
    pub hit_count: usize,
    pub created_at: u64,
}

/// Breakpoint location types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BreakpointLocation {
    Function { name: String },
    Line { file: String, line: u32 },
    Render { chart_type: String },
    DataProcessing { operation: String },
    Performance { threshold_ms: u64 },
}

/// Debug execution state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionState {
    Running,
    Paused { location: String, reason: String },
    Stopped,
    Error { message: String },
}

/// Debug step commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DebugCommand {
    Continue,
    StepOver,
    StepInto,
    StepOut,
    Stop,
    Restart,
}

/// Variable information for state inspection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableInfo {
    pub name: String,
    pub value: String,
    pub type_name: String,
    pub children: Vec<VariableInfo>,
    pub memory_address: Option<String>,
}

/// Call stack frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    pub function_name: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub variables: Vec<VariableInfo>,
}

/// Debug session state
#[derive(Debug, Clone)]
pub struct DebugSession {
    pub id: String,
    pub name: String,
    pub state: ExecutionState,
    pub breakpoints: HashMap<String, Breakpoint>,
    pub call_stack: Vec<StackFrame>,
    pub start_time: Instant,
}

/// Enhanced error message with debugging context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedError {
    pub original_error: String,
    pub error_type: String,
    pub location: Option<String>,
    pub context: HashMap<String, String>,
    pub suggestions: Vec<String>,
    pub related_documentation: Vec<String>,
    pub severity: ErrorSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Main interactive debugger
pub struct InteractiveDebugger {
    sessions: Arc<Mutex<HashMap<String, DebugSession>>>,
    current_session: Arc<Mutex<Option<String>>>,
    command_sender: broadcast::Sender<DebugCommand>,
    state_sender: broadcast::Sender<ExecutionState>,
    enabled: bool,
    enhanced_errors: bool,
}

impl InteractiveDebugger {
    /// Create a new interactive debugger
    pub fn new() -> Self {
        let (command_sender, _) = broadcast::channel(100);
        let (state_sender, _) = broadcast::channel(100);

        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            current_session: Arc::new(Mutex::new(None)),
            command_sender,
            state_sender,
            enabled: true,
            enhanced_errors: true,
        }
    }

    /// Start a new debug session
    pub fn start_session(&mut self, name: &str) -> Result<String, DebuggerError> {
        if !self.enabled {
            return Err(DebuggerError::NotStarted);
        }

        let session_id = uuid::Uuid::new_v4().to_string();
        let session = DebugSession {
            id: session_id.clone(),
            name: name.to_string(),
            state: ExecutionState::Running,
            breakpoints: HashMap::new(),
            call_stack: Vec::new(),
            start_time: Instant::now(),
        };

        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(session_id.clone(), session);

        let mut current = self.current_session.lock().unwrap();
        *current = Some(session_id.clone());

        println!("🐛 Started debug session: {} ({})", name, session_id);
        Ok(session_id)
    }

    /// Add a breakpoint
    pub fn add_breakpoint(
        &mut self,
        location: BreakpointLocation,
        condition: Option<String>,
    ) -> Result<String, DebuggerError> {
        let current_session = self.current_session.lock().unwrap();
        let session_id = current_session.as_ref().ok_or(DebuggerError::NotStarted)?;

        let breakpoint_id = uuid::Uuid::new_v4().to_string();
        let breakpoint = Breakpoint {
            id: breakpoint_id.clone(),
            location,
            condition,
            enabled: true,
            hit_count: 0,
            created_at: Instant::now().elapsed().as_millis() as u64,
        };

        let mut sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get_mut(session_id) {
            session
                .breakpoints
                .insert(breakpoint_id.clone(), breakpoint);
        }

        println!("🔴 Added breakpoint: {}", breakpoint_id);
        Ok(breakpoint_id)
    }

    /// Remove a breakpoint
    pub fn remove_breakpoint(&mut self, breakpoint_id: &str) -> Result<(), DebuggerError> {
        let current_session = self.current_session.lock().unwrap();
        let session_id = current_session.as_ref().ok_or(DebuggerError::NotStarted)?;

        let mut sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get_mut(session_id) {
            session
                .breakpoints
                .remove(breakpoint_id)
                .ok_or_else(|| DebuggerError::BreakpointNotFound(breakpoint_id.to_string()))?;
        }

        println!("⭕ Removed breakpoint: {}", breakpoint_id);
        Ok(())
    }

    /// Execute debug command
    pub fn execute_command(&mut self, command: DebugCommand) -> Result<(), DebuggerError> {
        let current_session = self.current_session.lock().unwrap();
        let session_id = current_session.as_ref().ok_or(DebuggerError::NotStarted)?;

        // Broadcast command to debug runtime
        let _ = self.command_sender.send(command.clone());

        // Update session state based on command
        let new_state = {
            let mut sessions = self.sessions.lock().unwrap();
            if let Some(session) = sessions.get_mut(session_id) {
                session.state = match command {
                    DebugCommand::Continue => ExecutionState::Running,
                    DebugCommand::Stop => ExecutionState::Stopped,
                    DebugCommand::StepOver | DebugCommand::StepInto | DebugCommand::StepOut => {
                        ExecutionState::Paused {
                            location: "next_instruction".to_string(),
                            reason: "step_command".to_string(),
                        }
                    }
                    DebugCommand::Restart => {
                        session.call_stack.clear();
                        ExecutionState::Running
                    }
                };
                session.state.clone()
            } else {
                ExecutionState::Error {
                    message: "Session not found".to_string(),
                }
            }
        };

        // Broadcast state change
        let _ = self.state_sender.send(new_state);

        println!("🎮 Executed debug command: {:?}", command);
        Ok(())
    }

    /// Check if execution should pause at current location
    pub fn should_pause(&self, location: &str, context: &HashMap<String, String>) -> bool {
        let current_session = self.current_session.lock().unwrap();
        if let Some(session_id) = current_session.as_ref() {
            let sessions = self.sessions.lock().unwrap();
            if let Some(session) = sessions.get(session_id) {
                for breakpoint in session.breakpoints.values() {
                    if !breakpoint.enabled {
                        continue;
                    }

                    let location_matches = match &breakpoint.location {
                        BreakpointLocation::Function { name } => location.contains(name),
                        BreakpointLocation::Line { file, line: _ } => location.contains(file),
                        BreakpointLocation::Render { chart_type } => {
                            location.contains("render")
                                && context.get("chart_type") == Some(chart_type)
                        }
                        BreakpointLocation::DataProcessing { operation } => {
                            location.contains("data") && context.get("operation") == Some(operation)
                        }
                        BreakpointLocation::Performance { threshold_ms } => {
                            if let Some(duration_str) = context.get("duration_ms") {
                                if let Ok(duration_ms) = duration_str.parse::<u64>() {
                                    return duration_ms > *threshold_ms;
                                }
                            }
                            false
                        }
                    };

                    if location_matches {
                        if let Some(condition) = &breakpoint.condition {
                            if self.evaluate_condition(condition, context) {
                                return true;
                            }
                        } else {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Inspect variable state
    pub fn inspect_variables(&self, scope: &str) -> Result<Vec<VariableInfo>, DebuggerError> {
        // Mock implementation - in real debugger would inspect actual runtime state
        let mut variables = Vec::new();

        match scope {
            "chart" => {
                variables.push(VariableInfo {
                    name: "spec".to_string(),
                    value: "ChartSpec { mark: Line, ... }".to_string(),
                    type_name: "ChartSpec".to_string(),
                    children: vec![VariableInfo {
                        name: "mark".to_string(),
                        value: "Line { stroke_width: 2.0 }".to_string(),
                        type_name: "MarkType".to_string(),
                        children: Vec::new(),
                        memory_address: Some("0x7fff5fbff890".to_string()),
                    }],
                    memory_address: Some("0x7fff5fbff880".to_string()),
                });
            }
            "data" => {
                variables.push(VariableInfo {
                    name: "dataframe".to_string(),
                    value: "DataFrame { shape: (1000, 5) }".to_string(),
                    type_name: "polars::DataFrame".to_string(),
                    children: Vec::new(),
                    memory_address: Some("0x7fff5fbff8a0".to_string()),
                });
            }
            _ => {
                return Err(DebuggerError::StateInspectionFailed(format!(
                    "Unknown scope: {}",
                    scope
                )));
            }
        }

        Ok(variables)
    }

    /// Get current call stack
    pub fn get_call_stack(&self) -> Result<Vec<StackFrame>, DebuggerError> {
        let current_session = self.current_session.lock().unwrap();
        let session_id = current_session.as_ref().ok_or(DebuggerError::NotStarted)?;

        let sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get(session_id) {
            Ok(session.call_stack.clone())
        } else {
            Ok(Vec::new())
        }
    }

    /// Create enhanced error message with context and suggestions
    pub fn enhance_error(&self, error: &str, location: Option<&str>) -> EnhancedError {
        if !self.enhanced_errors {
            return EnhancedError {
                original_error: error.to_string(),
                error_type: "Unknown".to_string(),
                location: location.map(|s| s.to_string()),
                context: HashMap::new(),
                suggestions: Vec::new(),
                related_documentation: Vec::new(),
                severity: ErrorSeverity::Error,
            };
        }

        let (error_type, suggestions, docs, severity) = self.analyze_error(error);
        let context = self.gather_error_context(error, location);

        EnhancedError {
            original_error: error.to_string(),
            error_type,
            location: location.map(|s| s.to_string()),
            context,
            suggestions,
            related_documentation: docs,
            severity,
        }
    }

    /// Analyze error and provide suggestions
    fn analyze_error(&self, error: &str) -> (String, Vec<String>, Vec<String>, ErrorSeverity) {
        let error_lower = error.to_lowercase();

        if error_lower.contains("webgpu") {
            (
                "WebGPU Error".to_string(),
                vec![
                    "Check if WebGPU is supported in current browser".to_string(),
                    "Verify GPU drivers are up to date".to_string(),
                    "Consider enabling WebGL2 fallback".to_string(),
                ],
                vec![
                    "https://docs.helios.dev/webgpu-support".to_string(),
                    "https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API".to_string(),
                ],
                ErrorSeverity::Error,
            )
        } else if error_lower.contains("polars") || error_lower.contains("dataframe") {
            (
                "Data Processing Error".to_string(),
                vec![
                    "Verify data format and schema compatibility".to_string(),
                    "Check for null values or invalid data types".to_string(),
                    "Consider using lazy evaluation for large datasets".to_string(),
                ],
                vec![
                    "https://docs.helios.dev/data-processing".to_string(),
                    "https://docs.pola.rs/".to_string(),
                ],
                ErrorSeverity::Error,
            )
        } else if error_lower.contains("leptos") || error_lower.contains("signal") {
            (
                "Reactive System Error".to_string(),
                vec![
                    "Ensure reactive updates are properly tracked".to_string(),
                    "Check for signal disposal and cleanup".to_string(),
                    "Verify component lifecycle management".to_string(),
                ],
                vec![
                    "https://docs.helios.dev/reactive-system".to_string(),
                    "https://leptos.dev/".to_string(),
                ],
                ErrorSeverity::Warning,
            )
        } else if error_lower.contains("performance") || error_lower.contains("timeout") {
            (
                "Performance Issue".to_string(),
                vec![
                    "Enable performance profiling to identify bottlenecks".to_string(),
                    "Consider reducing data complexity or visualization detail".to_string(),
                    "Check for memory leaks or excessive allocations".to_string(),
                ],
                vec!["https://docs.helios.dev/performance-optimization".to_string()],
                ErrorSeverity::Warning,
            )
        } else {
            (
                "General Error".to_string(),
                vec![
                    "Check the documentation for similar issues".to_string(),
                    "Enable debug logging for more details".to_string(),
                    "Consider filing an issue with reproduction steps".to_string(),
                ],
                vec![
                    "https://docs.helios.dev/troubleshooting".to_string(),
                    "https://github.com/helios-viz/helios/issues".to_string(),
                ],
                ErrorSeverity::Error,
            )
        }
    }

    /// Gather error context information
    fn gather_error_context(&self, error: &str, location: Option<&str>) -> HashMap<String, String> {
        let mut context = HashMap::new();

        context.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());

        if let Some(loc) = location {
            context.insert("location".to_string(), loc.to_string());
        }

        context.insert(
            "browser_info".to_string(),
            "Chrome/120.0 (mock)".to_string(),
        );

        context.insert("webgpu_supported".to_string(), "true".to_string());

        context.insert("memory_usage".to_string(), "45MB".to_string());

        // Add error-specific context
        if error.to_lowercase().contains("webgpu") {
            context.insert("webgpu_adapter".to_string(), "Default adapter".to_string());
        }

        context
    }

    /// Evaluate breakpoint condition
    fn evaluate_condition(&self, condition: &str, context: &HashMap<String, String>) -> bool {
        // Simple condition evaluation - in real implementation would use proper parser
        if condition.contains("==") {
            let parts: Vec<&str> = condition.split("==").collect();
            if parts.len() == 2 {
                let left = parts[0].trim();
                let right = parts[1].trim().trim_matches('"');
                return context.get(left) == Some(&right.to_string());
            }
        }

        if condition.contains(">") {
            let parts: Vec<&str> = condition.split(">").collect();
            if parts.len() == 2 {
                let left = parts[0].trim();
                let right = parts[1].trim();
                if let (Some(left_val), Ok(right_val)) = (context.get(left), right.parse::<f64>()) {
                    if let Ok(left_num) = left_val.parse::<f64>() {
                        return left_num > right_val;
                    }
                }
            }
        }

        // Default to true for simple conditions
        true
    }

    /// Subscribe to debug commands
    pub fn subscribe_to_commands(&self) -> broadcast::Receiver<DebugCommand> {
        self.command_sender.subscribe()
    }

    /// Subscribe to execution state changes
    pub fn subscribe_to_state(&self) -> broadcast::Receiver<ExecutionState> {
        self.state_sender.subscribe()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_debug_session_lifecycle() {
        let mut debugger = InteractiveDebugger::new();

        // Start session
        let session_id = debugger.start_session("test_debug").unwrap();
        assert!(!session_id.is_empty());

        // Add breakpoint
        let bp_id = debugger
            .add_breakpoint(
                BreakpointLocation::Function {
                    name: "render".to_string(),
                },
                None,
            )
            .unwrap();
        assert!(!bp_id.is_empty());

        // Execute command
        debugger.execute_command(DebugCommand::StepOver).unwrap();

        // Remove breakpoint
        debugger.remove_breakpoint(&bp_id).unwrap();
    }

    #[test]
    fn test_breakpoint_evaluation() {
        let debugger = InteractiveDebugger::new();
        let mut context = HashMap::new();
        context.insert("chart_type".to_string(), "Line".to_string());

        // Should not pause with empty breakpoints
        assert!(!debugger.should_pause("render_line_chart", &context));
    }

    #[test]
    fn test_error_enhancement() {
        let debugger = InteractiveDebugger::new();
        let error = "WebGPU adapter not found";
        let enhanced = debugger.enhance_error(error, Some("render.rs:45"));

        assert_eq!(enhanced.error_type, "WebGPU Error");
        assert!(!enhanced.suggestions.is_empty());
        assert!(!enhanced.related_documentation.is_empty());
        assert!(matches!(enhanced.severity, ErrorSeverity::Error));
    }

    #[test]
    fn test_condition_evaluation() {
        let debugger = InteractiveDebugger::new();
        let mut context = HashMap::new();
        context.insert("duration_ms".to_string(), "150".to_string());

        // Test numeric comparison
        assert!(debugger.evaluate_condition("duration_ms > 100", &context));
        assert!(!debugger.evaluate_condition("duration_ms > 200", &context));

        // Test string equality
        context.insert("type".to_string(), "LineChart".to_string());
        assert!(debugger.evaluate_condition("type == \"LineChart\"", &context));
    }

    #[test]
    fn test_variable_inspection() {
        let debugger = InteractiveDebugger::new();

        let chart_vars = debugger.inspect_variables("chart").unwrap();
        assert!(!chart_vars.is_empty());
        assert_eq!(chart_vars[0].name, "spec");
        assert_eq!(chart_vars[0].type_name, "ChartSpec");
    }
}
