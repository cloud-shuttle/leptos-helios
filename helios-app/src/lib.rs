//! Helios Web Application - Main entry point

use wasm_bindgen::prelude::*;

// Import the console.log function from the browser
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro to make console.log easier to use
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Initialize the panic hook for better error messages
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log!("Helios Web Application initialized successfully!");
}

// Export a simple function to test the WASM module
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Helios!", name)
}

// Export a function to test data processing
#[wasm_bindgen]
pub fn process_data(data: &str) -> String {
    console_log!("Processing data: {}", data);
    format!("Processed: {}", data.to_uppercase())
}

// Export a function to test chart creation
#[wasm_bindgen]
pub fn create_simple_chart() -> String {
    console_log!("Creating simple chart");
    r#"{
        "type": "line",
        "data": {
            "labels": ["A", "B", "C", "D"],
            "datasets": [{
                "label": "Sample Data",
                "data": [1, 2, 3, 4]
            }]
        }
    }"#
    .to_string()
}

// Export a function to test WebGPU capabilities
#[wasm_bindgen]
pub fn test_webgpu_support() -> String {
    console_log!("Testing WebGPU support");

    // Check if WebGPU is available in the browser
    if let Ok(webgpu_available) = check_webgpu_availability() {
        console_log!("WebGPU support check result: {}", webgpu_available);
        if webgpu_available {
            "Supported".to_string()
        } else {
            "Not Supported".to_string()
        }
    } else {
        console_log!("WebGPU support check failed");
        "Error".to_string()
    }
}

// Helper function to check WebGPU availability using JavaScript interop
fn check_webgpu_availability() -> Result<bool, JsValue> {
    // Use JavaScript to check for WebGPU support
    let window = web_sys::window().ok_or("No window object")?;

    // Access navigator through JavaScript reflection
    let navigator = js_sys::Reflect::get(&window, &JsValue::from_str("navigator"))?;

    if navigator.is_undefined() {
        return Ok(false);
    }

    // Check if navigator.gpu exists (WebGPU API)
    let gpu = js_sys::Reflect::get(&navigator, &JsValue::from_str("gpu"))?;

    if gpu.is_undefined() {
        return Ok(false);
    }

    // If we get here, WebGPU is available
    Ok(true)
}

// Export a function to create chart data
#[wasm_bindgen]
pub fn create_chart_data(chart_type: &str, data_points: usize) -> String {
    console_log!(
        "Creating chart data for type: {} with {} points",
        chart_type,
        data_points
    );

    let mut data = Vec::new();

    for i in 0..data_points {
        let value = match chart_type {
            "line" | "bar" => {
                // Generate sine wave with some noise
                let x = i as f64 * 0.5;
                (x.sin() * 50.0 + 50.0 + (i as f64 * 0.1).sin() * 10.0) as f32
            }
            "scatter" => {
                // Generate random scatter data
                ((i as f64 * 7.3) % 100.0) as f32
            }
            "heatmap" => {
                // Generate heatmap values
                ((i as f64 * 11.7) % 100.0) as f32
            }
            _ => i as f32,
        };

        data.push(serde_json::json!({
            "x": i,
            "y": value,
            "value": value
        }));
    }

    serde_json::to_string(&data).unwrap_or_else(|_| "[]".to_string())
}

// Export a function to process chart data
#[wasm_bindgen]
pub fn process_chart_data(data: &str) -> String {
    console_log!("Processing chart data");

    // Process chart data (e.g., normalize, filter, etc.)
    match serde_json::from_str::<Vec<serde_json::Value>>(data) {
        Ok(mut parsed_data) => {
            // Normalize data to 0-100 range
            if let Some(max_val) = parsed_data
                .iter()
                .filter_map(|v| v.get("y").and_then(|y| y.as_f64()))
                .fold(None, |acc, val| Some(acc.map_or(val, |a: f64| a.max(val))))
            {
                if max_val > 0.0 {
                    for item in &mut parsed_data {
                        if let Some(y) = item.get_mut("y") {
                            if let Some(val) = y.as_f64() {
                                *y = serde_json::Value::Number(
                                    serde_json::Number::from_f64((val / max_val) * 100.0)
                                        .unwrap_or(serde_json::Number::from(0)),
                                );
                            }
                        }
                    }
                }
            }

            serde_json::to_string(&parsed_data).unwrap_or_else(|_| "[]".to_string())
        }
        Err(_) => "[]".to_string(),
    }
}
