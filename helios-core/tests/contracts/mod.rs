//! Contract tests for API stability and behavior consistency
//!
//! This module contains contract tests that ensure all implementations
//! conform to expected interfaces and behavior patterns.

pub mod renderer_contract;
pub mod export_contract;
pub mod wasm_contract;

// Re-export contract test functions for easy access
pub use renderer_contract::*;
pub use export_contract::*;
pub use wasm_contract::*;
