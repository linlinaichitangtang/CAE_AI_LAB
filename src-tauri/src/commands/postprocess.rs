//! Post-processing data structures
//! Provides high-level data structures for nodes, elements, and result fields
//! This module bridges the CAE core with the frontend visualization

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 3D Vector for positions and displacements
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::default()
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn add(&self, other: &Vec3) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn scale(&self, factor: f64) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
}

/// Symmetric stress tensor (6 components for 3D)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StressTensor {
    pub s11: f64,
    pub s22: f64,
    pub s33: f64,
    pub s12: f64,
    pub s13: f64,
    pub s23: f64,
}

impl StressTensor {
    /// Calculate Von Mises equivalent stress
    pub fn von_mises(&self) -> f64 {
        0.5 * ((self.s11 - self.s22).powi(2)
            + (self.s22 - self.s33).powi(2)
            + (self.s33 - self.s11).powi(2))
            + 3.0 * (self.s12.powi(2) + self.s13.powi(2) + self.s23.powi(2))
    }

    /// Calculate Maximum Principal Stress
    pub fn principal_max(&self) -> f64 {
        // Simple approximation for principal stresses
        let p = (self.s11 + self.s22 + self.s33) / 3.0;
        let q = ((self.s11 - self.s22).powi(2)
            + (self.s22 - self.s33).powi(2)
            + (self.s33 - self.s11).powi(2)
            + 6.0 * (self.s12.powi(2) + self.s13.powi(2) + self.s23.powi(2))
            / 2.0).sqrt()
            / 3.0;
        
        p + q // Maximum principal
    }

    /// Calculate hydrostatic stress
    pub fn hydrostatic(&self) -> f64 {
        (self.s11 + self.s22 + self.s33) / 3.0
    }
}

/// Strain tensor (6 components)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StrainTensor {
    pub e11: f64,
    pub e22: f64,
    pub e33: f64,
    pub e12: f64,
    pub e13: f64,
    pub e23: f64,
}

impl StrainTensor {
    /// Calculate equivalent strain (von Mises)
    pub fn von_mises(&self) -> f64 {
        ((self.e11 - self.e22).powi(2)
            + (self.e22 - self.e33).powi(2)
            + (self.e33 - self.e11).powi(2)
            + 6.0 * (self.e12.powi(2) + self.e13.powi(2) + self.e23.powi(2)))
            .sqrt()
            / (2.0_f64).sqrt()
    }
}

/// Mesh node with results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshNode {
    pub id: usize,
    pub position: Vec3,
    /// Displacement vector
    pub displacement: Option<Vec3>,
    /// Temperature (for thermal analysis)
    pub temperature: Option<f64>,
    /// Nodal force
    pub force: Option<Vec3>,
    /// Reaction force
    pub reaction: Option<Vec3>,
}

impl MeshNode {
    pub fn new(id: usize, x: f64, y: f64, z: f64) -> Self {
        Self {
            id,
            position: Vec3::new(x, y, z),
            displacement: None,
            temperature: None,
            force: None,
            reaction: None,
        }
    }

    /// Get displaced position for visualization
    pub fn displaced_position(&self, scale_factor: f64) -> Vec3 {
        if let Some(ref disp) = self.displacement {
            self.position.add(&disp.scale(scale_factor))
        } else {
            self.position.clone()
        }
    }
}

/// Mesh element with geometry and results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshElement {
    pub id: usize,
    pub element_type: String,
    pub node_ids: Vec<usize>,
    /// Stress at integration points
    pub stresses: Option<Vec<StressTensor>>,
    /// Strain at integration points
    pub strains: Option<Vec<StrainTensor>>,
    /// Max von Mises stress (for visualization)
    pub max_von_mises: Option<f64>,
    /// Element quality metric
    pub quality: Option<f64>,
}

impl MeshElement {
    pub fn new(id: usize, element_type: String, node_ids: Vec<usize>) -> Self {
        Self {
            id,
            element_type,
            node_ids,
            stresses: None,
            strains: None,
            max_von_mises: None,
            quality: None,
        }
    }
}

/// Result field for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResultField {
    Displacement(Vec3),
    DisplacementMagnitude(f64),
    Stress(StressTensor),
    VonMisesStress(f64),
    PrincipalStressMax(f64),
    PrincipalStressMid(f64),
    PrincipalStressMin(f64),
    HydrostaticStress(f64),
    Strain(StrainTensor),
    VonMisesStrain(f64),
    Temperature(f64),
    SafetyFactor(f64),
}

/// Complete result set for a solution step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolutionStep {
    pub step_number: usize,
    pub time: f64,
    pub nodes: Vec<MeshNode>,
    pub elements: Vec<MeshElement>,
}

/// Result container - holds all solution steps
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResultSet {
    pub job_id: String,
    pub analysis_type: AnalysisType,
    pub units: Units,
    pub steps: Vec<SolutionStep>,
    pub min_displacement: f64,
    pub max_displacement: f64,
    pub max_von_mises: f64,
    pub max_stress_location: Option<usize>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum AnalysisType {
    #[default]
    Static,
    Dynamic,
    Thermal,
    ThermalMechanical,
    Modal,
    Buckling,
    FrequencyResponse,
    RandomVibration,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Units {
    SI,        // m, N, Pa, kg
    Imperial,  // in, lbf, psi, slug
    MMTS,      // mm, N, MPa, tonne
}

impl Default for Units {
    fn default() -> Self {
        Units::SI
    }
}

/// Mesh data structure for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshData {
    pub nodes: Vec<MeshNode>,
    pub elements: Vec<MeshElement>,
    pub node_map: HashMap<usize, usize>,  // node_id -> index
    pub element_map: HashMap<usize, usize>, // element_id -> index
}

impl MeshData {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            elements: vec![],
            node_map: HashMap::new(),
            element_map: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: MeshNode) -> usize {
        let index = self.nodes.len();
        self.node_map.insert(node.id, index);
        self.nodes.push(node);
        index
    }

    pub fn add_element(&mut self, element: MeshElement) -> usize {
        let index = self.elements.len();
        self.element_map.insert(element.id, index);
        self.elements.push(element);
        index
    }

    pub fn get_node(&self, id: usize) -> Option<&MeshNode> {
        self.node_map.get(&id).and_then(|&i| self.nodes.get(i))
    }

    pub fn get_element(&self, id: usize) -> Option<&MeshElement> {
        self.element_map.get(&id).and_then(|&i| self.elements.get(i))
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn element_count(&self) -> usize {
        self.elements.len()
    }
}

impl Default for MeshData {
    fn default() -> Self {
        Self::new()
    }
}

/// Summary statistics for result visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultStats {
    pub result_type: String,
    pub min: f64,
    pub max: f64,
    pub average: f64,
    pub std_dev: f64,
}

impl ResultStats {
    pub fn compute(values: &[f64]) -> Self {
        if values.is_empty() {
            return Self {
                result_type: "Unknown".to_string(),
                min: 0.0,
                max: 0.0,
                average: 0.0,
                std_dev: 0.0,
            };
        }

        let sum: f64 = values.iter().sum();
        let avg = sum / values.len() as f64;
        
        let variance: f64 = values.iter().map(|v| (v - avg).powi(2)).sum::<f64>() / values.len() as f64;
        let std_dev = variance.sqrt();

        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        Self {
            result_type: "Computed".to_string(),
            min,
            max,
            average: avg,
            std_dev,
        }
    }
}

/// Color map for visualization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ColorMap {
    Jet,
    Viridis,
    Plasma,
    Inferno,
    Magma,
    BlueRed,
    Rainbow,
    Gray,
}

impl ColorMap {
    pub fn to_rgb(&self, value: f64, min: f64, max: f64) -> (u8, u8, u8) {
        let t = if max > min {
            ((value - min) / (max - min)).clamp(0.0, 1.0)
        } else {
            0.0
        };

        match self {
            ColorMap::Jet => self.jet_color(t),
            ColorMap::Viridis => self.viridis_color(t),
            ColorMap::Gray => {
                let v = (t * 255.0) as u8;
                (v, v, v)
            }
            _ => self.jet_color(t),
        }
    }

    fn jet_color(&self, t: f64) -> (u8, u8, u8) {
        // Classic jet colormap
        let r: f64 = if t < 0.5 { 0.0 } else { ((t - 0.5) * 2.0 * 255.0) as u8 as f64 };
        let g = if t < 0.25 {
            (t * 4.0 * 255.0) as u8
        } else if t < 0.75 {
            255u8
        } else {
            ((1.0 - t) * 4.0 * 255.0) as u8
        };
        let b = if t < 0.5 { ((0.5 - t) * 2.0 * 255.0) as u8 } else { 0u8 };

        (r as u8, g, b)
    }

    fn viridis_color(&self, t: f64) -> (u8, u8, u8) {
        // Simplified viridis approximation
        let r = ((0.267 + t * 2.892) * 255.0) as u8;
        let g = ((t.powi(2) * -0.329 + t * 0.498 + 0.005) * 255.0) as u8;
        let b = ((-t.powi(2) * 1.363 + t * 0.890 + 0.078) * 255.0) as u8;

        (r.min(255), g.min(255), b.min(255))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stress_von_mises() {
        let stress = StressTensor {
            s11: 100.0,
            s22: -50.0,
            s33: 0.0,
            s12: 0.0,
            s13: 0.0,
            s23: 0.0,
        };
        assert!(stress.von_mises() > 0.0);
    }

    #[test]
    fn test_vec3_length() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert!((v.length() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_mesh_node() {
        let node = MeshNode::new(1, 0.0, 0.0, 0.0);
        assert_eq!(node.id, 1);
    }

    #[test]
    fn test_result_stats() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let stats = ResultStats::compute(&values);
        assert!((stats.average - 3.0).abs() < 0.001);
    }
}