# Troubleshooting

This guide helps you resolve common issues when using Helios.

## Common Issues

### WebGPU Not Available

**Problem**: WebGPU is not supported in your browser or environment.

**Solution**: Helios automatically falls back to WebGL2 or Canvas2D. Check browser compatibility:

```rust
let renderer = ChartRenderer::auto_detect()?;
match renderer.backend() {
    RendererBackend::WebGPU => println!("Using WebGPU"),
    RendererBackend::WebGL2 => println!("Using WebGL2 fallback"),
    RendererBackend::Canvas2D => println!("Using Canvas2D fallback"),
}
```

### Performance Issues

**Problem**: Charts are rendering slowly or stuttering.

**Solutions**:

1. **Enable LOD for large datasets**:
```rust
let mut lod_system = LodSystem::new();
lod_system.update_lod(viewport_scale, data.len());
let optimized_data = lod_system.sample_data(&data);
```

2. **Use performance mode**:
```rust
let config = PerformanceConfig {
    max_memory_mb: 100,
    target_fps: 60.0,
    enable_simd: true,
    enable_lod: true,
    batch_size: 1000,
};
```

3. **Check memory usage**:
```rust
let metrics = performance_manager.process_data(&data, 1.0)?;
println!("Memory usage: {} bytes", metrics.memory_usage_bytes);
```

### Chart Not Rendering

**Problem**: Chart appears blank or doesn't render.

**Solutions**:

1. **Check data format**:
```rust
// Ensure data points are valid
let data = vec![
    DataPoint { x: 1.0, y: 2.0 }, // Valid
    DataPoint { x: f64::NAN, y: 2.0 }, // Invalid - will cause issues
];
```

2. **Verify configuration**:
```rust
let config = LineChartConfig {
    base_config: BaseChartConfig {
        width: 800, // Must be > 0
        height: 600, // Must be > 0
        // ... other required fields
    },
    data: data, // Must not be empty
    // ... other required fields
};
```

3. **Check for errors**:
```rust
match renderer.render_chart(&config) {
    Ok(_) => println!("Chart rendered successfully"),
    Err(e) => println!("Rendering error: {}", e),
}
```

### Memory Issues

**Problem**: High memory usage or memory leaks.

**Solutions**:

1. **Use memory pooling**:
```rust
let mut memory_pool = AdvancedMemoryPool::new(1024 * 1024 * 100);
memory_pool.create_pool("vertex_buffer".to_string(), 1024 * 1024, 10)?;
```

2. **Deallocate unused buffers**:
```rust
memory_pool.deallocate_buffer("vertex_buffer", &buffer_id)?;
```

3. **Monitor memory usage**:
```rust
let metrics = performance_manager.process_data(&data, 1.0)?;
if metrics.memory_usage_bytes > 100 * 1024 * 1024 { // 100MB
    println!("High memory usage detected");
}
```

### Data Processing Errors

**Problem**: Data processing fails or produces incorrect results.

**Solutions**:

1. **Validate input data**:
```rust
fn validate_data(data: &[DataPoint]) -> Result<(), String> {
    for point in data {
        if point.x.is_nan() || point.y.is_nan() {
            return Err("Invalid data point: NaN values".to_string());
        }
        if point.x.is_infinite() || point.y.is_infinite() {
            return Err("Invalid data point: Infinite values".to_string());
        }
    }
    Ok(())
}
```

2. **Handle empty datasets**:
```rust
if data.is_empty() {
    return Err("Cannot render chart with empty data".into());
}
```

3. **Check data ranges**:
```rust
let x_min = data.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
let x_max = data.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
if x_min == x_max {
    println!("Warning: All x values are the same");
}
```

## Error Handling

### WebGpuError

```rust
use leptos_helios::webgpu_renderer::WebGpuError;

match result {
    Ok(data) => println!("Success: {:?}", data),
    Err(WebGpuError::DeviceInit(msg)) => {
        println!("Device initialization failed: {}", msg);
        // Try fallback renderer
    },
    Err(WebGpuError::ShaderCompilation(msg)) => {
        println!("Shader compilation failed: {}", msg);
        // Check shader syntax
    },
    Err(WebGpuError::BufferAllocation(msg)) => {
        println!("Buffer allocation failed: {}", msg);
        // Reduce memory usage
    },
    Err(e) => println!("Other error: {}", e),
}
```

### HeliosError

```rust
use leptos_helios::HeliosError;

match result {
    Ok(data) => println!("Success: {:?}", data),
    Err(HeliosError::DataProcessing(e)) => {
        println!("Data processing error: {}", e);
        println!("Suggested actions: {:?}", e.suggested_actions());
    },
    Err(HeliosError::Rendering(e)) => {
        println!("Rendering error: {}", e);
        println!("User message: {}", e.user_message());
    },
    Err(HeliosError::PerformanceBudget { details }) => {
        println!("Performance budget exceeded: {}", details);
        // Reduce data size or enable performance mode
    },
    Err(e) => println!("Other error: {}", e),
}
```

## Debugging Tips

### Enable Debug Logging

```rust
// Enable console logging in browser
#[cfg(target_arch = "wasm32")]
console_error_panic_hook::set_once();
```

### Performance Profiling

```rust
let profiler = performance_manager.profiler();
let timer = profiler.start_timer("operation".to_string());
// ... perform operation ...
let elapsed = timer.elapsed();
println!("Operation took: {:?}", elapsed);
```

### Memory Debugging

```rust
// Check memory usage before and after operations
let initial_memory = get_memory_usage();
// ... perform operations ...
let final_memory = get_memory_usage();
println!("Memory delta: {} bytes", final_memory - initial_memory);
```

## Browser Compatibility

### WebGPU Support

- **Chrome/Edge**: Version 113+ (with flag enabled)
- **Firefox**: Not yet supported
- **Safari**: Not yet supported

### WebGL2 Support

- **Chrome/Edge**: Version 56+
- **Firefox**: Version 51+
- **Safari**: Version 15+

### Canvas2D Support

- **All modern browsers**: Full support

## Performance Optimization

### For Large Datasets

1. **Use LOD system**:
```rust
let mut lod_system = LodSystem::new();
lod_system.update_lod(viewport_scale, data.len());
```

2. **Enable SIMD processing**:
```rust
let processor = SimdDataProcessor::new(1000, true);
```

3. **Use background processing**:
```rust
let worker = WebWorkerProcessor::new("data_worker".to_string());
```

### For Real-time Updates

1. **Use streaming**:
```rust
let streaming_manager = StreamingManager::new(StreamConfig {
    buffer_size: 1000,
    update_interval: Duration::from_millis(100),
    max_data_points: 10000,
});
```

2. **Optimize update frequency**:
```rust
// Update at most 60 times per second
let update_interval = Duration::from_millis(16); // ~60fps
```

## Getting Help

### Common Resources

1. **Documentation**: Check the [API Reference](api-reference.md)
2. **Examples**: See [Examples](examples.md) for usage patterns
3. **Performance**: Review [Performance Guide](performance-guide.md)

### Reporting Issues

When reporting issues, please include:

1. **Browser and version**
2. **Helios version**
3. **Error messages**
4. **Code sample**
5. **Expected vs actual behavior**

### Community Support

- **GitHub Issues**: For bug reports and feature requests
- **Discussions**: For questions and community support
- **Documentation**: For comprehensive guides and references

## Next Steps

- Check the [Getting Started](getting-started.md) guide for basic setup
- Explore [Examples](examples.md) for usage patterns
- Review the [Performance Guide](performance-guide.md) for optimization tips
- See the [API Reference](api-reference.md) for detailed documentation
