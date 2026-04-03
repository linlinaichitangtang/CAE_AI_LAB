// DFT Input Generation Commands - V1.7
// Provides VASP and Quantum ESPRESSO input generation, parsing, and template management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::command;
use tracing::info;

// ============================================================================
// Data Structures
// ============================================================================

/// Represents a single atom in the crystal basis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasisAtom {
    pub element: String,
    pub position: [f64; 3],
}

/// Crystal structure definition with lattice and basis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrystalStructure {
    pub lattice_vectors: [[f64; 3]; 3],
    pub basis: Vec<BasisAtom>,
    pub cartesian: bool,
}

/// Configuration for VASP input generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaspInputConfig {
    pub system_name: String,
    pub structure: CrystalStructure,
    pub calculation_type: String,
    pub incar_params: HashMap<String, serde_json::Value>,
    pub kpoints_scheme: String,
    pub kpoints_grid: [u32; 3],
    pub kpoints_shift: [f64; 3],
    pub potcar_path: Option<String>,
    pub pseudo_potentials: HashMap<String, String>,
}

/// Configuration for Quantum ESPRESSO input generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QeInputConfig {
    pub calculation_type: String,
    pub task: String,
    pub control_params: HashMap<String, serde_json::Value>,
    pub system_params: HashMap<String, serde_json::Value>,
    pub electrons_params: HashMap<String, serde_json::Value>,
    pub structure: CrystalStructure,
}

/// Result of DFT input generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DftInputResult {
    pub poscar_content: String,
    pub incar_content: String,
    pub kpoints_content: String,
    pub potcar_elements: Vec<String>,
    pub warnings: Vec<String>,
}

/// Parsed VASP POSCAR structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaspPoscar {
    pub comment: String,
    pub scaling_factor: f64,
    pub lattice_vectors: [[f64; 3]; 3],
    pub element_names: Vec<String>,
    pub element_counts: Vec<u32>,
    pub coordinate_type: String,
    pub basis: Vec<BasisAtom>,
}

/// Parsed VASP INCAR parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaspIncar {
    pub params: HashMap<String, serde_json::Value>,
}

/// VASP input template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaspTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub calculation_type: String,
    pub incar_params: HashMap<String, serde_json::Value>,
    pub kpoints_scheme: String,
    pub default_kpoints_grid: [u32; 3],
}

/// Quantum ESPRESSO input template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QeTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub calculation_type: String,
    pub task: String,
    pub control_params: HashMap<String, serde_json::Value>,
    pub system_params: HashMap<String, serde_json::Value>,
    pub electrons_params: HashMap<String, serde_json::Value>,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Formats a lattice vector line for POSCAR output.
fn format_lattice_line(vec: &[f64; 3]) -> String {
    format!("{:20.16} {:20.16} {:20.16}", vec[0], vec[1], vec[2])
}

/// Formats a position line for POSCAR output.
fn format_position_line(atom: &BasisAtom) -> String {
    format!("{:<4} {:20.16} {:20.16} {:20.16}",
        atom.element,
        atom.position[0],
        atom.position[1],
        atom.position[2],
    )
}

/// Collects unique elements in order of first appearance.
fn collect_elements(basis: &[BasisAtom]) -> Vec<String> {
    let mut seen = HashMap::new();
    let mut order = Vec::new();
    for atom in basis {
        if !seen.contains_key(&atom.element) {
            seen.insert(atom.element.clone(), true);
            order.push(atom.element.clone());
        }
    }
    order
}

/// Counts atoms per element.
fn count_elements(basis: &[BasisAtom], elements: &[String]) -> Vec<u32> {
    elements.iter().map(|el| {
        basis.iter().filter(|a| a.element == *el).count() as u32
    }).collect()
}

/// Validates INCAR parameters and returns warnings.
fn validate_incar_params(params: &HashMap<String, serde_json::Value>) -> Vec<String> {
    let mut warnings = Vec::new();

    if let Some(encut) = params.get("ENCUT") {
        if let Some(v) = encut.as_f64() {
            if v < 200.0 {
                warnings.push("ENCUT is very low (< 200 eV). Results may not be converged.".to_string());
            }
        }
    } else {
        warnings.push("ENCUT not specified. Using VASP default, which may be insufficient.".to_string());
    }

    if let Some(ismear) = params.get("ISMEAR") {
        if let Some(v) = ismear.as_i64() {
            if v == -5 {
                if let Some(sigma) = params.get("SIGMA") {
                    if let Some(sv) = sigma.as_f64() {
                        if sv > 0.2 {
                            warnings.push("ISMEAR = -5 with large SIGMA. Tetrahedron method should use small SIGMA.".to_string());
                        }
                    }
                }
            }
            if v == 0 {
                if let Some(_kpts) = params.get("KPOINTS") {
                    // Just a warning for metallic systems
                    warnings.push("ISMEAR = 0 (Gaussian) may not be suitable for metals. Consider ISMEAR = 1 or 2.".to_string());
                }
            }
        }
    }

    if let Some(ediag) = params.get("ALGO") {
        if let Some(v) = ediag.as_str() {
            if v == "All" {
                warnings.push("ALGO = All is computationally expensive. Use only for final high-accuracy runs.".to_string());
            }
        }
    }

    if let Some(nsw) = params.get("NSW") {
        if let Some(v) = nsw.as_i64() {
            if v > 100 {
                warnings.push(format!("NSW = {} is large. Consider IBRION = 2 for robust relaxation.", v));
            }
        }
    }

    if let Some(ediffg) = params.get("EDIFFG") {
        if let Some(v) = ediffg.as_f64() {
            if v > 0.0 && v < 0.01 {
                warnings.push("EDIFFG is positive and very small. Force convergence may be too tight.".to_string());
            }
        }
    }

    warnings
}

/// Builds INCAR content string from parameters.
fn build_incar_content(system_name: &str, params: &HashMap<String, serde_json::Value>) -> String {
    let mut lines = vec![format!("! {} - Generated by DFT Module V1.7", system_name)];
    lines.push("".to_string());

    // Sort parameters for deterministic output
    let mut keys: Vec<_> = params.keys().cloned().collect();
    keys.sort();

    for key in &keys {
        if let Some(value) = params.get(key) {
            let value_str = match value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => {
                    if n.is_i64() { n.as_i64().unwrap().to_string() }
                    else { n.as_f64().unwrap().to_string() }
                }
                serde_json::Value::Bool(b) => b.to_string(),
                _ => value.to_string(),
            };
            lines.push(format!("{} = {}", key, value_str));
        }
    }

    lines.join("\n")
}

/// Builds KPOINTS content string.
fn build_kpoints_content(scheme: &str, grid: &[u32; 3], shift: &[f64; 3]) -> String {
    let mut lines = vec![
        "K-points generated by DFT Module V1.7".to_string(),
        "0".to_string(),
    ];

    lines.push(match scheme {
        "Gamma" | "Gamma-centered" => "Gamma".to_string(),
        "Monkhorst-Pack" | "Monkhorst" => "Monkhorst-Pack".to_string(),
        _ => scheme.to_string(),
    });

    lines.push(format!("{} {} {}", grid[0], grid[1], grid[2]));
    lines.push(format!("{:.6} {:.6} {:.6}", shift[0], shift[1], shift[2]));

    lines.join("\n")
}

/// Builds POSCAR content string.
fn build_poscar_content(system_name: &str, structure: &CrystalStructure) -> (String, Vec<String>) {
    let elements = collect_elements(&structure.basis);
    let counts = count_elements(&structure.basis, &elements);

    let mut lines = vec![
        format!("{} - Generated by DFT Module V1.7", system_name),
        "1.0".to_string(),
    ];

    for vec in &structure.lattice_vectors {
        lines.push(format_lattice_line(vec));
    }

    lines.push(elements.join("  "));
    lines.push(counts.iter().map(|c| c.to_string()).collect::<Vec<_>>().join("  "));
    lines.push(if structure.cartesian { "Cartesian".to_string() } else { "Direct".to_string() });

    for atom in &structure.basis {
        lines.push(format!("{:20.16} {:20.16} {:20.16}",
            atom.position[0], atom.position[1], atom.position[2]));
    }

    (lines.join("\n"), elements)
}

/// Builds Quantum ESPRESSO input content.
fn build_qe_content(config: &QeInputConfig) -> (String, String, String) {
    let elements = collect_elements(&config.structure.basis);
    let _counts = count_elements(&config.structure.basis, &elements);
    let nat = config.structure.basis.len();
    let ibrav = 0; // General lattice vectors

    let mut lines = vec![
        format!("&CONTROL"),
        format!("    calculation = '{}{}'", config.calculation_type, if config.task.is_empty() { "" } else { "" }),
        format!("    title = 'QE calculation - DFT Module V1.7'"),
    ];

    // Control parameters
    let mut control_keys: Vec<_> = config.control_params.keys().cloned().collect();
    control_keys.sort();
    for key in &control_keys {
        if let Some(value) = config.control_params.get(key) {
            let value_str = format_qe_param(value);
            lines.push(format!("    {} = {}", key, value_str));
        }
    }
    lines.push("/".to_string());
    lines.push("".to_string());

    // System section
    lines.push("&SYSTEM".to_string());
    lines.push(format!("    ibrav = {}", ibrav));
    lines.push(format!("    nat = {}", nat));
    lines.push(format!("    ntyp = {}", elements.len()));

    let mut system_keys: Vec<_> = config.system_params.keys().cloned().collect();
    system_keys.sort();
    for key in &system_keys {
        if let Some(value) = config.system_params.get(key) {
            let value_str = format_qe_param(value);
            lines.push(format!("    {} = {}", key, value_str));
        }
    }
    lines.push("/".to_string());
    lines.push("".to_string());

    // Electrons section
    lines.push("&ELECTRONS".to_string());
    let mut electrons_keys: Vec<_> = config.electrons_params.keys().cloned().collect();
    electrons_keys.sort();
    for key in &electrons_keys {
        if let Some(value) = config.electrons_params.get(key) {
            let value_str = format_qe_param(value);
            lines.push(format!("    {} = {}", key, value_str));
        }
    }
    lines.push("/".to_string());
    lines.push("".to_string());

    // Atomic species
    lines.push("ATOMIC_SPECIES".to_string());
    for (_i, el) in elements.iter().enumerate() {
        let mass = get_element_mass(el);
        let pseudo = format!("{}.upf", el.to_lowercase());
        lines.push(format!("  {:6}  {:.3}  {}", el, mass, pseudo));
    }
    lines.push("".to_string());

    // Cell parameters
    lines.push("CELL_PARAMETERS { angstrom }".to_string());
    for vec in &config.structure.lattice_vectors {
        lines.push(format!("{:20.16} {:20.16} {:20.16}", vec[0], vec[1], vec[2]));
    }
    lines.push("".to_string());

    // Atomic positions
    lines.push(if config.structure.cartesian {
        "ATOMIC_POSITIONS { angstrom }".to_string()
    } else {
        "ATOMIC_POSITIONS { crystal }".to_string()
    });
    for atom in &config.structure.basis {
        lines.push(format!("{:<4} {:20.16} {:20.16} {:20.16}",
            atom.element,
            atom.position[0],
            atom.position[1],
            atom.position[2],
        ));
    }
    lines.push("".to_string());

    // K-points
    let kpoints_content = build_qe_kpoints(&config.control_params);

    (lines.join("\n"), kpoints_content, elements.join(" "))
}

fn format_qe_param(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => format!("'{}'", s),
        serde_json::Value::Number(n) => {
            if n.is_i64() { n.as_i64().unwrap().to_string() }
            else { format!("{:.6}", n.as_f64().unwrap()) }
        }
        serde_json::Value::Bool(b) => {
            if *b { ".true.".to_string() } else { ".false.".to_string() }
        }
        _ => value.to_string(),
    }
}

fn build_qe_kpoints(control_params: &HashMap<String, serde_json::Value>) -> String {
    let mut lines = vec![
        "K_POINTS { automatic }".to_string(),
    ];
    let nk1 = control_params.get("nk1").and_then(|v| v.as_i64()).unwrap_or(6) as u32;
    let nk2 = control_params.get("nk2").and_then(|v| v.as_i64()).unwrap_or(6) as u32;
    let nk3 = control_params.get("nk3").and_then(|v| v.as_i64()).unwrap_or(6) as u32;
    let k1 = control_params.get("k1").and_then(|v| v.as_i64()).unwrap_or(0);
    let k2 = control_params.get("k2").and_then(|v| v.as_i64()).unwrap_or(0);
    let k3 = control_params.get("k3").and_then(|v| v.as_i64()).unwrap_or(0);
    lines.push(format!("{} {} {} {} {} {}", nk1, nk2, nk3, k1, k2, k3));
    lines.join("\n")
}

fn get_element_mass(element: &str) -> f64 {
    match element {
        "H" => 1.008, "He" => 4.003, "Li" => 6.941, "Be" => 9.012, "B" => 10.811,
        "C" => 12.011, "N" => 14.007, "O" => 15.999, "F" => 18.998, "Ne" => 20.180,
        "Na" => 22.990, "Mg" => 24.305, "Al" => 26.982, "Si" => 28.086, "P" => 30.974,
        "S" => 32.065, "Cl" => 35.453, "Ar" => 39.948, "K" => 39.098, "Ca" => 40.078,
        "Sc" => 44.956, "Ti" => 47.867, "V" => 50.942, "Cr" => 51.996, "Mn" => 54.938,
        "Fe" => 55.845, "Co" => 58.933, "Ni" => 58.693, "Cu" => 63.546, "Zn" => 65.380,
        "Ga" => 69.723, "Ge" => 72.630, "As" => 74.922, "Se" => 78.971, "Br" => 79.904,
        "Kr" => 83.798, "Rb" => 85.468, "Sr" => 87.620, "Y" => 88.906, "Zr" => 91.224,
        "Nb" => 92.906, "Mo" => 95.950, "Tc" => 98.0, "Ru" => 101.07, "Rh" => 102.91,
        "Pd" => 106.42, "Ag" => 107.87, "Cd" => 112.41, "In" => 114.82, "Sn" => 118.71,
        "Sb" => 121.76, "Te" => 127.60, "I" => 126.90, "Xe" => 131.29, "Cs" => 132.91,
        "Ba" => 137.33, "La" => 138.91, "Ce" => 140.12, "Pr" => 140.91, "Nd" => 144.24,
        "Pm" => 145.0, "Sm" => 150.36, "Eu" => 151.96, "Gd" => 157.25, "Tb" => 158.93,
        "Dy" => 162.50, "Ho" => 164.93, "Er" => 167.26, "Tm" => 168.93, "Yb" => 173.05,
        "Lu" => 174.97, "Hf" => 178.49, "Ta" => 180.95, "W" => 183.84, "Re" => 186.21,
        "Os" => 190.23, "Ir" => 192.22, "Pt" => 195.08, "Au" => 196.97, "Hg" => 200.59,
        "Tl" => 204.38, "Pb" => 207.2, "Bi" => 208.98, "Th" => 232.04, "Pa" => 231.04,
        "U" => 238.03, "Np" => 237.0, "Pu" => 244.0,
        _ => 12.011, // Default to Carbon
    }
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// Generates VASP input files (POSCAR, INCAR, KPOINTS) from configuration.
#[command]
pub fn generate_vasp_input(config: VaspInputConfig) -> Result<DftInputResult, String> {
    info!(system_name = %config.system_name, calc_type = %config.calculation_type,
          "Generating VASP input files");

    // Validate calculation type
    let valid_calc_types = ["scf", "relax", "md", "bands", "dos"];
    if !valid_calc_types.contains(&config.calculation_type.as_str()) {
        return Err(format!(
            "Invalid calculation type '{}'. Must be one of: {}",
            config.calculation_type,
            valid_calc_types.join(", ")
        ));
    }

    // Validate structure
    if config.structure.basis.is_empty() {
        return Err("Crystal structure has no atoms".to_string());
    }

    // Check lattice vectors are non-degenerate
    let det = lattice_determinant(&config.structure.lattice_vectors);
    if det.abs() < 1e-10 {
        return Err("Lattice vectors are degenerate (zero volume cell)".to_string());
    }

    // Build POSCAR
    let (poscar_content, elements) = build_poscar_content(&config.system_name, &config.structure);

    // Build INCAR
    let mut incar_params = config.incar_params.clone();
    // Set calculation type if not already present
    if !incar_params.contains_key("SYSTEM") {
        incar_params.insert("SYSTEM".to_string(), serde_json::Value::String(config.system_name.clone()));
    }
    let incar_content = build_incar_content(&config.system_name, &incar_params);

    // Build KPOINTS
    let kpoints_content = build_kpoints_content(
        &config.kpoints_scheme,
        &config.kpoints_grid,
        &config.kpoints_shift,
    );

    // Validate and collect warnings
    let mut warnings = validate_incar_params(&incar_params);

    if config.kpoints_grid[0] == 0 || config.kpoints_grid[1] == 0 || config.kpoints_grid[2] == 0 {
        warnings.push("One or more KPOINTS grid dimensions are zero. Gamma-point only calculation.".to_string());
    }

    if let Some(ref potcar_path) = config.potcar_path {
        if !potcar_path.is_empty() {
            info!(potcar_path = %potcar_path, "Using custom POTCAR path");
        }
    }

    info!("VASP input generation complete with {} warnings", warnings.len());

    Ok(DftInputResult {
        poscar_content,
        incar_content,
        kpoints_content,
        potcar_elements: elements,
        warnings,
    })
}

/// Generates Quantum ESPRESSO input files from configuration.
#[command]
pub fn generate_qe_input(config: QeInputConfig) -> Result<DftInputResult, String> {
    info!(calc_type = %config.calculation_type, task = %config.task,
          "Generating QE input files");

    // Validate calculation type
    let valid_calc_types = ["pw", "cp", "pp"];
    if !valid_calc_types.contains(&config.calculation_type.as_str()) {
        return Err(format!(
            "Invalid calculation type '{}'. Must be one of: {}",
            config.calculation_type,
            valid_calc_types.join(", ")
        ));
    }

    let valid_tasks = ["scf", "relax", "md", "bands", "dos"];
    if !valid_tasks.contains(&config.task.as_str()) {
        return Err(format!(
            "Invalid task '{}'. Must be one of: {}",
            config.task,
            valid_tasks.join(", ")
        ));
    }

    if config.structure.basis.is_empty() {
        return Err("Crystal structure has no atoms".to_string());
    }

    let (input_content, kpoints_content, elements_str) = build_qe_content(&config);

    // For QE, we store the input in poscar_content field and kpoints separately
    let mut warnings = Vec::new();

    if config.system_params.get("ecutwfc").is_none() {
        warnings.push("ecutwfc not specified in system parameters. QE requires this for plane-wave cutoff.".to_string());
    }
    if config.system_params.get("ecutrho").is_none() {
        warnings.push("ecutrho not specified. Default charge density cutoff may be insufficient.".to_string());
    }

    let elements: Vec<String> = elements_str.split_whitespace().map(String::from).collect();

    info!("QE input generation complete with {} warnings", warnings.len());

    Ok(DftInputResult {
        poscar_content: input_content,
        incar_content: String::new(), // QE uses a single input file
        kpoints_content,
        potcar_elements: elements,
        warnings,
    })
}

/// Parses a VASP POSCAR file content.
#[command]
pub fn parse_vasp_poscar(content: String) -> Result<VaspPoscar, String> {
    info!("Parsing VASP POSCAR content");

    let lines: Vec<&str> = content.lines().collect();
    if lines.len() < 8 {
        return Err("POSCAR file too short. Expected at least 8 lines.".to_string());
    }

    // Line 1: Comment
    let comment = lines[0].trim().to_string();

    // Line 2: Scaling factor
    let scaling_factor: f64 = lines[1]
        .trim()
        .parse()
        .map_err(|_| format!("Failed to parse scaling factor: '{}'", lines[1].trim()))?;

    // Lines 3-5: Lattice vectors
    let mut lattice_vectors = [[0.0f64; 3]; 3];
    for i in 0..3 {
        let parts: Vec<f64> = lines[2 + i]
            .split_whitespace()
            .map(|s| s.parse().map_err(|_| format!("Failed to parse lattice vector {}: '{}'", i + 1, lines[2 + i])))
            .collect::<Result<Vec<_>, _>>()?;
        if parts.len() < 3 {
            return Err(format!("Lattice vector {} has fewer than 3 components", i + 1));
        }
        lattice_vectors[i] = [parts[0], parts[1], parts[2]];
    }

    // Line 6: Element names (optional)
    let mut element_names = Vec::new();
    let element_counts: Vec<u32>;
    let basis_start: usize;

    let line6_parts: Vec<&str> = lines[5].split_whitespace().collect();
    if line6_parts.iter().all(|s| s.parse::<u32>().is_err()) {
        // Line 6 contains element names
        element_names = line6_parts.iter().map(|s| s.to_string()).collect();

        // Line 7: Element counts
        let line7_parts: Vec<u32> = lines[6]
            .split_whitespace()
            .map(|s| s.parse().map_err(|_| format!("Failed to parse element counts: '{}'", lines[6])))
            .collect::<Result<Vec<_>, _>>()?;
        element_counts = line7_parts;

        // Line 8: Coordinate type
        basis_start = 8;
    } else {
        // Line 6 contains element counts directly (no element names)
        element_counts = line6_parts.iter().map(|s| s.parse().unwrap()).collect();
        basis_start = 7;
    }

    if basis_start >= lines.len() {
        return Err("POSCAR file ends before coordinate type line".to_string());
    }

    // Coordinate type
    let coordinate_type = lines[basis_start].trim().to_string();
    let _cartesian = coordinate_type.starts_with('C') || coordinate_type.starts_with('c');

    // Parse basis atoms
    let total_atoms: u32 = element_counts.iter().sum();
    let mut basis = Vec::new();
    let mut current_element_idx = 0;
    let mut count_in_element = 0;

    for i in (basis_start + 1)..lines.len() {
        let line = lines[i].trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<f64> = line
            .split_whitespace()
            .map(|s| s.parse().map_err(|_| format!("Failed to parse atom position at line {}: '{}'", i + 1, line)))
            .collect::<Result<Vec<_>, _>>()?;

        if parts.len() < 3 {
            continue;
        }

        let element = if !element_names.is_empty() && current_element_idx < element_names.len() {
            element_names[current_element_idx].clone()
        } else {
            "X".to_string()
        };

        basis.push(BasisAtom {
            element,
            position: [parts[0], parts[1], parts[2]],
        });

        count_in_element += 1;
        if current_element_idx < element_counts.len() && count_in_element >= element_counts[current_element_idx] {
            current_element_idx += 1;
            count_in_element = 0;
        }

        if basis.len() as u32 >= total_atoms {
            break;
        }
    }

    if element_names.is_empty() {
        element_names = (0..element_counts.len()).map(|i| format!("Element{}", i + 1)).collect();
    }

    info!(atoms = basis.len(), "POSCAR parsed successfully");

    Ok(VaspPoscar {
        comment,
        scaling_factor,
        lattice_vectors,
        element_names,
        element_counts,
        coordinate_type,
        basis,
    })
}

/// Parses a VASP INCAR file content.
#[command]
pub fn parse_vasp_incar(content: String) -> Result<VaspIncar, String> {
    info!("Parsing VASP INCAR content");

    let mut params = HashMap::new();

    for line in content.lines() {
        let line = line.trim();

        // Skip comments and empty lines
        if line.starts_with('!') || line.starts_with('#') || line.is_empty() {
            continue;
        }

        // Parse parameter = value
        if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim().to_uppercase();
            let value_str = line[eq_pos + 1..].trim();

            // Try to parse as different types
            let value = if let Ok(b) = value_str.parse::<bool>() {
                serde_json::Value::Bool(b)
            } else if let Ok(i) = value_str.parse::<i64>() {
                serde_json::Value::Number(i.into())
            } else if let Ok(f) = value_str.parse::<f64>() {
                serde_json::json!(f)
            } else {
                serde_json::Value::String(value_str.to_string())
            };

            params.insert(key, value);
        }
    }

    info!(num_params = params.len(), "INCAR parsed successfully");

    Ok(VaspIncar { params })
}

/// Returns available VASP input templates.
#[command]
pub fn get_vasp_templates() -> Result<Vec<VaspTemplate>, String> {
    info!("Fetching VASP templates");

    let mut scf_metal = HashMap::new();
    scf_metal.insert("PREC".to_string(), serde_json::Value::String("Accurate".to_string()));
    scf_metal.insert("ENCUT".to_string(), serde_json::json!(520));
    scf_metal.insert("ISMEAR".to_string(), serde_json::json!(1));
    scf_metal.insert("SIGMA".to_string(), serde_json::json!(0.2));
    scf_metal.insert("EDIFF".to_string(), serde_json::json!(1e-6));
    scf_metal.insert("NELM".to_string(), serde_json::json!(100));
    scf_metal.insert("LCHARG".to_string(), serde_json::Value::Bool(false));
    scf_metal.insert("LWAVE".to_string(), serde_json::Value::Bool(false));
    scf_metal.insert("ALGO".to_string(), serde_json::Value::String("Normal".to_string()));

    let mut relax_bulk = HashMap::new();
    relax_bulk.insert("PREC".to_string(), serde_json::Value::String("Accurate".to_string()));
    relax_bulk.insert("ENCUT".to_string(), serde_json::json!(520));
    relax_bulk.insert("ISMEAR".to_string(), serde_json::json!(1));
    relax_bulk.insert("SIGMA".to_string(), serde_json::json!(0.2));
    relax_bulk.insert("EDIFF".to_string(), serde_json::json!(1e-6));
    relax_bulk.insert("EDIFFG".to_string(), serde_json::json!(-0.01));
    relax_bulk.insert("NSW".to_string(), serde_json::json!(100));
    relax_bulk.insert("IBRION".to_string(), serde_json::json!(2));
    relax_bulk.insert("ISIF".to_string(), serde_json::json!(3));
    relax_bulk.insert("PSTRESS".to_string(), serde_json::json!(0));
    relax_bulk.insert("NELM".to_string(), serde_json::json!(100));
    relax_bulk.insert("ALGO".to_string(), serde_json::Value::String("Normal".to_string()));

    let mut bands_insulator = HashMap::new();
    bands_insulator.insert("PREC".to_string(), serde_json::Value::String("Accurate".to_string()));
    bands_insulator.insert("ENCUT".to_string(), serde_json::json!(600));
    bands_insulator.insert("ISMEAR".to_string(), serde_json::json!(-5));
    bands_insulator.insert("SIGMA".to_string(), serde_json::json!(0.05));
    bands_insulator.insert("EDIFF".to_string(), serde_json::json!(1e-8));
    bands_insulator.insert("ICHARG".to_string(), serde_json::json!(11));
    bands_insulator.insert("LORBIT".to_string(), serde_json::json!(11));
    bands_insulator.insert("NBANDS".to_string(), serde_json::json!(24));
    bands_insulator.insert("LWAVE".to_string(), serde_json::Value::Bool(true));

    let mut dos_metal = HashMap::new();
    dos_metal.insert("PREC".to_string(), serde_json::Value::String("Accurate".to_string()));
    dos_metal.insert("ENCUT".to_string(), serde_json::json!(520));
    dos_metal.insert("ISMEAR".to_string(), serde_json::json!(-5));
    dos_metal.insert("SIGMA".to_string(), serde_json::json!(0.05));
    dos_metal.insert("EDIFF".to_string(), serde_json::json!(1e-8));
    dos_metal.insert("ICHARG".to_string(), serde_json::json!(11));
    dos_metal.insert("LORBIT".to_string(), serde_json::json!(11));
    dos_metal.insert("NEDOS".to_string(), serde_json::json!(3001));
    dos_metal.insert("LWAVE".to_string(), serde_json::Value::Bool(false));

    let templates = vec![
        VaspTemplate {
            id: "scf_metal".to_string(),
            name: "SCF - Metal".to_string(),
            description: "Self-consistent field calculation for metallic systems with Methfessel-Paxton smearing.".to_string(),
            calculation_type: "scf".to_string(),
            incar_params: scf_metal,
            kpoints_scheme: "Monkhorst-Pack".to_string(),
            default_kpoints_grid: [12, 12, 12],
        },
        VaspTemplate {
            id: "relax_bulk".to_string(),
            name: "Relax - Bulk".to_string(),
            description: "Full geometry optimization of bulk materials with cell shape and volume relaxation.".to_string(),
            calculation_type: "relax".to_string(),
            incar_params: relax_bulk,
            kpoints_scheme: "Monkhorst-Pack".to_string(),
            default_kpoints_grid: [8, 8, 8],
        },
        VaspTemplate {
            id: "bands_insulator".to_string(),
            description: "Non-self-consistent band structure calculation for insulators using tetrahedron method.".to_string(),
            name: "Bands - Insulator".to_string(),
            calculation_type: "bands".to_string(),
            incar_params: bands_insulator,
            kpoints_scheme: "Gamma".to_string(),
            default_kpoints_grid: [1, 1, 1],
        },
        VaspTemplate {
            id: "dos_metal".to_string(),
            name: "DOS - Metal".to_string(),
            description: "Density of states calculation for metallic systems with fine energy grid.".to_string(),
            calculation_type: "dos".to_string(),
            incar_params: dos_metal,
            kpoints_scheme: "Monkhorst-Pack".to_string(),
            default_kpoints_grid: [16, 16, 16],
        },
    ];

    info!(count = templates.len(), "VASP templates returned");
    Ok(templates)
}

/// Returns available Quantum ESPRESSO input templates.
#[command]
pub fn get_qe_templates() -> Result<Vec<QeTemplate>, String> {
    info!("Fetching QE templates");

    let mut scf_control = HashMap::new();
    scf_control.insert("calculation".to_string(), serde_json::Value::String("scf".to_string()));
    scf_control.insert("restart_mode".to_string(), serde_json::Value::String("from_scratch".to_string()));
    scf_control.insert("pseudo_dir".to_string(), serde_json::Value::String("./pseudo".to_string()));
    scf_control.insert("outdir".to_string(), serde_json::Value::String("./out".to_string()));
    scf_control.insert("prefix".to_string(), serde_json::Value::String("dft_calc".to_string()));
    scf_control.insert("tprnfor".to_string(), serde_json::Value::Bool(true));
    scf_control.insert("tstress".to_string(), serde_json::Value::Bool(true));

    let mut scf_system = HashMap::new();
    scf_system.insert("ecutwfc".to_string(), serde_json::json!(60.0));
    scf_system.insert("ecutrho".to_string(), serde_json::json!(480.0));
    scf_system.insert("occupations".to_string(), serde_json::Value::String("smearing".to_string()));
    scf_system.insert("smearing".to_string(), serde_json::Value::String("mp".to_string()));
    scf_system.insert("degauss".to_string(), serde_json::json!(0.02));

    let mut scf_electrons = HashMap::new();
    scf_electrons.insert("conv_thr".to_string(), serde_json::json!(1e-8));
    scf_electrons.insert("mixing_beta".to_string(), serde_json::json!(0.3));
    scf_electrons.insert("electron_maxstep".to_string(), serde_json::json!(100));

    let mut relax_control = scf_control.clone();
    relax_control.insert("calculation".to_string(), serde_json::Value::String("vc-relax".to_string()));

    let mut relax_system = scf_system.clone();
    relax_system.insert("ecutwfc".to_string(), serde_json::json!(50.0));
    relax_system.insert("ecutrho".to_string(), serde_json::json!(400.0));

    let relax_electrons = scf_electrons.clone();

    let mut md_control = HashMap::new();
    md_control.insert("calculation".to_string(), serde_json::Value::String("cp".to_string()));
    md_control.insert("restart_mode".to_string(), serde_json::Value::String("from_scratch".to_string()));
    md_control.insert("pseudo_dir".to_string(), serde_json::Value::String("./pseudo".to_string()));
    md_control.insert("outdir".to_string(), serde_json::Value::String("./out".to_string()));
    md_control.insert("prefix".to_string(), serde_json::Value::String("dft_md".to_string()));
    md_control.insert("dt".to_string(), serde_json::json!(1.0));
    md_control.insert("nstep".to_string(), serde_json::json!(1000));

    let mut md_system = scf_system.clone();
    md_system.insert("ecutwfc".to_string(), serde_json::json!(40.0));
    md_system.insert("ecutrho".to_string(), serde_json::json!(320.0));

    let mut md_electrons = scf_electrons.clone();
    md_electrons.insert("electron_maxstep".to_string(), serde_json::json!(50));

    let templates = vec![
        QeTemplate {
            id: "scf_pw".to_string(),
            name: "SCF - PW".to_string(),
            description: "Self-consistent field calculation using plane-wave pseudopotential method.".to_string(),
            calculation_type: "pw".to_string(),
            task: "scf".to_string(),
            control_params: scf_control,
            system_params: scf_system,
            electrons_params: scf_electrons,
        },
        QeTemplate {
            id: "relax_pw".to_string(),
            name: "Relax - PW".to_string(),
            description: "Variable-cell relaxation using plane-wave pseudopotential method.".to_string(),
            calculation_type: "pw".to_string(),
            task: "relax".to_string(),
            control_params: relax_control,
            system_params: relax_system,
            electrons_params: relax_electrons,
        },
        QeTemplate {
            id: "md_cp".to_string(),
            name: "MD - CP".to_string(),
            description: "Car-Parrinello molecular dynamics simulation.".to_string(),
            calculation_type: "cp".to_string(),
            task: "md".to_string(),
            control_params: md_control,
            system_params: md_system,
            electrons_params: md_electrons,
        },
    ];

    info!(count = templates.len(), "QE templates returned");
    Ok(templates)
}

/// Exports generated input files to a specified directory.
#[command]
pub fn export_input_files(
    poscar: String,
    incar: String,
    kpoints: String,
    output_dir: String,
) -> Result<(), String> {
    info!(output_dir = %output_dir, "Exporting DFT input files");

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&output_dir)
        .map_err(|e| format!("Failed to create output directory '{}': {}", output_dir, e))?;

    // Write POSCAR
    let poscar_path = std::path::Path::new(&output_dir).join("POSCAR");
    std::fs::write(&poscar_path, &poscar)
        .map_err(|e| format!("Failed to write POSCAR: {}", e))?;

    // Write INCAR
    if !incar.is_empty() {
        let incar_path = std::path::Path::new(&output_dir).join("INCAR");
        std::fs::write(&incar_path, &incar)
            .map_err(|e| format!("Failed to write INCAR: {}", e))?;
    }

    // Write KPOINTS
    let kpoints_path = std::path::Path::new(&output_dir).join("KPOINTS");
    std::fs::write(&kpoints_path, &kpoints)
        .map_err(|e| format!("Failed to write KPOINTS: {}", e))?;

    info!("Input files exported successfully to {}", output_dir);
    Ok(())
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Computes the determinant of a 3x3 matrix (lattice vectors).
fn lattice_determinant(vectors: &[[f64; 3]; 3]) -> f64 {
    let a = &vectors[0];
    let b = &vectors[1];
    let c = &vectors[2];

    a[0] * (b[1] * c[2] - b[2] * c[1])
        - a[1] * (b[0] * c[2] - b[2] * c[0])
        + a[2] * (b[0] * c[1] - b[1] * c[0])
}
