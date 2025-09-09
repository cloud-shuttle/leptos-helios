//! Developer Experience Tools
//!
//! This module integrates all development tools for an enhanced developer experience,
//! including hot reload, debugging, profiling, and code generation utilities.

pub use crate::debugger::{
    Breakpoint, BreakpointLocation, DebugCommand, DebuggerError, EnhancedError, ExecutionState,
    InteractiveDebugger, StackFrame, VariableInfo,
};
pub use crate::dev_server::{
    DevServer, DevServerConfig, DevServerError, FileChangeEvent, FileChangeType, HotReloadMessage,
    HotReloadMessageType, MockBrowserClient, MockFileWatcher,
};

use crate::chart::MarkType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Dev tools errors
#[derive(Debug, thiserror::Error)]
pub enum DevToolsError {
    #[error("Dev server error: {0}")]
    DevServerError(#[from] DevServerError),

    #[error("Debugger error: {0}")]
    DebuggerError(#[from] DebuggerError),

    #[error("Code generation failed: {0}")]
    CodeGenerationError(String),

    #[error("Profiling error: {0}")]
    ProfilingError(String),

    #[error("Linting error: {0}")]
    LintingError(String),

    #[error("Template error: {0}")]
    TemplateError(String),
}

impl From<String> for DevToolsError {
    fn from(err: String) -> Self {
        DevToolsError::LintingError(err)
    }
}

/// Performance profiling data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileData {
    pub function_name: String,
    pub duration_ms: f64,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub call_count: u64,
    pub children: Vec<ProfileData>,
}

/// Code generation template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeTemplate {
    pub name: String,
    pub description: String,
    pub language: String,
    pub template: String,
    pub placeholders: Vec<TemplatePlaceholder>,
}

/// Template placeholder for code generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatePlaceholder {
    pub name: String,
    pub description: String,
    pub default_value: Option<String>,
    pub required: bool,
}

/// Live reload configuration for different file types
#[derive(Debug, Clone)]
pub struct LiveReloadConfig {
    pub rust_files: bool,
    pub css_files: bool,
    pub js_files: bool,
    pub html_files: bool,
    pub asset_files: bool,
    pub custom_extensions: Vec<String>,
}

impl Default for LiveReloadConfig {
    fn default() -> Self {
        Self {
            rust_files: true,
            css_files: true,
            js_files: true,
            html_files: true,
            asset_files: true,
            custom_extensions: vec!["toml".to_string(), "json".to_string()],
        }
    }
}

/// Integrated development environment
pub struct HeliosDev {
    dev_server: Option<DevServer>,
    debugger: InteractiveDebugger,
    profiler: PerformanceProfiler,
    code_generator: CodeGenerator,
    linter: CodeLinter,
    project_root: PathBuf,
    config: DevToolsConfig,
}

/// Development tools configuration
#[derive(Debug, Clone)]
pub struct DevToolsConfig {
    pub dev_server_port: u16,
    pub enable_hot_reload: bool,
    pub enable_debugging: bool,
    pub enable_profiling: bool,
    pub enable_linting: bool,
    pub live_reload: LiveReloadConfig,
    pub auto_save: bool,
    pub show_performance_overlay: bool,
}

impl Default for DevToolsConfig {
    fn default() -> Self {
        Self {
            dev_server_port: 3000,
            enable_hot_reload: true,
            enable_debugging: true,
            enable_profiling: true,
            enable_linting: true,
            live_reload: LiveReloadConfig::default(),
            auto_save: true,
            show_performance_overlay: false,
        }
    }
}

impl HeliosDev {
    /// Create a new development environment
    pub fn new<P: AsRef<Path>>(project_root: P) -> Self {
        let project_root = project_root.as_ref().to_path_buf();
        let config = DevToolsConfig::default();

        Self {
            dev_server: None,
            debugger: InteractiveDebugger::new(),
            profiler: PerformanceProfiler::new(),
            code_generator: CodeGenerator::new(),
            linter: CodeLinter::new(),
            project_root,
            config,
        }
    }

    /// Start the development environment
    pub async fn start(&mut self) -> Result<(), DevToolsError> {
        println!("🚀 Starting Helios Development Environment...");

        // Start dev server if enabled
        if self.config.enable_hot_reload {
            let mut server = DevServer::new(&self.project_root, self.config.dev_server_port);
            server.start_with_websockets().await?;
            self.dev_server = Some(server);
            println!("✅ Development server started");
        }

        // Initialize debugger if enabled
        if self.config.enable_debugging {
            let session_id = self.debugger.start_session("main")?;
            println!("✅ Debug session started: {}", session_id);
        }

        // Start profiler if enabled
        if self.config.enable_profiling {
            self.profiler.start_profiling()?;
            println!("✅ Performance profiler started");
        }

        // Initialize linter if enabled
        if self.config.enable_linting {
            self.linter.initialize(&self.project_root).await?;
            println!("✅ Code linter initialized");
        }

        println!("🎉 Development environment ready!");
        Ok(())
    }

    /// Stop the development environment
    pub fn stop(&mut self) {
        if let Some(server) = &mut self.dev_server {
            server.stop();
        }

        self.profiler.stop_profiling();
        println!("🛑 Development environment stopped");
    }

    /// Generate chart component code
    pub fn generate_chart_component(
        &self,
        name: &str,
        chart_type: MarkType,
        options: GenerationOptions,
    ) -> Result<String, DevToolsError> {
        self.code_generator
            .generate_chart_component(name, chart_type, options)
            .map_err(DevToolsError::CodeGenerationError)
    }

    /// Add debug breakpoint
    pub fn add_breakpoint(
        &mut self,
        location: BreakpointLocation,
        condition: Option<String>,
    ) -> Result<String, DevToolsError> {
        Ok(self.debugger.add_breakpoint(location, condition)?)
    }

    /// Start performance profiling for specific operation
    pub fn profile_operation(&self, name: &str) -> ProfileScope {
        self.profiler.start_operation(name)
    }

    /// Get performance profiling results
    pub fn get_profile_data(&self) -> Vec<ProfileData> {
        self.profiler.get_profile_data()
    }

    /// Lint project files
    pub async fn lint_project(&self) -> Result<Vec<LintResult>, DevToolsError> {
        self.linter
            .lint_project(&self.project_root)
            .await
            .map_err(DevToolsError::LintingError)
    }

    /// Watch files for changes and trigger appropriate actions
    pub fn watch_files(&self) -> MockFileWatcher {
        if let Some(server) = &self.dev_server {
            server.file_watcher()
        } else {
            // Create a mock watcher for testing
            use tokio::sync::broadcast;
            let (_, rx) = broadcast::channel(100);
            MockFileWatcher::new(rx)
        }
    }

    /// Create new project from template
    pub async fn create_project_from_template(
        &self,
        template_name: &str,
        project_name: &str,
        output_dir: &Path,
    ) -> Result<(), DevToolsError> {
        self.code_generator
            .create_project_from_template(template_name, project_name, output_dir)
            .await
            .map_err(DevToolsError::TemplateError)
    }
}

/// Performance profiler for development insights
pub struct PerformanceProfiler {
    active_profiles: Arc<Mutex<HashMap<String, ProfileScope>>>,
    completed_profiles: Arc<Mutex<Vec<ProfileData>>>,
    enabled: bool,
}

impl PerformanceProfiler {
    fn new() -> Self {
        Self {
            active_profiles: Arc::new(Mutex::new(HashMap::new())),
            completed_profiles: Arc::new(Mutex::new(Vec::new())),
            enabled: false,
        }
    }

    fn start_profiling(&mut self) -> Result<(), DevToolsError> {
        self.enabled = true;
        Ok(())
    }

    fn stop_profiling(&mut self) {
        self.enabled = false;
    }

    fn start_operation(&self, name: &str) -> ProfileScope {
        if !self.enabled {
            return ProfileScope::disabled();
        }

        let scope = ProfileScope::new(name);
        let mut profiles = self.active_profiles.lock().unwrap();
        profiles.insert(name.to_string(), scope.clone());
        scope
    }

    fn get_profile_data(&self) -> Vec<ProfileData> {
        self.completed_profiles.lock().unwrap().clone()
    }
}

/// Performance profiling scope
#[derive(Clone)]
pub struct ProfileScope {
    name: String,
    start_time: Instant,
    enabled: bool,
}

impl ProfileScope {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            start_time: Instant::now(),
            enabled: true,
        }
    }

    fn disabled() -> Self {
        Self {
            name: String::new(),
            start_time: Instant::now(),
            enabled: false,
        }
    }

    pub fn finish(self) -> ProfileData {
        if !self.enabled {
            return ProfileData {
                function_name: self.name,
                duration_ms: 0.0,
                memory_usage_mb: 0.0,
                cpu_usage_percent: 0.0,
                call_count: 0,
                children: Vec::new(),
            };
        }

        let duration = self.start_time.elapsed();
        ProfileData {
            function_name: self.name,
            duration_ms: duration.as_secs_f64() * 1000.0,
            memory_usage_mb: 25.0,   // Mock memory usage
            cpu_usage_percent: 15.0, // Mock CPU usage
            call_count: 1,
            children: Vec::new(),
        }
    }
}

/// Code generation utilities
pub struct CodeGenerator {
    templates: HashMap<String, CodeTemplate>,
}

/// Code generation options
#[derive(Debug, Clone)]
pub struct GenerationOptions {
    pub include_typescript: bool,
    pub include_tests: bool,
    pub include_docs: bool,
    pub framework: String,
}

impl Default for GenerationOptions {
    fn default() -> Self {
        Self {
            include_typescript: true,
            include_tests: true,
            include_docs: true,
            framework: "leptos".to_string(),
        }
    }
}

impl CodeGenerator {
    fn new() -> Self {
        let mut generator = Self {
            templates: HashMap::new(),
        };
        generator.initialize_templates();
        generator
    }

    fn initialize_templates(&mut self) {
        // Chart component template
        let chart_template = CodeTemplate {
            name: "chart_component".to_string(),
            description: "Generate a new chart component".to_string(),
            language: "rust".to_string(),
            template: r#"//! {{component_name}} Chart Component
//!
//! {{description}}

use helios_core::prelude::*;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// {{component_name}} component properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct {{component_name}}Props {
    pub data: DataFrame,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub title: Option<String>,
    {{#if include_typescript}}
    pub on_click: Option<Callback<ChartEvent>>,
    {{/if}}
}

/// {{component_name}} chart component
#[component]
pub fn {{component_name}}(props: {{component_name}}Props) -> impl IntoView {
    let chart_spec = ChartSpec::new()
        .mark({{mark_type}})
        .width(props.width.unwrap_or(400))
        .height(props.height.unwrap_or(300));

    {{#if include_typescript}}
    let handle_click = move |event: ChartEvent| {
        if let Some(callback) = &props.on_click {
            callback.call(event);
        }
    };
    {{/if}}

    view! {
        <div class="helios-{{component_name_lower}}-chart">
            {{#if title}}
            <h3 class="chart-title">{props.title}</h3>
            {{/if}}
            <HeliosChart
                spec={chart_spec}
                data={props.data}
                {{#if include_typescript}}
                on:click=handle_click
                {{/if}}
            />
        </div>
    }
}

{{#if include_tests}}
#[cfg(test)]
mod tests {
    use super::*;
    use polars::prelude::*;

    #[test]
    fn test_{{component_name_lower}}_creation() {
        let data = df! {
            "x" => [1, 2, 3, 4],
            "y" => [10, 20, 15, 25],
        }.unwrap();

        let props = {{component_name}}Props {
            data,
            width: Some(800),
            height: Some(600),
            title: Some("Test Chart".to_string()),
            {{#if include_typescript}}
            on_click: None,
            {{/if}}
        };

        // Component creation should not panic
        let _component = {{component_name}}(props);
    }
}
{{/if}}
"#
            .to_string(),
            placeholders: vec![
                TemplatePlaceholder {
                    name: "component_name".to_string(),
                    description: "Name of the chart component".to_string(),
                    default_value: Some("MyChart".to_string()),
                    required: true,
                },
                TemplatePlaceholder {
                    name: "description".to_string(),
                    description: "Component description".to_string(),
                    default_value: Some("A custom chart component".to_string()),
                    required: false,
                },
                TemplatePlaceholder {
                    name: "mark_type".to_string(),
                    description: "Chart mark type".to_string(),
                    default_value: Some("MarkType::Line".to_string()),
                    required: true,
                },
            ],
        };

        self.templates
            .insert("chart_component".to_string(), chart_template);
    }

    fn generate_chart_component(
        &self,
        name: &str,
        chart_type: MarkType,
        options: GenerationOptions,
    ) -> Result<String, String> {
        let template = self
            .templates
            .get("chart_component")
            .ok_or("Chart component template not found")?;

        let mark_type_str = match chart_type {
            MarkType::Line { .. } => {
                "MarkType::Line { interpolate: None, stroke_width: None, stroke_dash: None }"
            }
            MarkType::Bar { .. } => "MarkType::Bar { width: None, corner_radius: None }",
            MarkType::Point { .. } => "MarkType::Point { size: None, shape: None, opacity: None }",
            MarkType::Area { .. } => "MarkType::Area { interpolate: None, opacity: None }",
            _ => "MarkType::Line { interpolate: None, stroke_width: None, stroke_dash: None }",
        };

        let mut code = template.template.clone();
        code = code.replace("{{component_name}}", name);
        code = code.replace("{{component_name_lower}}", &name.to_lowercase());
        code = code.replace("{{mark_type}}", mark_type_str);
        code = code.replace("{{description}}", &format!("A {} chart component", name));

        // Handle conditional blocks (simplified)
        if options.include_typescript {
            code = code.replace("{{#if include_typescript}}", "");
            code = code.replace("{{/if}}", "");
        } else {
            // Remove typescript blocks
            let lines: Vec<&str> = code.lines().collect();
            let mut filtered_lines = Vec::new();
            let mut skip = false;

            for line in lines {
                if line.contains("{{#if include_typescript}}") {
                    skip = true;
                    continue;
                }
                if line.contains("{{/if}}") && skip {
                    skip = false;
                    continue;
                }
                if !skip {
                    filtered_lines.push(line);
                }
            }
            code = filtered_lines.join("\n");
        }

        if options.include_tests {
            code = code.replace("{{#if include_tests}}", "");
        } else {
            // Remove test blocks
            let lines: Vec<&str> = code.lines().collect();
            let mut filtered_lines = Vec::new();
            let mut skip = false;

            for line in lines {
                if line.contains("{{#if include_tests}}") {
                    skip = true;
                    continue;
                }
                if line.contains("{{/if}}") && skip {
                    skip = false;
                    continue;
                }
                if !skip {
                    filtered_lines.push(line);
                }
            }
            code = filtered_lines.join("\n");
        }

        // Clean up remaining template syntax
        code = code.replace("{{#if title}}", "");
        code = code.replace("{{/if}}", "");

        Ok(code)
    }

    async fn create_project_from_template(
        &self,
        _template_name: &str,
        _project_name: &str,
        _output_dir: &Path,
    ) -> Result<(), String> {
        // Mock implementation - would create actual project structure
        Ok(())
    }
}

/// Code linting utilities
pub struct CodeLinter {
    rules: Vec<LintRule>,
}

/// Lint rule definition
#[derive(Debug, Clone)]
pub struct LintRule {
    pub name: String,
    pub description: String,
    pub severity: LintSeverity,
    pub enabled: bool,
}

/// Lint severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum LintSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

/// Lint result for a file
#[derive(Debug, Clone)]
pub struct LintResult {
    pub file_path: String,
    pub rule_name: String,
    pub message: String,
    pub severity: LintSeverity,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

impl CodeLinter {
    fn new() -> Self {
        Self {
            rules: Self::default_rules(),
        }
    }

    fn default_rules() -> Vec<LintRule> {
        vec![
            LintRule {
                name: "unused_imports".to_string(),
                description: "Detect unused imports".to_string(),
                severity: LintSeverity::Warning,
                enabled: true,
            },
            LintRule {
                name: "missing_docs".to_string(),
                description: "Public items should have documentation".to_string(),
                severity: LintSeverity::Info,
                enabled: true,
            },
            LintRule {
                name: "performance_warning".to_string(),
                description: "Detect potential performance issues".to_string(),
                severity: LintSeverity::Hint,
                enabled: true,
            },
        ]
    }

    async fn initialize(&mut self, _project_root: &Path) -> Result<(), String> {
        // Initialize linting configuration
        Ok(())
    }

    async fn lint_project(&self, project_root: &Path) -> Result<Vec<LintResult>, String> {
        // Mock implementation - would actually lint files
        Ok(vec![
            LintResult {
                file_path: project_root
                    .join("src/main.rs")
                    .to_string_lossy()
                    .to_string(),
                rule_name: "unused_imports".to_string(),
                message: "Unused import `std::collections::HashMap`".to_string(),
                severity: LintSeverity::Warning,
                line: Some(3),
                column: Some(5),
            },
            LintResult {
                file_path: project_root
                    .join("src/chart.rs")
                    .to_string_lossy()
                    .to_string(),
                rule_name: "missing_docs".to_string(),
                message: "Public function `render` is missing documentation".to_string(),
                severity: LintSeverity::Info,
                line: Some(45),
                column: Some(1),
            },
        ])
    }
}

/// Development utilities and helpers
pub mod utils {
    use super::*;

    /// Generate component boilerplate
    pub fn generate_component_boilerplate(name: &str) -> String {
        format!(
            r#"//! {} Component
//!
//! Generated by Helios Dev Tools

use leptos::prelude::*;

#[component]
pub fn {}() -> impl IntoView {{
    view! {{
        <div class="helios-{}">
            <p>"Hello from {}!"</p>
        </div>
    }}
}}
"#,
            name,
            name,
            name.to_lowercase(),
            name
        )
    }

    /// Format code using rustfmt
    pub async fn format_code(code: &str) -> Result<String, String> {
        // Mock implementation - would use rustfmt
        Ok(code.to_string())
    }

    /// Generate documentation from code
    pub async fn generate_docs(project_root: &Path) -> Result<String, String> {
        // Mock implementation - would generate docs
        Ok(format!(
            "# Documentation for {}\n\nGenerated by Helios Dev Tools",
            project_root.file_name().unwrap().to_string_lossy()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_helios_dev_creation() {
        let temp_dir = tempdir().unwrap();
        let dev = HeliosDev::new(temp_dir.path());

        assert_eq!(dev.project_root, temp_dir.path());
        assert_eq!(dev.config.dev_server_port, 3000);
    }

    #[tokio::test]
    async fn test_code_generation() {
        let temp_dir = tempdir().unwrap();
        let dev = HeliosDev::new(temp_dir.path());

        let code = dev
            .generate_chart_component(
                "LineChart",
                MarkType::Line {
                    interpolate: None,
                    stroke_width: None,
                    stroke_dash: None,
                },
                GenerationOptions::default(),
            )
            .unwrap();

        assert!(code.contains("LineChart"));
        assert!(code.contains("MarkType::Line"));
        assert!(code.contains("#[component]"));
    }

    #[test]
    fn test_performance_profiling() {
        use std::time::Duration;
        let profiler = PerformanceProfiler::new();
        let scope = profiler.start_operation("test_function");

        // Simulate some work
        std::thread::sleep(Duration::from_millis(10));

        let profile_data = scope.finish();
        assert_eq!(profile_data.function_name, "test_function");
        assert!(profile_data.duration_ms >= 10.0);
    }

    #[tokio::test]
    async fn test_code_linting() {
        let temp_dir = tempdir().unwrap();
        let linter = CodeLinter::new();

        let results = linter.lint_project(temp_dir.path()).await.unwrap();
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.rule_name == "unused_imports"));
    }

    #[test]
    fn test_utils_component_generation() {
        let code = utils::generate_component_boilerplate("TestComponent");
        assert!(code.contains("TestComponent"));
        assert!(code.contains("#[component]"));
        assert!(code.contains("Hello from TestComponent!"));
    }
}
