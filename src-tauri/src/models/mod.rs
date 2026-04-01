use serde::{Deserialize, Serialize};

/// Project model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Project file model (notes, modeling data, code files)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFile {
    pub id: String,
    pub project_id: String,
    pub file_type: FileType,
    pub file_name: String,
    pub content: Option<String>,
    pub file_path: String,
    pub category: String,
    pub created_at: String,
    pub updated_at: String,
}

/// File type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FileType {
    Note,
    ModelingData,
    CodeFile,
}

/// Project settings model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub id: String,
    pub project_id: String,
    pub settings_json: String,
    pub updated_at: String,
}

/// Input for creating a new project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectInput {
    pub name: String,
    pub description: String,
}

/// Input for updating a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProjectInput {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

/// Input for creating a project file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileInput {
    pub project_id: String,
    pub file_type: FileType,
    pub file_name: String,
    pub content: Option<String>,
    pub file_path: String,
}

/// Input for updating a project file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFileInput {
    pub id: String,
    pub file_name: Option<String>,
    pub content: Option<String>,
}

/// Input for saving project settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveSettingsInput {
    pub project_id: String,
    pub settings_json: String,
}

// ============================================
// Material models
// ============================================

/// Material properties for structural analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub id: String,
    pub name: String,
    /// Young's modulus (Elastic modulus) in GPa
    pub youngs_modulus: f64,
    /// Poisson's ratio (dimensionless)
    pub poissons_ratio: f64,
    /// Density in kg/m³
    pub density: f64,
    /// Thermal expansion coefficient in 1/K
    pub thermal_expansion: f64,
    /// Yield strength in MPa
    pub yield_strength: f64,
    /// Ultimate strength in MPa
    pub ultimate_strength: Option<f64>,
    /// Description text
    pub description: Option<String>,
    /// Whether this is a built-in material
    pub is_builtin: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Input for creating a custom material
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMaterialInput {
    pub name: String,
    pub youngs_modulus: f64,
    pub poissons_ratio: f64,
    pub density: f64,
    pub thermal_expansion: f64,
    pub yield_strength: f64,
    pub ultimate_strength: Option<f64>,
    pub description: Option<String>,
}

/// Input for updating a material
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMaterialInput {
    pub id: String,
    pub name: Option<String>,
    pub youngs_modulus: Option<f64>,
    pub poissons_ratio: Option<f64>,
    pub density: Option<f64>,
    pub thermal_expansion: Option<f64>,
    pub yield_strength: Option<f64>,
    pub ultimate_strength: Option<f64>,
    pub description: Option<String>,
}