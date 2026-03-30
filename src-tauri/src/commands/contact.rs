//! Contact Analysis Commands
//! Handles contact pair generation, surface definitions, and contact diagnostics

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use crate::commands::input_gen::{ContactPair as InpContactPair, Model as InpModel};

#[derive(Error, Debug)]
pub enum ContactError {
    #[error("Contact pair error: {0}")]
    ContactPairError(String),
    #[error("Surface definition error: {0}")]
    SurfaceError(String),
    #[error("Diagnostic error: {0}")]
    DiagnosticError(String),
}

/// Contact pair configuration from frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactConfig {
    pub master_surface: String,
    pub slave_surface: String,
    pub contact_type: ContactType,
    pub algorithm: ContactAlgorithm,
    pub properties: ContactProperties,
}

/// Contact types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContactType {
    SurfaceToSurface,
    NodeToSurface,
    Tie,
    BoltPreload,
}

impl ContactType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContactType::SurfaceToSurface => "surface_to_surface",
            ContactType::NodeToSurface => "node_to_surface",
            ContactType::Tie => "tie",
            ContactType::BoltPreload => "bolt_preload",
        }
    }
}

/// Contact algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContactAlgorithm {
    Penalty,
    Lagrange,
    AugmentedLagrange,
    Direct,
    Mpc,
}

impl ContactAlgorithm {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContactAlgorithm::Penalty => "Penalty",
            ContactAlgorithm::Lagrange => "Lagrange",
            ContactAlgorithm::AugmentedLagrange => "Augmented Lagrange",
            ContactAlgorithm::Direct => "Direct",
            ContactAlgorithm::Mpc => "MPC",
        }
    }
}

/// Contact mechanical properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactProperties {
    pub normal_stiffness: Option<f64>,
    pub tangential_stiffness: Option<f64>,
    pub friction_coefficient: f64,
    pub initial_clearance: Option<f64>,
    pub initial_penetration: Option<f64>,
    pub pressure_overclosure: PressureOverclosure,
}

impl Default for ContactProperties {
    fn default() -> Self {
        Self {
            normal_stiffness: None,
            tangential_stiffness: None,
            friction_coefficient: 0.0,
            initial_clearance: None,
            initial_penetration: None,
            pressure_overclosure: PressureOverclosure::Linear,
        }
    }
}

/// Pressure-overclosure relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PressureOverclosure {
    Linear,
    Exponential,
    Tabular,
    Hard,
    Tied,
}

/// Surface definition for contact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceDefinition {
    pub name: String,
    pub surface_type: SurfaceType,
    pub element_set: Option<String>,
    pub node_set: Option<String>,
    /// For node-based surfaces: list of nodes
    pub nodes: Option<Vec<usize>>,
    /// For element-based surfaces: list of elements and their faces
    pub elements: Option<Vec<ElementSurface>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementSurface {
    pub element_id: usize,
    pub face: String, // e.g., "S1", "S2", "S3", "S4", "S5", "S6"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SurfaceType {
    ElementBased,
    NodeBased,
}

/// Contact diagnostics result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactDiagnostic {
    pub contact_pair: String,
    pub issues: Vec<ContactIssue>,
    pub recommendations: Vec<String>,
    pub severity: DiagnosticSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactIssue {
    pub issue_type: ContactIssueType,
    pub description: String,
    pub affected_elements: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContactIssueType {
    PenetrationTooLarge,
    StiffnessMismatch,
    MeshIncompatibility,
    InitialGapTooLarge,
    MixedElementTypes,
    Underconstrained,
    Overconstrained,
    LargeSliding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    None,
    Info,
    Warning,
    Error,
}

/// Contact convergence history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceHistory {
    pub iteration: usize,
    pub contact_force: f64,
    pub penetration: f64,
    pub error: f64,
    pub converged: bool,
}

/// Assembly configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyConfig {
    pub parts: Vec<AssemblyPart>,
    pub mates: Vec<MateConstraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyPart {
    pub id: String,
    pub name: String,
    pub file_path: String,
    pub position: [f64; 3],
    pub rotation: [f64; 3],
    pub material: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MateConstraint {
    pub id: String,
    pub part1: String,
    pub surface1: String,
    pub part2: String,
    pub surface2: String,
    pub mate_type: MateType,
    pub offset: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MateType {
    Coincident,
    Parallel,
    Distance,
    Angle,
    Tangent,
}

/// Generate CONTACT PAIR section for INP file
pub fn generate_contact_section(configs: &[ContactConfig]) -> String {
    let mut output = String::new();
    
    for (idx, config) in configs.iter().enumerate() {
        let interaction_name = format!("Contact_{}", idx + 1);
        
        // Surface Interaction definition
        output.push_str(&format!("*SURFACE INTERACTION, NAME={}\n", interaction_name));
        
        // Contact behavior based on algorithm
        match config.properties.pressure_overclosure {
            PressureOverclosure::Hard => {
                output.push_str("*SURFACE BEHAVIOR, PRESSURE-OVERCLOSURE=HARD\n");
            }
            PressureOverclosure::Linear => {
                if let Some(stiffness) = config.properties.normal_stiffness {
                    output.push_str(&format!(
                        "*SURFACE BEHAVIOR, PRESSURE-OVERCLOSURE=LINEAR\n{}\n",
                        stiffness
                    ));
                } else {
                    output.push_str("*SURFACE BEHAVIOR, PRESSURE-OVERCLOSURE=LINEAR\n1.0\n");
                }
            }
            PressureOverclosure::Exponential => {
                output.push_str("*SURFACE BEHAVIOR, PRESSURE-OVERCLOSURE=EXPONENTIAL\n");
                if let Some(stiffness) = config.properties.normal_stiffness {
                    output.push_str(&format!("0.001, {}\n", stiffness));
                }
            }
            PressureOverclosure::Tied => {
                output.push_str("*SURFACE BEHAVIOR, PRESSURE-OVERCLOSURE=TIED\n");
            }
            PressureOverclosure::Tabular => {
                output.push_str("*SURFACE BEHAVIOR, PRESSURE-OVERCLOSURE=TABULAR\n");
            }
        }
        
        // Friction if specified
        if config.properties.friction_coefficient > 0.0 {
            output.push_str("*FRICTION\n");
            output.push_str(&format!(
                "{:.6}\n",
                config.properties.friction_coefficient
            ));
        }
        
        // Contact pair
        output.push_str("\n*CONTACT PAIR, INTERACTION=");
        output.push_str(&interaction_name);
        
        // Add algorithm-specific options
        match config.algorithm {
            ContactAlgorithm::Lagrange => {
                output.push_str(", LAGRANGE\n");
            }
            ContactAlgorithm::AugmentedLagrange => {
                output.push_str(", AUGMENTED LAGRANGE\n");
            }
            ContactAlgorithm::Mpc => {
                output.push_str(", MPC\n");
            }
            _ => {
                output.push_str("\n");
            }
        }
        
        output.push_str(&format!(
            "{}, {}\n",
            config.slave_surface, config.master_surface
        ));
        
        output.push_str("\n");
    }
    
    output
}

/// Generate *SURFACE DEFINITION for INP file
pub fn generate_surface_definition(surfaces: &[SurfaceDefinition]) -> String {
    let mut output = String::new();
    
    for surface in surfaces {
        match surface.surface_type {
            SurfaceType::ElementBased => {
                output.push_str(&format!("*SURFACE, NAME={}, TYPE=ELEMENT, FACE\n", surface.name));
                
                if let Some(elements) = &surface.elements {
                    for elem_surf in elements {
                        output.push_str(&format!(
                            "{}, {}\n",
                            elem_surf.element_id, elem_surf.face
                        ));
                    }
                }
            }
            SurfaceType::NodeBased => {
                output.push_str(&format!("*SURFACE, NAME={}, TYPE=NODE\n", surface.name));
                
                if let Some(nodes) = &surface.nodes {
                    // Write in groups of 8 for readability
                    for chunk in nodes.chunks(8) {
                        output.push_str(&format!(
                            "{}\n",
                            chunk.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ")
                        ));
                    }
                }
            }
        }
        
        output.push_str("\n");
    }
    
    output
}

/// Generate TIE CONSTRAINT for bonded contact
pub fn generate_tie_constraint(name: &str, slave_surface: &str, master_surface: &str) -> String {
    let mut output = String::new();
    
    output.push_str("*TIE, NAME=");
    output.push_str(name);
    output.push_str("\n");
    
    output.push_str(slave_surface);
    output.push_str(", ");
    output.push_str(master_surface);
    output.push_str("\n");
    
    output
}

/// Convert ContactConfig to InpContactPair for model
impl From<ContactConfig> for InpContactPair {
    fn from(config: ContactConfig) -> Self {
        InpContactPair {
            master_surface: config.master_surface,
            slave_surface: config.slave_surface,
            friction: config.properties.friction_coefficient,
            contact_type: config.contact_type.as_str().to_string(),
        }
    }
}

/// Run contact diagnostics on a model
pub fn diagnose_contacts(
    configs: &[ContactConfig],
    model: &InpModel,
) -> Vec<ContactDiagnostic> {
    let mut diagnostics = Vec::new();
    
    for config in configs {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        
        // Check for element type compatibility
        let master_elems: Vec<_> = model.elements.iter()
            .filter(|e| e.nodes.len() >= 4)
            .collect();
        
        // Check contact type vs element types
        match config.contact_type {
            ContactType::SurfaceToSurface => {
                // S2S works best with solid elements
                if model.elements.iter().all(|e| e.nodes.len() <= 2) {
                    issues.push(ContactIssue {
                        issue_type: ContactIssueType::MeshIncompatibility,
                        description: "Surface-to-surface contact with beam/shell elements may be inaccurate".to_string(),
                        affected_elements: model.elements.len(),
                    });
                    recommendations.push("Consider using Node-to-Surface contact for beam models".to_string());
                    recommendations.push("Or add solid elements at contact region".to_string());
                }
            }
            ContactType::Tie => {
                // Tie is forgiving
                recommendations.push("Tie constraint provides robust bonding - good choice!".to_string());
            }
            _ => {}
        }
        
        // Check friction value
        if config.properties.friction_coefficient > 0.5 {
            issues.push(ContactIssue {
                issue_type: ContactIssueType::Overconstrained,
                description: format!("High friction coefficient ({}) may cause convergence issues", 
                    config.properties.friction_coefficient),
                affected_elements: 0,
            });
            recommendations.push("Consider reducing friction coefficient to below 0.5".to_string());
            recommendations.push("Or use Augmented Lagrange algorithm for better convergence".to_string());
        }
        
        // Check stiffness
        if let Some(normal_stiffness) = config.properties.normal_stiffness {
            if normal_stiffness < 0.1 {
                issues.push(ContactIssue {
                    issue_type: ContactIssueType::StiffnessMismatch,
                    description: "Very low contact stiffness may cause excessive penetration".to_string(),
                    affected_elements: 0,
                });
                recommendations.push("Increase normal contact stiffness (try 1.0-10.0)".to_string());
            }
            if normal_stiffness > 1000.0 {
                issues.push(ContactIssue {
                    issue_type: ContactIssueType::StiffnessMismatch,
                    description: "Very high contact stiffness may cause convergence difficulties".to_string(),
                    affected_elements: 0,
                });
                recommendations.push("Consider using Augmented Lagrange algorithm".to_string());
            }
        }
        
        // Check initial gap
        if let Some(clearance) = config.properties.initial_clearance {
            if clearance > 0.1 {
                issues.push(ContactIssue {
                    issue_type: ContactIssueType::InitialGapTooLarge,
                    description: format!("Large initial gap ({}) may prevent contact establishment", clearance),
                    affected_elements: 0,
                });
                recommendations.push("Ensure parts are properly positioned before analysis".to_string());
                recommendations.push("Or use *CONTACT CLEARANCE option".to_string());
            }
        }
        
        // Determine severity
        let severity = if issues.iter().any(|i| matches!(i.issue_type, 
            ContactIssueType::PenetrationTooLarge | ContactIssueType::MeshIncompatibility)) {
            DiagnosticSeverity::Error
        } else if issues.len() > 2 {
            DiagnosticSeverity::Warning
        } else if !issues.is_empty() {
            DiagnosticSeverity::Info
        } else {
            DiagnosticSeverity::None
        };
        
        if recommendations.is_empty() && severity == DiagnosticSeverity::None {
            recommendations.push("Contact settings look good!".to_string());
        }
        
        diagnostics.push(ContactDiagnostic {
            contact_pair: config.master_surface.clone(),
            issues,
            recommendations,
            severity,
        });
    }
    
    diagnostics
}

/// Generate assembly template INP sections
pub fn generate_assembly_template(template_type: &str) -> String {
    match template_type {
        "bolt_joint" => {
            let mut output = String::new();
            output.push_str("** Template: Bolt Joint Analysis\n");
            output.push_str("** Preloaded bolt connection with frictional contact\n\n");
            
            // Bolt preload definition
            output.push_str("*BOUNDARY\n");
            output.push_str("Bolt_Position, 1, 6, 0\n");
            output.push_str("** Apply preload via temperature or displacement\n\n");
            
            // Contact pairs
            output.push_str("*SURFACE INTERACTION, NAME=Bolt_Friction\n");
            output.push_str("*FRICTION\n");
            output.push_str("0.3\n");
            output.push_str("*CONTACT PAIR, INTERACTION=Bolt_Friction\n");
            output.push_str("Bolt_Slave, Bolt_Master\n\n");
            
            output
        }
        "lap_splice" => {
            let mut output = String::new();
            output.push_str("** Template: Lap Splice Joint\n");
            output.push_str("** Overlapping plate joint with interface contact\n\n");
            
            // Interface contact
            output.push_str("*SURFACE INTERACTION, NAME=Interface\n");
            output.push_str("*SURFACE BEHAVIOR, PRESSURE-OVERCLOSURE=TIED\n");
            output.push_str("*CONTACT PAIR, INTERACTION=Interface\n");
            output.push_str("Plate_Bottom, Plate_Top\n\n");
            
            output
        }
        "interference_fit" => {
            let mut output = String::new();
            output.push_str("** Template: Interference Fit (Press Fit)\n");
            output.push_str("** Shrink fit or press fit analysis\n\n");
            
            // Initial penetration to model interference
            output.push_str("*SURFACE INTERACTION, NAME=Interference\n");
            output.push_str("*SURFACE BEHAVIOR, PRESSURE-OVERCLOSURE=HARD\n");
            output.push_str("*CONTACT PAIR, INTERACTION=Interference\n");
            output.push_str("Shaft_Slave, Hub_Master\n");
            output.push_str("** Note: Use negative clearance or initial overclosure\n\n");
            
            output
        }
        "weld" => {
            let mut output = String::new();
            output.push_str("** Template: Weld Joint (Tie Constraint)\n");
            output.push_str("** Bonded contact for welded joints\n\n");
            
            output.push_str("*TIE, NAME=WeldBond\n");
            output.push_str("Weld_Slave, Weld_Master\n\n");
            
            output
        }
        _ => String::new(),
    }
}

/// Analyze contact convergence and suggest improvements
pub fn suggest_convergence_improvements(
    iterations: usize,
    max_penetration: f64,
    contact_force_error: f64,
) -> Vec<String> {
    let mut suggestions = Vec::new();
    
    if iterations > 50 {
        suggestions.push("High iteration count detected. Consider:");
        suggestions.push("- Increasing contact stiffness (normal stiffness * 2-5)");
        suggestions.push("- Using Augmented Lagrange algorithm");
        suggestions.push("- Refining mesh at contact interface");
    }
    
    if max_penetration > 0.01 {
        suggestions.push("Significant penetration detected:");
        suggestions.push("- Increase contact stiffness");
        suggestions.push("- Reduce time increment for first few steps");
        suggestions.push("- Consider using Lagrange multiplier method");
    }
    
    if contact_force_error > 0.05 {
        suggestions.push("Contact force error is high:");
        suggestions.push("- Check that contact surfaces are properly defined");
        suggestions.push("- Verify master/slave surface assignment");
        suggestions.push("- Consider adjusting search distance tolerance");
    }
    
    if suggestions.is_empty() {
        suggestions.push("Analysis appears well-converged.".to_string());
    }
    
    suggestions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_contact_section() {
        let config = ContactConfig {
            master_surface: "Surf_Master".to_string(),
            slave_surface: "Surf_Slave".to_string(),
            contact_type: ContactType::SurfaceToSurface,
            algorithm: ContactAlgorithm::Penalty,
            properties: ContactProperties {
                normal_stiffness: Some(1.0),
                tangential_stiffness: Some(0.5),
                friction_coefficient: 0.2,
                initial_clearance: None,
                initial_penetration: None,
                pressure_overclosure: PressureOverclosure::Linear,
            },
        };
        
        let output = generate_contact_section(&[config]);
        assert!(output.contains("Contact_1"));
        assert!(output.contains("SURFACE INTERACTION"));
        assert!(output.contains("CONTACT PAIR"));
    }

    #[test]
    fn test_generate_tie_constraint() {
        let output = generate_tie_constraint("Bonded", "Part_Slave", "Part_Master");
        assert!(output.contains("*TIE"));
        assert!(output.contains("Bonded"));
    }

    #[test]
    fn test_assembly_templates() {
        for template in &["bolt_joint", "lap_splice", "interference_fit", "weld"] {
            let output = generate_assembly_template(template);
            assert!(!output.is_empty());
            assert!(output.contains("** Template:"));
        }
    }
}

// ============ Tauri Commands ============

use tauri::command;

/// Create a contact pair from frontend configuration
#[command]
pub fn create_contact_config(
    master_surface: String,
    slave_surface: String,
    contact_type: String,
    friction_coefficient: f64,
    normal_stiffness: Option<f64>,
    algorithm: String,
) -> Result<ContactConfig, String> {
    tracing::info!("Creating contact config: {} -> {}", slave_surface, master_surface);
    
    let ct = match contact_type.to_lowercase().as_str() {
        "bonded" | "tie" => ContactType::Tie,
        "bolt_preload" | "bolt" => ContactType::BoltPreload,
        "node_to_surface" | "node" => ContactType::NodeToSurface,
        _ => ContactType::SurfaceToSurface,
    };
    
    let alg = match algorithm.to_lowercase().as_str() {
        "lagrange" => ContactAlgorithm::Lagrange,
        "augmented_lagrange" => ContactAlgorithm::AugmentedLagrange,
        "direct" => ContactAlgorithm::Direct,
        "mpc" => ContactAlgorithm::Mpc,
        _ => ContactAlgorithm::Penalty,
    };
    
    Ok(ContactConfig {
        master_surface,
        slave_surface,
        contact_type: ct,
        algorithm: alg,
        properties: ContactProperties {
            normal_stiffness,
            tangential_stiffness: None,
            friction_coefficient,
            initial_clearance: None,
            initial_penetration: None,
            pressure_overclosure: PressureOverclosure::Linear,
        },
    })
}

/// Generate contact section for INP file
#[command]
pub fn generate_contact_inp(configs: Vec<ContactConfig>) -> String {
    tracing::info!("Generating contact INP for {} pairs", configs.len());
    generate_contact_section(&configs)
}

/// Run contact diagnostics
#[command]
pub fn diagnose_contact_pairs(
    configs: Vec<ContactConfig>,
    num_nodes: usize,
    num_elements: usize,
) -> Vec<ContactDiagnostic> {
    tracing::info!("Diagnosing {} contact pairs", configs.len());
    
    // Create a minimal model for diagnostics
    let model = InpModel {
        nodes: (0..num_nodes).map(|i| {
            crate::commands::input_gen::Node {
                id: i,
                x: 0.0, y: 0.0, z: 0.0,
            }
        }).collect(),
        elements: (0..num_elements).map(|i| {
            crate::commands::input_gen::Element {
                id: i,
                element_type: crate::commands::input_gen::ElementType::C3D8,
                nodes: vec![0, 1, 2, 3, 4, 5, 6, 7],
            }
        }).collect(),
        materials: vec![],
        steps: vec![],
        boundary_conditions: vec![],
        loads: vec![],
        contact_pairs: vec![],
    };
    
    diagnose_contacts(&configs, &model)
}

/// Generate assembly template INP
#[command]
pub fn get_contact_template_inp(template_name: String) -> String {
    tracing::info!("Getting contact template: {}", template_name);
    
    let template_map: HashMap<&str, &str> = [
        ("bolt_joint", "bolt_joint"),
        ("lap_splice", "lap_splice"),
        ("interference_fit", "interference_fit"),
        ("weld", "weld"),
    ].into_iter().collect();
    
    let template_key = template_map.get(template_name.to_lowercase().as_str()).unwrap_or(&"weld");
    generate_assembly_template(template_key)
}

/// Generate surface definition for contact
#[command]
pub fn generate_surface_def(
    surface_name: String,
    surface_type: String,
    element_ids: Vec<usize>,
    faces: Vec<String>,
) -> String {
    let st = if surface_type.to_lowercase() == "node" {
        SurfaceType::NodeBased
    } else {
        SurfaceType::ElementBased
    };
    
    let elements = if !element_ids.is_empty() {
        Some(element_ids.iter().zip(faces.iter()).map(|(id, face)| {
            ElementSurface {
                element_id: *id,
                face: face.clone(),
            }
        }).collect())
    } else {
        None
    };
    
    let surface = SurfaceDefinition {
        name: surface_name,
        surface_type: st,
        element_set: None,
        node_set: None,
        nodes: None,
        elements,
    };
    
    generate_surface_definition(&[surface])
}

/// Get contact algorithm recommendations
#[command]
pub fn get_contact_algorithm_recommendations() -> Vec<String> {
    vec![
        "Penalty: 通用性好，收敛快，适合大多数接触问题".to_string(),
        "Lagrange: 精度高，适合需要精确接触压力的分析".to_string(),
        "Augmented Lagrange: 平衡精度与收敛性，适合难收敛问题".to_string(),
        "MPC: 适合绑定约束和梁壳连接".to_string(),
    ]
}

/// Get convergence improvement suggestions
#[command]
pub fn get_convergence_suggestions(
    iterations: usize,
    max_penetration: f64,
    contact_force_error: f64,
) -> Vec<String> {
    suggest_convergence_improvements(iterations, max_penetration, contact_force_error)
}