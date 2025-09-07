use crate::canvas_surface::*;
use crate::chart_config::*;
use crate::line_chart_renderer::*;
use crate::webgpu_real::*;
use leptos::prelude::*;
use leptos::*;
use web_sys::*;

/// Props for the HeliosChart component
#[derive(Clone, PartialEq)]
pub struct HeliosChartProps {
    /// Chart configuration
    pub config: LineChartConfig,
    /// Chart data as (x, y) coordinates
    pub data: Vec<(f32, f32)>,
    /// Optional canvas ID for rendering
    pub canvas_id: Option<String>,
}

/// HeliosChart component for rendering interactive charts
pub fn create_helios_chart(props: HeliosChartProps) -> impl IntoView {
    // For now, create a simple view without complex state management
    view! {
        <div>
            <h3>{props.config.base.title.clone()}</h3>
            <p>"X: " {props.config.base.x_label.clone()}</p>
            <p>"Y: " {props.config.base.y_label.clone()}</p>
            <canvas
                id={props.canvas_id.unwrap_or_else(|| "helios-chart".to_string())}
                width={props.config.base.width}
                height={props.config.base.height}
            />
            <Show when=move || props.config.show_legend>
                <p>"Legend: Data Series"</p>
            </Show>
            <p>"Data points: " {props.data.len()}</p>
        </div>
    }
}

/// Find the closest data point to the given mouse coordinates
fn find_closest_point(
    data: &[(f32, f32)],
    mouse_pos: (f32, f32),
    config: &LineChartConfig,
) -> Option<(f32, f32)> {
    if data.is_empty() {
        return None;
    }

    let (mouse_x, mouse_y) = mouse_pos;
    let mut closest_point = data[0];
    let mut min_distance = f32::INFINITY;

    for &(x, y) in data {
        // Convert data coordinates to screen coordinates
        let screen_x =
            (x / data.iter().map(|(x, _)| *x).fold(0.0, f32::max)) * config.base.width as f32;
        let screen_y = config.base.height as f32
            - (y / data.iter().map(|(_, y)| *y).fold(0.0, f32::max)) * config.base.height as f32;

        let distance = ((screen_x - mouse_x).powi(2) + (screen_y - mouse_y).powi(2)).sqrt();

        if distance < min_distance {
            min_distance = distance;
            closest_point = (x, y);
        }
    }

    // Only return the point if it's within a reasonable distance
    if min_distance < 20.0 {
        Some(closest_point)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_closest_point() {
        let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0), (3.0, 3.0), (4.0, 4.0)];

        let config = LineChartConfig {
            base: BaseChartConfig {
                width: 400,
                height: 300,
                title: "Test".to_string(),
                x_label: "X".to_string(),
                y_label: "Y".to_string(),
                show_grid: true,
                background_color: "#ffffff".to_string(),
                text_color: "#000000".to_string(),
            },
            color: "#ff0000".to_string(),
            line_width: 2.0,
            show_points: true,
            point_size: 4.0,
            interpolation: InterpolationType::Linear,
            show_legend: true,
        };

        // Test finding closest point
        let closest = find_closest_point(&data, (200.0, 150.0), &config);
        assert!(closest.is_some());

        // Test with empty data
        let empty_data: Vec<(f32, f32)> = vec![];
        let closest_empty = find_closest_point(&empty_data, (200.0, 150.0), &config);
        assert!(closest_empty.is_none());
    }

    #[test]
    fn test_helios_chart_props() {
        let config = LineChartConfig {
            base: BaseChartConfig {
                width: 800,
                height: 600,
                title: "Test Chart".to_string(),
                x_label: "X Axis".to_string(),
                y_label: "Y Axis".to_string(),
                show_grid: true,
                background_color: "#ffffff".to_string(),
                text_color: "#000000".to_string(),
            },
            color: "#ff0000".to_string(),
            line_width: 2.0,
            show_points: true,
            point_size: 4.0,
            interpolation: InterpolationType::Linear,
            show_legend: true,
        };

        let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)];

        let props = HeliosChartProps {
            config: config.clone(),
            data: data.clone(),
            canvas_id: Some("test-canvas".to_string()),
        };

        assert_eq!(props.config.base.width, 800);
        assert_eq!(props.data.len(), 3);
        assert_eq!(props.canvas_id, Some("test-canvas".to_string()));
    }
}
