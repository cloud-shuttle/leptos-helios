//! Helios WASM Core - Minimal WASM-compatible version
//!
//! This crate provides a minimal, WASM-compatible version of Helios core
//! that focuses on WebGPU rendering without heavy data processing dependencies.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use wasm_bindgen::prelude::*;

/// Initialize Helios for WASM
#[wasm_bindgen]
pub async fn init_helios() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}

/// Get Helios version
#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Simple chart specification for WASM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmChartSpec {
    pub chart_type: String,
    pub data: Vec<f32>,
    pub width: u32,
    pub height: u32,
    pub title: Option<String>,
}

/// Create a simple chart specification
#[wasm_bindgen]
pub fn create_chart_spec(
    chart_type: &str,
    data: &[f32],
    width: u32,
    height: u32,
    title: Option<String>,
) -> Result<JsValue, JsValue> {
    let spec = WasmChartSpec {
        chart_type: chart_type.to_string(),
        data: data.to_vec(),
        width,
        height,
        title,
    };

    serde_wasm_bindgen::to_value(&spec).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// WebGPU device wrapper for WASM
#[wasm_bindgen]
pub struct WasmWebGpuDevice {
    // We'll store the actual device here when we implement it
    _private: (),
}

#[wasm_bindgen]
impl WasmWebGpuDevice {
    /// Create a new WebGPU device
    #[wasm_bindgen]
    pub async fn new() -> Result<WasmWebGpuDevice, JsValue> {
        // TODO: Implement actual WebGPU device creation
        Ok(WasmWebGpuDevice { _private: () })
    }

    /// Check if WebGPU is available
    #[wasm_bindgen]
    pub fn is_available() -> bool {
        // TODO: Check actual WebGPU availability
        true
    }
}

/// Error types for WASM
#[derive(Error, Debug)]
pub enum WasmError {
    #[error("WebGPU not available")]
    WebGpuNotAvailable,
    #[error("Invalid chart specification: {0}")]
    InvalidChartSpec(String),
    #[error("Rendering error: {0}")]
    RenderingError(String),
}

impl From<WasmError> for JsValue {
    fn from(err: WasmError) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}
