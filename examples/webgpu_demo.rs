//! WebGPU Demo
//!
//! This example demonstrates the real WebGPU implementation working with actual GPU resources.

use leptos_helios::webgpu_real::*;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 WebGPU Demo - Real Implementation");
    println!("=====================================");

    // Check WebGPU support
    let supported = WebGpuRealRenderer::is_webgpu_supported();
    println!(
        "WebGPU Support: {}",
        if supported {
            "✅ Supported"
        } else {
            "❌ Not Supported"
        }
    );

    if !supported {
        println!("⚠️  WebGPU is not supported in this environment");
        return Ok(());
    }

    // Create renderer
    println!("\n📱 Creating WebGPU Renderer...");
    let start_time = Instant::now();

    let renderer_result = WebGpuRealRenderer::new(None).await;
    let creation_time = start_time.elapsed();

    match renderer_result {
        Ok(renderer) => {
            println!("✅ Renderer created successfully in {:?}", creation_time);
            println!("Device info: {}", renderer.get_device_info());

            // Test shader compilation
            println!("\n🎨 Compiling Shaders...");
            let shader_start = Instant::now();

            let shader_source = r#"
                @vertex
                fn vs_main(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
                    return vec4<f32>(position, 0.0, 1.0);
                }

                @fragment
                fn fs_main() -> @location(0) vec4<f32> {
                    return vec4<f32>(0.0, 0.5, 1.0, 1.0); // Blue color
                }
            "#;

            let mut renderer = renderer;
            match renderer.compile_shader("demo", shader_source) {
                Ok(_) => {
                    let shader_time = shader_start.elapsed();
                    println!("✅ Shader compiled successfully in {:?}", shader_time);
                    println!("Shader cached: {}", renderer.has_shader("demo"));
                }
                Err(e) => {
                    println!("❌ Shader compilation failed: {}", e);
                    return Err(e.into());
                }
            }

            // Test pipeline creation
            println!("\n🔧 Creating Render Pipeline...");
            let pipeline_start = Instant::now();

            match renderer.create_line_pipeline() {
                Ok(_) => {
                    let pipeline_time = pipeline_start.elapsed();
                    println!(
                        "✅ Line pipeline created successfully in {:?}",
                        pipeline_time
                    );
                    println!("Pipeline cached: {}", renderer.has_pipeline("line"));
                }
                Err(e) => {
                    println!("❌ Pipeline creation failed: {}", e);
                    return Err(e.into());
                }
            }

            // Test vertex buffer creation
            println!("\n📊 Creating Vertex Buffer...");
            let buffer_start = Instant::now();

            // Create a simple sine wave
            let vertices: Vec<[f32; 2]> = (0..100)
                .map(|i| {
                    let x = (i as f32 / 100.0) * 2.0 - 1.0; // -1 to 1
                    let y = (i as f32 * 0.1).sin() * 0.5; // Sine wave
                    [x, y]
                })
                .collect();

            match renderer.create_vertex_buffer(&vertices) {
                Ok(buffer) => {
                    let buffer_time = buffer_start.elapsed();
                    println!("✅ Vertex buffer created successfully in {:?}", buffer_time);
                    println!("Buffer size: {} bytes", buffer.size());
                    println!("Vertices: {}", vertices.len());
                }
                Err(e) => {
                    println!("❌ Vertex buffer creation failed: {}", e);
                    return Err(e.into());
                }
            }

            // Performance test
            println!("\n⚡ Performance Test...");
            let perf_start = Instant::now();

            // Create a larger dataset
            let large_vertices: Vec<[f32; 2]> = (0..10000)
                .map(|i| {
                    let x = (i as f32 / 10000.0) * 2.0 - 1.0;
                    let y = (i as f32 * 0.01).sin() * 0.5;
                    [x, y]
                })
                .collect();

            match renderer.create_vertex_buffer(&large_vertices) {
                Ok(buffer) => {
                    let perf_time = perf_start.elapsed();
                    println!("✅ Large vertex buffer created in {:?}", perf_time);
                    println!("Large buffer size: {} bytes", buffer.size());
                    println!(
                        "Performance: {:.2} MB/s",
                        (buffer.size() as f64 / 1_000_000.0) / perf_time.as_secs_f64()
                    );
                }
                Err(e) => {
                    println!("❌ Large vertex buffer creation failed: {}", e);
                    return Err(e.into());
                }
            }

            // Renderer status
            println!("\n📋 Renderer Status:");
            println!("Ready for rendering: {}", renderer.is_ready());
            println!("Shaders compiled: {}", renderer.has_shader("demo"));
            println!("Pipelines created: {}", renderer.has_pipeline("line"));

            println!("\n🎉 WebGPU Demo completed successfully!");
            println!("This demonstrates real WebGPU functionality with:");
            println!("  • Device initialization");
            println!("  • Shader compilation");
            println!("  • Render pipeline creation");
            println!("  • Vertex buffer management");
            println!("  • Performance benchmarking");
        }
        Err(WebGpuRealError::NotSupported(msg)) => {
            println!("⚠️  WebGPU not supported: {}", msg);
            println!("This is expected in some environments (e.g., headless CI)");
        }
        Err(e) => {
            println!("❌ Unexpected error: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
