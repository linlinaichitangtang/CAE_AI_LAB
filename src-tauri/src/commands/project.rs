use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::command;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

#[command]
pub fn create_project(name: String, description: String) -> Result<Project, String> {
    tracing::info!("Creating project: {}", name);
    
    let project = Project {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        description,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };
    
    Ok(project)
}

#[command]
pub fn list_projects() -> Result<Vec<Project>, String> {
    tracing::info!("Listing projects");
    Ok(vec![])
}