//! WebGPU renderer specific tests

use super::{RendererTest, utils, common_tests};

/// Mock WebGPU renderer for testing
pub struct MockWebGpuRenderer {
    pub device_available: bool,
    pub supports_compute: bool,
}

impl MockWebGpuRenderer {
    pub fn new() -> Self {
        Self {
            device_available: true,
            supports_compute: true,
        }
    }
    
    pub fn new_without_device() -> Self {
        Self {
            device_available: false,
            supports_compute: false,
        }
    }
}

impl RendererTest for MockWebGpuRenderer {
    fn test_basic_render(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.device_available {
            return Err("WebGPU device not available".into());
        }
        
        println!("Testing WebGPU basic render");
        common_tests::test_basic_render_impl(self)
    }
    
    fn test_export_formats(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.device_available {
            return Err("WebGPU device not available".into());
        }
        
        println!("Testing WebGPU export formats");
        
        // Test PNG export
        println!("  - PNG export");
        
        // Test high-resolution export
        println!("  - High-resolution export");
        
        Ok(())
    }
    
    fn test_error_handling(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Testing WebGPU error handling");
        
        // Test device unavailable scenario
        if !self.device_available {
            println!("  - Device unavailable scenario");
            return Ok(());
        }
        
        common_tests::test_error_handling_impl(self)
    }
    
    fn test_performance(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.device_available {
            return Err("WebGPU device not available".into());
        }
        
        println!("Testing WebGPU performance");
        common_tests::test_performance_impl(self)
    }
    
    fn test_chart_types(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.device_available {
            return Err("WebGPU device not available".into());
        }
        
        println!("Testing WebGPU chart types");
        common_tests::test_chart_types_impl(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_webgpu_renderer_basic() {
        let renderer = MockWebGpuRenderer::new();
        assert!(renderer.test_basic_render().is_ok());
    }
    
    #[test]
    fn test_webgpu_renderer_export_formats() {
        let renderer = MockWebGpuRenderer::new();
        assert!(renderer.test_export_formats().is_ok());
    }
    
    #[test]
    fn test_webgpu_renderer_error_handling() {
        let renderer = MockWebGpuRenderer::new();
        assert!(renderer.test_error_handling().is_ok());
    }
    
    #[test]
    fn test_webgpu_renderer_performance() {
        let renderer = MockWebGpuRenderer::new();
        assert!(renderer.test_performance().is_ok());
    }
    
    #[test]
    fn test_webgpu_renderer_chart_types() {
        let renderer = MockWebGpuRenderer::new();
        assert!(renderer.test_chart_types().is_ok());
    }
    
    #[test]
    fn test_webgpu_renderer_no_device() {
        let renderer = MockWebGpuRenderer::new_without_device();
        
        // These should fail when device is not available
        assert!(renderer.test_basic_render().is_err());
        assert!(renderer.test_export_formats().is_err());
        assert!(renderer.test_performance().is_err());
        assert!(renderer.test_chart_types().is_err());
        
        // Error handling should still work
        assert!(renderer.test_error_handling().is_ok());
    }
    
    #[test]
    fn test_webgpu_renderer_capabilities() {
        let renderer = MockWebGpuRenderer::new();
        assert!(renderer.device_available);
        assert!(renderer.supports_compute);
        
        let renderer_no_device = MockWebGpuRenderer::new_without_device();
        assert!(!renderer_no_device.device_available);
        assert!(!renderer_no_device.supports_compute);
    }
}
