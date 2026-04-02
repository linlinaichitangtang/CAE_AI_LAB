use crate::db::Database;
use crate::models::{CreateFileInput, FileType, ProjectFile, UpdateFileInput};
use serde::{Deserialize, Serialize};
use tauri::command;
use tauri::State;

/// Note version model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteVersion {
    pub id: String,
    pub note_id: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
}

/// Note link model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteLink {
    pub id: String,
    pub source_note_id: String,
    pub target_note_id: String,
    pub created_at: String,
}

/// Search result model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub note_id: String,
    pub title: String,
    pub snippet: String,
    pub score: f64,
}

/// Embed record model (embedded targets in notes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedRecord {
    pub id: String,
    pub note_id: String,
    pub target_type: String,
    pub target_id: String,
    pub target_name: String,
    pub config: Option<String>,
    pub created_at: String,
}

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
        category: "未分类".to_string(),
        created_at: now.clone(),
        updated_at: now,
    };

    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO project_files (id, project_id, file_type, file_name, content, file_path, category, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        (&file.id, &file.project_id, file_type_str, &file.file_name, &file.content, &file.file_path, &file.category, &file.created_at, &file.updated_at),
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
        .prepare("SELECT id, project_id, file_type, file_name, content, file_path, category, created_at, updated_at FROM project_files WHERE project_id = ?1 ORDER BY updated_at DESC")
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
                category: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
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
            "SELECT id, project_id, file_type, file_name, content, file_path, category, created_at, updated_at FROM project_files WHERE id = ?1",
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
                    category: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
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
            "SELECT id, project_id, file_type, file_name, content, file_path, category, created_at, updated_at FROM project_files WHERE id = ?1",
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
                    category: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            },
        )
        .map_err(|e| format!("File not found: {}", e))?;

    let now = chrono::Utc::now().to_rfc3339();
    let file_name = input.file_name.unwrap_or_else(|| current.file_name.clone());
    let content = input.content.or(current.content.clone());

    // Save version history before updating (only for notes)
    if current.file_type == FileType::Note {
        let version_id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO note_versions (id, note_id, title, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            (&version_id, &input.id, &current.file_name, &current.content.clone().unwrap_or_default(), &now),
        ).map_err(|e| format!("Failed to save version: {}", e))?;
    }

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
        category: current.category,
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

// ============ Note Version History Commands ============

/// Save a version manually (not auto on every update)
#[command]
pub fn save_note_version(
    db: State<'_, Database>,
    note_id: String,
    title: String,
    content: String,
) -> Result<NoteVersion, String> {
    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();
    
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO note_versions (id, note_id, title, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        (&id, &note_id, &title, &content, &now),
    ).map_err(|e| format!("Failed to save version: {}", e))?;

    tracing::info!("Saved version for note: {}", note_id);
    Ok(NoteVersion { id, note_id, title, content, created_at: now })
}

/// Get all versions of a note
#[command]
pub fn get_note_versions(
    db: State<'_, Database>,
    note_id: String,
) -> Result<Vec<NoteVersion>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, note_id, title, content, created_at FROM note_versions WHERE note_id = ?1 ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;

    let versions = stmt
        .query_map([&note_id], |row| {
            Ok(NoteVersion {
                id: row.get(0)?,
                note_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                created_at: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(versions)
}

/// Get a specific version
#[command]
pub fn get_note_version(
    db: State<'_, Database>,
    version_id: String,
) -> Result<NoteVersion, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let version = conn
        .query_row(
            "SELECT id, note_id, title, content, created_at FROM note_versions WHERE id = ?1",
            [&version_id],
            |row| {
                Ok(NoteVersion {
                    id: row.get(0)?,
                    note_id: row.get(1)?,
                    title: row.get(2)?,
                    content: row.get(3)?,
                    created_at: row.get(4)?,
                })
            },
        )
        .map_err(|e| format!("Version not found: {}", e))?;

    Ok(version)
}

/// Restore a note to a specific version
#[command]
pub fn restore_note_version(
    db: State<'_, Database>,
    note_id: String,
    version_id: String,
) -> Result<ProjectFile, String> {
    // First get the version
    let version = get_note_version(db.clone(), version_id.clone())?;
    
    // Make sure it belongs to this note
    if version.note_id != note_id {
        return Err("Version does not belong to this note".to_string());
    }
    
    // Save current state as a version first
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    
    let current: ProjectFile = conn
        .query_row(
            "SELECT id, project_id, file_type, file_name, content, file_path, category, created_at, updated_at FROM project_files WHERE id = ?1",
            [&note_id],
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
                    category: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            },
        )
        .map_err(|e| format!("Note not found: {}", e))?;
    
    let now = chrono::Utc::now().to_rfc3339();
    let backup_id = uuid::Uuid::new_v4().to_string();
    
    conn.execute(
        "INSERT INTO note_versions (id, note_id, title, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        (&backup_id, &note_id, &current.file_name, &current.content.clone().unwrap_or_default(), &now),
    ).map_err(|e| format!("Failed to backup current state: {}", e))?;
    
    // Restore to the selected version
    conn.execute(
        "UPDATE project_files SET file_name = ?1, content = ?2, updated_at = ?3 WHERE id = ?4",
        (&version.title, &version.content, &now, &note_id),
    ).map_err(|e| format!("Failed to restore: {}", e))?;

    tracing::info!("Restored note {} to version {}", note_id, version_id);
    
    Ok(ProjectFile {
        id: current.id,
        project_id: current.project_id,
        file_type: current.file_type,
        file_name: version.title,
        content: Some(version.content),
        file_path: current.file_path,
        category: current.category,
        created_at: current.created_at,
        updated_at: now,
    })
}

/// Delete a specific version
#[command]
pub fn delete_note_version(
    db: State<'_, Database>,
    version_id: String,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM note_versions WHERE id = ?1", [&version_id])
        .map_err(|e| format!("Failed to delete version: {}", e))?;

    tracing::info!("Deleted version: {}", version_id);
    Ok(())
}

// ============ Note Link Commands ============

/// Create a link between two notes
#[command]
pub fn create_note_link(
    db: State<'_, Database>,
    source_note_id: String,
    target_note_id: String,
) -> Result<NoteLink, String> {
    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();
    
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    
    // Check if link already exists
    let exists: Result<i32, _> = conn.query_row(
        "SELECT COUNT(*) FROM note_links WHERE source_note_id = ?1 AND target_note_id = ?2",
        [&source_note_id, &target_note_id],
        |row| row.get(0),
    );
    
    if exists.unwrap_or(0) > 0 {
        return Err("Link already exists".to_string());
    }
    
    conn.execute(
        "INSERT INTO note_links (id, source_note_id, target_note_id, created_at) VALUES (?1, ?2, ?3, ?4)",
        (&id, &source_note_id, &target_note_id, &now),
    ).map_err(|e| format!("Failed to create link: {}", e))?;

    tracing::info!("Created link: {} -> {}", source_note_id, target_note_id);
    Ok(NoteLink { id, source_note_id, target_note_id, created_at: now })
}

/// Get all links from a note (outgoing)
#[command]
pub fn get_note_links(
    db: State<'_, Database>,
    note_id: String,
) -> Result<Vec<NoteLink>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, source_note_id, target_note_id, created_at FROM note_links WHERE source_note_id = ?1")
        .map_err(|e| e.to_string())?;

    let links = stmt
        .query_map([&note_id], |row| {
            Ok(NoteLink {
                id: row.get(0)?,
                source_note_id: row.get(1)?,
                target_note_id: row.get(2)?,
                created_at: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(links)
}

/// Get all links to a note (backlinks)
#[command]
pub fn get_note_backlinks(
    db: State<'_, Database>,
    note_id: String,
) -> Result<Vec<NoteLink>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, source_note_id, target_note_id, created_at FROM note_links WHERE target_note_id = ?1")
        .map_err(|e| e.to_string())?;

    let links = stmt
        .query_map([&note_id], |row| {
            Ok(NoteLink {
                id: row.get(0)?,
                source_note_id: row.get(1)?,
                target_note_id: row.get(2)?,
                created_at: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(links)
}

/// Delete a note link
#[command]
pub fn delete_note_link(
    db: State<'_, Database>,
    link_id: String,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM note_links WHERE id = ?1", [&link_id])
        .map_err(|e| format!("Failed to delete link: {}", e))?;

    tracing::info!("Deleted link: {}", link_id);
    Ok(())
}

// ============ Search Commands ============

/// Search notes by title and content
#[command]
pub fn search_notes(
    db: State<'_, Database>,
    project_id: String,
    query: String,
) -> Result<Vec<SearchResult>, String> {
    if query.trim().is_empty() {
        return Ok(vec![]);
    }
    
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let search_pattern = format!("%{}%", query.to_lowercase());
    
    let mut stmt = conn
        .prepare(
            "SELECT id, file_name, content FROM project_files 
             WHERE project_id = ?1 AND file_type = 'note' 
             AND (LOWER(file_name) LIKE ?2 OR LOWER(content) LIKE ?2)
             ORDER BY updated_at DESC
             LIMIT 20"
        )
        .map_err(|e| e.to_string())?;

    let results = stmt
        .query_map([&project_id, &search_pattern], |row| {
            let id: String = row.get(0)?;
            let title: String = row.get(1)?;
            let content: String = row.get(2)?;
            
            // Generate snippet
            let content_lower = content.to_lowercase();
            let query_lower = query.to_lowercase();
            
            let snippet = if let Some(pos) = content_lower.find(&query_lower) {
                let start = if pos > 50 { pos - 50 } else { 0 };
                let end = std::cmp::min(pos + query.len() + 100, content.len());
                let mut s = content[start..end].to_string();
                if start > 0 { s = format!("...{}", s); }
                if end < content.len() { s = format!("{}...", s); }
                s.replace('\n', " ").replace('<', "&lt;").replace('>', "&gt;")
            } else {
                content[..std::cmp::min(150, content.len())].to_string()
                    .replace('\n', " ").replace('<', "&lt;").replace('>', "&gt;")
            };
            
            Ok(SearchResult {
                note_id: id,
                title,
                snippet,
                score: 1.0,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(results)
}

// ============ Embed Record Commands ============

/// Add an embed record to a note
#[command]
pub fn add_embed_record(
    db: State<'_, Database>,
    note_id: String,
    target_type: String,
    target_id: String,
    target_name: String,
    config: Option<String>,
) -> Result<EmbedRecord, String> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO embed_records (id, note_id, target_type, target_id, target_name, config, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (&id, &note_id, &target_type, &target_id, &target_name, &config, &now),
    ).map_err(|e| format!("Failed to insert embed record: {}", e))?;

    tracing::info!("Added embed record: {} -> {} ({})", note_id, target_type, target_name);
    Ok(EmbedRecord {
        id,
        note_id,
        target_type,
        target_id,
        target_name,
        config,
        created_at: now,
    })
}

/// Get all embed records for a note
#[command]
pub fn get_embed_records(
    db: State<'_, Database>,
    note_id: String,
) -> Result<Vec<EmbedRecord>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, note_id, target_type, target_id, target_name, config, created_at FROM embed_records WHERE note_id = ?1 ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;

    let records = stmt
        .query_map([&note_id], |row| {
            Ok(EmbedRecord {
                id: row.get(0)?,
                note_id: row.get(1)?,
                target_type: row.get(2)?,
                target_id: row.get(3)?,
                target_name: row.get(4)?,
                config: row.get(5)?,
                created_at: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(records)
}

/// Delete an embed record by ID
#[command]
pub fn delete_embed_record(
    db: State<'_, Database>,
    id: String,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM embed_records WHERE id = ?1", [&id])
        .map_err(|e| format!("Failed to delete embed record: {}", e))?;

    tracing::info!("Deleted embed record: {}", id);
    Ok(())
}

/// Update an embed record's target_name and config
#[command]
pub fn update_embed_record(
    db: State<'_, Database>,
    id: String,
    target_name: String,
    config: Option<String>,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE embed_records SET target_name = ?1, config = ?2 WHERE id = ?3",
        (&target_name, &config, &id),
    )
    .map_err(|e| format!("Failed to update embed record: {}", e))?;

    tracing::info!("Updated embed record: {}", id);
    Ok(())
}

// ============ Knowledge Graph Commands ============

/// Knowledge graph node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraphNode {
    pub id: String,
    pub title: String,
    pub file_type: String,
}

/// Knowledge graph edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraphEdge {
    pub source: String,
    pub target: String,
}

/// Knowledge graph data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    pub nodes: Vec<KnowledgeGraphNode>,
    pub edges: Vec<KnowledgeGraphEdge>,
}

/// Get the knowledge graph for a project (all notes and their links)
#[command]
pub fn get_knowledge_graph(
    db: State<'_, Database>,
    project_id: String,
) -> Result<KnowledgeGraph, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // 1. Query all note files in the project
    let mut stmt = conn
        .prepare("SELECT id, file_name, file_type FROM project_files WHERE project_id = ?1 AND file_type = 'note'")
        .map_err(|e| e.to_string())?;

    let note_ids: Vec<(String, String, String)> = stmt
        .query_map([&project_id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    let note_id_set: std::collections::HashSet<String> =
        note_ids.iter().map(|(id, _, _)| id.clone()).collect();

    // 2. Build nodes
    let nodes: Vec<KnowledgeGraphNode> = note_ids
        .into_iter()
        .map(|(id, title, file_type)| KnowledgeGraphNode {
            id,
            title,
            file_type,
        })
        .collect();

    // 3. Query all note_links where both source and target are in this project
    let mut link_stmt = conn
        .prepare("SELECT source_note_id, target_note_id FROM note_links")
        .map_err(|e| e.to_string())?;

    let all_links: Vec<(String, String)> = link_stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    // 4. Build edges (only include links where both endpoints are in the project)
    let edges: Vec<KnowledgeGraphEdge> = all_links
        .into_iter()
        .filter(|(source, target)| note_id_set.contains(source) && note_id_set.contains(target))
        .map(|(source, target)| KnowledgeGraphEdge { source, target })
        .collect();

    tracing::info!(
        "Knowledge graph for project {}: {} nodes, {} edges",
        project_id,
        nodes.len(),
        edges.len()
    );

    Ok(KnowledgeGraph { nodes, edges })
}

// ============ Category Commands ============

/// Update a file's category
#[command]
pub fn update_file_category(
    db: State<'_, Database>,
    file_id: String,
    category: String,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE project_files SET category = ?1 WHERE id = ?2",
        (&category, &file_id),
    )
    .map_err(|e| format!("Failed to update file category: {}", e))?;

    tracing::info!("Updated file {} category to: {}", file_id, category);
    Ok(())
}

/// Get all distinct categories for a project
#[command]
pub fn get_file_categories(
    db: State<'_, Database>,
    project_id: String,
) -> Result<Vec<String>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT DISTINCT category FROM project_files WHERE project_id = ?1 AND category IS NOT NULL ORDER BY category")
        .map_err(|e| e.to_string())?;

    let categories = stmt
        .query_map([&project_id], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(categories)
}
