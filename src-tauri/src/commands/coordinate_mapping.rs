// Coordinate Mapping Commands - V1.8
// MD box to Phase Field grid coordinate mapping with 4x4 transformation matrices.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::command;

// ============================================================================
// Data Structures
// ============================================================================

/// A 4x4 homogeneous transformation matrix stored as a flat array (row-major).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformMatrix4x4 {
    pub elements: [f64; 16], // row-major: [m00, m01, m02, m03, m10, ...]
}

/// Configuration for creating a coordinate mapping between two simulation domains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMappingConfig {
    pub source_type: String,       // "MD", "DFT", "PF", "FE"
    pub target_type: String,       // "MD", "DFT", "PF", "FE"
    pub source_box: [f64; 3],      // Source domain size [Lx, Ly, Lz]
    pub target_box: [f64; 3],      // Target domain size [Lx, Ly, Lz]
    pub source_origin: [f64; 3],   // Source origin [x0, y0, z0]
    pub target_origin: [f64; 3],   // Target origin [x0, y0, z0]
    pub target_grid_size: [u32; 3], // Target grid resolution [nx, ny, nz]
    pub rotation_angle_deg: Option<f64>, // Optional rotation around z-axis
    pub periodic: Vec<bool>,       // Periodic boundary flags [x, y, z]
}

/// A stored coordinate mapping with its metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinateMapping {
    pub id: String,
    pub source_type: String,
    pub target_type: String,
    pub source_box: [f64; 3],
    pub target_box: [f64; 3],
    pub source_origin: [f64; 3],
    pub target_origin: [f64; 3],
    pub target_grid_size: [u32; 3],
    pub forward_transform: TransformMatrix4x4,
    pub inverse_transform: TransformMatrix4x4,
    pub periodic: Vec<bool>,
    pub created_at: String,
    pub description: String,
}

/// A mapped point with source and target coordinates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappedPoint {
    pub source_coords: [f64; 3],
    pub target_coords: [f64; 3],
    pub grid_indices: [i32; 3],
    pub interpolation_weights: [f64; 8], // trilinear weights for 8 neighbors
    pub is_valid: bool,
}

/// Result of mapping multiple points.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapPointsResult {
    pub mapping_id: String,
    pub points: Vec<MappedPoint>,
    pub total_points: u32,
    pub valid_points: u32,
    pub out_of_bounds_points: u32,
    pub statistics: HashMap<String, f64>,
}

/// Alignment validation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlignmentValidation {
    pub mapping_id: String,
    pub is_valid: bool,
    pub overall_score: f64,
    pub metrics: HashMap<String, f64>,
    pub issues: Vec<String>,
    pub warnings: Vec<String>,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Build a 4x4 transformation matrix from source/target box and origin.
fn build_transform(
    source_box: [f64; 3],
    target_box: [f64; 3],
    source_origin: [f64; 3],
    target_origin: [f64; 3],
    rotation_deg: Option<f64>,
) -> (TransformMatrix4x4, TransformMatrix4x4) {
    let angle = rotation_deg.unwrap_or(0.0);
    let cos_a = angle.to_radians().cos();
    let sin_a = angle.to_radians().sin();

    // Scale factors
    let sx = if source_box[0].abs() > 1e-15 {
        target_box[0] / source_box[0]
    } else {
        1.0
    };
    let sy = if source_box[1].abs() > 1e-15 {
        target_box[1] / source_box[1]
    } else {
        1.0
    };
    let sz = if source_box[2].abs() > 1e-15 {
        target_box[2] / source_box[2]
    } else {
        1.0
    };

    // Forward: source -> target
    // T = Translate(target_origin) * Rotate(angle) * Scale(sx, sy, sz) * Translate(-source_origin)
    // Simplified for the common case (no rotation):
    // [sx*cos  -sy*sin   0   tx]
    // [sx*sin   sy*cos   0   ty]
    // [0        0        sz  tz]
    // [0        0        0   1 ]
    let tx = target_origin[0] - source_origin[0] * sx * cos_a + source_origin[1] * sy * sin_a;
    let ty = target_origin[1] - source_origin[0] * sx * sin_a - source_origin[1] * sy * cos_a;
    let tz = target_origin[2] - source_origin[2] * sz;

    let forward = TransformMatrix4x4 {
        elements: [
            sx * cos_a, -sy * sin_a, 0.0, tx,
            sx * sin_a,  sy * cos_a, 0.0, ty,
            0.0,         0.0,        sz,  tz,
            0.0,         0.0,        0.0, 1.0,
        ],
    };

    // Inverse: target -> source
    // For the simplified case:
    let inv_sx = if sx.abs() > 1e-15 { 1.0 / sx } else { 0.0 };
    let inv_sy = if sy.abs() > 1e-15 { 1.0 / sy } else { 0.0 };
    let inv_sz = if sz.abs() > 1e-15 { 1.0 / sz } else { 0.0 };

    let inv_tx = source_origin[0] - target_origin[0] * inv_sx * cos_a + target_origin[1] * inv_sy * sin_a;
    let inv_ty = source_origin[1] - target_origin[0] * inv_sx * sin_a - target_origin[1] * inv_sy * cos_a;
    let inv_tz = source_origin[2] - target_origin[2] * inv_sz;

    let inverse = TransformMatrix4x4 {
        elements: [
            inv_sx * cos_a, -inv_sy * sin_a, 0.0, inv_tx,
            inv_sx * sin_a,  inv_sy * cos_a, 0.0, inv_ty,
            0.0,             0.0,            inv_sz, inv_tz,
            0.0,             0.0,            0.0,    1.0,
        ],
    };

    (forward, inverse)
}

/// Apply a 4x4 homogeneous transform to a 3D point.
fn apply_transform(matrix: &TransformMatrix4x4, point: [f64; 3]) -> [f64; 3] {
    let e = &matrix.elements;
    let x = e[0] * point[0] + e[1] * point[1] + e[2] * point[2] + e[3];
    let y = e[4] * point[0] + e[5] * point[1] + e[6] * point[2] + e[7];
    let z = e[8] * point[0] + e[9] * point[1] + e[10] * point[2] + e[11];
    let w = e[12] * point[0] + e[13] * point[1] + e[14] * point[2] + e[15];
    if w.abs() > 1e-15 {
        [x / w, y / w, z / w]
    } else {
        [x, y, z]
    }
}

/// Compute trilinear interpolation weights for a point in a grid.
fn trilinear_weights(
    coords: [f64; 3],
    grid_size: [u32; 3],
    box_size: [f64; 3],
) -> ([i32; 3], [f64; 8], bool) {
    let dx = box_size[0] / grid_size[0] as f64;
    let dy = box_size[1] / grid_size[1] as f64;
    let dz = box_size[2] / grid_size[2] as f64;

    let gx = coords[0] / dx;
    let gy = coords[1] / dy;
    let gz = coords[2] / dz;

    let ix = gx.floor() as i32;
    let iy = gy.floor() as i32;
    let iz = gz.floor() as i32;

    let fx = gx - ix as f64;
    let fy = gy - iy as f64;
    let fz = gz - iz as f64;

    let is_valid = ix >= 0
        && iy >= 0
        && iz >= 0
        && ix < grid_size[0] as i32 - 1
        && iy < grid_size[1] as i32 - 1
        && iz < grid_size[2] as i32 - 1;

    // 8 trilinear weights: (0,0,0), (1,0,0), (0,1,0), (1,1,0),
    //                       (0,0,1), (1,0,1), (0,1,1), (1,1,1)
    let weights = [
        (1.0 - fx) * (1.0 - fy) * (1.0 - fz),
        fx * (1.0 - fy) * (1.0 - fz),
        (1.0 - fx) * fy * (1.0 - fz),
        fx * fy * (1.0 - fz),
        (1.0 - fx) * (1.0 - fy) * fz,
        fx * (1.0 - fy) * fz,
        (1.0 - fx) * fy * fz,
        fx * fy * fz,
    ];

    ([ix, iy, iz], weights, is_valid)
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// Create a new coordinate mapping between two simulation domains.
#[command]
pub fn create_mapping(config: CreateMappingConfig) -> Result<CoordinateMapping, String> {
    tracing::info!(
        source = %config.source_type,
        target = %config.target_type,
        "create_mapping called"
    );

    if config.source_box.iter().any(|&v| v <= 0.0) {
        return Err("Source box dimensions must be positive".to_string());
    }
    if config.target_box.iter().any(|&v| v <= 0.0) {
        return Err("Target box dimensions must be positive".to_string());
    }
    if config.target_grid_size.iter().any(|&v| v == 0) {
        return Err("Target grid dimensions must be non-zero".to_string());
    }

    let (forward, inverse) = build_transform(
        config.source_box,
        config.target_box,
        config.source_origin,
        config.target_origin,
        config.rotation_angle_deg,
    );

    let id = uuid::Uuid::new_v4().to_string();
    let created_at = chrono::Utc::now().to_rfc3339();

    let description = format!(
        "{} -> {} coordinate mapping: {}x{}x{} -> {}x{}x{} grid",
        config.source_type,
        config.target_type,
        config.source_box[0],
        config.source_box[1],
        config.source_box[2],
        config.target_grid_size[0],
        config.target_grid_size[1],
        config.target_grid_size[2],
    );

    tracing::info!(mapping_id = %id, "Mapping created");

    Ok(CoordinateMapping {
        id,
        source_type: config.source_type,
        target_type: config.target_type,
        source_box: config.source_box,
        target_box: config.target_box,
        source_origin: config.source_origin,
        target_origin: config.target_origin,
        target_grid_size: config.target_grid_size,
        forward_transform: forward,
        inverse_transform: inverse,
        periodic: config.periodic,
        created_at,
        description,
    })
}

/// Map a set of source points to the target grid using a stored mapping.
#[command]
pub fn map_points(
    mapping_id: String,
    source_points: Vec<[f64; 3]>,
) -> Result<MapPointsResult, String> {
    tracing::info!(
        mapping_id = %mapping_id,
        num_points = source_points.len(),
        "map_points called"
    );

    if source_points.is_empty() {
        return Err("No source points provided".to_string());
    }

    // In a real implementation, we would look up the mapping from a store.
    // Here we reconstruct a mock mapping.
    let mock_mapping = create_mock_mapping(&mapping_id);

    let mut points = Vec::with_capacity(source_points.len());
    let mut valid_count = 0u32;
    let mut oob_count = 0u32;
    let mut min_dist = f64::MAX;
    let mut max_dist = 0.0_f64;
    let mut total_dist = 0.0_f64;

    for src in &source_points {
        let target = apply_transform(&mock_mapping.forward_transform, *src);

        let (grid_idx, weights, is_valid) = trilinear_weights(
            target,
            mock_mapping.target_grid_size,
            mock_mapping.target_box,
        );

        if !is_valid {
            oob_count += 1;
        } else {
            valid_count += 1;
        }

        // Compute displacement magnitude
        let dist = ((target[0] - src[0]).powi(2)
            + (target[1] - src[1]).powi(2)
            + (target[2] - src[2]).powi(2))
        .sqrt();
        min_dist = min_dist.min(dist);
        max_dist = max_dist.max(dist);
        total_dist += dist;

        points.push(MappedPoint {
            source_coords: *src,
            target_coords: target,
            grid_indices: grid_idx,
            interpolation_weights: weights,
            is_valid,
        });
    }

    let mut statistics = HashMap::new();
    statistics.insert("avg_displacement".to_string(), total_dist / source_points.len() as f64);
    statistics.insert("min_displacement".to_string(), min_dist);
    statistics.insert("max_displacement".to_string(), max_dist);
    statistics.insert("valid_ratio".to_string(), valid_count as f64 / source_points.len() as f64);

    tracing::info!(
        valid = valid_count,
        oob = oob_count,
        "Point mapping complete"
    );

    Ok(MapPointsResult {
        mapping_id,
        points,
        total_points: source_points.len() as u32,
        valid_points: valid_count,
        out_of_bounds_points: oob_count,
        statistics,
    })
}

/// Get detailed information about a stored coordinate mapping.
#[command]
pub fn get_mapping_info(mapping_id: String) -> Result<CoordinateMapping, String> {
    tracing::info!(mapping_id = %mapping_id, "get_mapping_info called");

    let mapping = create_mock_mapping(&mapping_id);

    tracing::info!(
        source = %mapping.source_type,
        target = %mapping.target_type,
        "Mapping info retrieved"
    );

    Ok(mapping)
}

/// Validate the alignment quality of a coordinate mapping.
#[command]
pub fn validate_alignment(
    mapping_id: String,
    test_points: Option<Vec<[f64; 3]>>,
) -> Result<AlignmentValidation, String> {
    tracing::info!(
        mapping_id = %mapping_id,
        has_test_points = test_points.is_some(),
        "validate_alignment called"
    );

    let mapping = create_mock_mapping(&mapping_id);
    let mut metrics = HashMap::new();
    let mut issues = Vec::new();
    let mut warnings = Vec::new();

    // Check volume conservation
    let src_vol = mapping.source_box[0] * mapping.source_box[1] * mapping.source_box[2];
    let tgt_vol = mapping.target_box[0] * mapping.target_box[1] * mapping.target_box[2];
    let vol_ratio = tgt_vol / src_vol;
    metrics.insert("volume_ratio".to_string(), vol_ratio);

    if (vol_ratio - 1.0).abs() > 0.1 {
        issues.push(format!(
            "Significant volume change: ratio = {:.4} (expected ~1.0)",
            vol_ratio
        ));
    }

    // Check grid resolution adequacy
    let min_spacing = mapping.target_box.iter()
        .zip(mapping.target_grid_size.iter())
        .map(|(l, n)| l / *n as f64)
        .fold(f64::MAX, f64::min);
    metrics.insert("min_grid_spacing".to_string(), min_spacing);

    if min_spacing > src_vol.cbrt() * 0.5 {
        warnings.push("Grid spacing may be too coarse for accurate mapping".to_string());
    }

    // Check aspect ratio preservation
    let src_aspect = [
        mapping.source_box[0] / mapping.source_box[1],
        mapping.source_box[1] / mapping.source_box[2],
        mapping.source_box[0] / mapping.source_box[2],
    ];
    let tgt_aspect = [
        mapping.target_box[0] / mapping.target_box[1],
        mapping.target_box[1] / mapping.target_box[2],
        mapping.target_box[0] / mapping.target_box[2],
    ];
    let aspect_error: f64 = src_aspect.iter()
        .zip(tgt_aspect.iter())
        .map(|(s, t)| ((s - t) / s).abs())
        .sum::<f64>() / 3.0;
    metrics.insert("aspect_ratio_error".to_string(), aspect_error);

    if aspect_error > 0.1 {
        warnings.push(format!(
            "Aspect ratio distortion: {:.2}% average error",
            aspect_error * 100.0
        ));
    }

    // Check transform orthogonality (should be close to identity for rotation=0)
    let e = &mapping.forward_transform.elements;
    let dot_01 = (e[0] * e[1] + e[4] * e[5] + e[8] * e[9]).abs();
    let dot_02 = (e[0] * e[2] + e[4] * e[6] + e[8] * e[10]).abs();
    let dot_12 = (e[1] * e[2] + e[5] * e[6] + e[9] * e[10]).abs();
    let orthogonality_error = dot_01.max(dot_02).max(dot_12);
    metrics.insert("orthogonality_error".to_string(), orthogonality_error);

    // If test points provided, check mapping consistency
    if let Some(pts) = &test_points {
        if !pts.is_empty() {
            let mut max_roundtrip_error = 0.0_f64;
            for pt in pts {
                let fwd = apply_transform(&mapping.forward_transform, *pt);
                let back = apply_transform(&mapping.inverse_transform, fwd);
                let err = ((pt[0] - back[0]).powi(2)
                    + (pt[1] - back[1]).powi(2)
                    + (pt[2] - back[2]).powi(2))
                .sqrt();
                max_roundtrip_error = max_roundtrip_error.max(err);
            }
            metrics.insert("max_roundtrip_error".to_string(), max_roundtrip_error);

            if max_roundtrip_error > 1e-6 {
                issues.push(format!(
                    "Roundtrip error too large: {:.2e}",
                    max_roundtrip_error
                ));
            }
        }
    }

    // Overall score: 1.0 = perfect
    let mut score = 1.0;
    score -= (vol_ratio - 1.0).abs() * 2.0;
    score -= aspect_error;
    score -= orthogonality_error * 10.0;
    score = score.max(0.0).min(1.0);

    let is_valid = issues.is_empty() && score > 0.5;

    if issues.is_empty() {
        issues.push("No critical alignment issues detected".to_string());
    }

    tracing::info!(
        score = score,
        is_valid = is_valid,
        issues = issues.len(),
        "Alignment validation complete"
    );

    Ok(AlignmentValidation {
        mapping_id,
        is_valid,
        overall_score: score,
        metrics,
        issues,
        warnings,
    })
}

/// Create a mock mapping for demonstration purposes.
fn create_mock_mapping(id: &str) -> CoordinateMapping {
    let source_box = [50.0, 50.0, 50.0]; // 50 Angstrom MD box
    let target_box = [100.0, 100.0, 100.0]; // 100 nm Phase Field domain
    let source_origin = [0.0, 0.0, 0.0];
    let target_origin = [0.0, 0.0, 0.0];
    let grid_size = [64, 64, 64];

    let (forward, inverse) = build_transform(
        source_box,
        target_box,
        source_origin,
        target_origin,
        None,
    );

    CoordinateMapping {
        id: id.to_string(),
        source_type: "MD".to_string(),
        target_type: "PF".to_string(),
        source_box,
        target_box,
        source_origin,
        target_origin,
        target_grid_size: grid_size,
        forward_transform: forward,
        inverse_transform: inverse,
        periodic: vec![true, true, true],
        created_at: chrono::Utc::now().to_rfc3339(),
        description: "MD box (50x50x50 A) -> Phase Field grid (64x64x64, 100x100x100 nm)".to_string(),
    }
}
