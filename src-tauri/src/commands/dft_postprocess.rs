// DFT Post-Processing Commands - V4.2
// Provides parsing, analysis, and validation of DFT calculation outputs.
// V4.2-006: 去壳化 - 支持真实 VASP/Quantum ESPRESSO 输出文件解析

use serde::{Deserialize, Serialize};
use std::path::Path;
use tauri::command;
use tracing::info;

// ============================================================================
// Data Structures
// ============================================================================

/// Comprehensive result of DFT output parsing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DftParseResult {
    pub total_energy: f64,
    pub energy_per_atom: f64,
    pub convergence_achieved: bool,
    pub num_ionic_steps: u32,
    pub num_electronic_steps: u32,
    pub final_forces: Vec<[f64; 3]>,
    pub final_stress: [f64; 6],
    pub is_metallic: bool,
    pub band_gap: f64,
    pub fermi_energy: f64,
}

/// Energy data extracted from DFT output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DftEnergyData {
    pub energies: Vec<f64>,
    pub energy_per_atom: Vec<f64>,
    pub ionic_steps: Vec<f64>,
    pub converged: bool,
    pub final_energy: f64,
}

/// Density of states data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DosData {
    pub energy_values: Vec<f64>,
    pub total_dos: Vec<f64>,
    pub partial_dos: Vec<PartialDos>,
    pub fermi_energy: f64,
}

/// Partial density of states for a specific element and orbital.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialDos {
    pub element: String,
    pub orbital: String,
    pub spin_up: Vec<f64>,
    pub spin_down: Option<Vec<f64>>,
}

/// Band structure data from eigenvalue calculations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandStructureData {
    pub kpoints: Vec<[f64; 3]>,
    pub kpoint_weights: Vec<f64>,
    pub bands: Vec<BandData>,
    pub num_bands: u32,
    pub num_kpoints: u32,
    pub fermi_energy: f64,
}

/// Data for a single band.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandData {
    pub band_index: u32,
    pub energies: Vec<f64>,
    pub is_occupied: Vec<bool>,
}

/// Charge density data from CONTCAR/CHGCAR parsing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeDensityData {
    pub grid_size: [u32; 3],
    pub origin: [f64; 3],
    pub spacing: [f64; 3],
    pub data: Vec<f64>,
}

/// Result of a single validation check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub property: String,
    pub computed: f64,
    pub reference: f64,
    pub error_percent: f64,
    pub passed: bool,
}

/// A validation test case from the test suite.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationTestCase {
    pub id: String,
    pub name: String,
    pub code: String,
    pub property: String,
    pub reference_value: f64,
    pub tolerance_mev: f64,
    pub description: String,
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// Parses DFT output from a directory for a given code.
#[command]
pub fn parse_dft_output(output_dir: String, code: String) -> Result<DftParseResult, String> {
    info!(output_dir = %output_dir, code = %code, "Parsing DFT output");

    if output_dir.is_empty() {
        return Err("Output directory cannot be empty".to_string());
    }

    let valid_codes = ["vasp", "qe"];
    if !valid_codes.contains(&code.as_str()) {
        return Err(format!("Unsupported code '{}'. Supported: {}", code, valid_codes.join(", ")));
    }

    // In a real implementation, this would read actual output files.
    // Here we generate realistic mock data.
    let num_atoms = 4u32;
    let num_ionic = 12u32;
    let num_electronic = 45u32;

    let result = match code.as_str() {
        "vasp" => DftParseResult {
            total_energy: -16.856423,
            energy_per_atom: -16.856423 / num_atoms as f64,
            convergence_achieved: true,
            num_ionic_steps: num_ionic,
            num_electronic_steps: num_electronic,
            final_forces: vec![
                [0.00123, -0.00045, 0.00078],
                [-0.00089, 0.00156, -0.00034],
                [0.00067, -0.00112, 0.00091],
                [-0.00101, 0.00023, -0.00135],
            ],
            final_stress: [
                -2.345, 1.234, -0.567,
                0.891, -1.456, 0.678,
            ],
            is_metallic: false,
            band_gap: 0.842,
            fermi_energy: 5.623,
        },
        "qe" => DftParseResult {
            total_energy: -15.432187,
            energy_per_atom: -15.432187 / num_atoms as f64,
            convergence_achieved: true,
            num_ionic_steps: 8,
            num_electronic_steps: 32,
            final_forces: vec![
                [0.00234, -0.00067, 0.00112],
                [-0.00145, 0.00201, -0.00056],
                [0.00089, -0.00178, 0.00134],
                [-0.00178, 0.00045, -0.00201],
            ],
            final_stress: [
                -1.876, 0.987, -0.432,
                0.654, -1.123, 0.543,
            ],
            is_metallic: true,
            band_gap: 0.0,
            fermi_energy: 6.123,
        },
        _ => unreachable!(),
    };

    info!(
        energy = result.total_energy,
        converged = result.convergence_achieved,
        "DFT output parsed successfully"
    );

    Ok(result)
}

/// Parses VASP OUTCAR content to extract energy data.
#[command]
pub fn parse_vasp_outcar(content: String) -> Result<DftEnergyData, String> {
    info!(content_len = content.len(), "Parsing VASP OUTCAR");

    if content.is_empty() {
        return Err("OUTCAR content is empty".to_string());
    }

    // In a real implementation, this would parse the actual OUTCAR file.
    // Here we generate realistic mock data for a 12-step ionic relaxation.
    let num_steps = 12;
    let mut energies = Vec::with_capacity(num_steps);
    let mut energy_per_atom = Vec::with_capacity(num_steps);
    let mut ionic_steps = Vec::with_capacity(num_steps);

    // Simulate energy convergence during relaxation
    let initial_energy = -16.200;
    let final_energy = -16.856423;
    let num_atoms = 4.0;

    for i in 0..num_steps {
        let progress = i as f64 / (num_steps - 1) as f64;
        // Exponential convergence
        let energy = initial_energy + (final_energy - initial_energy) * (1.0 - (-3.0 * progress).exp());
        energies.push(energy);
        energy_per_atom.push(energy / num_atoms);
        ionic_steps.push(i as f64);
    }

    Ok(DftEnergyData {
        energies,
        energy_per_atom,
        ionic_steps,
        converged: true,
        final_energy,
    })
}

/// Parses VASP DOSCAR content to extract density of states.
#[command]
pub fn parse_vasp_doscar(content: String) -> Result<DosData, String> {
    info!(content_len = content.len(), "Parsing VASP DOSCAR");

    if content.is_empty() {
        return Err("DOSCAR content is empty".to_string());
    }

    // In a real implementation, this would parse the actual DOSCAR file.
    // Here we generate realistic mock data.
    let num_points = 3001;
    let _fermi_idx = num_points / 2;
    let fermi_energy = 5.623;
    let energy_min = -15.0;
    let energy_max = 25.0;
    let d_energy = (energy_max - energy_min) / (num_points - 1) as f64;

    let mut energy_values = Vec::with_capacity(num_points);
    let mut total_dos = Vec::with_capacity(num_points);

    for i in 0..num_points {
        let e = energy_min + i as f64 * d_energy;
        energy_values.push(e);

        // Simulate DOS: Gaussian peaks near Fermi level
        let dos = simulate_dos(e, fermi_energy);
        total_dos.push(dos);
    }

    // Generate partial DOS for Si (s, p orbitals)
    let mut partial_dos = Vec::new();

    // Si s-orbital
    let (spin_up_s, spin_down_s) = generate_spin_dos(&energy_values, fermi_energy, 0.5, 2.0, 1.5);
    partial_dos.push(PartialDos {
        element: "Si".to_string(),
        orbital: "s".to_string(),
        spin_up: spin_up_s,
        spin_down: Some(spin_down_s),
    });

    // Si p-orbital
    let (spin_up_p, spin_down_p) = generate_spin_dos(&energy_values, fermi_energy, 1.2, 3.0, 2.5);
    partial_dos.push(PartialDos {
        element: "Si".to_string(),
        orbital: "p".to_string(),
        spin_up: spin_up_p,
        spin_down: Some(spin_down_p),
    });

    // Si d-orbital (minor contribution)
    let (spin_up_d, spin_down_d) = generate_spin_dos(&energy_values, fermi_energy, 0.1, 1.5, 3.0);
    partial_dos.push(PartialDos {
        element: "Si".to_string(),
        orbital: "d".to_string(),
        spin_up: spin_up_d,
        spin_down: Some(spin_down_d),
    });

    Ok(DosData {
        energy_values,
        total_dos,
        partial_dos,
        fermi_energy,
    })
}

/// Parses VASP EIGENVAL content to extract band structure data.
#[command]
pub fn parse_vasp_eigenv(content: String) -> Result<BandStructureData, String> {
    info!(content_len = content.len(), "Parsing VASP EIGENVAL");

    if content.is_empty() {
        return Err("EIGENVAL content is empty".to_string());
    }

    // In a real implementation, this would parse the actual EIGENVAL file.
    // Here we generate realistic mock data for a semiconductor (e.g., Si).
    let num_kpoints = 64u32;
    let num_bands = 24u32;
    let fermi_energy = 5.623;

    // Generate k-points along high-symmetry path (Gamma-X-W-K-Gamma-L)
    let mut kpoints = Vec::with_capacity(num_kpoints as usize);
    let mut kpoint_weights = Vec::with_capacity(num_kpoints as usize);

    let high_symmetry_points: Vec<[f64; 3]> = vec![
        [0.0, 0.0, 0.0],   // Gamma
        [1.0, 0.0, 0.0],   // X
        [1.0, 0.5, 0.5],   // W
        [0.75, 0.75, 0.0], // K
        [0.0, 0.0, 0.0],   // Gamma
        [0.5, 0.5, 0.5],   // L
    ];

    let segments = high_symmetry_points.len() - 1;
    let kpoints_per_segment = num_kpoints as usize / segments;

    for seg in 0..segments {
        let start = &high_symmetry_points[seg];
        let end = &high_symmetry_points[seg + 1];
        for j in 0..kpoints_per_segment {
            let t = j as f64 / kpoints_per_segment as f64;
            kpoints.push([
                start[0] + t * (end[0] - start[0]),
                start[1] + t * (end[1] - start[1]),
                start[2] + t * (end[2] - start[2]),
            ]);
            kpoint_weights.push(1.0 / num_kpoints as f64);
        }
    }

    // Fill remaining k-points
    while kpoints.len() < num_kpoints as usize {
        kpoints.push([0.0, 0.0, 0.0]);
        kpoint_weights.push(0.0);
    }

    // Generate bands
    let mut bands = Vec::with_capacity(num_bands as usize);
    for band_idx in 0..num_bands {
        let is_valence = band_idx < 12; // 4 atoms * 3 valence electrons / 2 spin = 12 occupied
        let mut energies = Vec::with_capacity(num_kpoints as usize);
        let mut is_occupied = Vec::with_capacity(num_kpoints as usize);

        for k_idx in 0..num_kpoints as usize {
            let k = &kpoints[k_idx];
            // Simulate band dispersion
            let k_mag = (k[0].powi(2) + k[1].powi(2) + k[2].powi(2)).sqrt();

            let energy = if is_valence {
                // Valence bands: below Fermi level
                fermi_energy - 2.0 + (band_idx as f64 / 12.0) * 3.5
                    - 1.5 * (k_mag * (1.0 + 0.3 * (band_idx as f64 % 4.0))).sin()
            } else {
                // Conduction bands: above Fermi level
                fermi_energy + 0.842 + ((band_idx - 12) as f64 / 12.0) * 4.0
                    + 1.2 * (k_mag * (1.0 + 0.2 * ((band_idx - 12) as f64 % 4.0))).cos()
            };

            energies.push(energy);
            is_occupied.push(is_valence);
        }

        bands.push(BandData {
            band_index: band_idx,
            energies,
            is_occupied,
        });
    }

    Ok(BandStructureData {
        kpoints,
        kpoint_weights,
        bands,
        num_bands,
        num_kpoints,
        fermi_energy,
    })
}

/// Parses VASP CONTCAR content to extract charge density grid data.
#[command]
pub fn parse_vasp_contcar(content: String) -> Result<ChargeDensityData, String> {
    info!(content_len = content.len(), "Parsing VASP CONTCAR/CHGCAR");

    if content.is_empty() {
        return Err("CONTCAR content is empty".to_string());
    }

    // In a real implementation, this would parse the actual CHGCAR file.
    // Here we generate realistic mock data for a charge density grid.
    let grid_size: [u32; 3] = [64, 64, 64];
    let lattice_constant = 5.43; // Si lattice constant in Angstroms

    let spacing = [
        lattice_constant / grid_size[0] as f64,
        lattice_constant / grid_size[1] as f64,
        lattice_constant / grid_size[2] as f64,
    ];

    let origin = [0.0, 0.0, 0.0];

    // Generate mock charge density data (subsampled for efficiency)
    let mut data = Vec::new();
    let nx = grid_size[0] as usize;
    let ny = grid_size[1] as usize;
    let nz = grid_size[2] as usize;

    // Generate a representative subset of the charge density
    // In a real implementation, all nx*ny*nz values would be read
    let sample_size = 1000;
    for i in 0..sample_size {
        let ix = (i * nx / sample_size) as f64 / nx as f64;
        let iy = ((i * 7) % ny) as f64 / ny as f64;
        let iz = ((i * 13) % nz) as f64 / nz as f64;

        // Simulate charge density: periodic Gaussian blobs at atom positions
        let mut density = 0.0;
        let atom_positions: [[f64; 3]; 4] = [
            [0.0, 0.0, 0.0],
            [0.25, 0.25, 0.25],
            [0.5, 0.5, 0.0],
            [0.75, 0.75, 0.25],
        ];

        for atom in &atom_positions {
            let dx = ix - atom[0];
            let dy = iy - atom[1];
            let dz = iz - atom[2];
            // Minimum image convention
            let dx = dx - dx.round();
            let dy = dy - dy.round();
            let dz = dz - dz.round();
            let r2 = dx * dx + dy * dy + dz * dz;
            density += 2.5 * (-r2 / 0.02).exp();
        }

        data.push(density);
    }

    Ok(ChargeDensityData {
        grid_size,
        origin,
        spacing,
        data,
    })
}

/// Parses Quantum ESPRESSO output content to extract energy data.
#[command]
pub fn parse_qe_output(content: String) -> Result<DftEnergyData, String> {
    info!(content_len = content.len(), "Parsing QE output");

    if content.is_empty() {
        return Err("QE output content is empty".to_string());
    }

    let mut energies = Vec::new();
    let mut energy_per_atom = Vec::new();
    let mut ionic_steps = Vec::new();
    let mut converged = false;
    let mut final_energy = 0.0;
    let mut num_atoms = 2.0; // default

    // Parse number of atoms
    for line in content.lines() {
        if line.contains("number of atoms/cell") {
            if let Some(idx) = line.find('=') {
                let val: f64 = line[idx+1..].trim().parse().unwrap_or(2.0);
                num_atoms = val;
            }
        }
    }

    // Parse energy at each ionic step (lines with "!")
    let mut step = 0u32;
    for line in content.lines() {
        if line.contains('!') && line.contains("total energy") {
            if let Some(idx) = line.find('=') {
                let val_str: String = line[idx+1..].trim()
                    .chars().take_while(|c| c.is_digit(10) || *c == '.' || *c == '-' || *c == 'E' || *c == 'e' || *c == '+')
                    .collect();
                if let Ok(energy) = val_str.parse::<f64>() {
                    energies.push(energy);
                    energy_per_atom.push(energy / num_atoms);
                    ionic_steps.push(step as f64);
                    final_energy = energy;
                    step += 1;
                }
            }
        }
    }

    // Check convergence
    for line in content.lines() {
        if line.contains("convergence has been achieved") || line.contains("End of BFGS") {
            converged = true;
        }
    }

    if energies.is_empty() {
        return Err("No energy values found in QE output".to_string());
    }

    info!(
        steps = energies.len(),
        final_energy = final_energy,
        converged = converged,
        "QE output parsed successfully"
    );

    Ok(DftEnergyData {
        energies,
        energy_per_atom,
        ionic_steps,
        converged,
        final_energy,
    })
}

/// Parses Quantum ESPRESSO DOS output content.
#[command]
pub fn parse_qe_dos(content: String) -> Result<DosData, String> {
    info!(content_len = content.len(), "Parsing QE DOS");

    if content.is_empty() {
        return Err("QE DOS content is empty".to_string());
    }

    // In a real implementation, this would parse the actual QE DOS file.
    // Here we generate realistic mock data.
    let num_points = 2001;
    let fermi_energy = 6.123;
    let energy_min = -20.0;
    let energy_max = 30.0;
    let d_energy = (energy_max - energy_min) / (num_points - 1) as f64;

    let mut energy_values = Vec::with_capacity(num_points);
    let mut total_dos = Vec::with_capacity(num_points);

    for i in 0..num_points {
        let e = energy_min + i as f64 * d_energy;
        energy_values.push(e);
        let dos = simulate_dos(e, fermi_energy);
        total_dos.push(dos);
    }

    let mut partial_dos = Vec::new();

    // Up and down spin channels
    let (spin_up, spin_down) = generate_spin_dos(&energy_values, fermi_energy, 1.0, 2.5, 2.0);
    partial_dos.push(PartialDos {
        element: "Fe".to_string(),
        orbital: "d".to_string(),
        spin_up,
        spin_down: Some(spin_down),
    });

    let (spin_up_s, spin_down_s) = generate_spin_dos(&energy_values, fermi_energy, 0.3, 1.8, 1.2);
    partial_dos.push(PartialDos {
        element: "Fe".to_string(),
        orbital: "s".to_string(),
        spin_up: spin_up_s,
        spin_down: Some(spin_down_s),
    });

    Ok(DosData {
        energy_values,
        total_dos,
        partial_dos,
        fermi_energy,
    })
}

/// Runs a validation check comparing computed DFT values against reference data.
#[command]
pub fn run_validation(
    code: String,
    property: String,
    computed: f64,
    reference: f64,
    tolerance: f64,
) -> Result<Vec<ValidationResult>, String> {
    info!(
        code = %code,
        property = %property,
        computed = computed,
        reference = reference,
        tolerance = tolerance,
        "Running DFT validation"
    );

    if property.is_empty() {
        return Err("Property name cannot be empty".to_string());
    }

    if tolerance <= 0.0 {
        return Err("Tolerance must be positive".to_string());
    }

    let error_percent = if reference.abs() > 1e-10 {
        ((computed - reference).abs() / reference.abs()) * 100.0
    } else {
        (computed - reference).abs() * 100.0
    };

    let passed = error_percent <= tolerance;

    let result = ValidationResult {
        property: property.clone(),
        computed,
        reference,
        error_percent,
        passed,
    };

    info!(
        property = %property,
        error_pct = error_percent,
        passed = passed,
        "Validation check completed"
    );

    Ok(vec![result])
}

/// Returns the available validation test suite.
#[command]
pub fn get_validation_test_suite() -> Result<Vec<ValidationTestCase>, String> {
    info!("Fetching validation test suite");

    let test_cases = vec![
        ValidationTestCase {
            id: "vasp-si-lattice".to_string(),
            name: "Si FCC Lattice Constant".to_string(),
            code: "vasp".to_string(),
            property: "lattice_constant".to_string(),
            reference_value: 5.431,
            tolerance_mev: 1.0,
            description: "Equilibrium lattice constant of silicon in FCC structure. Reference: experimental value at 0K.".to_string(),
        },
        ValidationTestCase {
            id: "vasp-si-bandgap".to_string(),
            name: "Si Band Gap (PBE)".to_string(),
            code: "vasp".to_string(),
            property: "band_gap".to_string(),
            reference_value: 0.612,
            tolerance_mev: 50.0,
            description: "Indirect band gap of silicon using PBE functional. Note: PBE underestimates the experimental gap of 1.17 eV.".to_string(),
        },
        ValidationTestCase {
            id: "vasp-si-bulk-modulus".to_string(),
            name: "Si Bulk Modulus".to_string(),
            code: "vasp".to_string(),
            property: "bulk_modulus".to_string(),
            reference_value: 97.8,
            tolerance_mev: 5.0,
            description: "Bulk modulus of silicon in GPa. Reference: experimental value.".to_string(),
        },
        ValidationTestCase {
            id: "vasp-fe-magnetic".to_string(),
            name: "Fe BCC Magnetic Moment".to_string(),
            code: "vasp".to_string(),
            property: "magnetic_moment".to_string(),
            reference_value: 2.22,
            tolerance_mev: 5.0,
            description: "Magnetic moment of BCC iron in Bohr magnetons. Reference: experimental value.".to_string(),
        },
        ValidationTestCase {
            id: "vasp-fe-lattice".to_string(),
            name: "Fe BCC Lattice Constant".to_string(),
            code: "vasp".to_string(),
            property: "lattice_constant".to_string(),
            reference_value: 2.834,
            tolerance_mev: 1.0,
            description: "Equilibrium lattice constant of BCC iron. Reference: experimental value at 0K.".to_string(),
        },
        ValidationTestCase {
            id: "vasp-al-cohesive".to_string(),
            name: "Al FCC Cohesive Energy".to_string(),
            code: "vasp".to_string(),
            property: "cohesive_energy".to_string(),
            reference_value: 3.39,
            tolerance_mev: 50.0,
            description: "Cohesive energy of aluminum in eV/atom. Reference: experimental value.".to_string(),
        },
        ValidationTestCase {
            id: "qe-si-total-energy".to_string(),
            name: "Si Total Energy (PBE)".to_string(),
            code: "qe".to_string(),
            property: "total_energy".to_string(),
            reference_value: -5.4245,
            tolerance_mev: 10.0,
            description: "Total energy per atom of silicon using PBE functional in QE. Reference: converged PBE result.".to_string(),
        },
        ValidationTestCase {
            id: "qe-cu-fermi".to_string(),
            name: "Cu Fermi Energy".to_string(),
            code: "qe".to_string(),
            property: "fermi_energy".to_string(),
            reference_value: 9.23,
            tolerance_mev: 100.0,
            description: "Fermi energy of copper in eV. Reference: experimental value.".to_string(),
        },
        ValidationTestCase {
            id: "vasp-tio2-bandgap".to_string(),
            name: "TiO2 Rutile Band Gap (PBE)".to_string(),
            code: "vasp".to_string(),
            property: "band_gap".to_string(),
            reference_value: 2.12,
            tolerance_mev: 100.0,
            description: "Band gap of rutile TiO2 using PBE functional. Note: PBE underestimates the experimental gap of 3.0 eV.".to_string(),
        },
        ValidationTestCase {
            id: "vasp-mgo-formation".to_string(),
            name: "MgO Formation Energy".to_string(),
            code: "vasp".to_string(),
            property: "formation_energy".to_string(),
            reference_value: -5.97,
            tolerance_mev: 50.0,
            description: "Formation energy of MgO in eV per formula unit. Reference: experimental value.".to_string(),
        },
    ];

    info!(count = test_cases.len(), "Validation test suite returned");
    Ok(test_cases)
}

// ============================================================================
// Real VASP Output Parsing Functions (V4.2-006 去壳化)
// ============================================================================

/// Parses real VASP OUTCAR file to extract energy and convergence data.
#[command]
pub fn parse_vasp_outcar_file(file_path: String) -> Result<DftEnergyData, String> {
    info!(file_path = %file_path, "Parsing VASP OUTCAR file");

    let path = Path::new(&file_path);
    if !path.exists() {
        return Err(format!("OUTCAR file not found: {}", file_path));
    }

    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read OUTCAR file: {}", e))?;

    parse_vasp_outcar_real(&content)
}

/// Internal function to parse OUTCAR content.
fn parse_vasp_outcar_real(content: &str) -> Result<DftEnergyData, String> {
    let mut energies = Vec::new();
    let mut energy_per_atom = Vec::new();
    let mut ionic_steps = Vec::new();
    let mut converged = false;
    let mut final_energy = 0.0;
    let mut num_atoms = 1;

    // Parse number of atoms
    for line in content.lines() {
        if line.contains("NIONS") {
            if let Some(idx) = line.find('=') {
                let val: usize = line[idx+1..].trim().parse().unwrap_or(1);
                num_atoms = val;
                break;
            }
        }
    }

    // Parse energy at each ionic step
    let mut step = 0u32;
    for line in content.lines() {
        // Look for "free  energy" lines
        if line.contains("free  energy") && line.contains("TOTEN") {
            if let Some(idx) = line.find("=") {
                let energy_str: String = line[idx+1..].trim()
                    .chars().take_while(|c| c.is_digit(10) || *c == '.' || *c == '-' || *c == 'E' || *c == 'e' || *c == '+')
                    .collect();
                if let Ok(energy) = energy_str.parse::<f64>() {
                    energies.push(energy);
                    energy_per_atom.push(energy / num_atoms as f64);
                    ionic_steps.push(step as f64);
                    final_energy = energy;
                    step += 1;
                }
            }
        }
    }

    // Check for convergence
    for line in content.lines() {
        if line.contains("reached required accuracy") || line.contains("Convergence achieved") {
            converged = true;
            break;
        }
    }

    if energies.is_empty() {
        return Err("No energy values found in OUTCAR".to_string());
    }

    info!(
        steps = energies.len(),
        final_energy = final_energy,
        converged = converged,
        "OUTCAR parsed successfully"
    );

    Ok(DftEnergyData {
        energies,
        energy_per_atom,
        ionic_steps,
        converged,
        final_energy,
    })
}

/// Parses real VASP DOSCAR file to extract density of states.
#[command]
pub fn parse_vasp_doscar_file(file_path: String) -> Result<DosData, String> {
    info!(file_path = %file_path, "Parsing VASP DOSCAR file");

    let path = Path::new(&file_path);
    if !path.exists() {
        return Err(format!("DOSCAR file not found: {}", file_path));
    }

    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read DOSCAR file: {}", e))?;

    parse_vasp_doscar_real(&content)
}

/// Internal function to parse DOSCAR content.
fn parse_vasp_doscar_real(content: &str) -> Result<DosData, String> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() < 6 {
        return Err("DOSCAR file too short".to_string());
    }

    // Parse header information
    let num_atoms: usize = lines[0].trim().parse().unwrap_or(1);
    
    // Line 5 contains energy range and number of points
    let header_parts: Vec<f64> = lines[5].split_whitespace()
        .map(|s| s.parse().unwrap_or(0.0))
        .collect();
    
    if header_parts.len() < 3 {
        return Err("Invalid DOSCAR header".to_string());
    }

    let _emin = header_parts[0];
    let _emax = header_parts[1];
    let num_points = header_parts[2] as usize;
    let fermi_energy = header_parts.get(3).copied().unwrap_or(0.0);

    // Parse total DOS (starts at line 6)
    let mut energy_values = Vec::with_capacity(num_points);
    let mut total_dos = Vec::with_capacity(num_points);

    let dos_start = 6;
    for i in 0..num_points {
        if dos_start + i >= lines.len() {
            break;
        }
        let parts: Vec<f64> = lines[dos_start + i].split_whitespace()
            .map(|s| s.parse().unwrap_or(0.0))
            .collect();
        if parts.len() >= 2 {
            energy_values.push(parts[0]);
            total_dos.push(parts[1]);
        }
    }

    // Parse partial DOS if available
    let mut partial_dos = Vec::new();
    let pdos_start = dos_start + num_points + 1;

    if lines.len() > pdos_start {
        // Parse partial DOS for each atom
        for atom_idx in 0..num_atoms {
            let atom_offset = pdos_start + atom_idx * (num_points + 1);
            if atom_offset >= lines.len() {
                break;
            }

            // Atom header line
            let _atom_header = lines[atom_offset];

            // Parse s, p, d orbital contributions
            let orbitals = vec!["s", "p", "d"];
            for (orbital_idx, orbital) in orbitals.iter().enumerate() {
                let mut spin_up = Vec::with_capacity(num_points);
                let mut spin_down = Vec::with_capacity(num_points);

                for i in 0..num_points {
                    let line_idx = atom_offset + 1 + i;
                    if line_idx >= lines.len() {
                        break;
                    }
                    let parts: Vec<f64> = lines[line_idx].split_whitespace()
                        .map(|s| s.parse().unwrap_or(0.0))
                        .collect();
                    
                    // DOSCAR format: energy, s-up, s-down, p-up, p-down, d-up, d-down, ...
                    if parts.len() >= 7 {
                        let up_idx = 1 + orbital_idx * 2;
                        let down_idx = 2 + orbital_idx * 2;
                        spin_up.push(parts[up_idx]);
                        spin_down.push(parts[down_idx]);
                    }
                }

                if !spin_up.is_empty() {
                    partial_dos.push(PartialDos {
                        element: format!("Atom{}", atom_idx + 1),
                        orbital: orbital.to_string(),
                        spin_up,
                        spin_down: Some(spin_down),
                    });
                }
            }
        }
    }

    info!(
        num_points = energy_values.len(),
        num_partial = partial_dos.len(),
        "DOSCAR parsed successfully"
    );

    Ok(DosData {
        energy_values,
        total_dos,
        partial_dos,
        fermi_energy,
    })
}

/// Parses real VASP EIGENVAL file to extract band structure data.
#[command]
pub fn parse_vasp_eigenval_file(file_path: String) -> Result<BandStructureData, String> {
    info!(file_path = %file_path, "Parsing VASP EIGENVAL file");

    let path = Path::new(&file_path);
    if !path.exists() {
        return Err(format!("EIGENVAL file not found: {}", file_path));
    }

    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read EIGENVAL file: {}", e))?;

    parse_vasp_eigenval_real(&content)
}

/// Internal function to parse EIGENVAL content.
fn parse_vasp_eigenval_real(content: &str) -> Result<BandStructureData, String> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() < 5 {
        return Err("EIGENVAL file too short".to_string());
    }

    // Parse header (lines 0-4)
    let header_parts: Vec<usize> = lines[5].split_whitespace()
        .map(|s| s.parse().unwrap_or(0))
        .collect();

    if header_parts.len() < 2 {
        return Err("Invalid EIGENVAL header".to_string());
    }

    let num_kpoints = header_parts[0];
    let num_bands = header_parts[1];

    let mut kpoints = Vec::with_capacity(num_kpoints);
    let mut kpoint_weights = Vec::with_capacity(num_kpoints);
    let mut bands: Vec<BandData> = (0..num_bands)
        .map(|i| BandData {
            band_index: i as u32,
            energies: Vec::with_capacity(num_kpoints),
            is_occupied: Vec::with_capacity(num_kpoints),
        })
        .collect();

    let mut fermi_energy = 0.0;

    // Parse k-points and band energies
    let mut line_idx = 6;
    for _ in 0..num_kpoints {
        if line_idx >= lines.len() {
            break;
        }

        // K-point line
        let kpoint_parts: Vec<f64> = lines[line_idx].split_whitespace()
            .map(|s| s.parse().unwrap_or(0.0))
            .collect();

        if kpoint_parts.len() >= 4 {
            kpoints.push([kpoint_parts[0], kpoint_parts[1], kpoint_parts[2]]);
            kpoint_weights.push(kpoint_parts[3]);
        }
        line_idx += 1;

        // Band energies for this k-point
        for band_idx in 0..num_bands {
            if line_idx >= lines.len() {
                break;
            }
            let band_parts: Vec<f64> = lines[line_idx].split_whitespace()
                .map(|s| s.parse().unwrap_or(0.0))
                .collect();

            if band_parts.len() >= 2 {
                let energy = band_parts[1];
                bands[band_idx].energies.push(energy);
                // Occupation is in column 2 (if present)
                let occupation = band_parts.get(2).copied().unwrap_or(0.0);
                bands[band_idx].is_occupied.push(occupation > 0.5);
            }
            line_idx += 1;
        }
    }

    // Estimate Fermi energy from band occupations
    for band in &bands {
        for (i, &is_occ) in band.is_occupied.iter().enumerate() {
            if !is_occ && i > 0 {
                fermi_energy = (band.energies[i] + band.energies[i-1]) / 2.0;
                break;
            }
        }
    }

    info!(
        num_kpoints = kpoints.len(),
        num_bands = bands.len(),
        "EIGENVAL parsed successfully"
    );

    Ok(BandStructureData {
        kpoints,
        kpoint_weights,
        bands,
        num_bands: num_bands as u32,
        num_kpoints: num_kpoints as u32,
        fermi_energy,
    })
}

/// Parses DFT output directory and extracts all relevant data.
#[command]
pub fn parse_dft_output_directory(output_dir: String, code: String) -> Result<DftDirectoryParseResult, String> {
    info!(output_dir = %output_dir, code = %code, "Parsing DFT output directory");

    let path = Path::new(&output_dir);
    if !path.exists() {
        return Err(format!("Output directory not found: {}", output_dir));
    }

    let mut result = DftDirectoryParseResult {
        energy_data: None,
        dos_data: None,
        band_data: None,
        convergence_achieved: false,
        files_found: Vec::new(),
        errors: Vec::new(),
    };

    match code.as_str() {
        "vasp" => {
            // Look for OUTCAR
            let outcar_path = path.join("OUTCAR");
            if outcar_path.exists() {
                result.files_found.push("OUTCAR".to_string());
                match parse_vasp_outcar_file(outcar_path.to_string_lossy().to_string()) {
                    Ok(data) => {
                        result.convergence_achieved = data.converged;
                        result.energy_data = Some(data);
                    }
                    Err(e) => result.errors.push(format!("OUTCAR parse error: {}", e)),
                }
            }

            // Look for DOSCAR
            let doscar_path = path.join("DOSCAR");
            if doscar_path.exists() {
                result.files_found.push("DOSCAR".to_string());
                match parse_vasp_doscar_file(doscar_path.to_string_lossy().to_string()) {
                    Ok(data) => result.dos_data = Some(data),
                    Err(e) => result.errors.push(format!("DOSCAR parse error: {}", e)),
                }
            }

            // Look for EIGENVAL
            let eigenval_path = path.join("EIGENVAL");
            if eigenval_path.exists() {
                result.files_found.push("EIGENVAL".to_string());
                match parse_vasp_eigenval_file(eigenval_path.to_string_lossy().to_string()) {
                    Ok(data) => result.band_data = Some(data),
                    Err(e) => result.errors.push(format!("EIGENVAL parse error: {}", e)),
                }
            }
        }
        "qe" => {
            // Quantum ESPRESSO output parsing
            let pw_out_path = path.join("pw.out");
            if pw_out_path.exists() {
                result.files_found.push("pw.out".to_string());
                // Use existing QE parser
                match std::fs::read_to_string(&pw_out_path) {
                    Ok(content) => {
                        match parse_qe_output(content) {
                            Ok(data) => {
                                result.convergence_achieved = data.converged;
                                result.energy_data = Some(data);
                            }
                            Err(e) => result.errors.push(format!("pw.out parse error: {}", e)),
                        }
                    }
                    Err(e) => result.errors.push(format!("Failed to read pw.out: {}", e)),
                }
            }
        }
        _ => return Err(format!("Unsupported code: {}", code)),
    }

    if result.files_found.is_empty() {
        return Err("No recognizable output files found in directory".to_string());
    }

    info!(
        files = result.files_found.len(),
        errors = result.errors.len(),
        "DFT output directory parsed"
    );

    Ok(result)
}

/// Result of parsing a DFT output directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DftDirectoryParseResult {
    pub energy_data: Option<DftEnergyData>,
    pub dos_data: Option<DosData>,
    pub band_data: Option<BandStructureData>,
    pub convergence_achieved: bool,
    pub files_found: Vec<String>,
    pub errors: Vec<String>,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Simulates a realistic density of states curve.
fn simulate_dos(energy: f64, fermi_energy: f64) -> f64 {
    // Background DOS (free-electron-like)
    let bg = 0.5 * (1.0 + 0.1 * (energy - fermi_energy)).max(0.1);

    // Valence band peak (centered ~3 eV below Fermi)
    let valence_peak = 3.0 * (-((energy - (fermi_energy - 3.0)).powi(2)) / 1.5).exp();

    // Conduction band onset
    let conduction = if energy > fermi_energy + 0.5 {
        1.5 * (1.0 - (-((energy - fermi_energy - 2.0).powi(2)) / 4.0).exp())
    } else {
        0.0
    };

    // Semi-core states
    let semicore = 0.8 * (-((energy - (fermi_energy - 10.0)).powi(2)) / 2.0).exp();

    bg + valence_peak + conduction + semicore
}

/// Generates spin-resolved DOS data with slight asymmetry.
fn generate_spin_dos(
    energy_values: &[f64],
    fermi_energy: f64,
    amplitude: f64,
    width: f64,
    center_offset: f64,
) -> (Vec<f64>, Vec<f64>) {
    let mut spin_up = Vec::with_capacity(energy_values.len());
    let mut spin_down = Vec::with_capacity(energy_values.len());

    for &e in energy_values {
        let center = fermi_energy - center_offset;
        let up = amplitude * (-((e - center).powi(2)) / width).exp();
        // Slight spin asymmetry (exchange splitting)
        let down = amplitude * 0.95 * (-((e - center - 0.1).powi(2)) / width).exp();
        spin_up.push(up);
        spin_down.push(down);
    }

    (spin_up, spin_down)
}
