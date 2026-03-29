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
