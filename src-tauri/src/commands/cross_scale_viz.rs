// Cross-Scale Visualization Commands - V1.8
// Multi-layer visualization for bridging atomistic, mesoscale, and continuum views.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tauri::command;

// ============================================================================
// Data Structures
// ============================================================================

/// Configuration for creating a cross-scale visualization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossScaleVizConfig {
    pub name: String,
    pub layers: Vec<VizLayerConfig>,
    pub domain_size: [f64; 3],
    pub camera_position: [f64; 3],
    pub background_color: String,
    pub show_axes: bool,
    pub show_scale_bar: bool,
}

/// Configuration for a single visualization layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VizLayerConfig {
    pub id: String,
    pub name: String,
    pub scale: String,            // "atomistic", "mesoscale", "continuum"
    pub data_type: String,        // "atoms", "field", "mesh", "contours"
    pub visible: bool,
    pub opacity: f64,
    pub color_map: String,
    pub data_range: Option<[f64; 2]>,
    pub transform: Option<VizTransform>,
}

/// A 3D transform for a visualization layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VizTransform {
    pub translation: [f64; 3],
    pub rotation: [f64; 3],      // Euler angles in degrees
    pub scale: [f64; 3],
}

/// A complete cross-scale visualization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossScaleViz {
    pub id: String,
    pub name: String,
    pub layers: Vec<VizLayer>,
    pub domain_size: [f64; 3],
    pub camera_position: [f64; 3],
    pub created_at: String,
    pub metadata: HashMap<String, Value>,
}

/// A rendered visualization layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VizLayer {
    pub id: String,
    pub name: String,
    pub scale: String,
    pub data_type: String,
    pub visible: bool,
    pub opacity: f64,
    pub color_map: String,
    pub vertex_count: u32,
    pub face_count: u32,
    pub data_bounds: [f64; 6],    // [xmin, xmax, ymin, ymax, zmin, zmax]
    pub render_data: VizRenderData,
}

/// Render data for a visualization layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VizRenderData {
    pub vertices: Vec<f64>,
    pub colors: Vec<f64>,
    pub indices: Vec<u32>,
    pub scalars: Vec<f64>,
    pub scalar_range: [f64; 2],
}

/// Result of mapping atoms to a continuous field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtomToFieldMapping {
    pub viz_id: String,
    pub atom_count: u32,
    pub grid_size: [u32; 3],
    pub field_data: Vec<f64>,
    pub mapping_quality: f64,
    pub statistics: HashMap<String, f64>,
}

/// Result of overlaying a field onto a mesh.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldOverlayResult {
    pub viz_id: String,
    pub mesh_node_count: u32,
    pub mesh_element_count: u32,
    pub field_values: Vec<f64>,
    extrapolated_count: u32,
    pub interpolation_method: String,
    color_map: String,
}

/// Animation data for scale bridging visualization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleBridgeAnimation {
    pub id: String,
    pub name: String,
    pub frames: Vec<AnimationFrame>,
    pub frame_count: u32,
    pub duration_ms: f64,
    pub scales: Vec<String>,
    pub transitions: Vec<ScaleTransition>,
}

/// A single animation frame.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationFrame {
    pub frame_index: u32,
    pub timestamp_ms: f64,
    pub active_scale: String,
    pub zoom_level: f64,
    pub layer_opacities: HashMap<String, f64>,
    pub camera_position: [f64; 3],
    pub camera_target: [f64; 3],
}

/// A transition between scales in the animation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleTransition {
    pub from_scale: String,
    pub to_scale: String,
    pub start_frame: u32,
    pub end_frame: u32,
    pub transition_type: String,   // "zoom_in", "zoom_out", "crossfade", "morph"
    pub description: String,
}

/// A color map definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorMapDefinition {
    pub id: String,
    pub name: String,
    pub category: String,         // "sequential", "diverging", "categorical", "perceptual"
    pub colors: Vec<ColorStop>,
    pub is_reversible: bool,
    pub is_colorblind_safe: bool,
}

/// A color stop in a color map.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorStop {
    pub position: f64,            // 0.0 to 1.0
    pub color: String,            // hex color string
}

// ============================================================================
// Mock Data
// ============================================================================

fn build_color_maps() -> Vec<ColorMapDefinition> {
    vec![
        ColorMapDefinition {
            id: "viridis".into(), name: "Viridis".into(), category: "perceptual".into(),
            colors: vec![
                ColorStop { position: 0.0, color: "#440154".into() },
                ColorStop { position: 0.25, color: "#3b528b".into() },
                ColorStop { position: 0.5, color: "#21918c".into() },
                ColorStop { position: 0.75, color: "#5ec962".into() },
                ColorStop { position: 1.0, color: "#fde725".into() },
            ],
            is_reversible: true, is_colorblind_safe: true,
        },
        ColorMapDefinition {
            id: "plasma".into(), name: "Plasma".into(), category: "perceptual".into(),
            colors: vec![
                ColorStop { position: 0.0, color: "#0d0887".into() },
                ColorStop { position: 0.25, color: "#7e03a8".into() },
                ColorStop { position: 0.5, color: "#cc4778".into() },
                ColorStop { position: 0.75, color: "#f89540".into() },
                ColorStop { position: 1.0, color: "#f0f921".into() },
            ],
            is_reversible: true, is_colorblind_safe: true,
        },
        ColorMapDefinition {
            id: "inferno".into(), name: "Inferno".into(), category: "perceptual".into(),
            colors: vec![
                ColorStop { position: 0.0, color: "#000004".into() },
                ColorStop { position: 0.25, color: "#56106e".into() },
                ColorStop { position: 0.5, color: "#bb3754".into() },
                ColorStop { position: 0.75, color: "#f98c0a".into() },
                ColorStop { position: 1.0, color: "#fcffa4".into() },
            ],
            is_reversible: true, is_colorblind_safe: true,
        },
        ColorMapDefinition {
            id: "coolwarm".into(), name: "Cool-Warm".into(), category: "diverging".into(),
            colors: vec![
                ColorStop { position: 0.0, color: "#3b4cc0".into() },
                ColorStop { position: 0.25, color: "#7b9ff9".into() },
                ColorStop { position: 0.5, color: "#f7f7f7".into() },
                ColorStop { position: 0.75, color: "#f4a582".into() },
                ColorStop { position: 1.0, color: "#b40426".into() },
            ],
            is_reversible: true, is_colorblind_safe: true,
        },
        ColorMapDefinition {
            id: "jet".into(), name: "Jet".into(), category: "sequential".into(),
            colors: vec![
                ColorStop { position: 0.0, color: "#00007f".into() },
                ColorStop { position: 0.25, color: "#007fff".into() },
                ColorStop { position: 0.5, color: "#00ff7f".into() },
                ColorStop { position: 0.75, color: "#ffff00".into() },
                ColorStop { position: 1.0, color: "#ff0000".into() },
            ],
            is_reversible: true, is_colorblind_safe: false,
        },
        ColorMapDefinition {
            id: "hot".into(), name: "Hot".into(), category: "sequential".into(),
            colors: vec![
                ColorStop { position: 0.0, color: "#000000".into() },
                ColorStop { position: 0.33, color: "#ff0000".into() },
                ColorStop { position: 0.66, color: "#ffff00".into() },
                ColorStop { position: 1.0, color: "#ffffff".into() },
            ],
            is_reversible: true, is_colorblind_safe: false,
        },
        ColorMapDefinition {
            id: "turbo".into(), name: "Turbo".into(), category: "perceptual".into(),
            colors: vec![
                ColorStop { position: 0.0, color: "#30123b".into() },
                ColorStop { position: 0.25, color: "#4675ed".into() },
                ColorStop { position: 0.5, color: "#36b677".into() },
                ColorStop { position: 0.75, color: "#f0a624".into() },
                ColorStop { position: 1.0, color: "#7a0403".into() },
            ],
            is_reversible: true, is_colorblind_safe: false,
        },
        ColorMapDefinition {
            id: "rainbow".into(), name: "Rainbow".into(), category: "categorical".into(),
            colors: vec![
                ColorStop { position: 0.0, color: "#ff0000".into() },
                ColorStop { position: 0.17, color: "#ff8800".into() },
                ColorStop { position: 0.33, color: "#ffff00".into() },
                ColorStop { position: 0.5, color: "#00ff00".into() },
                ColorStop { position: 0.67, color: "#0088ff".into() },
                ColorStop { position: 0.83, color: "#8800ff".into() },
                ColorStop { position: 1.0, color: "#ff00ff".into() },
            ],
            is_reversible: true, is_colorblind_safe: false,
        },
    ]
}

fn generate_mock_render_data(vertex_count: u32, face_count: u32) -> VizRenderData {
    let mut vertices = Vec::with_capacity(vertex_count as usize * 3);
    let mut colors = Vec::with_capacity(vertex_count as usize * 3);
    let mut scalars = Vec::with_capacity(vertex_count as usize);
    let mut indices = Vec::with_capacity(face_count as usize * 3);

    let mut state: u64 = 12345;
    for _ in 0..vertex_count {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let x = (state >> 11) as f64 / u32::MAX as f64;
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let y = (state >> 11) as f64 / u32::MAX as f64;
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let z = (state >> 11) as f64 / u32::MAX as f64;

        vertices.push(x * 100.0);
        vertices.push(y * 100.0);
        vertices.push(z * 100.0);

        colors.push(x);
        colors.push(y);
        colors.push(z);

        scalars.push(x * 0.5 + y * 0.3 + z * 0.2);
    }

    for i in 0..face_count {
        indices.push(i * 3);
        indices.push(i * 3 + 1);
        indices.push(i * 3 + 2);
    }

    let min_s = scalars.iter().cloned().fold(f64::MAX, f64::min);
    let max_s = scalars.iter().cloned().fold(f64::MIN, f64::max);

    VizRenderData {
        vertices,
        colors,
        indices,
        scalars,
        scalar_range: [min_s, max_s],
    }
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// Create a new cross-scale visualization with multiple layers.
#[command]
pub fn create_cross_scale_viz(config: CrossScaleVizConfig) -> Result<CrossScaleViz, String> {
    tracing::info!(name = %config.name, layers = config.layers.len(), "create_cross_scale_viz called");

    if config.name.is_empty() {
        return Err("Visualization name cannot be empty".to_string());
    }

    if config.layers.is_empty() {
        return Err("At least one layer is required".to_string());
    }

    let mut viz_layers = Vec::new();

    for layer_config in &config.layers {
        let (vertices, faces) = match layer_config.data_type.as_str() {
            "atoms" => (5000u32, 0u32),
            "field" => (4096, 2048),
            "mesh" => (2000, 1000),
            "contours" => (3000, 1500),
            _ => (1000, 500),
        };

        let render_data = generate_mock_render_data(vertices, faces);

        viz_layers.push(VizLayer {
            id: layer_config.id.clone(),
            name: layer_config.name.clone(),
            scale: layer_config.scale.clone(),
            data_type: layer_config.data_type.clone(),
            visible: layer_config.visible,
            opacity: layer_config.opacity,
            color_map: layer_config.color_map.clone(),
            vertex_count: vertices,
            face_count: faces,
            data_bounds: [0.0, 100.0, 0.0, 100.0, 0.0, 100.0],
            render_data,
        });
    }

    let id = uuid::Uuid::new_v4().to_string();

    let mut metadata = HashMap::new();
    metadata.insert("layer_count".into(), Value::Number(serde_json::Number::from(viz_layers.len())));
    metadata.insert("scales".into(), Value::Array(
        viz_layers.iter().map(|l| Value::String(l.scale.clone())).collect()
    ));

    tracing::info!(viz_id = %id, "Cross-scale visualization created");

    Ok(CrossScaleViz {
        id,
        name: config.name,
        layers: viz_layers,
        domain_size: config.domain_size,
        camera_position: config.camera_position,
        created_at: chrono::Utc::now().to_rfc3339(),
        metadata,
    })
}

/// Map atom positions to a continuous field for visualization.
#[command]
pub fn map_atom_to_field(
    atom_positions: Vec<[f64; 3]>,
    atom_values: Vec<f64>,
    grid_size: [u32; 3],
    _domain_size: [f64; 3],
    method: Option<String>,
) -> Result<AtomToFieldMapping, String> {
    tracing::info!(
        atoms = atom_positions.len(),
        grid = format!("{}x{}x{}", grid_size[0], grid_size[1], grid_size[2]),
        "map_atom_to_field called"
    );

    if atom_positions.is_empty() {
        return Err("No atom positions provided".to_string());
    }

    if atom_positions.len() != atom_values.len() {
        return Err(format!(
            "Position count ({}) != value count ({})",
            atom_positions.len(),
            atom_values.len()
        ));
    }

    let total_cells = (grid_size[0] as usize) * (grid_size[1] as usize) * (grid_size[2] as usize);
    let _method = method.unwrap_or_else(|| "gaussian".into());

    // Generate mock field data
    let mut field_data = vec![0.0_f64; total_cells];
    let mut state: u64 = 42;
    for i in 0..total_cells {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        field_data[i] = (state >> 11) as f64 / u32::MAX as f64;
    }

    let mut statistics = HashMap::new();
    statistics.insert("mean_field_value".into(), 0.5);
    statistics.insert("std_field_value".into(), 0.289);
    statistics.insert("min_field_value".into(), 0.001);
    statistics.insert("max_field_value".into(), 0.999);
    statistics.insert("atoms_per_cell_avg".into(), atom_positions.len() as f64 / total_cells as f64);

    let viz_id = uuid::Uuid::new_v4().to_string();

    tracing::info!(
        viz_id = %viz_id,
        cells = total_cells,
        quality = 0.92,
        "Atom-to-field mapping complete"
    );

    Ok(AtomToFieldMapping {
        viz_id,
        atom_count: atom_positions.len() as u32,
        grid_size,
        field_data,
        mapping_quality: 0.92,
        statistics,
    })
}

/// Overlay a scalar field onto a finite element mesh.
#[command]
pub fn overlay_field_on_mesh(
    field_data: Vec<f64>,
    _field_grid_size: [u32; 3],
    mesh_node_count: u32,
    interpolation_method: Option<String>,
    color_map: Option<String>,
) -> Result<FieldOverlayResult, String> {
    tracing::info!(
        field_points = field_data.len(),
        mesh_nodes = mesh_node_count,
        "overlay_field_on_mesh called"
    );

    if field_data.is_empty() {
        return Err("Field data cannot be empty".to_string());
    }

    if mesh_node_count == 0 {
        return Err("Mesh node count must be positive".to_string());
    }

    let interp = interpolation_method.unwrap_or_else(|| "trilinear".into());
    let cmap = color_map.unwrap_or_else(|| "viridis".into());

    // Generate mock interpolated field values at mesh nodes
    let mut field_values = Vec::with_capacity(mesh_node_count as usize);
    let mut state: u64 = 99;
    for _ in 0..mesh_node_count {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        field_values.push((state >> 11) as f64 / u32::MAX as f64);
    }

    let extrapolated = (mesh_node_count as f64 * 0.02) as u32;

    let viz_id = uuid::Uuid::new_v4().to_string();

    tracing::info!(
        viz_id = %viz_id,
        method = %interp,
        cmap = %cmap,
        extrapolated = extrapolated,
        "Field overlay complete"
    );

    Ok(FieldOverlayResult {
        viz_id,
        mesh_node_count,
        mesh_element_count: mesh_node_count / 8 * 6, // mock hex mesh
        field_values,
        extrapolated_count: extrapolated,
        interpolation_method: interp,
        color_map: cmap,
    })
}

/// Generate an animation that transitions between different scales.
#[command]
pub fn generate_scale_bridge_animation(
    name: String,
    scales: Vec<String>,
    frame_count: Option<u32>,
    duration_ms: Option<f64>,
) -> Result<ScaleBridgeAnimation, String> {
    tracing::info!(
        name = %name,
        scales = ?scales,
        "generate_scale_bridge_animation called"
    );

    if name.is_empty() {
        return Err("Animation name cannot be empty".to_string());
    }

    if scales.is_empty() {
        return Err("At least one scale is required".to_string());
    }

    let frames = frame_count.unwrap_or(60);
    let duration = duration_ms.unwrap_or(5000.0);
    let frames_per_scale = frames / scales.len().max(1) as u32;

    let mut animation_frames = Vec::new();
    let mut transitions = Vec::new();

    for (si, scale) in scales.iter().enumerate() {
        let start = si as u32 * frames_per_scale;
        let end = ((si + 1) as u32 * frames_per_scale).min(frames);

        for f in start..end {
            let t = (f - start) as f64 / (end - start).max(1) as f64;
            let zoom = match scale.as_str() {
                "atomistic" => 0.1 + t * 0.2,
                "mesoscale" => 1.0 + t * 0.5,
                "continuum" => 5.0 + t * 2.0,
                _ => 1.0,
            };

            let mut opacities = HashMap::new();
            for (oj, os) in scales.iter().enumerate() {
                if oj == si {
                    opacities.insert(os.clone(), 0.3 + t * 0.7);
                } else if oj == si + 1 {
                    opacities.insert(os.clone(), t * 0.3);
                } else {
                    opacities.insert(os.clone(), 0.0);
                }
            }

            animation_frames.push(AnimationFrame {
                frame_index: f,
                timestamp_ms: f as f64 / frames as f64 * duration,
                active_scale: scale.clone(),
                zoom_level: zoom,
                layer_opacities: opacities,
                camera_position: [50.0, 50.0, 50.0 + zoom * 20.0],
                camera_target: [50.0, 50.0, 50.0],
            });
        }

        // Add transition
        if si + 1 < scales.len() {
            transitions.push(ScaleTransition {
                from_scale: scale.clone(),
                to_scale: scales[si + 1].clone(),
                start_frame: end - 5,
                end_frame: end + 5,
                transition_type: "zoom_in".into(),
                description: format!("Transition from {} to {} scale", scale, scales[si + 1]),
            });
        }
    }

    let id = uuid::Uuid::new_v4().to_string();

    tracing::info!(
        anim_id = %id,
        frames = animation_frames.len(),
        transitions = transitions.len(),
        "Animation generated"
    );

    Ok(ScaleBridgeAnimation {
        id,
        name,
        frames: animation_frames,
        frame_count: frames,
        duration_ms: duration,
        scales,
        transitions,
    })
}

/// Get available color maps for visualization.
#[command]
pub fn get_available_color_maps(
    category: Option<String>,
) -> Result<Vec<ColorMapDefinition>, String> {
    tracing::info!(
        category = category.as_deref().unwrap_or("all"),
        "get_available_color_maps called"
    );

    let all_maps = build_color_maps();

    let filtered: Vec<ColorMapDefinition> = match &category {
        Some(c) if !c.is_empty() => {
            all_maps.into_iter().filter(|m| m.category == *c).collect()
        }
        _ => all_maps,
    };

    tracing::info!("Found {} color maps", filtered.len());
    Ok(filtered)
}
