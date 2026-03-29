use crate::db::Database;
use crate::models::{ProjectSettings, SaveSettingsInput};
use tauri::command;
use tauri::State;

/// Save project settings (JSON)
#[command]
pub fn save_settings(
    db: State<'_, Database>,
    input: SaveSettingsInput,
) -> Result<ProjectSettings, String> {
    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();

    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    
    // Check if settings already exist for this project
    let existing: Option<ProjectSettings> = conn
        .query_row(
            "SELECT id, project_id, settings_json, updated_at FROM project_settings WHERE project_id = ?1",
            [&input.project_id],
            |row| {
                Ok(ProjectSettings {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    settings_json: row.get(2)?,
                    updated_at: row.get(3)?,
                })
            },
        )
        .ok();

    if let Some(existing) = existing {
        // Update existing settings
        conn.execute(
            "UPDATE project_settings SET settings_json = ?1, updated_at = ?2 WHERE project_id = ?3",
            (&input.settings_json, &now, &input.project_id),
        )
        .map_err(|e| format!("Failed to update settings: {}", e))?;

        tracing::info!("Updated settings for project: {}", input.project_id);
        Ok(ProjectSettings {
            id: existing.id,
            project_id: input.project_id,
            settings_json: input.settings_json,
            updated_at: now,
        })
    } else {
        // Insert new settings
        conn.execute(
            "INSERT INTO project_settings (id, project_id, settings_json, updated_at) VALUES (?1, ?2, ?3, ?4)",
            (&id, &input.project_id, &input.settings_json, &now),
        )
        .map_err(|e| format!("Failed to insert settings: {}", e))?;

        tracing::info!("Created settings for project: {}", input.project_id);
        Ok(ProjectSettings {
            id,
            project_id: input.project_id,
            settings_json: input.settings_json,
            updated_at: now,
        })
    }
}

/// Get project settings
#[command]
pub fn get_settings(
    db: State<'_, Database>,
    project_id: String,
) -> Result<ProjectSettings, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let settings = conn
        .query_row(
            "SELECT id, project_id, settings_json, updated_at FROM project_settings WHERE project_id = ?1",
            [&project_id],
            |row| {
                Ok(ProjectSettings {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    settings_json: row.get(2)?,
                    updated_at: row.get(3)?,
                })
            },
        )
        .map_err(|e| format!("Settings not found: {}", e))?;

    Ok(settings)
}

/// Delete project settings
#[command]
pub fn delete_settings(db: State<'_, Database>, project_id: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM project_settings WHERE project_id = ?1", [&project_id])
        .map_err(|e| format!("Failed to delete settings: {}", e))?;

    tracing::info!("Deleted settings for project: {}", project_id);
    Ok(())
}
