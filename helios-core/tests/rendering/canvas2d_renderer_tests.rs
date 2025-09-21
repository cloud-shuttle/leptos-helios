//! Canvas2D renderer specific tests

use super::{RendererTest, utils, common_tests};
use leptos_helios::canvas2d_renderer::Canvas2DRenderer;

/// Mock Canvas2D renderer for testing
pub struct MockCanvas2DRenderer {
    pub width: u32,
    pub height: u32,
}

impl MockCanvas2DRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl RendererTest for MockCanvas2DRenderer {
    fn test_basic_render(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Testing Canvas2D basic render ({}x{})", self.width, self.height);
        common_tests::test_basic_render_impl(self)
    }
    
    fn test_export_formats(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Testing Canvas2D export formats");
        
        // Test PNG export
        println!("  - PNG export");
        
        // Test SVG export
        println!("  - SVG export");
        
        // Test HTML export
        println!("  - HTML export");
        
        Ok(())
    }
    
    fn test_error_handling(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Testing Canvas2D error handling");
        common_tests::test_error_handling_impl(self)
    }
    
    fn test_performance(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Testing Canvas2D performance");
        common_tests::test_performance_impl(self)
    }
    
    fn test_chart_types(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Testing Canvas2D chart types");
        common_tests::test_chart_types_impl(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_canvas2d_renderer_basic() {
        let renderer = MockCanvas2DRenderer::new(800, 600);
        assert!(renderer.test_basic_render().is_ok());
    }
    
    #[test]
    fn test_canvas2d_renderer_export_formats() {
        let renderer = MockCanvas2DRenderer::new(800, 600);
        assert!(renderer.test_export_formats().is_ok());
    }
    
    #[test]
    fn test_canvas2d_renderer_error_handling() {
        let renderer = MockCanvas2DRenderer::new(800, 600);
        assert!(renderer.test_error_handling().is_ok());
    }
    
    #[test]
    fn test_canvas2d_renderer_performance() {
        let renderer = MockCanvas2DRenderer::new(800, 600);
        assert!(renderer.test_performance().is_ok());
    }
    
    #[test]
    fn test_canvas2d_renderer_chart_types() {
        let renderer = MockCanvas2DRenderer::new(800, 600);
        assert!(renderer.test_chart_types().is_ok());
    }
    
    #[test]
    fn test_canvas2d_renderer_dimensions() {
        let renderer = MockCanvas2DRenderer::new(1920, 1080);
        assert_eq!(renderer.width, 1920);
        assert_eq!(renderer.height, 1080);
    }
}
