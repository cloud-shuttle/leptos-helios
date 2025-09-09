//! Custom Components System
//!
//! This module provides a framework for creating custom chart components
//! that can be registered and used within the Helios theming system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

use crate::theme_engine::{ComponentType, ThemeError, ThemeValue};

/// Custom component errors
#[derive(Debug, Error)]
pub enum ComponentError {
    #[error("Component not found: {id}")]
    ComponentNotFound { id: String },

    #[error("Invalid component definition: {message}")]
    InvalidComponentDefinition { message: String },

    #[error("Component validation failed: {message}")]
    ComponentValidationFailed { message: String },

    #[error("Component rendering failed: {message}")]
    ComponentRenderingFailed { message: String },

    #[error("Component registration failed: {message}")]
    ComponentRegistrationFailed { message: String },
}

/// Component identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentId(pub String);

/// Component property definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentProperty {
    pub name: String,
    pub property_type: PropertyType,
    pub required: bool,
    pub default_value: Option<ThemeValue>,
    pub description: Option<String>,
    pub validation: Option<ValidationRule>,
}

/// Property types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyType {
    String,
    Number,
    Boolean,
    Color,
    Array,
    Object,
    Function,
}

/// Validation rules for component properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub allowed_values: Option<Vec<String>>,
    pub pattern: Option<String>,
    pub custom_validator: Option<String>,
}

/// Component event definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentEvent {
    pub name: String,
    pub event_type: EventType,
    pub description: Option<String>,
    pub parameters: Vec<ComponentProperty>,
}

/// Event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Click,
    Hover,
    Focus,
    Blur,
    Change,
    Custom(String),
}

/// Component lifecycle hooks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentLifecycle {
    pub on_mount: Option<String>,
    pub on_unmount: Option<String>,
    pub on_update: Option<String>,
    pub on_render: Option<String>,
}

/// Custom component definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomComponent {
    pub id: ComponentId,
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub component_type: ComponentType,
    pub properties: Vec<ComponentProperty>,
    pub events: Vec<ComponentEvent>,
    pub lifecycle: ComponentLifecycle,
    pub template: String,
    pub styles: HashMap<String, ThemeValue>,
    pub dependencies: Vec<String>,
    pub metadata: HashMap<String, ThemeValue>,
}

/// Component renderer trait
pub trait ComponentRenderer {
    fn render(
        &self,
        component: &CustomComponent,
        props: &HashMap<String, ThemeValue>,
    ) -> Result<String, ComponentError>;
    fn validate_props(
        &self,
        component: &CustomComponent,
        props: &HashMap<String, ThemeValue>,
    ) -> Result<(), ComponentError>;
    fn get_required_props(&self, component: &CustomComponent) -> Vec<String>;
}

/// Default component renderer
#[derive(Debug)]
pub struct DefaultComponentRenderer {
    template_engine: TemplateEngine,
    validator: ComponentValidator,
}

impl DefaultComponentRenderer {
    pub fn new() -> Self {
        Self {
            template_engine: TemplateEngine::new(),
            validator: ComponentValidator::new(),
        }
    }
}

impl ComponentRenderer for DefaultComponentRenderer {
    fn render(
        &self,
        component: &CustomComponent,
        props: &HashMap<String, ThemeValue>,
    ) -> Result<String, ComponentError> {
        // Validate props first
        self.validate_props(component, props)?;

        // Render the component
        self.template_engine
            .render(&component.template, props)
            .map_err(|e| ComponentError::ComponentRenderingFailed {
                message: format!("Template rendering failed: {}", e),
            })
    }

    fn validate_props(
        &self,
        component: &CustomComponent,
        props: &HashMap<String, ThemeValue>,
    ) -> Result<(), ComponentError> {
        self.validator.validate_component_props(component, props)
    }

    fn get_required_props(&self, component: &CustomComponent) -> Vec<String> {
        component
            .properties
            .iter()
            .filter(|prop| prop.required)
            .map(|prop| prop.name.clone())
            .collect()
    }
}

/// Template engine for component rendering
#[derive(Debug)]
pub struct TemplateEngine {
    // Simple template engine - can be enhanced with more sophisticated templating
}

impl TemplateEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        template: &str,
        props: &HashMap<String, ThemeValue>,
    ) -> Result<String, TemplateError> {
        let mut result = template.to_string();

        // Simple variable substitution
        for (key, value) in props {
            let placeholder = format!("{{{{{}}}}}", key);
            let value_str = self.theme_value_to_string(value);
            result = result.replace(&placeholder, &value_str);
        }

        Ok(result)
    }

    fn theme_value_to_string(&self, value: &ThemeValue) -> String {
        match value {
            ThemeValue::String(s) => s.clone(),
            ThemeValue::Number(n) => n.to_string(),
            ThemeValue::Boolean(b) => b.to_string(),
            ThemeValue::Color(c) => c.clone(),
            ThemeValue::Array(arr) => {
                let values: Vec<String> =
                    arr.iter().map(|v| self.theme_value_to_string(v)).collect();
                format!("[{}]", values.join(", "))
            }
            ThemeValue::Object(obj) => {
                let pairs: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, self.theme_value_to_string(v)))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
        }
    }
}

/// Template engine errors
#[derive(Debug, Error)]
pub enum TemplateError {
    #[error("Template rendering failed: {message}")]
    RenderingFailed { message: String },

    #[error("Invalid template syntax: {message}")]
    InvalidSyntax { message: String },

    #[error("Missing required variable: {variable}")]
    MissingVariable { variable: String },
}

/// Component validator
#[derive(Debug)]
pub struct ComponentValidator {
    // Component validation logic
}

impl ComponentValidator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn validate_component_props(
        &self,
        component: &CustomComponent,
        props: &HashMap<String, ThemeValue>,
    ) -> Result<(), ComponentError> {
        // Check required properties
        for prop in &component.properties {
            if prop.required && !props.contains_key(&prop.name) {
                return Err(ComponentError::ComponentValidationFailed {
                    message: format!("Required property '{}' is missing", prop.name),
                });
            }
        }

        // Validate property types and values
        for (prop_name, prop_value) in props {
            if let Some(prop_def) = component.properties.iter().find(|p| &p.name == prop_name) {
                self.validate_property_value(prop_def, prop_value)?;
            }
        }

        Ok(())
    }

    fn validate_property_value(
        &self,
        prop_def: &ComponentProperty,
        value: &ThemeValue,
    ) -> Result<(), ComponentError> {
        // Type validation
        match (&prop_def.property_type, value) {
            (PropertyType::String, ThemeValue::String(_)) => {}
            (PropertyType::Number, ThemeValue::Number(_)) => {}
            (PropertyType::Boolean, ThemeValue::Boolean(_)) => {}
            (PropertyType::Color, ThemeValue::Color(_)) => {}
            (PropertyType::Array, ThemeValue::Array(_)) => {}
            (PropertyType::Object, ThemeValue::Object(_)) => {}
            _ => {
                return Err(ComponentError::ComponentValidationFailed {
                    message: format!("Property '{}' has invalid type", prop_def.name),
                });
            }
        }

        // Custom validation rules
        if let Some(rule) = &prop_def.validation {
            self.validate_with_rule(prop_def, value, rule)?;
        }

        Ok(())
    }

    fn validate_with_rule(
        &self,
        prop_def: &ComponentProperty,
        value: &ThemeValue,
        rule: &ValidationRule,
    ) -> Result<(), ComponentError> {
        match value {
            ThemeValue::Number(n) => {
                if let Some(min) = rule.min_value {
                    if *n < min {
                        return Err(ComponentError::ComponentValidationFailed {
                            message: format!(
                                "Property '{}' value {} is below minimum {}",
                                prop_def.name, n, min
                            ),
                        });
                    }
                }
                if let Some(max) = rule.max_value {
                    if *n > max {
                        return Err(ComponentError::ComponentValidationFailed {
                            message: format!(
                                "Property '{}' value {} is above maximum {}",
                                prop_def.name, n, max
                            ),
                        });
                    }
                }
            }
            ThemeValue::String(s) => {
                if let Some(allowed) = &rule.allowed_values {
                    if !allowed.contains(s) {
                        return Err(ComponentError::ComponentValidationFailed {
                            message: format!(
                                "Property '{}' value '{}' is not in allowed values",
                                prop_def.name, s
                            ),
                        });
                    }
                }
                if let Some(pattern) = &rule.pattern {
                    // Simple pattern matching - can be enhanced with regex
                    if !s.contains(pattern) {
                        return Err(ComponentError::ComponentValidationFailed {
                            message: format!(
                                "Property '{}' value '{}' does not match pattern '{}'",
                                prop_def.name, s, pattern
                            ),
                        });
                    }
                }
            }
            _ => {} // Other types don't have specific validation rules yet
        }

        Ok(())
    }
}

/// Component factory for creating and managing custom components
#[derive(Debug)]
pub struct ComponentFactory {
    components: HashMap<ComponentId, CustomComponent>,
    renderer: Box<dyn ComponentRenderer>,
    validator: ComponentValidator,
}

impl ComponentFactory {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            renderer: Box::new(DefaultComponentRenderer::new()),
            validator: ComponentValidator::new(),
        }
    }

    pub fn with_renderer(renderer: Box<dyn ComponentRenderer>) -> Self {
        Self {
            components: HashMap::new(),
            renderer,
            validator: ComponentValidator::new(),
        }
    }

    /// Register a custom component
    pub fn register_component(&mut self, component: CustomComponent) -> Result<(), ComponentError> {
        // Validate component definition
        self.validator.validate_component_definition(&component)?;

        // Check for conflicts
        if self.components.contains_key(&component.id) {
            return Err(ComponentError::ComponentRegistrationFailed {
                message: format!(
                    "Component with ID '{}' is already registered",
                    component.id.0
                ),
            });
        }

        // Register the component
        self.components.insert(component.id.clone(), component);

        Ok(())
    }

    /// Get a component by ID
    pub fn get_component(&self, id: &ComponentId) -> Result<&CustomComponent, ComponentError> {
        self.components
            .get(id)
            .ok_or_else(|| ComponentError::ComponentNotFound { id: id.0.clone() })
    }

    /// Render a component
    pub fn render_component(
        &self,
        id: &ComponentId,
        props: &HashMap<String, ThemeValue>,
    ) -> Result<String, ComponentError> {
        let component = self.get_component(id)?;
        self.renderer.render(component, props)
    }

    /// List all registered components
    pub fn list_components(&self) -> Vec<&CustomComponent> {
        self.components.values().collect()
    }

    /// Get components by type
    pub fn get_components_by_type(&self, component_type: &ComponentType) -> Vec<&CustomComponent> {
        self.components
            .values()
            .filter(|component| &component.component_type == component_type)
            .collect()
    }

    /// Unregister a component
    pub fn unregister_component(&mut self, id: &ComponentId) -> Result<(), ComponentError> {
        self.components
            .remove(id)
            .ok_or_else(|| ComponentError::ComponentNotFound { id: id.0.clone() })?;
        Ok(())
    }

    /// Get component count
    pub fn component_count(&self) -> usize {
        self.components.len()
    }
}

impl ComponentValidator {
    fn validate_component_definition(
        &self,
        component: &CustomComponent,
    ) -> Result<(), ComponentError> {
        // Validate component ID
        if component.id.0.is_empty() {
            return Err(ComponentError::InvalidComponentDefinition {
                message: "Component ID cannot be empty".to_string(),
            });
        }

        // Validate component name
        if component.name.is_empty() {
            return Err(ComponentError::InvalidComponentDefinition {
                message: "Component name cannot be empty".to_string(),
            });
        }

        // Validate version
        if component.version.is_empty() {
            return Err(ComponentError::InvalidComponentDefinition {
                message: "Component version cannot be empty".to_string(),
            });
        }

        // Validate template
        if component.template.is_empty() {
            return Err(ComponentError::InvalidComponentDefinition {
                message: "Component template cannot be empty".to_string(),
            });
        }

        // Validate properties
        for prop in &component.properties {
            if prop.name.is_empty() {
                return Err(ComponentError::InvalidComponentDefinition {
                    message: "Component property name cannot be empty".to_string(),
                });
            }
        }

        // Validate events
        for event in &component.events {
            if event.name.is_empty() {
                return Err(ComponentError::InvalidComponentDefinition {
                    message: "Component event name cannot be empty".to_string(),
                });
            }
        }

        Ok(())
    }
}

impl Default for ComponentFactory {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a sample custom component
pub fn create_sample_component() -> CustomComponent {
    CustomComponent {
        id: ComponentId("sample-chart".to_string()),
        name: "Sample Chart Component".to_string(),
        description: Some("A sample custom chart component for demonstration".to_string()),
        version: "1.0.0".to_string(),
        component_type: ComponentType::Custom("sample-chart".to_string()),
        properties: vec![
            ComponentProperty {
                name: "title".to_string(),
                property_type: PropertyType::String,
                required: true,
                default_value: None,
                description: Some("Chart title".to_string()),
                validation: None,
            },
            ComponentProperty {
                name: "width".to_string(),
                property_type: PropertyType::Number,
                required: false,
                default_value: Some(ThemeValue::Number(400.0)),
                description: Some("Chart width in pixels".to_string()),
                validation: Some(ValidationRule {
                    min_value: Some(100.0),
                    max_value: Some(2000.0),
                    allowed_values: None,
                    pattern: None,
                    custom_validator: None,
                }),
            },
            ComponentProperty {
                name: "height".to_string(),
                property_type: PropertyType::Number,
                required: false,
                default_value: Some(ThemeValue::Number(300.0)),
                description: Some("Chart height in pixels".to_string()),
                validation: Some(ValidationRule {
                    min_value: Some(100.0),
                    max_value: Some(1500.0),
                    allowed_values: None,
                    pattern: None,
                    custom_validator: None,
                }),
            },
            ComponentProperty {
                name: "color".to_string(),
                property_type: PropertyType::Color,
                required: false,
                default_value: Some(ThemeValue::Color("#3b82f6".to_string())),
                description: Some("Chart color".to_string()),
                validation: None,
            },
        ],
        events: vec![
            ComponentEvent {
                name: "click".to_string(),
                event_type: EventType::Click,
                description: Some("Fired when the chart is clicked".to_string()),
                parameters: vec![],
            },
            ComponentEvent {
                name: "hover".to_string(),
                event_type: EventType::Hover,
                description: Some("Fired when hovering over the chart".to_string()),
                parameters: vec![],
            },
        ],
        lifecycle: ComponentLifecycle {
            on_mount: Some("console.log('Component mounted')".to_string()),
            on_unmount: Some("console.log('Component unmounted')".to_string()),
            on_update: Some("console.log('Component updated')".to_string()),
            on_render: Some("console.log('Component rendered')".to_string()),
        },
        template: r#"
<div class="sample-chart" style="width: {{width}}px; height: {{height}}px; background-color: {{color}};">
    <h3>{{title}}</h3>
    <div class="chart-content">
        <!-- Chart content will be rendered here -->
    </div>
</div>
        "#.to_string(),
        styles: HashMap::new(),
        dependencies: vec!["chart.js".to_string()],
        metadata: HashMap::new(),
    }
}
