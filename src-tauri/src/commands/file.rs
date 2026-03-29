use crate::db::Database;
use crate::models::{CreateFileInput, FileType, ProjectFile, UpdateFileInput};
use tauri::command;
use tauri::State;

/// Create a new file in a project
#[command]
pub fn create_file(
    db: State<'_, Database>,
    input: CreateFileInput,
) -> Result<ProjectFile, String> {
    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();
    
    let file_type_str = match input.file_type {
        FileType::Note => "note",
        FileType::ModelingData => "modeling_data",
        FileType::CodeFile => "code_file",
    };

    let file = ProjectFile {
        id: id.clone(),
        project_id: input.project_id,
        file_type: input.file_type,
        file_name: input.file_name,
        content: input.content,
        file_path: input.file_path,
        created_at: now.clone(),
        updated_at: now,
    };

    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO project_files (id, project_id, file_type, file_name, content, file_path, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        (&file.id, &file.project_id, file_type_str, &file.file_name, &file.content, &file.file_path, &file.created_at, &file.updated_at),
    ).map_err(|e| format!("Failed to insert file: {}", e))?;

    tracing::info!("Created file: {} - {}", file.id, file.file_name);
    Ok(file)
}

/// List all files in a project
#[command]
pub fn list_files(
    db: State<'_, Database>,
    project_id: String,
) -> Result<Vec<ProjectFile>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, project_id, file_type, file_name, content, file_path, created_at, updated_at FROM project_files WHERE project_id = ?1 ORDER BY updated_at DESC")
        .map_err(|e| e.to_string())?;

    let files = stmt
        .query_map([&project_id], |row| {
            let file_type_str: String = row.get(2)?;
            let file_type = match file_type_str.as_str() {
                "note" => FileType::Note,
                "modeling_data" => FileType::ModelingData,
                "code_file" => FileType::CodeFile,
                _ => FileType::Note,
            };
            Ok(ProjectFile {
                id: row.get(0)?,
                project_id: row.get(1)?,
                file_type,
                file_name: row.get(3)?,
                content: row.get(4)?,
                file_path: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    tracing::info!("Listed {} files for project {}", files.len(), project_id);
    Ok(files)
}

/// Get a single file by ID
#[command]
pub fn get_file(db: State<'_, Database>, id: String) -> Result<ProjectFile, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let file = conn
        .query_row(
            "SELECT id, project_id, file_type, file_name, content, file_path, created_at, updated_at FROM project_files WHERE id = ?1",
            [&id],
            |row| {
                let file_type_str: String = row.get(2)?;
                let file_type = match file_type_str.as_str() {
                    "note" => FileType::Note,
                    "modeling_data" => FileType::ModelingData,
                    "code_file" => FileType::CodeFile,
                    _ => FileType::Note,
                };
                Ok(ProjectFile {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    file_type,
                    file_name: row.get(3)?,
                    content: row.get(4)?,
                    file_path: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            },
        )
        .map_err(|e| format!("File not found: {}", e))?;

    Ok(file)
}

/// Update a file (rename, update content)
#[command]
pub fn update_file(
    db: State<'_, Database>,
    input: UpdateFileInput,
) -> Result<ProjectFile, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    
    // Get current file
    let current: ProjectFile = conn
        .query_row(
            "SELECT id, project_id, file_type, file_name, content, file_path, created_at, updated_at FROM project_files WHERE id = ?1",
            [&input.id],
            |row| {
                let file_type_str: String = row.get(2)?;
                let file_type = match file_type_str.as_str() {
                    "note" => FileType::Note,
                    "modeling_data" => FileType::ModelingData,
                    "code_file" => FileType::CodeFile,
                    _ => FileType::Note,
                };
                Ok(ProjectFile {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    file_type,
                    file_name: row.get(3)?,
                    content: row.get(4)?,
                    file_path: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            },
        )
        .map_err(|e| format!("File not found: {}", e))?;

    let now = chrono::Utc::now().to_rfc3339();
    let file_name = input.file_name.unwrap_or(current.file_name);
    let content = input.content.or(current.content);

    conn.execute(
        "UPDATE project_files SET file_name = ?1, content = ?2, updated_at = ?3 WHERE id = ?4",
        (&file_name, &content, &now, &input.id),
    )
    .map_err(|e| format!("Failed to update file: {}", e))?;

    let updated = ProjectFile {
        id: current.id,
        project_id: current.project_id,
        file_type: current.file_type,
        file_name,
        content,
        file_path: current.file_path,
        created_at: current.created_at,
        updated_at: now,
    };

    tracing::info!("Updated file: {} - {}", updated.id, updated.file_name);
    Ok(updated)
}

/// Delete a file
#[command]
pub fn delete_file(db: State<'_, Database>, id: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM project_files WHERE id = ?1", [&id])
        .map_err(|e| format!("Failed to delete file: {}", e))?;

    tracing::info!("Deleted file: {}", id);
    Ok(())
}

/// Read file content by ID
#[command]
pub fn read_file_content(db: State<'_, Database>, id: String) -> Result<String, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let content: String = conn
        .query_row(
            "SELECT content FROM project_files WHERE id = ?1",
            [&id],
            |row| row.get(0),
        )
        .map_err(|e| format!("File not found: {}", e))?;

    Ok(content)
}

/// Write content to a file
#[command]
pub fn write_file_content(
    db: State<'_, Database>,
    id: String,
    content: String,
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE project_files SET content = ?1, updated_at = ?2 WHERE id = ?3",
        (&content, &now, &id),
    )
    .map_err(|e| format!("Failed to write file content: {}", e))?;

    tracing::info!("Wrote content to file: {}", id);
    Ok(())
}
