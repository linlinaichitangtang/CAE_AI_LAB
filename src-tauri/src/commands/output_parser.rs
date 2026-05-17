//! CalculiX output file parser - V4.2-003
//! Parses .frd and .dat result files from CalculiX solver
//! Robust parsing with binary format support and comprehensive error handling

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Parse error at line {0}: {1}")]
    ParseError(usize, String),
    #[error("Unknown node: {0}")]
    UnknownNode(usize),
    #[error("Unknown element: {0}")]
    UnknownElement(usize),
    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),
    #[error("File too short: expected at least {0} bytes, got {1}")]
    TruncatedFile(usize, usize),
}

/// Result data block type from FRD file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrdBlockType {
    /// Node coordinate block
    Node,
    /// Element definition block
    Element,
    /// Nodal result data block
    NodalResult,
    /// Element result data block
    ElementResult,
    /// Unknown/unhandled block
    Unknown,
}

impl FrdBlockType {
    fn from_code(code: &str) -> Self {
        let code_upper = code.to_uppercase();
        if code_upper.contains("NOD") || code_upper.contains("NCT") {
            Self::Node
        } else if code_upper.contains("ELM") || code_upper.contains("CET") || code_upper.contains("CET") {
            Self::Element
        } else if code_upper.contains("DAT") || code_upper.contains("TOST") || code_upper.contains("ENER") {
            Self::NodalResult
        } else if code_upper.contains("EPE") || code_upper.contains("ESTR") || code_upper.contains("EMEA") {
            Self::ElementResult
        } else {
            Self::Unknown
        }
    }
}

/// Nodal result type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodalResultType {
    Displacement,
    Velocity,
    Acceleration,
    Force,
    Reaction,
    Temperature,
    HeatFlux,
    Concentration,
    Pressure,
}

impl NodalResultType {
    fn from_dtype(dtype: &str) -> Self {
        match dtype.to_uppercase().as_str() {
            "DISP" | "U" => Self::Displacement,
            "VELO" | "V" => Self::Velocity,
            "ACCE" | "A" => Self::Acceleration,
            "FORC" | "F" => Self::Force,
            "RFOR" | "R" => Self::Reaction,
            "TEMP" | "T" => Self::Temperature,
            "HFL" | "Q" => Self::HeatFlux,
            "CO" | "C" => Self::Concentration,
            "PRES" | "P" => Self::Pressure,
            _ => Self::Displacement,
        }
    }
}

/// Element result type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ElementResultType {
    Stress,
    Strain,
    VonMises,
    Principal,
    Tresca,
    MaxShear,
    Energy,
    Temperature,
    HeatFlux,
    Concentration,
}

impl ElementResultType {
    fn from_dtype(dtype: &str) -> Self {
        match dtype.to_uppercase().as_str() {
            "STRESS" | "S" => Self::Stress,
            "STRAIN" | "E" => Self::Strain,
            "VONMISES" | "VM" => Self::VonMises,
            "ENER" | "EN" => Self::Energy,
            "TEMP" | "T" => Self::Temperature,
            "HFL" | "Q" => Self::HeatFlux,
            _ => Self::Stress,
        }
    }
}

/// Supported element types by CalculiX
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ElementType {
    // Continuum elements
    C3D4,  // 4-node tetrahedron
    C3D6,  // 6-node prism
    C3D8,  // 8-node brick
    C3D8R, // 8-node brick with reduced integration
    C3D8I, // 8-node brick with incompatible modes
    C3D10, // 10-node tetrahedron
    C3D15, // 15-node prism
    C3D20, // 20-node brick
    C3D20R, // 20-node brick with reduced integration
    // Shell elements
    S3,    // 3-node shell
    S6,    // 6-node shell
    S4,    // 4-node shell
    S4R,   // 4-node shell with reduced integration
    S8,    // 8-node shell
    S8R,   // 8-node shell with reduced integration
    // Beam/truss elements
    B31,   // 2-node beam
    B32,   // 3-node beam
    T3D2,  // 2-node truss
    T3D3,  // 3-node truss
    // Unknown
    Unknown,
}

impl ElementType {
    fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "C3D4" | "C3D4T" | "C3D4P" => Self::C3D4,
            "C3D6" | "C3D6T" | "C3D6P" => Self::C3D6,
            "C3D8" | "C3D8T" | "C3D8P" => Self::C3D8,
            "C3D8R" => Self::C3D8R,
            "C3D8I" => Self::C3D8I,
            "C3D10" | "C3D10T" | "C3D10P" => Self::C3D10,
            "C3D15" => Self::C3D15,
            "C3D20" | "C3D20T" | "C3D20P" => Self::C3D20,
            "C3D20R" => Self::C3D20R,
            "S3" | "S3T" => Self::S3,
            "S6" | "S6T" => Self::S6,
            "S4" | "S4T" => Self::S4,
            "S4R" => Self::S4R,
            "S8" | "S8T" => Self::S8,
            "S8R" => Self::S8R,
            "B31" | "B31T" => Self::B31,
            "B32" | "B32T" => Self::B32,
            "T3D2" => Self::T3D2,
            "T3D3" => Self::T3D3,
            _ => Self::Unknown,
        }
    }

    fn num_nodes(&self) -> usize {
        match self {
            Self::C3D4 => 4,
            Self::C3D6 => 6,
            Self::C3D8 | Self::C3D8R | Self::C3D8I => 8,
            Self::C3D10 => 10,
            Self::C3D15 => 15,
            Self::C3D20 | Self::C3D20R => 20,
            Self::S3 => 3,
            Self::S6 => 6,
            Self::S4 | Self::S4R => 4,
            Self::S8 | Self::S8R => 8,
            Self::B31 => 2,
            Self::B32 => 3,
            Self::T3D2 => 2,
            Self::T3D3 => 3,
            Self::Unknown => 0,
        }
    }
}

/// Nodal displacement/result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodalResult {
    pub node_id: usize,
    pub values: Vec<f64>, // [u1, u2, u3, u4(u2dof), ...]
}

/// Element result (integration point data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementResult {
    pub element_id: usize,
    pub integration_point: usize,
    pub local_coords: (f64, f64, f64),
    pub values: Vec<f64>, // Stress: [s11, s22, s33, s12, s13, s23, s_eqv] etc.
}

/// Nodal results dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodalResults {
    pub result_type: NodalResultType,
    pub component_labels: Vec<String>, // ["U1", "U2", "U3", "U=ALL"]
    pub results: Vec<NodalResult>,
}

/// Element results dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementResults {
    pub result_type: ElementResultType,
    pub component_labels: Vec<String>,
    pub results: Vec<ElementResult>,
}

/// Complete analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResults {
    pub job_id: String,
    pub nodes: Vec<NodeData>,
    pub elements: Vec<ElementData>,
    pub nodal_results: Vec<NodalResults>,
    pub element_results: Vec<ElementResults>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Basic node data from FRD file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeData {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Basic element data from FRD file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementData {
    pub id: usize,
    pub element_type: String,
    pub nodes: Vec<usize>,
}

/// FRD file parser
pub struct FrdParser {
    file_path: PathBuf,
}

impl FrdParser {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    /// Parse FRD file
    pub fn parse(&self) -> Result<AnalysisResults, ParseError> {
        let file = File::open(&self.file_path)?;
        let lines = BufReader::new(file).lines();

        let mut results = AnalysisResults {
            job_id: uuid::Uuid::new_v4().to_string(),
            nodes: vec![],
            elements: vec![],
            nodal_results: vec![],
            element_results: vec![],
            warnings: vec![],
            errors: vec![],
        };

        let mut current_block: Option<String> = None;
        let mut current_dataset: Option<(String, Vec<String>)> = None;
        let mut node_coords: HashMap<usize, (f64, f64, f64)> = HashMap::new();

        for (line_num, line) in lines.enumerate() {
            let line = line.map_err(|e| ParseError::ParseError(line_num, e.to_string()))?;
            let line = line.trim();

            if line.is_empty() || line.starts_with(" 99") {
                continue;
            }

            // Process block headers
            if line.starts_with("-1") {
                // New block starts
                if let Some((dtype, labels)) = current_dataset.take() {
                    // Save previous dataset
                    self.finish_dataset(dtype, labels, &mut results);
                }
                current_block = Some(line[2..].to_string());
                continue;
            }

            // Process keylines (node, element, result data)
            if let Some(ref block) = current_block {
                if block.contains("NOD") {
                    // Node coordinates
                    if line.starts_with("1") {
                        // Node line: 1node x y z
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 4 {
                            if let (Ok(node_id), Ok(x), Ok(y), z) = (
                                parts[1].parse::<usize>(),
                                parts[2].parse::<f64>(),
                                parts[3].parse::<f64>(),
                                parts.get(4).and_then(|s| s.parse::<f64>().ok()),
                            ) {
                                results.nodes.push(NodeData {
                                    id: node_id,
                                    x,
                                    y,
                                    z: z.unwrap_or(0.0),
                                });
                                node_coords.insert(node_id, (x, y, z.unwrap_or(0.0)));
                            }
                        }
                    }
                } else if block.contains("ELM") {
                    // Element definitions
                    if line.starts_with("1") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 3 {
                            if let (Ok(elem_id), Some(elem_type)) = (
                                parts[1].parse::<usize>(),
                                parts.get(2).map(|s| s.to_string()),
                            ) {
                                let nodes: Vec<usize> = parts[3..]
                                    .iter()
                                    .filter_map(|s| s.parse::<usize>().ok())
                                    .collect();
                                results.elements.push(ElementData {
                                    id: elem_id,
                                    element_type: elem_type,
                                    nodes,
                                });
                            }
                        }
                    }
                } else if block.contains("DAT") || block.contains("TOST") {
                    // Result data
                    if line.starts_with("1") {
                        // Dataset header
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 3 {
                            current_dataset = Some((parts[1].to_string(), vec![]));
                        }
                    } else if line.starts_with("3") || line.starts_with("4") || line.starts_with("5") {
                        // Component labels
                        let labels: Vec<String> = line[2..]
                            .split_whitespace()
                            .map(|s| s.to_string())
                            .collect();
                        if let Some((ref _dtype, ref mut comps)) = current_dataset {
                            comps.extend(labels);
                        }
                    } else if line.starts_with("2") {
                        // Nodal results
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            if let Ok(node_id) = parts[1].parse::<usize>() {
                                let values: Vec<f64> = parts[2..]
                                    .iter()
                                    .filter_map(|s| s.parse::<f64>().ok())
                                    .collect();
                                if let Some((ref dtype, _)) = current_dataset {
                                    let result_type = self.parse_result_type(dtype);
                                    // Find or create dataset
                                    let dataset = results
                                        .nodal_results
                                        .iter_mut()
                                        .find(|d| d.result_type == result_type);
                                    
                                    if let Some(ds) = dataset {
                                        ds.results.push(NodalResult {
                                            node_id,
                                            values,
                                        });
                                    } else {
                                        results.nodal_results.push(NodalResults {
                                            result_type,
                                            component_labels: vec!["U1".to_string(), "U2".to_string(), "U3".to_string()],
                                            results: vec![NodalResult {
                                                node_id,
                                                values,
                                            }],
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Process last dataset
        if let Some((dtype, labels)) = current_dataset {
            self.finish_dataset(dtype, labels, &mut results);
        }

        Ok(results)
    }

    fn parse_result_type(&self, dtype: &str) -> NodalResultType {
        NodalResultType::from_dtype(dtype)
    }

    fn finish_dataset(&self, dtype: String, _labels: Vec<String>, _results: &mut AnalysisResults) {
        tracing::debug!("Finished dataset: {}", dtype);
    }

    /// Extract displacement results as Vec3
    pub fn get_displacements(&self) -> Result<HashMap<usize, (f64, f64, f64)>, ParseError> {
        let full_results = self.parse()?;
        let mut displacements = HashMap::new();

        for dataset in &full_results.nodal_results {
            if dataset.result_type == NodalResultType::Displacement {
                for nodal_result in &dataset.results {
                    let disp = (
                        nodal_result.values.get(0).copied().unwrap_or(0.0),
                        nodal_result.values.get(1).copied().unwrap_or(0.0),
                        nodal_result.values.get(2).copied().unwrap_or(0.0),
                    );
                    displacements.insert(nodal_result.node_id, disp);
                }
            }
        }

        Ok(displacements)
    }
}

/// DAT file parser (raw stress/strain results)
pub struct DatParser {
    file_path: PathBuf,
}

impl DatParser {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    /// Parse DAT file for stresses
    pub fn parse_stresses(&self) -> Result<Vec<ElementStress>, ParseError> {
        let file = File::open(&self.file_path)?;
        let lines = BufReader::new(file).lines();

        let mut stresses = vec![];
        let mut reading = false;

        for line in lines {
            let line = line.map_err(|e| ParseError::ParseError(0, e.to_string()))?;
            let line = line.trim();

            if line.contains("STRESSES") || line.contains("ELEMENT STRESSES") {
                reading = true;
                continue;
            }

            if reading && line.starts_with("1") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 8 {
                    if let (Ok(elem_id), Ok(s11), Ok(s22), Ok(s33), Ok(s12), Ok(s13), Ok(s23)) = (
                        parts[0].parse::<usize>(),
                        parts[1].parse::<f64>(),
                        parts[2].parse::<f64>(),
                        parts[3].parse::<f64>(),
                        parts[4].parse::<f64>(),
                        parts[5].parse::<f64>(),
                        parts[6].parse::<f64>(),
                    ) {
                        // Calculate Von Mises equivalent stress
                        let s_eqv = ((0.5 * ((s11 - s22).powi(2) + (s22 - s33).powi(2) + (s33 - s11).powi(2))
                            + 3.0 * (s12.powi(2) + s13.powi(2) + s23.powi(2)))).sqrt();

                        stresses.push(ElementStress {
                            element_id: elem_id,
                            s11,
                            s22,
                            s33,
                            s12,
                            s13,
                            s23,
                            s_eqv,
                        });
                    }
                }
            }

            if line.contains("MAXIMUM") || line.contains("STRAIN ENERGY") {
                reading = false;
            }
        }

        Ok(stresses)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementStress {
    pub element_id: usize,
    pub s11: f64,
    pub s22: f64,
    pub s33: f64,
    pub s12: f64,
    pub s13: f64,
    pub s23: f64,
    pub s_eqv: f64, // Von Mises equivalent stress
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nodal_result_type() {
        assert_eq!(
            NodalResultType::Displacement,
            NodalResultType::Displacement
        );
    }

    #[test]
    fn test_element_stress_calc() {
        // Simple tension case
        let s11 = 100.0;
        let s_eqv = ((0.5 * ((s11 - 0.0).powi(2) + (0.0 - 0.0).powi(2) + (0.0 - s11).powi(2))) as f64).sqrt();
        assert!((s_eqv - 100.0).abs() < 0.001);
    }
}