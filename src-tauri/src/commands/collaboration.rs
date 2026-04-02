use crate::db::Database;
use serde::{Deserialize, Serialize};
use tauri::command;
use tauri::State;

// ============================================
// Collaboration Models
// ============================================

/// Project share model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectShare {
    pub id: String,
    pub project_id: String,
    pub shared_with_name: String,
    pub permission: String,
    pub created_at: String,
}

/// Comment model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub project_id: String,
    pub note_id: Option<String>,
    pub author_name: String,
    pub content: String,
    pub mention_ids: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub resolved: bool,
}

// ============================================
// Project Share Commands
// ============================================

/// Share a project with a team member
#[command]
pub fn share_project(
    db: State<'_, Database>,
    project_id: String,
    shared_with_name: String,
    permission: String,
) -> Result<ProjectShare, String> {
    // Validate permission
    if !["read", "write", "admin"].contains(&permission.as_str()) {
        return Err("Invalid permission. Must be 'read', 'write', or 'admin'".to_string());
    }

    if shared_with_name.trim().is_empty() {
        return Err("Member name cannot be empty".to_string());
    }

    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();

    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Check if already shared with this member
    let existing: Result<String, _> = conn.query_row(
        "SELECT id FROM project_shares WHERE project_id = ?1 AND shared_with_name = ?2",
        rusqlite::params![&project_id, &shared_with_name],
        |row| row.get(0),
    );

    if existing.is_ok() {
        return Err(format!("Project is already shared with '{}'", shared_with_name));
    }

    conn.execute(
        "INSERT INTO project_shares (id, project_id, shared_with_name, permission, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![&id, &project_id, &shared_with_name, &permission, &now],
    )
    .map_err(|e| format!("Failed to share project: {}", e))?;

    tracing::info!("Shared project {} with {} (permission: {})", project_id, shared_with_name, permission);

    Ok(ProjectShare {
        id,
        project_id,
        shared_with_name,
        permission,
        created_at: now,
    })
}

/// List all shares for a project
#[command]
pub fn list_project_shares(
    db: State<'_, Database>,
    project_id: String,
) -> Result<Vec<ProjectShare>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, project_id, shared_with_name, permission, created_at FROM project_shares WHERE project_id = ?1 ORDER BY created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let shares = stmt
        .query_map(rusqlite::params![&project_id], |row| {
            Ok(ProjectShare {
                id: row.get(0)?,
                project_id: row.get(1)?,
                shared_with_name: row.get(2)?,
                permission: row.get(3)?,
                created_at: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(shares)
}

/// Remove a project share
#[command]
pub fn remove_project_share(
    db: State<'_, Database>,
    share_id: String,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "DELETE FROM project_shares WHERE id = ?1",
        rusqlite::params![&share_id],
    )
    .map_err(|e| format!("Failed to remove project share: {}", e))?;

    tracing::info!("Removed project share: {}", share_id);
    Ok(())
}

/// Update share permission
#[command]
pub fn update_share_permission(
    db: State<'_, Database>,
    share_id: String,
    permission: String,
) -> Result<(), String> {
    if !["read", "write", "admin"].contains(&permission.as_str()) {
        return Err("Invalid permission. Must be 'read', 'write', or 'admin'".to_string());
    }

    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE project_shares SET permission = ?1 WHERE id = ?2",
        rusqlite::params![&permission, &share_id],
    )
    .map_err(|e| format!("Failed to update share permission: {}", e))?;

    tracing::info!("Updated share {} permission to {}", share_id, permission);
    Ok(())
}

// ============================================
// Comment Commands
// ============================================

/// Add a comment
#[command]
pub fn add_comment(
    db: State<'_, Database>,
    project_id: String,
    note_id: Option<String>,
    author_name: String,
    content: String,
    mention_ids: Option<String>,
) -> Result<Comment, String> {
    if author_name.trim().is_empty() {
        return Err("Author name cannot be empty".to_string());
    }

    if content.trim().is_empty() {
        return Err("Comment content cannot be empty".to_string());
    }

    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();

    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO comments (id, project_id, note_id, author_name, content, mention_ids, created_at, updated_at, resolved) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 0)",
        rusqlite::params![&id, &project_id, &note_id, &author_name, &content, &mention_ids, &now, &now],
    )
    .map_err(|e| format!("Failed to add comment: {}", e))?;

    tracing::info!("Added comment {} to project {}", id, project_id);

    Ok(Comment {
        id,
        project_id,
        note_id,
        author_name,
        content,
        mention_ids,
        created_at: now.clone(),
        updated_at: now,
        resolved: false,
    })
}

/// List comments for a project, optionally filtered by note_id
#[command]
pub fn list_comments(
    db: State<'_, Database>,
    project_id: String,
    note_id: Option<String>,
) -> Result<Vec<Comment>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    fn row_to_comment(row: &rusqlite::Row<'_>) -> Result<Comment, rusqlite::Error> {
        Ok(Comment {
            id: row.get(0)?,
            project_id: row.get(1)?,
            note_id: row.get(2)?,
            author_name: row.get(3)?,
            content: row.get(4)?,
            mention_ids: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            resolved: row.get::<_, i32>(8)? != 0,
        })
    }

    let comments = if let Some(ref nid) = note_id {
        let mut stmt = conn
            .prepare(
                "SELECT id, project_id, note_id, author_name, content, mention_ids, created_at, updated_at, resolved FROM comments WHERE project_id = ?1 AND note_id = ?2 ORDER BY created_at DESC",
            )
            .map_err(|e| e.to_string())?;

        let result: Vec<Comment> = stmt
            .query_map(rusqlite::params![&project_id, nid], row_to_comment)
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        result
    } else {
        let mut stmt = conn
            .prepare(
                "SELECT id, project_id, note_id, author_name, content, mention_ids, created_at, updated_at, resolved FROM comments WHERE project_id = ?1 ORDER BY created_at DESC",
            )
            .map_err(|e| e.to_string())?;

        let result: Vec<Comment> = stmt
            .query_map(rusqlite::params![&project_id], row_to_comment)
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        result
    };

    Ok(comments)
}

/// Update a comment's content
#[command]
pub fn update_comment(
    db: State<'_, Database>,
    comment_id: String,
    content: String,
) -> Result<Comment, String> {
    if content.trim().is_empty() {
        return Err("Comment content cannot be empty".to_string());
    }

    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "UPDATE comments SET content = ?1, updated_at = ?2 WHERE id = ?3",
        rusqlite::params![&content, &now, &comment_id],
    )
    .map_err(|e| format!("Failed to update comment: {}", e))?;

    let comment = conn
        .query_row(
            "SELECT id, project_id, note_id, author_name, content, mention_ids, created_at, updated_at, resolved FROM comments WHERE id = ?1",
            rusqlite::params![&comment_id],
            |row| {
                Ok(Comment {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    note_id: row.get(2)?,
                    author_name: row.get(3)?,
                    content: row.get(4)?,
                    mention_ids: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                    resolved: row.get::<_, i32>(8)? != 0,
                })
            },
        )
        .map_err(|e| format!("Comment not found: {}", e))?;

    tracing::info!("Updated comment: {}", comment_id);
    Ok(comment)
}

/// Delete a comment
#[command]
pub fn delete_comment(
    db: State<'_, Database>,
    comment_id: String,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "DELETE FROM comments WHERE id = ?1",
        rusqlite::params![&comment_id],
    )
    .map_err(|e| format!("Failed to delete comment: {}", e))?;

    tracing::info!("Deleted comment: {}", comment_id);
    Ok(())
}

/// Resolve or unresolve a comment
#[command]
pub fn resolve_comment(
    db: State<'_, Database>,
    comment_id: String,
    resolved: bool,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = chrono::Utc::now().to_rfc3339();
    let resolved_int = if resolved { 1 } else { 0 };

    conn.execute(
        "UPDATE comments SET resolved = ?1, updated_at = ?2 WHERE id = ?3",
        rusqlite::params![resolved_int, &now, &comment_id],
    )
    .map_err(|e| format!("Failed to resolve comment: {}", e))?;

    tracing::info!("Comment {} resolved status: {}", comment_id, resolved);
    Ok(())
}
