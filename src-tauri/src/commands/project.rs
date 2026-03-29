use crate::db::Database;
use crate::models::{CreateProjectInput, Project, UpdateProjectInput};
use tauri::command;
use tauri::State;

/// Create a new project
#[command]
pub fn create_project(
    db: State<'_, Database>,
    input: CreateProjectInput,
) -> Result<Project, String> {
    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();
    
    let project = Project {
        id: id.clone(),
        name: input.name,
        description: input.description,
        created_at: now.clone(),
        updated_at: now,
    };

    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO projects (id, name, description, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        (&project.id, &project.name, &project.description, &project.created_at, &project.updated_at),
    ).map_err(|e| format!("Failed to insert project: {}", e))?;

    tracing::info!("Created project: {} - {}", project.id, project.name);
    Ok(project)
}

/// List all projects
#[command]
pub fn list_projects(db: State<'_, Database>) -> Result<Vec<Project>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, description, created_at, updated_at FROM projects ORDER BY updated_at DESC")
        .map_err(|e| e.to_string())?;

    let projects = stmt
        .query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    tracing::info!("Listed {} projects", projects.len());
    Ok(projects)
}

/// Get a single project by ID
#[command]
pub fn get_project(db: State<'_, Database>, id: String) -> Result<Project, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let project = conn
        .query_row(
            "SELECT id, name, description, created_at, updated_at FROM projects WHERE id = ?1",
            [&id],
            |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            },
        )
        .map_err(|e| format!("Project not found: {}", e))?;

    Ok(project)
}

/// Update a project (rename, update description)
#[command]
pub fn update_project(
    db: State<'_, Database>,
    input: UpdateProjectInput,
) -> Result<Project, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    
    // Get current project
    let current: Project = conn
        .query_row(
            "SELECT id, name, description, created_at, updated_at FROM projects WHERE id = ?1",
            [&input.id],
            |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            },
        )
        .map_err(|e| format!("Project not found: {}", e))?;

    let now = chrono::Utc::now().to_rfc3339();
    let name = input.name.unwrap_or(current.name);
    let description = input.description.unwrap_or(current.description);

    conn.execute(
        "UPDATE projects SET name = ?1, description = ?2, updated_at = ?3 WHERE id = ?4",
        (&name, &description, &now, &input.id),
    )
    .map_err(|e| format!("Failed to update project: {}", e))?;

    let updated = Project {
        id: current.id,
        name,
        description,
        created_at: current.created_at,
        updated_at: now,
    };

    tracing::info!("Updated project: {} - {}", updated.id, updated.name);
    Ok(updated)
}

/// Delete a project
#[command]
pub fn delete_project(db: State<'_, Database>, id: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    
    // Delete associated files first
    conn.execute("DELETE FROM project_files WHERE project_id = ?1", [&id])
        .map_err(|e| format!("Failed to delete project files: {}", e))?;
    
    // Delete settings
    conn.execute("DELETE FROM project_settings WHERE project_id = ?1", [&id])
        .map_err(|e| format!("Failed to delete project settings: {}", e))?;
    
    // Delete project
    conn.execute("DELETE FROM projects WHERE id = ?1", [&id])
        .map_err(|e| format!("Failed to delete project: {}", e))?;

    tracing::info!("Deleted project: {}", id);
    Ok(())
}
