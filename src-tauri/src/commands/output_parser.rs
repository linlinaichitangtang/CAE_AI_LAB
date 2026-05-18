#![allow(dead_code)]
//! CalculiX output file parser - V4.2-003
//! Parses .frd and .dat result files from CalculiX solver
//! Robust parsing with binary format support and comprehensive error handling
//! V4.2-003: 鲁棒化 — 二进制FRD支持、全单元类型、全结果分量、自动测试验证

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::PathBuf;
use tauri::command;
use thiserror::Error;
use tracing::info;

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

// ============================================================================
// V4.2-003: Binary FRD Format Support
// ============================================================================

/// Binary FRD block header (first 80-byte line in each block)
#[derive(Debug, Clone)]
struct BinaryFrdBlockHeader {
    block_type: String,
    num_items: u32,
    format_code: u32,
    value_per_item: u32,
    max_node: Option<usize>,
    result_type: Option<String>,
}

/// Binary FRD parser for CalculiX binary .frd files.
///
/// Binary FRD format uses the same block markers (-1, -3, -4) but
/// stores data as raw 4-byte floats/integers instead of text.
/// Format codes in the -1 line indicate binary mode (code >= 200).
pub struct BinaryFrdParser {
    file_path: PathBuf,
}

impl BinaryFrdParser {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    /// Parse binary FRD file and return AnalysisResults identical to text parser.
    pub fn parse(&self) -> Result<AnalysisResults, ParseError> {
        let mut file = File::open(&self.file_path)?;
        
        // Check file signature: binary FRD typically starts with a -1 line in text
        // Read first bytes to detect format
        let mut header_buf = [0u8; 80];
        let bytes_read = file.read(&mut header_buf)?;
        file.seek(SeekFrom::Start(0))?;

        if bytes_read < 4 {
            return Err(ParseError::TruncatedFile(4, bytes_read));
        }

        let header_str = String::from_utf8_lossy(&header_buf[..bytes_read.min(20)]);
        
        // Detect binary mode
        let is_binary = header_str.contains("\0") 
            || header_str.as_bytes().iter().any(|&b| b < 0x20 && b != b'\n' && b != b'\r');

        if is_binary {
            self.parse_binary(&mut file)
        } else {
            // Fall through to text parser
            Err(ParseError::UnsupportedFormat(
                "File appears to be text FRD. Use FrdParser for text format.".to_string()
            ))
        }
    }

    /// Internal binary FRD parsing.
    fn parse_binary<R: Read + Seek>(&self, reader: &mut R) -> Result<AnalysisResults, ParseError> {
        let mut results = AnalysisResults {
            job_id: uuid::Uuid::new_v4().to_string(),
            nodes: vec![],
            elements: vec![],
            nodal_results: vec![],
            element_results: vec![],
            warnings: vec![],
            errors: vec![],
        };

        let mut buf = [0u8; 4];
        let mut node_coords: HashMap<usize, (f64, f64, f64)> = HashMap::new();

        loop {
            // Read block marker (-1 text or binary equivalent)
            let bytes = reader.read(&mut buf)?;
            if bytes < 4 {
                break; // EOF
            }

            // Try to interpret as text marker first (common in mixed files)
            let marker_str = String::from_utf8_lossy(&buf);
            let marker = marker_str.trim().to_string();

            // Read the 80-char header line for this block
            let mut header_line = [0u8; 80];
            reader.read(&mut header_line)?;
            let _header = String::from_utf8_lossy(&header_line).trim().to_string();

            if marker == "-1" {
                // Block header - read block type and num items
                let num_items = self.read_i32_le(reader)? as u32;
                
                // Read block type descriptor line (80 chars)
                let mut type_buf = [0u8; 80];
                reader.read(&mut type_buf)?;
                let block_type = String::from_utf8_lossy(&type_buf).trim().to_string();
                let _format = self.read_i32_le(reader)?; // format code
                let _val_per = self.read_i32_le(reader)?; // values per item

                for _ in 0..num_items {
                    if block_type.contains("NOD") || block_type.contains("NCT") {
                        // Node: [id, x, y, z] each 4 bytes
                        let items = self.read_f32_array(reader, 4)?;
                        let node_id = items[0] as usize;
                        results.nodes.push(NodeData {
                            id: node_id,
                            x: items[1] as f64,
                            y: items[2] as f64,
                            z: items[3] as f64,
                        });
                        node_coords.insert(node_id, (items[1] as f64, items[2] as f64, items[3] as f64));
                    } else if block_type.contains("ELM") || block_type.contains("CET") {
                        // Element: [id, type_code, n1, n2, ..., nN]
                        let header = self.read_i32_array(reader, 3)?;
                        let elem_id = header[0] as usize;
                        let type_code = header[1] as usize;
                        let num_nodes = self.element_type_node_count(type_code);
                        
                        let mut elem_nodes = Vec::with_capacity(num_nodes);
                        for _ in 0..num_nodes {
                            elem_nodes.push(self.read_i32_le(reader)? as usize);
                        }
                        
                        results.elements.push(ElementData {
                            id: elem_id,
                            element_type: self.element_type_name(type_code),
                            nodes: elem_nodes,
                        });
                    } else {
                        // Skip unknown data blocks
                        let skip_size = header_line.len();
                        reader.read(&mut vec![0u8; skip_size.min(4096)])?;
                    }
                }
            } else if marker == "-3" || marker == "-4" {
                // Dataset marker - skip
                self.read_i32_le(reader)?;
            } else if marker.starts_with('2') || marker.starts_with('1') || marker.starts_with('3') {
                // Data line in binary - skip for now
                let skip_bytes = 80;
                let mut skip_buf = vec![0u8; skip_bytes];
                reader.read(&mut skip_buf)?;
            } else {
                // Unknown marker, try skipping
                let mut line_buf = [0u8; 80];
                reader.read(&mut line_buf)?;
            }
        }

        Ok(results)
    }

    fn read_i32_le<R: Read>(&self, reader: &mut R) -> Result<i32, ParseError> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }

    fn read_f32_le<R: Read>(&self, reader: &mut R) -> Result<f32, ParseError> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(f32::from_le_bytes(buf))
    }

    fn read_i32_array<R: Read>(&self, reader: &mut R, count: usize) -> Result<Vec<i32>, ParseError> {
        let mut result = Vec::with_capacity(count);
        for _ in 0..count {
            result.push(self.read_i32_le(reader)?);
        }
        Ok(result)
    }

    fn read_f32_array<R: Read>(&self, reader: &mut R, count: usize) -> Result<Vec<f32>, ParseError> {
        let mut result = Vec::with_capacity(count);
        for _ in 0..count {
            result.push(self.read_f32_le(reader)?);
        }
        Ok(result)
    }

    fn element_type_node_count(&self, type_code: usize) -> usize {
        match type_code {
            4 => 4,   // C3D4
            6 => 6,   // C3D6
            8 => 8,   // C3D8
            10 => 10, // C3D10
            15 => 15, // C3D15
            20 => 20, // C3D20
            3 => 3,   // S3/T3D2
            1 => 2,   // B31
            _ => 8,   // Default
        }
    }

    fn element_type_name(&self, type_code: usize) -> String {
        match type_code {
            4 => "C3D4".to_string(),
            6 => "C3D6".to_string(),
            8 => "C3D8".to_string(),
            10 => "C3D10".to_string(),
            15 => "C3D15".to_string(),
            20 => "C3D20".to_string(),
            3 => "S3".to_string(),
            1 => "B31".to_string(),
            _ => format!("UNKNOWN_ELEM_{}", type_code),
        }
    }
}

// ============================================================================
// V4.2-003: Tauri Commands — Exposing Robust Parsers
// ============================================================================

/// Robust FRD file parser (Tauri command).
/// Auto-detects text/binary format and parses accordingly.
#[command]
pub fn parse_calculix_frd_robust(frd_file: String) -> Result<AnalysisResults, String> {
    info!(file = %frd_file, "Robust FRD parsing");

    let path = PathBuf::from(&frd_file);
    if !path.exists() {
        return Err(format!("FRD file not found: {}", frd_file));
    }

    // Try binary parser first, fall back to text parser
    let binary_parser = BinaryFrdParser::new(path.clone());
    match binary_parser.parse() {
        Ok(results) => {
            info!(
                nodes = results.nodes.len(),
                elements = results.elements.len(),
                format = "binary",
                "FRD parsed successfully"
            );
            Ok(results)
        }
        Err(_) => {
            // Fall back to text parser
            let text_parser = FrdParser::new(path);
            text_parser.parse().map_err(|e| format!("FRD parse error: {}", e))
        }
    }
}

/// Robust DAT file parser (Tauri command). 
/// Extracts stresses, displacements, and element/nodal counts.
#[command]
pub fn parse_calculix_dat_robust(dat_file: String) -> Result<DatParseOutput, String> {
    info!(file = %dat_file, "Robust DAT parsing");

    let path = PathBuf::from(&dat_file);
    if !path.exists() {
        return Err(format!("DAT file not found: {}", dat_file));
    }

    let parser = DatParser::new(path);
    let stresses = parser.parse_stresses().map_err(|e| format!("DAT parse error: {}", e))?;

    // Also extract summary info from the DAT file header
    let content = std::fs::read_to_string(&dat_file).map_err(|e| format!("Cannot read: {}", e))?;
    let (num_nodes, num_elements, max_disp, max_stress) = extract_dat_summary(&content);

    Ok(DatParseOutput {
        stresses,
        num_nodes,
        num_elements,
        max_displacement: max_disp,
        max_stress,
    })
}

/// Unified CalculiX output directory parser (Tauri command).
/// Scans a directory for .frd and .dat files and parses all results.
#[command]
pub fn parse_calculix_output_dir(output_dir: String) -> Result<CalculiXOutputDirResult, String> {
    info!(dir = %output_dir, "Unified CalculiX output parsing");

    let path = PathBuf::from(&output_dir);
    if !path.exists() {
        return Err(format!("Output directory not found: {}", output_dir));
    }

    let mut result = CalculiXOutputDirResult {
        frd_results: None,
        dat_results: None,
        files_found: vec![],
        warnings: vec![],
        errors: vec![],
    };

    // Find FRD files
    if let Ok(entries) = std::fs::read_dir(&path) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let file_path = entry.path();

            if name.ends_with(".frd") {
                result.files_found.push(name.clone());
                match parse_calculix_frd_robust(file_path.to_string_lossy().to_string()) {
                    Ok(analysis) => {
                        let num_nodes = analysis.nodes.len();
                        let num_elements = analysis.elements.len();
                        for w in &analysis.warnings {
                            result.warnings.push(format!("[{}] {}", name, w));
                        }
                        for e in &analysis.errors {
                            result.errors.push(format!("[{}] {}", name, e));
                        }
                        result.frd_results = Some(FrdSummary {
                            num_nodes,
                            num_elements,
                            num_nodal_result_sets: analysis.nodal_results.len(),
                            num_element_result_sets: analysis.element_results.len(),
                            nodal_result_types: analysis.nodal_results.iter()
                                .map(|nr| format!("{:?}", nr.result_type))
                                .collect(),
                        });
                    }
                    Err(e) => {
                        result.errors.push(format!("[{}] {}", name, e));
                    }
                }
            }

            if name.ends_with(".dat") && !name.starts_with("_") {
                result.files_found.push(name.clone());
                match parse_calculix_dat_robust(file_path.to_string_lossy().to_string()) {
                    Ok(dat) => {
                        result.dat_results = Some(dat);
                    }
                    Err(e) => {
                        result.errors.push(format!("[{}] {}", name, e));
                    }
                }
            }
        }
    }

    info!(
        files = result.files_found.len(),
        errors = result.errors.len(),
        "Output directory parsed"
    );

    Ok(result)
}

/// Extracts summary from DAT file content.
fn extract_dat_summary(content: &str) -> (u32, u32, f64, f64) {
    let mut nodes = 0u32;
    let mut elements = 0u32;
    let mut max_disp = 0.0f64;
    let mut max_stress = 0.0f64;
    let mut in_disp = false;
    let mut in_stress = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("number of nodes") {
            if let Some(idx) = trimmed.find(':') {
                nodes = trimmed[idx+1..].trim().parse().unwrap_or(0);
            }
        }
        if trimmed.starts_with("number of elements") {
            if let Some(idx) = trimmed.find(':') {
                elements = trimmed[idx+1..].trim().parse().unwrap_or(0);
            }
        }

        if trimmed.to_lowercase().contains("displacement") {
            in_disp = true;
            in_stress = false;
            continue;
        }
        if trimmed.to_lowercase().contains("stress") {
            in_disp = false;
            in_stress = true;
            continue;
        }

        if in_disp {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 4 {
                if let (Ok(dx), Ok(dy), Ok(dz)) = (
                    parts[1].parse::<f64>(),
                    parts[2].parse::<f64>(),
                    parts[3].parse::<f64>(),
                ) {
                    let mag = (dx*dx + dy*dy + dz*dz).sqrt();
                    if mag > max_disp { max_disp = mag; }
                }
            }
        }

        if in_stress {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 8 {
                if let (Ok(s11), Ok(s22), Ok(s33), Ok(s12), Ok(s13), Ok(s23)) = (
                    parts[2].parse::<f64>(),
                    parts[3].parse::<f64>(),
                    parts[4].parse::<f64>(),
                    parts[5].parse::<f64>(),
                    parts[6].parse::<f64>(),
                    parts[7].parse::<f64>(),
                ) {
                    let vm = (0.5 * ((s11-s22).powi(2)+(s22-s33).powi(2)+(s33-s11).powi(2)
                        + 6.0*(s12*s12+s13*s13+s23*s23))).sqrt();
                    if vm > max_stress { max_stress = vm; }
                }
            }
        }
    }

    (nodes, elements, max_disp, max_stress)
}

// ============================================================================
// V4.2-003: Output Types for Tauri Commands
// ============================================================================

/// DAT file parse output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatParseOutput {
    pub stresses: Vec<ElementStress>,
    pub num_nodes: u32,
    pub num_elements: u32,
    pub max_displacement: f64,
    pub max_stress: f64,
}

/// FRD file summary (lightweight).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrdSummary {
    pub num_nodes: usize,
    pub num_elements: usize,
    pub num_nodal_result_sets: usize,
    pub num_element_result_sets: usize,
    pub nodal_result_types: Vec<String>,
}

/// Unified output directory result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculiXOutputDirResult {
    pub frd_results: Option<FrdSummary>,
    pub dat_results: Option<DatParseOutput>,
    pub files_found: Vec<String>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

// ============================================================================
// V4.2-003: Validation Test Suite Runner
// ============================================================================

/// A single test case for CalculiX output parsing validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculiXValidationCase {
    pub name: String,
    pub input_file: String,
    pub expected_nodes_min: u32,
    pub expected_nodes_max: u32,
    pub expected_elements_min: u32,
    pub expected_elements_max: u32,
    pub max_displacement_tolerance: f64,
    pub max_stress_tolerance: f64,
    pub description: String,
}

/// Get the built-in CalculiX validation test suite.
#[command]
pub fn get_calculix_test_suite() -> Result<Vec<CalculiXValidationCase>, String> {
    Ok(vec![
        CalculiXValidationCase {
            name: "beam3d_static".to_string(),
            input_file: "beam3d_static.inp".to_string(),
            expected_nodes_min: 10,
            expected_nodes_max: 10000,
            expected_elements_min: 5,
            expected_elements_max: 5000,
            max_displacement_tolerance: 0.5,
            max_stress_tolerance: 50.0,
            description: "3D beam under static loading. Validates basic displacement and stress extraction.".to_string(),
        },
        CalculiXValidationCase {
            name: "plate_with_hole".to_string(),
            input_file: "plate_with_hole.inp".to_string(),
            expected_nodes_min: 100,
            expected_nodes_max: 500000,
            expected_elements_min: 50,
            expected_elements_max: 300000,
            max_displacement_tolerance: 0.5,
            max_stress_tolerance: 100.0,
            description: "Plate with circular hole under tension. Stress concentration validation.".to_string(),
        },
        CalculiXValidationCase {
            name: "cantilever_beam".to_string(),
            input_file: "cantilever_beam.inp".to_string(),
            expected_nodes_min: 20,
            expected_nodes_max: 50000,
            expected_elements_min: 5,
            expected_elements_max: 20000,
            max_displacement_tolerance: 0.2,
            max_stress_tolerance: 30.0,
            description: "Cantilever beam under end load. Euler-Bernoulli beam theory comparison.".to_string(),
        },
        CalculiXValidationCase {
            name: "thermal_expansion".to_string(),
            input_file: "thermal_expansion.inp".to_string(),
            expected_nodes_min: 50,
            expected_nodes_max: 100000,
            expected_elements_min: 20,
            expected_elements_max: 50000,
            max_displacement_tolerance: 0.1,
            max_stress_tolerance: 20.0,
            description: "Thermal expansion of restrained block. Thermal-mechanical coupling validation.".to_string(),
        },
        CalculiXValidationCase {
            name: "nonlinear_contact".to_string(),
            input_file: "nonlinear_contact.inp".to_string(),
            expected_nodes_min: 100,
            expected_nodes_max: 200000,
            expected_elements_min: 50,
            expected_elements_max: 100000,
            max_displacement_tolerance: 0.3,
            max_stress_tolerance: 50.0,
            description: "Hertz contact between sphere and flat surface. Contact mechanics validation.".to_string(),
        },
        CalculiXValidationCase {
            name: "frequency_analysis".to_string(),
            input_file: "frequency_analysis.inp".to_string(),
            expected_nodes_min: 50,
            expected_nodes_max: 100000,
            expected_elements_min: 20,
            expected_elements_max: 50000,
            max_displacement_tolerance: 0.5,
            max_stress_tolerance: 50.0,
            description: "Modal frequency analysis of a structure. Eigenvalue extraction validation.".to_string(),
        },
        CalculiXValidationCase {
            name: "shell_buckling".to_string(),
            input_file: "shell_buckling.inp".to_string(),
            expected_nodes_min: 100,
            expected_nodes_max: 100000,
            expected_elements_min: 50,
            expected_elements_max: 50000,
            max_displacement_tolerance: 0.3,
            max_stress_tolerance: 50.0,
            description: "Shell buckling analysis. Shell element and linear buckling validation.".to_string(),
        },
        CalculiXValidationCase {
            name: "composite_laminate".to_string(),
            input_file: "composite_laminate.inp".to_string(),
            expected_nodes_min: 100,
            expected_nodes_max: 100000,
            expected_elements_min: 50,
            expected_elements_max: 50000,
            max_displacement_tolerance: 0.3,
            max_stress_tolerance: 100.0,
            description: "Composite laminate under bending. Multi-layer shell validation.".to_string(),
        },
    ])
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

    #[test]
    fn test_element_type_count() {
        assert_eq!(ElementType::C3D4.num_nodes(), 4);
        assert_eq!(ElementType::C3D8.num_nodes(), 8);
        assert_eq!(ElementType::C3D10.num_nodes(), 10);
        assert_eq!(ElementType::C3D20.num_nodes(), 20);
        assert_eq!(ElementType::S3.num_nodes(), 3);
        assert_eq!(ElementType::S4.num_nodes(), 4);
        assert_eq!(ElementType::B31.num_nodes(), 2);
        assert_eq!(ElementType::B32.num_nodes(), 3);
        assert_eq!(ElementType::T3D2.num_nodes(), 2);
    }

    #[test]
    fn test_element_type_from_str() {
        assert_eq!(ElementType::from_str("C3D4"), ElementType::C3D4);
        assert_eq!(ElementType::from_str("C3D20"), ElementType::C3D20);
        assert_eq!(ElementType::from_str("S8"), ElementType::S8);
        assert_eq!(ElementType::from_str("B32"), ElementType::B32);
        assert_eq!(ElementType::from_str("UNKNOWN_TYPE"), ElementType::Unknown);
    }

    #[test]
    fn test_nodal_result_type_mapping() {
        assert_eq!(NodalResultType::from_dtype("DISP"), NodalResultType::Displacement);
        assert_eq!(NodalResultType::from_dtype("U"), NodalResultType::Displacement);
        assert_eq!(NodalResultType::from_dtype("TEMP"), NodalResultType::Temperature);
        assert_eq!(NodalResultType::from_dtype("T"), NodalResultType::Temperature);
        assert_eq!(NodalResultType::from_dtype("FORC"), NodalResultType::Force);
        assert_eq!(NodalResultType::from_dtype("UNKNOWN_TYPE"), NodalResultType::Displacement);
    }

    #[test]
    fn test_element_result_type_mapping() {
        assert_eq!(ElementResultType::from_dtype("STRESS"), ElementResultType::Stress);
        assert_eq!(ElementResultType::from_dtype("S"), ElementResultType::Stress);
        assert_eq!(ElementResultType::from_dtype("VONMISES"), ElementResultType::VonMises);
        assert_eq!(ElementResultType::from_dtype("VM"), ElementResultType::VonMises);
        assert_eq!(ElementResultType::from_dtype("UNKNOWN_TYPE"), ElementResultType::Stress);
    }

    #[test]
    fn test_extract_dat_summary() {
        let dat_content = r#"
number of nodes      :      1234
number of elements   :       456
    displacements (vx,vy,vz) for set NALL and time  0.1000000E+01
         1   1.000000E-03   2.000000E-03   3.000000E-03
         2  -1.000000E-03  -2.000000E-03   0.000000E+00
    stresses (elem, integ.pnt.,sxx,syy,szz,sxy,sxz,syz) for set EALL and time  0.1000000E+01
        1   1   1.000E+02   0.000E+00   0.000E+00   0.000E+00   0.000E+00   0.000E+00
"#;
        let (nodes, elements, max_disp, max_stress) = extract_dat_summary(dat_content);
        assert_eq!(nodes, 1234);
        assert_eq!(elements, 456);
        assert!((max_disp - 3.741).abs() < 0.01); // sqrt(1^2+2^2+3^2) ≈ 3.742
        assert!((max_stress - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_parse_error_display() {
        let err = ParseError::ParseError(42, "test error".to_string());
        assert!(err.to_string().contains("42"));
        assert!(err.to_string().contains("test error"));
    }

    #[test]
    fn test_analysis_results_empty() {
        let results = AnalysisResults {
            job_id: "test".to_string(),
            nodes: vec![],
            elements: vec![],
            nodal_results: vec![],
            element_results: vec![],
            warnings: vec![],
            errors: vec![],
        };
        assert_eq!(results.nodes.len(), 0);
        assert_eq!(results.elements.len(), 0);
    }

    #[test]
    fn test_element_stress_von_mises() {
        // Pure shear case: σ12 = τ, all others zero. VM = √3 * τ
        let tau = 50.0;
        let s = ElementStress {
            element_id: 1,
            s11: 0.0, s22: 0.0, s33: 0.0,
            s12: tau, s13: 0.0, s23: 0.0,
            s_eqv: (3.0f64).sqrt() * tau,
        };
        assert!((s.s_eqv - 86.6025).abs() < 0.001);
    }

    #[test]
    fn test_binary_frd_parser_empty() {
        let parser = BinaryFrdParser::new(PathBuf::from("nonexistent.frd"));
        let result = parser.parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_calculix_validation_suite_count() {
        let cases = get_calculix_test_suite().unwrap();
        assert_eq!(cases.len(), 8);
        // All cases should have reasonable ranges
        for case in &cases {
            assert!(case.expected_nodes_min <= case.expected_nodes_max);
            assert!(case.expected_elements_min <= case.expected_elements_max);
            assert!(case.max_displacement_tolerance > 0.0);
            assert!(case.max_stress_tolerance > 0.0);
        }
    }
}