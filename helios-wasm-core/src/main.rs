//! Helios WASM Binary Entry Point
//! 
//! This is the main entry point for the Helios WASM application.
//! It provides a simple interface for testing and demonstration purposes.

use wasm_bindgen::prelude::*;
use console_error_panic_hook;

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn main() {
    // Set up panic hook for better error messages in the browser
    console_error_panic_hook::set_once();
    
    // Log that the module has been initialized
    web_sys::console::log_1(&"🚀 Helios WASM Module Initialized".into());
}

/// Greet function for testing
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Helios!", name)
}

/// Process data function for testing
#[wasm_bindgen]
pub fn process_data(input: &str) -> String {
    format!("Processed: {}", input.to_uppercase())
}

/// Check WebGPU support
#[wasm_bindgen]
pub fn check_webgpu_support() -> bool {
    // In a real implementation, this would check for WebGPU support
    // For now, we'll return true as a placeholder
    true
}

/// Get sample chart configuration
#[wasm_bindgen]
pub fn get_sample_chart_config() -> String {
    serde_json::json!({
        "type": "line",
        "data": {
            "labels": ["A", "B", "C", "D"],
            "datasets": [{
                "label": "Sample Data",
                "data": [1, 2, 3, 4]
            }]
        }
    }).to_string()
}
