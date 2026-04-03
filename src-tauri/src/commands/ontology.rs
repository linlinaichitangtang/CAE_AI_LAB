// Ontology Commands - V1.8
// Physical quantity ontology management with unit conversion across SI/atomic/CGS systems.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::command;

// ============================================================================
// Data Structures
// ============================================================================

/// A single unit definition within a quantity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitDefinition {
    pub symbol: String,
    pub name: String,
    pub system: String,       // "SI", "atomic", "CGS", "engineering"
    pub to_si_factor: f64,    // multiply by this to convert to SI base
    pub offset: f64,          // additive offset (e.g., Celsius to Kelvin)
}

/// A physical quantity with its available units and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalQuantity {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub category: String,
    pub si_unit: String,
    pub dimension: Vec<String>,  // e.g., ["length", "mass", "time", ...]
    pub description: String,
    pub units: Vec<UnitDefinition>,
}

/// Result of a unit conversion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    pub input_value: f64,
    pub input_unit: String,
    pub output_value: f64,
    pub output_unit: String,
    pub quantity_id: String,
    pub conversion_factor: f64,
}

/// Result of a batch unit conversion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchConversionResult {
    pub conversions: Vec<ConversionResult>,
    pub total_count: u32,
    pub successful_count: u32,
    pub failed_count: u32,
    pub errors: Vec<String>,
}

/// A category of physical quantities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantityCategory {
    pub id: String,
    pub name: String,
    pub description: String,
    pub quantity_count: u32,
    pub subcategories: Vec<String>,
}

// ============================================================================
// Mock Data
// ============================================================================

fn build_ontology() -> Vec<PhysicalQuantity> {
    vec![
        PhysicalQuantity {
            id: "stress".to_string(),
            name: "Stress".to_string(),
            symbol: "sigma".to_string(),
            category: "mechanics".to_string(),
            si_unit: "Pa".to_string(),
            dimension: vec!["mass".into(), "length".into(), "time".into()],
            description: "Force per unit area. In Voigt notation, 6 independent components for symmetric stress tensor.".to_string(),
            units: vec![
                UnitDefinition { symbol: "Pa".into(), name: "Pascal".into(), system: "SI".into(), to_si_factor: 1.0, offset: 0.0 },
                UnitDefinition { symbol: "MPa".into(), name: "Megapascal".into(), system: "SI".into(), to_si_factor: 1e6, offset: 0.0 },
                UnitDefinition { symbol: "GPa".into(), name: "Gigapascal".into(), system: "SI".into(), to_si_factor: 1e9, offset: 0.0 },
                UnitDefinition { symbol: "bar".into(), name: "Bar".into(), system: "CGS".into(), to_si_factor: 1e5, offset: 0.0 },
                UnitDefinition { symbol: "atm".into(), name: "Atmosphere".into(), system: "CGS".into(), to_si_factor: 101325.0, offset: 0.0 },
                UnitDefinition { symbol: "psi".into(), name: "Pound per square inch".into(), system: "engineering".into(), to_si_factor: 6894.757, offset: 0.0 },
                UnitDefinition { symbol: "kbar".into(), name: "Kilobar".into(), system: "CGS".into(), to_si_factor: 1e8, offset: 0.0 },
                UnitDefinition { symbol: "eV/A^3".into(), name: "Electronvolt per cubic angstrom".into(), system: "atomic".into(), to_si_factor: 1.602176634e11, offset: 0.0 },
            ],
        },
        PhysicalQuantity {
            id: "strain".to_string(),
            name: "Strain".to_string(),
            symbol: "epsilon".to_string(),
            category: "mechanics".to_string(),
            si_unit: "1 (dimensionless)".to_string(),
            dimension: vec![],
            description: "Dimensionless measure of deformation. Engineering strain = delta_L / L0.".to_string(),
            units: vec![
                UnitDefinition { symbol: "1".into(), name: "Dimensionless".into(), system: "SI".into(), to_si_factor: 1.0, offset: 0.0 },
                UnitDefinition { symbol: "%".into(), name: "Percent strain".into(), system: "engineering".into(), to_si_factor: 0.01, offset: 0.0 },
                UnitDefinition { symbol: "microstrain".into(), name: "Microstrain".into(), system: "SI".into(), to_si_factor: 1e-6, offset: 0.0 },
            ],
        },
        PhysicalQuantity {
            id: "temperature".to_string(),
            name: "Temperature".to_string(),
            symbol: "T".to_string(),
            category: "thermodynamics".to_string(),
            si_unit: "K".to_string(),
            dimension: vec!["temperature".into()],
            description: "Thermodynamic temperature. Base SI unit.".to_string(),
            units: vec![
                UnitDefinition { symbol: "K".into(), name: "Kelvin".into(), system: "SI".into(), to_si_factor: 1.0, offset: 0.0 },
                UnitDefinition { symbol: "C".into(), name: "Celsius".into(), system: "SI".into(), to_si_factor: 1.0, offset: 273.15 },
                UnitDefinition { symbol: "F".into(), name: "Fahrenheit".into(), system: "engineering".into(), to_si_factor: 5.0 / 9.0, offset: 255.372 },
                UnitDefinition { symbol: "eV".into(), name: "Electronvolt (thermal)".into(), system: "atomic".into(), to_si_factor: 11604.518, offset: 0.0 },
                UnitDefinition { symbol: "Ha".into(), name: "Hartree (thermal)".into(), system: "atomic".into(), to_si_factor: 315775.024, offset: 0.0 },
            ],
        },
        PhysicalQuantity {
            id: "energy".to_string(),
            name: "Energy".to_string(),
            symbol: "E".to_string(),
            category: "thermodynamics".to_string(),
            si_unit: "J".to_string(),
            dimension: vec!["mass".into(), "length".into(), "time".into()],
            description: "The capacity to do work. Central quantity in DFT and MD simulations.".to_string(),
            units: vec![
                UnitDefinition { symbol: "J".into(), name: "Joule".into(), system: "SI".into(), to_si_factor: 1.0, offset: 0.0 },
                UnitDefinition { symbol: "eV".into(), name: "Electronvolt".into(), system: "atomic".into(), to_si_factor: 1.602176634e-19, offset: 0.0 },
                UnitDefinition { symbol: "Ha".into(), name: "Hartree".into(), system: "atomic".into(), to_si_factor: 4.359744722e-18, offset: 0.0 },
                UnitDefinition { symbol: "kcal/mol".into(), name: "Kilocalorie per mole".into(), system: "chemistry".into(), to_si_factor: 6.9477e-21, offset: 0.0 },
                UnitDefinition { symbol: "kJ/mol".into(), name: "Kilojoule per mole".into(), system: "chemistry".into(), to_si_factor: 1.66054e-21, offset: 0.0 },
                UnitDefinition { symbol: "erg".into(), name: "Erg".into(), system: "CGS".into(), to_si_factor: 1e-7, offset: 0.0 },
                UnitDefinition { symbol: "Ry".into(), name: "Rydberg".into(), system: "atomic".into(), to_si_factor: 2.179872361e-18, offset: 0.0 },
                UnitDefinition { symbol: "meV".into(), name: "Millielectronvolt".into(), system: "atomic".into(), to_si_factor: 1.602176634e-22, offset: 0.0 },
            ],
        },
        PhysicalQuantity {
            id: "force".to_string(),
            name: "Force".to_string(),
            symbol: "F".to_string(),
            category: "mechanics".to_string(),
            si_unit: "N".to_string(),
            dimension: vec!["mass".into(), "length".into(), "time".into()],
            description: "Interaction that changes the motion of an object. F = ma.".to_string(),
            units: vec![
                UnitDefinition { symbol: "N".into(), name: "Newton".into(), system: "SI".into(), to_si_factor: 1.0, offset: 0.0 },
                UnitDefinition { symbol: "eV/A".into(), name: "Electronvolt per angstrom".into(), system: "atomic".into(), to_si_factor: 1.602176634e-9, offset: 0.0 },
                UnitDefinition { symbol: "dyn".into(), name: "Dyne".into(), system: "CGS".into(), to_si_factor: 1e-5, offset: 0.0 },
                UnitDefinition { symbol: "lbf".into(), name: "Pound-force".into(), system: "engineering".into(), to_si_factor: 4.448222, offset: 0.0 },
                UnitDefinition { symbol: "Ha/Bohr".into(), name: "Hartree per Bohr".into(), system: "atomic".into(), to_si_factor: 8.2387236e-8, offset: 0.0 },
            ],
        },
        PhysicalQuantity {
            id: "length".to_string(),
            name: "Length".to_string(),
            symbol: "L".to_string(),
            category: "geometry".to_string(),
            si_unit: "m".to_string(),
            dimension: vec!["length".into()],
            description: "Spatial extent. Base SI unit.".to_string(),
            units: vec![
                UnitDefinition { symbol: "m".into(), name: "Meter".into(), system: "SI".into(), to_si_factor: 1.0, offset: 0.0 },
                UnitDefinition { symbol: "A".into(), name: "Angstrom".into(), system: "atomic".into(), to_si_factor: 1e-10, offset: 0.0 },
                UnitDefinition { symbol: "nm".into(), name: "Nanometer".into(), system: "SI".into(), to_si_factor: 1e-9, offset: 0.0 },
                UnitDefinition { symbol: "pm".into(), name: "Picometer".into(), system: "SI".into(), to_si_factor: 1e-12, offset: 0.0 },
                UnitDefinition { symbol: "Bohr".into(), name: "Bohr radius".into(), system: "atomic".into(), to_si_factor: 5.291772109e-11, offset: 0.0 },
                UnitDefinition { symbol: "cm".into(), name: "Centimeter".into(), system: "CGS".into(), to_si_factor: 0.01, offset: 0.0 },
                UnitDefinition { symbol: "mm".into(), name: "Millimeter".into(), system: "SI".into(), to_si_factor: 0.001, offset: 0.0 },
            ],
        },
        PhysicalQuantity {
            id: "mass".to_string(),
            name: "Mass".to_string(),
            symbol: "m".to_string(),
            category: "fundamental".to_string(),
            si_unit: "kg".to_string(),
            dimension: vec!["mass".into()],
            description: "Amount of matter. Base SI unit.".to_string(),
            units: vec![
                UnitDefinition { symbol: "kg".into(), name: "Kilogram".into(), system: "SI".into(), to_si_factor: 1.0, offset: 0.0 },
                UnitDefinition { symbol: "g".into(), name: "Gram".into(), system: "CGS".into(), to_si_factor: 1e-3, offset: 0.0 },
                UnitDefinition { symbol: "amu".into(), name: "Atomic mass unit".into(), system: "atomic".into(), to_si_factor: 1.660539066e-27, offset: 0.0 },
                UnitDefinition { symbol: "Da".into(), name: "Dalton".into(), system: "atomic".into(), to_si_factor: 1.660539066e-27, offset: 0.0 },
                UnitDefinition { symbol: "lb".into(), name: "Pound".into(), system: "engineering".into(), to_si_factor: 0.45359237, offset: 0.0 },
                UnitDefinition { symbol: "me".into(), name: "Electron mass".into(), system: "atomic".into(), to_si_factor: 9.1093837015e-31, offset: 0.0 },
            ],
        },
        PhysicalQuantity {
            id: "time".to_string(),
            name: "Time".to_string(),
            symbol: "t".to_string(),
            category: "fundamental".to_string(),
            si_unit: "s".to_string(),
            dimension: vec!["time".into()],
            description: "Duration. Base SI unit.".to_string(),
            units: vec![
                UnitDefinition { symbol: "s".into(), name: "Second".into(), system: "SI".into(), to_si_factor: 1.0, offset: 0.0 },
                UnitDefinition { symbol: "fs".into(), name: "Femtosecond".into(), system: "atomic".into(), to_si_factor: 1e-15, offset: 0.0 },
                UnitDefinition { symbol: "ps".into(), name: "Picosecond".into(), system: "atomic".into(), to_si_factor: 1e-12, offset: 0.0 },
                UnitDefinition { symbol: "ns".into(), name: "Nanosecond".into(), system: "SI".into(), to_si_factor: 1e-9, offset: 0.0 },
                UnitDefinition { symbol: "min".into(), name: "Minute".into(), system: "SI".into(), to_si_factor: 60.0, offset: 0.0 },
                UnitDefinition { symbol: "h".into(), name: "Hour".into(), system: "SI".into(), to_si_factor: 3600.0, offset: 0.0 },
            ],
        },
        PhysicalQuantity {
            id: "pressure".to_string(),
            name: "Pressure".to_string(),
            symbol: "P".to_string(),
            category: "thermodynamics".to_string(),
            si_unit: "Pa".to_string(),
            dimension: vec!["mass".into(), "length".into(), "time".into()],
            description: "Force per unit area. In MD simulations, controls the thermodynamic ensemble.".to_string(),
            units: vec![
                UnitDefinition { symbol: "Pa".into(), name: "Pascal".into(), system: "SI".into(), to_si_factor: 1.0, offset: 0.0 },
                UnitDefinition { symbol: "GPa".into(), name: "Gigapascal".into(), system: "SI".into(), to_si_factor: 1e9, offset: 0.0 },
                UnitDefinition { symbol: "bar".into(), name: "Bar".into(), system: "CGS".into(), to_si_factor: 1e5, offset: 0.0 },
                UnitDefinition { symbol: "atm".into(), name: "Atmosphere".into(), system: "CGS".into(), to_si_factor: 101325.0, offset: 0.0 },
                UnitDefinition { symbol: "MPa".into(), name: "Megapascal".into(), system: "SI".into(), to_si_factor: 1e6, offset: 0.0 },
            ],
        },
        PhysicalQuantity {
            id: "velocity".to_string(),
            name: "Velocity".to_string(),
            symbol: "v".to_string(),
            category: "kinematics".to_string(),
            si_unit: "m/s".to_string(),
            dimension: vec!["length".into(), "time".into()],
            description: "Rate of change of position with respect to time.".to_string(),
            units: vec![
                UnitDefinition { symbol: "m/s".into(), name: "Meter per second".into(), system: "SI".into(), to_si_factor: 1.0, offset: 0.0 },
                UnitDefinition { symbol: "A/fs".into(), name: "Angstrom per femtosecond".into(), system: "atomic".into(), to_si_factor: 1e5, offset: 0.0 },
                UnitDefinition { symbol: "km/s".into(), name: "Kilometer per second".into(), system: "SI".into(), to_si_factor: 1e3, offset: 0.0 },
                UnitDefinition { symbol: "nm/ps".into(), name: "Nanometer per picosecond".into(), system: "atomic".into(), to_si_factor: 1e3, offset: 0.0 },
            ],
        },
        PhysicalQuantity {
            id: "elastic_modulus".to_string(),
            name: "Elastic Modulus".to_string(),
            symbol: "E".to_string(),
            category: "mechanics".to_string(),
            si_unit: "Pa".to_string(),
            dimension: vec!["mass".into(), "length".into(), "time".into()],
            description: "Ratio of stress to strain in the elastic regime. Includes Young's modulus, bulk modulus, and shear modulus.".to_string(),
            units: vec![
                UnitDefinition { symbol: "Pa".into(), name: "Pascal".into(), system: "SI".into(), to_si_factor: 1.0, offset: 0.0 },
                UnitDefinition { symbol: "GPa".into(), name: "Gigapascal".into(), system: "SI".into(), to_si_factor: 1e9, offset: 0.0 },
                UnitDefinition { symbol: "Mbar".into(), name: "Megabar".into(), system: "CGS".into(), to_si_factor: 1e11, offset: 0.0 },
                UnitDefinition { symbol: "eV/A^3".into(), name: "Electronvolt per cubic angstrom".into(), system: "atomic".into(), to_si_factor: 1.602176634e11, offset: 0.0 },
            ],
        },
        PhysicalQuantity {
            id: "thermal_conductivity".to_string(),
            name: "Thermal Conductivity".to_string(),
            symbol: "kappa".to_string(),
            category: "transport".to_string(),
            si_unit: "W/(m*K)".to_string(),
            dimension: vec!["mass".into(), "length".into(), "time".into(), "temperature".into()],
            description: "Rate of heat transfer through a material per unit temperature gradient.".to_string(),
            units: vec![
                UnitDefinition { symbol: "W/(m*K)".into(), name: "Watt per meter kelvin".into(), system: "SI".into(), to_si_factor: 1.0, offset: 0.0 },
                UnitDefinition { symbol: "eV/(A*fs*K)".into(), name: "eV per angstrom-fs-K".into(), system: "atomic".into(), to_si_factor: 1.602176634e6, offset: 0.0 },
                UnitDefinition { symbol: "cal/(cm*s*K)".into(), name: "Calorie per cm-s-K".into(), system: "CGS".into(), to_si_factor: 418.4, offset: 0.0 },
            ],
        },
        PhysicalQuantity {
            id: "diffusion_coefficient".to_string(),
            name: "Diffusion Coefficient".to_string(),
            symbol: "D".to_string(),
            category: "transport".to_string(),
            si_unit: "m^2/s".to_string(),
            dimension: vec!["length".into(), "time".into()],
            description: "Proportionality constant between molar flux and concentration gradient (Fick's first law).".to_string(),
            units: vec![
                UnitDefinition { symbol: "m^2/s".into(), name: "Square meter per second".into(), system: "SI".into(), to_si_factor: 1.0, offset: 0.0 },
                UnitDefinition { symbol: "cm^2/s".into(), name: "Square centimeter per second".into(), system: "CGS".into(), to_si_factor: 1e-4, offset: 0.0 },
                UnitDefinition { symbol: "A^2/fs".into(), name: "Square angstrom per femtosecond".into(), system: "atomic".into(), to_si_factor: 1e-4, offset: 0.0 },
                UnitDefinition { symbol: "mm^2/s".into(), name: "Square millimeter per second".into(), system: "SI".into(), to_si_factor: 1e-6, offset: 0.0 },
            ],
        },
        PhysicalQuantity {
            id: "charge_density".to_string(),
            name: "Charge Density".to_string(),
            symbol: "rho_e".to_string(),
            category: "electromagnetic".to_string(),
            si_unit: "C/m^3".to_string(),
            dimension: vec!["charge".into(), "length".into()],
            description: "Electric charge per unit volume. Key quantity in DFT (electron density).".to_string(),
            units: vec![
                UnitDefinition { symbol: "C/m^3".into(), name: "Coulomb per cubic meter".into(), system: "SI".into(), to_si_factor: 1.0, offset: 0.0 },
                UnitDefinition { symbol: "e/A^3".into(), name: "Electron per cubic angstrom".into(), system: "atomic".into(), to_si_factor: 1.602176634e9, offset: 0.0 },
                UnitDefinition { symbol: "e/Bohr^3".into(), name: "Electron per cubic Bohr".into(), system: "atomic".into(), to_si_factor: 6.748334e30, offset: 0.0 },
            ],
        },
        PhysicalQuantity {
            id: "viscosity".to_string(),
            name: "Dynamic Viscosity".to_string(),
            symbol: "eta".to_string(),
            category: "transport".to_string(),
            si_unit: "Pa*s".to_string(),
            dimension: vec!["mass".into(), "length".into(), "time".into()],
            description: "Resistance to shear deformation in a fluid. Important for CFD simulations.".to_string(),
            units: vec![
                UnitDefinition { symbol: "Pa*s".into(), name: "Pascal second".into(), system: "SI".into(), to_si_factor: 1.0, offset: 0.0 },
                UnitDefinition { symbol: "cP".into(), name: "Centipoise".into(), system: "CGS".into(), to_si_factor: 1e-3, offset: 0.0 },
                UnitDefinition { symbol: "Poise".into(), name: "Poise".into(), system: "CGS".into(), to_si_factor: 0.1, offset: 0.0 },
                UnitDefinition { symbol: "mPa*s".into(), name: "Millipascal second".into(), system: "SI".into(), to_si_factor: 1e-3, offset: 0.0 },
            ],
        },
        PhysicalQuantity {
            id: "surface_energy".to_string(),
            name: "Surface Energy".to_string(),
            symbol: "gamma".to_string(),
            category: "thermodynamics".to_string(),
            si_unit: "J/m^2".to_string(),
            dimension: vec!["mass".into(), "time".into()],
            description: "Excess free energy per unit area at a surface or interface. Critical for phase-field models.".to_string(),
            units: vec![
                UnitDefinition { symbol: "J/m^2".into(), name: "Joule per square meter".into(), system: "SI".into(), to_si_factor: 1.0, offset: 0.0 },
                UnitDefinition { symbol: "mJ/m^2".into(), name: "Millijoule per square meter".into(), system: "SI".into(), to_si_factor: 1e-3, offset: 0.0 },
                UnitDefinition { symbol: "eV/A^2".into(), name: "Electronvolt per square angstrom".into(), system: "atomic".into(), to_si_factor: 16.02176634, offset: 0.0 },
                UnitDefinition { symbol: "erg/cm^2".into(), name: "Erg per square centimeter".into(), system: "CGS".into(), to_si_factor: 1e-3, offset: 0.0 },
            ],
        },
        PhysicalQuantity {
            id: "frequency".to_string(),
            name: "Frequency".to_string(),
            symbol: "f".to_string(),
            category: "kinematics".to_string(),
            si_unit: "Hz".to_string(),
            dimension: vec!["time".into()],
            description: "Number of events per unit time. Related to vibrational modes in phonon calculations.".to_string(),
            units: vec![
                UnitDefinition { symbol: "Hz".into(), name: "Hertz".into(), system: "SI".into(), to_si_factor: 1.0, offset: 0.0 },
                UnitDefinition { symbol: "THz".into(), name: "Terahertz".into(), system: "SI".into(), to_si_factor: 1e12, offset: 0.0 },
                UnitDefinition { symbol: "cm^-1".into(), name: "Wavenumber".into(), system: "spectroscopy".into(), to_si_factor: 2.99792458e10, offset: 0.0 },
                UnitDefinition { symbol: "eV".into(), name: "Electronvolt (energy)".into(), system: "atomic".into(), to_si_factor: 2.417989242e14, offset: 0.0 },
            ],
        },
    ]
}

fn build_categories() -> Vec<QuantityCategory> {
    vec![
        QuantityCategory {
            id: "mechanics".into(),
            name: "Mechanics".into(),
            description: "Stress, strain, elastic moduli, and related mechanical properties.".into(),
            quantity_count: 4,
            subcategories: vec!["elastic".into(), "plastic".into(), "fracture".into()],
        },
        QuantityCategory {
            id: "thermodynamics".into(),
            name: "Thermodynamics".into(),
            description: "Temperature, energy, pressure, and thermodynamic potentials.".into(),
            quantity_count: 4,
            subcategories: vec!["calorimetry".into(), "phase_equilibria".into()],
        },
        QuantityCategory {
            id: "transport".into(),
            name: "Transport Properties".into(),
            description: "Diffusion, thermal conductivity, viscosity, and other transport phenomena.".into(),
            quantity_count: 3,
            subcategories: vec!["mass_transport".into(), "heat_transport".into(), "momentum_transport".into()],
        },
        QuantityCategory {
            id: "geometry".into(),
            name: "Geometry".into(),
            description: "Length, area, volume, and related geometric quantities.".into(),
            quantity_count: 1,
            subcategories: vec![],
        },
        QuantityCategory {
            id: "fundamental".into(),
            name: "Fundamental".into(),
            description: "Base SI quantities: mass, time, amount of substance.".into(),
            quantity_count: 2,
            subcategories: vec![],
        },
        QuantityCategory {
            id: "kinematics".into(),
            name: "Kinematics".into(),
            description: "Velocity, acceleration, frequency, and related kinematic quantities.".into(),
            quantity_count: 2,
            subcategories: vec!["vibrational".into()],
        },
        QuantityCategory {
            id: "electromagnetic".into(),
            name: "Electromagnetic".into(),
            description: "Charge, electric field, magnetic field, and related quantities.".into(),
            quantity_count: 1,
            subcategories: vec!["electrostatics".into(), "magnetism".into()],
        },
    ]
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// Search physical quantities by keyword, category, or symbol.
#[command]
pub fn search_quantities(
    query: Option<String>,
    category: Option<String>,
    limit: Option<u32>,
) -> Result<Vec<PhysicalQuantity>, String> {
    tracing::info!(
        query = query.as_deref().unwrap_or(""),
        category = category.as_deref().unwrap_or(""),
        "search_quantities called"
    );

    let ontology = build_ontology();
    let limit = limit.unwrap_or(50) as usize;

    let results: Vec<PhysicalQuantity> = ontology
        .into_iter()
        .filter(|q| {
            let query_match = match &query {
                Some(q_str) if !q_str.is_empty() => {
                    q.name.to_lowercase().contains(&q_str.to_lowercase())
                        || q.symbol.to_lowercase().contains(&q_str.to_lowercase())
                        || q.id.to_lowercase().contains(&q_str.to_lowercase())
                        || q.description.to_lowercase().contains(&q_str.to_lowercase())
                }
                _ => true,
            };
            let cat_match = match &category {
                Some(c) if !c.is_empty() => q.category == *c,
                _ => true,
            };
            query_match && cat_match
        })
        .take(limit)
        .collect();

    tracing::info!("Found {} quantities", results.len());
    Ok(results)
}

/// Convert a value from one unit to another within the same quantity.
#[command]
pub fn convert_unit(
    quantity_id: String,
    value: f64,
    from_unit: String,
    to_unit: String,
) -> Result<ConversionResult, String> {
    tracing::info!(
        quantity = %quantity_id,
        value = value,
        from = %from_unit,
        to = %to_unit,
        "convert_unit called"
    );

    let ontology = build_ontology();

    let quantity = ontology
        .iter()
        .find(|q| q.id == quantity_id)
        .ok_or_else(|| format!("Quantity '{}' not found in ontology", quantity_id))?;

    let from_def = quantity
        .units
        .iter()
        .find(|u| u.symbol == from_unit)
        .ok_or_else(|| format!("Unit '{}' not found for quantity '{}'", from_unit, quantity_id))?;

    let to_def = quantity
        .units
        .iter()
        .find(|u| u.symbol == to_unit)
        .ok_or_else(|| format!("Unit '{}' not found for quantity '{}'", to_unit, quantity_id))?;

    // Convert: value -> SI -> target
    let si_value = (value - from_def.offset) * from_def.to_si_factor;
    let output_value = si_value / to_def.to_si_factor + to_def.offset;
    let conversion_factor = from_def.to_si_factor / to_def.to_si_factor;

    tracing::info!(
        result = output_value,
        factor = conversion_factor,
        "Conversion complete"
    );

    Ok(ConversionResult {
        input_value: value,
        input_unit: from_unit,
        output_value,
        output_unit: to_unit,
        quantity_id,
        conversion_factor,
    })
}

/// Batch convert multiple values at once.
#[command]
pub fn batch_convert(
    conversions: Vec<Value>,
) -> Result<BatchConversionResult, String> {
    tracing::info!(
        count = conversions.len(),
        "batch_convert called"
    );

    let ontology = build_ontology();
    let mut results = Vec::new();
    let mut errors = Vec::new();

    for conv in &conversions {
        let quantity_id = conv.get("quantity_id").and_then(|v| v.as_str()).unwrap_or("");
        let value = conv.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let from_unit = conv.get("from_unit").and_then(|v| v.as_str()).unwrap_or("");
        let to_unit = conv.get("to_unit").and_then(|v| v.as_str()).unwrap_or("");

        let quantity = ontology.iter().find(|q| q.id == quantity_id);

        match quantity {
            Some(q) => {
                let from_def = q.units.iter().find(|u| u.symbol == from_unit);
                let to_def = q.units.iter().find(|u| u.symbol == to_unit);

                match (from_def, to_def) {
                    (Some(fd), Some(td)) => {
                        let si_value = (value - fd.offset) * fd.to_si_factor;
                        let output_value = si_value / td.to_si_factor + td.offset;
                        let factor = fd.to_si_factor / td.to_si_factor;

                        results.push(ConversionResult {
                            input_value: value,
                            input_unit: from_unit.to_string(),
                            output_value,
                            output_unit: to_unit.to_string(),
                            quantity_id: quantity_id.to_string(),
                            conversion_factor: factor,
                        });
                    }
                    _ => {
                        errors.push(format!(
                            "Units '{}' or '{}' not found for '{}'",
                            from_unit, to_unit, quantity_id
                        ));
                    }
                }
            }
            None => {
                errors.push(format!("Quantity '{}' not found", quantity_id));
            }
        }
    }

    let successful = results.len() as u32;
    let failed = errors.len() as u32;

    tracing::info!(
        successful = successful,
        failed = failed,
        "Batch conversion complete"
    );

    Ok(BatchConversionResult {
        conversions: results,
        total_count: conversions.len() as u32,
        successful_count: successful,
        failed_count: failed,
        errors,
    })
}

/// Get the full definition of a physical quantity by ID.
#[command]
pub fn get_quantity_definition(quantity_id: String) -> Result<PhysicalQuantity, String> {
    tracing::info!(quantity = %quantity_id, "get_quantity_definition called");

    let ontology = build_ontology();

    let quantity = ontology
        .into_iter()
        .find(|q| q.id == quantity_id)
        .ok_or_else(|| format!("Quantity '{}' not found in ontology", quantity_id))?;

    Ok(quantity)
}

/// List all available quantity categories.
#[command]
pub fn list_categories() -> Result<Vec<QuantityCategory>, String> {
    tracing::info!("list_categories called");

    let categories = build_categories();
    tracing::info!("Found {} categories", categories.len());
    Ok(categories)
}
