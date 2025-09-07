//! Performance Examples
//!
//! Examples demonstrating performance optimization features in Helios

use leptos::*;
use leptos_helios::*;
use std::time::Duration;

/// Large Dataset Rendering Example
#[component]
pub fn LargeDatasetExample() -> impl IntoView {
    let (data, set_data) = create_signal(generate_large_dataset(100000));
    let (performance_metrics, set_performance_metrics) = create_signal(None);

    let config = LineChartConfig {
        base_config: BaseChartConfig {
            width: 1200,
            height: 800,
            title: "Large Dataset (100K points)".to_string(),
            x_label: "Index".to_string(),
            y_label: "Value".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        data: data(),
        color: "#3b82f6".to_string(),
        show_legend: true,
    };

    // Performance monitoring
    let update_performance = move || {
        let mut performance_manager = PerformanceManager::new(PerformanceConfig::default());
        let metrics = performance_manager.process_data(&data(), 1.0).unwrap();
        set_performance_metrics.set(Some(metrics));
    };

    view! {
        <div>
            <HeliosChart config=config />
            <div>
                <button on:click=move |_| update_performance()>
                    "Check Performance"
                </button>
                {move || {
                    if let Some(metrics) = performance_metrics() {
                        view! {
                            <div>
                                <p>"FPS: " {metrics.fps}</p>
                                <p>"Frame Time: " {metrics.frame_time_ms} "ms"</p>
                                <p>"Memory Usage: " {metrics.memory_usage_bytes} " bytes"</p>
                            </div>
                        }
                    } else {
                        view! { <div></div> }
                    }
                }}
            </div>
        </div>
    }
}

/// Real-time Streaming Example
#[component]
pub fn StreamingExample() -> impl IntoView {
    let (data, set_data) = create_signal(Vec::<DataPoint>::new());
    let (is_streaming, set_is_streaming) = create_signal(false);

    let config = LineChartConfig {
        base_config: BaseChartConfig {
            width: 800,
            height: 600,
            title: "Real-time Data Stream".to_string(),
            x_label: "Time".to_string(),
            y_label: "Value".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        data: data(),
        color: "#ef4444".to_string(),
        show_legend: true,
    };

    let start_streaming = move || {
        set_is_streaming.set(true);
        // In real implementation, this would start a WebSocket or similar
        spawn_local(async move {
            let mut counter = 0.0;
            loop {
                if !is_streaming() {
                    break;
                }

                let new_point = DataPoint {
                    x: counter,
                    y: (counter * 0.1).sin() * 100.0,
                };

                set_data.update(|data| {
                    data.push(new_point);
                    if data.len() > 1000 {
                        data.remove(0);
                    }
                });

                counter += 1.0;
                gloo_timers::future::TimeoutFuture::new(100).await;
            }
        });
    };

    let stop_streaming = move || {
        set_is_streaming.set(false);
    };

    view! {
        <div>
            <HeliosChart config=config />
            <div>
                <button on:click=move |_| start_streaming() disabled=is_streaming>
                    "Start Streaming"
                </button>
                <button on:click=move |_| stop_streaming() disabled=!is_streaming>
                    "Stop Streaming"
                </button>
            </div>
        </div>
    }
}

/// Memory Optimization Example
#[component]
pub fn MemoryOptimizationExample() -> impl IntoView {
    let (memory_usage, set_memory_usage) = create_signal(0);
    let (buffer_count, set_buffer_count) = create_signal(0);

    let create_memory_pool = move || {
        let mut memory_pool = AdvancedMemoryPool::new(1024 * 1024 * 100); // 100MB
        memory_pool
            .create_pool("vertex_buffer".to_string(), 1024 * 1024, 10)
            .unwrap();

        // Allocate some buffers
        let mut buffers = Vec::new();
        for _ in 0..5 {
            if let Some(buffer) = memory_pool.allocate_buffer("vertex_buffer").unwrap() {
                buffers.push(buffer);
            }
        }

        set_buffer_count.set(buffers.len());
        set_memory_usage.set(memory_pool.get_memory_usage());
    };

    view! {
        <div>
            <h3>"Memory Optimization Demo"</h3>
            <button on:click=move |_| create_memory_pool()>
                "Create Memory Pool"
            </button>
            <div>
                <p>"Allocated Buffers: " {buffer_count}</p>
                <p>"Memory Usage: " {memory_usage} " bytes"</p>
            </div>
        </div>
    }
}

/// LOD Demo Example
#[component]
pub fn LodDemoExample() -> impl IntoView {
    let (viewport_scale, set_viewport_scale) = create_signal(1.0);
    let (data_size, set_data_size) = create_signal(100000);
    let (lod_level, set_lod_level) = create_signal(0);

    let update_lod = move || {
        let mut lod_system = LodSystem::new();
        lod_system.update_lod(viewport_scale(), data_size());
        let current_lod = lod_system.get_current_lod();
        set_lod_level.set(current_lod.level);
    };

    let config = LineChartConfig {
        base_config: BaseChartConfig {
            width: 800,
            height: 600,
            title: "LOD System Demo".to_string(),
            x_label: "Index".to_string(),
            y_label: "Value".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        data: generate_large_dataset(data_size()),
        color: "#10b981".to_string(),
        show_legend: true,
    };

    view! {
        <div>
            <HeliosChart config=config />
            <div>
                <label>
                    "Viewport Scale: "
                    <input
                        type="range"
                        min="0.1"
                        max="2.0"
                        step="0.1"
                        value=viewport_scale
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<f64>().unwrap();
                            set_viewport_scale.set(value);
                        }
                    />
                    {viewport_scale}
                </label>
                <label>
                    "Data Size: "
                    <input
                        type="range"
                        min="1000"
                        max="1000000"
                        step="1000"
                        value=data_size
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<usize>().unwrap();
                            set_data_size.set(value);
                        }
                    />
                    {data_size}
                </label>
                <button on:click=move |_| update_lod()>
                    "Update LOD"
                </button>
                <p>"Current LOD Level: " {lod_level}</p>
            </div>
        </div>
    }
}

fn generate_large_dataset(size: usize) -> Vec<DataPoint> {
    (0..size)
        .map(|i| DataPoint {
            x: i as f64,
            y: (i as f64 * 0.1).sin() * 100.0 + (i as f64 * 0.05).cos() * 50.0,
        })
        .collect()
}
