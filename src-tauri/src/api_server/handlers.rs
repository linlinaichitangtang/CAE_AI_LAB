use axum::{
    extract::{Path, State, Extension},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use rand::Rng;

// Use the http crate directly to avoid Tauri's http re-export conflict
use http::StatusCode;

use super::AppState;

// ============================================================================
// Common response types
// ============================================================================

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Json<Self> {
        Json(Self {
            success: true,
            data: Some(data),
            error: None,
        })
    }

    pub fn err(msg: impl Into<String>) -> (StatusCode, Json<Self>) {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(Self {
            success: false,
            data: None,
            error: Some(msg.into()),
        }))
    }

    pub fn err_with_status(status: StatusCode, msg: impl Into<String>) -> (StatusCode, Json<Self>) {
        (status, Json(Self {
            success: false,
            data: None,
            error: Some(msg.into()),
        }))
    }
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime: u64,
}

// ============================================================================
// Auth types
// ============================================================================

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub nickname: Option<String>,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

// ============================================================================
// Project types
// ============================================================================

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

// ============================================================================
// File types
// ============================================================================

#[derive(Deserialize)]
pub struct CreateFileRequest {
    pub file_type: String,
    pub file_name: String,
    pub content: Option<String>,
    pub file_path: String,
}

#[derive(Deserialize)]
pub struct UpdateFileRequest {
    pub file_name: Option<String>,
    pub content: Option<String>,
}

// ============================================================================
// Simulation types
// ============================================================================

#[derive(Deserialize)]
pub struct RunSimulationRequest {
    pub project_id: String,
    pub job_name: String,
    pub config: serde_json::Value,
}

// ============================================================================
// Mesh types
// ============================================================================

#[derive(Deserialize)]
pub struct GenerateMeshRequest {
    pub mesh_type: String,
    pub config: serde_json::Value,
}

// ============================================================================
// User types
// ============================================================================

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub nickname: Option<String>,
    pub company: Option<String>,
    pub position: Option<String>,
    pub avatar_url: Option<String>,
}

// ============================================================================
// Collaboration types
// ============================================================================

#[derive(Deserialize)]
pub struct ShareProjectRequest {
    pub shared_with_name: String,
    pub permission: String,
}

// ============================================================================
// API Key types
// ============================================================================

#[derive(Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub permissions: Option<String>,
    pub expires_days: Option<i64>,
}

// ============================================================================
// Authenticated user extension
// ============================================================================

#[derive(Clone)]
pub struct AuthUser {
    pub user_id: String,
}

// ============================================================================
// Handlers
// ============================================================================

/// Health check endpoint
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: "0.6.0".to_string(),
        uptime: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0),
    })
}

/// Login handler
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    // Find user by email
    let (user_id, password_hash): (String, String) = match conn.query_row(
        "SELECT id, password_hash FROM users WHERE email = ?1",
        rusqlite::params![req.email],
        |row| Ok((row.get(0)?, row.get(1)?)),
    ) {
        Ok(r) => r,
        Err(_) => return ApiResponse::err_with_status(StatusCode::UNAUTHORIZED, "Invalid email or password".to_string()),
    };

    // Verify password
    let valid = match bcrypt::verify(&req.password, &password_hash) {
        Ok(v) => v,
        Err(e) => return ApiResponse::err(format!("Password verification error: {}", e)),
    };

    if !valid {
        return ApiResponse::err_with_status(StatusCode::UNAUTHORIZED, "Invalid email or password".to_string());
    }

    // Update last login
    conn.execute(
        "UPDATE users SET last_login_at = datetime('now'), updated_at = datetime('now') WHERE id = ?1",
        rusqlite::params![user_id],
    ).ok();

    // Generate tokens
    let access_token = match crate::commands::auth::generate_access_token_public(&user_id) {
        Ok(t) => t,
        Err(e) => return ApiResponse::err(e),
    };
    let refresh_token = match crate::commands::auth::generate_refresh_token_public(&user_id) {
        Ok(t) => t,
        Err(e) => return ApiResponse::err(e),
    };

    // Store refresh token hash
    let refresh_token_hash = crate::commands::auth::hash_token_public(&refresh_token);
    conn.execute(
        "INSERT INTO refresh_tokens (token_hash, user_id, expires_at, created_at) VALUES (?1, ?2, datetime('now', '+7 days'), datetime('now'))",
        rusqlite::params![refresh_token_hash, user_id],
    ).ok();

    // Get user profile
    let (profile, membership) = match crate::commands::auth::get_user_profile_and_membership_public(&conn, &user_id) {
        Ok(r) => r,
        Err(e) => return ApiResponse::err(e),
    };

    let response = serde_json::json!({
        "access_token": access_token,
        "refresh_token": refresh_token,
        "user": profile,
        "membership": membership,
    });

    (StatusCode::OK, ApiResponse::ok(response))
}

/// Register handler
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    if let Err(e) = crate::commands::auth::validate_email_public(&req.email) {
        return ApiResponse::err_with_status(StatusCode::BAD_REQUEST, e);
    }
    if let Err(e) = crate::commands::auth::validate_password_public(&req.password) {
        return ApiResponse::err_with_status(StatusCode::BAD_REQUEST, e);
    }

    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    let exists: bool = conn
        .query_row("SELECT COUNT(*) FROM users WHERE email = ?1", rusqlite::params![req.email], |row| row.get::<_, i32>(0))
        .map(|c| c > 0)
        .unwrap_or(false);

    if exists {
        return ApiResponse::err_with_status(StatusCode::CONFLICT, "Email already registered".to_string());
    }

    let password_hash = match bcrypt::hash(&req.password, bcrypt::DEFAULT_COST) {
        Ok(h) => h,
        Err(e) => return ApiResponse::err(format!("Password hashing error: {}", e)),
    };

    let user_id = nanoid::nanoid!(21);

    conn.execute(
        "INSERT INTO users (id, email, password_hash, nickname, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, datetime('now'), datetime('now'))",
        rusqlite::params![user_id, req.email, password_hash, req.nickname],
    ).map_err(|e| format!("Failed to create user: {}", e))
    .unwrap_or_else(|e| { tracing::error!("{}", e); 0 });

    let access_token = match crate::commands::auth::generate_access_token_public(&user_id) {
        Ok(t) => t,
        Err(e) => return ApiResponse::err(e),
    };
    let refresh_token = match crate::commands::auth::generate_refresh_token_public(&user_id) {
        Ok(t) => t,
        Err(e) => return ApiResponse::err(e),
    };

    let refresh_token_hash = crate::commands::auth::hash_token_public(&refresh_token);
    conn.execute(
        "INSERT INTO refresh_tokens (token_hash, user_id, expires_at, created_at) VALUES (?1, ?2, datetime('now', '+7 days'), datetime('now'))",
        rusqlite::params![refresh_token_hash, user_id],
    ).ok();

    let (profile, membership) = match crate::commands::auth::get_user_profile_and_membership_public(&conn, &user_id) {
        Ok(r) => r,
        Err(e) => return ApiResponse::err(e),
    };

    let response = serde_json::json!({
        "access_token": access_token,
        "refresh_token": refresh_token,
        "user": profile,
        "membership": membership,
    });

    (StatusCode::CREATED, ApiResponse::ok(response))
}

/// Refresh token handler
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(req): Json<RefreshRequest>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let user_id = match crate::commands::auth::verify_token_public(&req.refresh_token, "refresh") {
        Ok(uid) => uid,
        Err(e) => return ApiResponse::err_with_status(StatusCode::UNAUTHORIZED, e),
    };

    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    let refresh_token_hash = crate::commands::auth::hash_token_public(&req.refresh_token);
    let token_valid: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM refresh_tokens WHERE token_hash = ?1 AND user_id = ?2 AND expires_at > datetime('now')",
            rusqlite::params![refresh_token_hash, user_id],
            |row| row.get::<_, i32>(0),
        )
        .map(|c| c > 0)
        .unwrap_or(false);

    if !token_valid {
        return ApiResponse::err_with_status(StatusCode::UNAUTHORIZED, "Invalid or expired refresh token".to_string());
    }

    conn.execute("DELETE FROM refresh_tokens WHERE token_hash = ?1", rusqlite::params![refresh_token_hash]).ok();

    let new_access_token = match crate::commands::auth::generate_access_token_public(&user_id) {
        Ok(t) => t,
        Err(e) => return ApiResponse::err(e),
    };
    let new_refresh_token = match crate::commands::auth::generate_refresh_token_public(&user_id) {
        Ok(t) => t,
        Err(e) => return ApiResponse::err(e),
    };

    let new_hash = crate::commands::auth::hash_token_public(&new_refresh_token);
    conn.execute(
        "INSERT INTO refresh_tokens (token_hash, user_id, expires_at, created_at) VALUES (?1, ?2, datetime('now', '+7 days'), datetime('now'))",
        rusqlite::params![new_hash, user_id],
    ).ok();

    let (profile, membership) = match crate::commands::auth::get_user_profile_and_membership_public(&conn, &user_id) {
        Ok(r) => r,
        Err(e) => return ApiResponse::err(e),
    };

    let response = serde_json::json!({
        "access_token": new_access_token,
        "refresh_token": new_refresh_token,
        "user": profile,
        "membership": membership,
    });

    (StatusCode::OK, ApiResponse::ok(response))
}

/// List all projects
pub async fn list_projects(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
) -> (StatusCode, Json<ApiResponse<Vec<serde_json::Value>>>) {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    let mut stmt = match conn.prepare(
        "SELECT id, name, description, created_at, updated_at FROM projects ORDER BY updated_at DESC"
    ) {
        Ok(s) => s,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    let projects: Vec<serde_json::Value> = match stmt.query_map([], |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, String>(0)?,
            "name": row.get::<_, String>(1)?,
            "description": row.get::<_, String>(2)?,
            "created_at": row.get::<_, String>(3)?,
            "updated_at": row.get::<_, String>(4)?,
        }))
    }) {
        Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    (StatusCode::OK, ApiResponse::ok(projects))
}

/// Create a new project
pub async fn create_project(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Json(req): Json<CreateProjectRequest>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();

    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    if let Err(e) = conn.execute(
        "INSERT INTO projects (id, name, description, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![id, req.name, req.description, now, now],
    ) {
        return ApiResponse::err(format!("Failed to create project: {}", e));
    }

    let project = serde_json::json!({
        "id": id, "name": req.name, "description": req.description,
        "created_at": now, "updated_at": now,
    });

    (StatusCode::CREATED, ApiResponse::ok(project))
}

/// Get a single project
pub async fn get_project(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    match conn.query_row(
        "SELECT id, name, description, created_at, updated_at FROM projects WHERE id = ?1",
        rusqlite::params![id],
        |row| Ok(serde_json::json!({
            "id": row.get::<_, String>(0)?, "name": row.get::<_, String>(1)?,
            "description": row.get::<_, String>(2)?, "created_at": row.get::<_, String>(3)?,
            "updated_at": row.get::<_, String>(4)?,
        })),
    ) {
        Ok(project) => (StatusCode::OK, ApiResponse::ok(project)),
        Err(_) => ApiResponse::err_with_status(StatusCode::NOT_FOUND, "Project not found".to_string()),
    }
}

/// Update a project
pub async fn update_project(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
    Json(req): Json<UpdateProjectRequest>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    let now = chrono::Utc::now().to_rfc3339();
    if let Some(name) = &req.name {
        conn.execute("UPDATE projects SET name = ?1, updated_at = ?2 WHERE id = ?3", rusqlite::params![name, now, id]).ok();
    }
    if let Some(desc) = &req.description {
        conn.execute("UPDATE projects SET description = ?1, updated_at = ?2 WHERE id = ?3", rusqlite::params![desc, now, id]).ok();
    }

    match conn.query_row(
        "SELECT id, name, description, created_at, updated_at FROM projects WHERE id = ?1",
        rusqlite::params![id],
        |row| Ok(serde_json::json!({
            "id": row.get::<_, String>(0)?, "name": row.get::<_, String>(1)?,
            "description": row.get::<_, String>(2)?, "created_at": row.get::<_, String>(3)?,
            "updated_at": row.get::<_, String>(4)?,
        })),
    ) {
        Ok(project) => (StatusCode::OK, ApiResponse::ok(project)),
        Err(_) => ApiResponse::err_with_status(StatusCode::NOT_FOUND, "Project not found".to_string()),
    }
}

/// Delete a project
pub async fn delete_project(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<String>>) {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    match conn.execute("DELETE FROM projects WHERE id = ?1", rusqlite::params![id]) {
        Ok(_) => (StatusCode::OK, ApiResponse::ok("Project deleted".to_string())),
        Err(_) => ApiResponse::err_with_status(StatusCode::NOT_FOUND, "Project not found".to_string()),
    }
}

/// List files in a project
pub async fn list_files(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(project_id): Path<String>,
) -> (StatusCode, Json<ApiResponse<Vec<serde_json::Value>>>) {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    let mut stmt = match conn.prepare(
        "SELECT id, project_id, file_type, file_name, content, file_path, category, created_at, updated_at FROM project_files WHERE project_id = ?1 ORDER BY updated_at DESC"
    ) {
        Ok(s) => s,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    let files: Vec<serde_json::Value> = match stmt.query_map(rusqlite::params![project_id], |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, String>(0)?, "project_id": row.get::<_, String>(1)?,
            "file_type": row.get::<_, String>(2)?, "file_name": row.get::<_, String>(3)?,
            "content": row.get::<_, Option<String>>(4)?, "file_path": row.get::<_, String>(5)?,
            "category": row.get::<_, String>(6)?, "created_at": row.get::<_, String>(7)?,
            "updated_at": row.get::<_, String>(8)?,
        }))
    }) {
        Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    (StatusCode::OK, ApiResponse::ok(files))
}

/// Create a file in a project
pub async fn create_file(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(project_id): Path<String>,
    Json(req): Json<CreateFileRequest>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();

    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    if let Err(e) = conn.execute(
        "INSERT INTO project_files (id, project_id, file_type, file_name, content, file_path, category, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![id, project_id, req.file_type, req.file_name, req.content, req.file_path, "uncategorized", now, now],
    ) {
        return ApiResponse::err(format!("Failed to create file: {}", e));
    }

    let file = serde_json::json!({
        "id": id, "project_id": project_id, "file_type": req.file_type,
        "file_name": req.file_name, "content": req.content, "file_path": req.file_path,
        "category": "uncategorized", "created_at": now, "updated_at": now,
    });

    (StatusCode::CREATED, ApiResponse::ok(file))
}

/// Get a single file
pub async fn get_file(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    match conn.query_row(
        "SELECT id, project_id, file_type, file_name, content, file_path, category, created_at, updated_at FROM project_files WHERE id = ?1",
        rusqlite::params![id],
        |row| Ok(serde_json::json!({
            "id": row.get::<_, String>(0)?, "project_id": row.get::<_, String>(1)?,
            "file_type": row.get::<_, String>(2)?, "file_name": row.get::<_, String>(3)?,
            "content": row.get::<_, Option<String>>(4)?, "file_path": row.get::<_, String>(5)?,
            "category": row.get::<_, String>(6)?, "created_at": row.get::<_, String>(7)?,
            "updated_at": row.get::<_, String>(8)?,
        })),
    ) {
        Ok(file) => (StatusCode::OK, ApiResponse::ok(file)),
        Err(_) => ApiResponse::err_with_status(StatusCode::NOT_FOUND, "File not found".to_string()),
    }
}

/// Update a file
pub async fn update_file(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
    Json(req): Json<UpdateFileRequest>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    let now = chrono::Utc::now().to_rfc3339();
    if let Some(name) = &req.file_name {
        conn.execute("UPDATE project_files SET file_name = ?1, updated_at = ?2 WHERE id = ?3", rusqlite::params![name, now, id]).ok();
    }
    if let Some(content) = &req.content {
        conn.execute("UPDATE project_files SET content = ?1, updated_at = ?2 WHERE id = ?3", rusqlite::params![content, now, id]).ok();
    }

    match conn.query_row(
        "SELECT id, project_id, file_type, file_name, content, file_path, category, created_at, updated_at FROM project_files WHERE id = ?1",
        rusqlite::params![id],
        |row| Ok(serde_json::json!({
            "id": row.get::<_, String>(0)?, "project_id": row.get::<_, String>(1)?,
            "file_type": row.get::<_, String>(2)?, "file_name": row.get::<_, String>(3)?,
            "content": row.get::<_, Option<String>>(4)?, "file_path": row.get::<_, String>(5)?,
            "category": row.get::<_, String>(6)?, "created_at": row.get::<_, String>(7)?,
            "updated_at": row.get::<_, String>(8)?,
        })),
    ) {
        Ok(file) => (StatusCode::OK, ApiResponse::ok(file)),
        Err(_) => ApiResponse::err_with_status(StatusCode::NOT_FOUND, "File not found".to_string()),
    }
}

/// Delete a file
pub async fn delete_file(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<String>>) {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    match conn.execute("DELETE FROM project_files WHERE id = ?1", rusqlite::params![id]) {
        Ok(_) => (StatusCode::OK, ApiResponse::ok("File deleted".to_string())),
        Err(_) => ApiResponse::err_with_status(StatusCode::NOT_FOUND, "File not found".to_string()),
    }
}

/// Run a simulation
pub async fn run_simulation(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Json(req): Json<RunSimulationRequest>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    let job_id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    let sim_record = serde_json::json!({
        "job_id": job_id, "project_id": req.project_id,
        "job_name": req.job_name, "status": "pending",
        "config": req.config, "created_at": now, "created_by": user.user_id,
    });

    let settings_id = uuid::Uuid::new_v4().to_string();
    if let Err(e) = conn.execute(
        "INSERT INTO project_settings (id, project_id, settings_json, updated_at) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![settings_id, req.project_id, sim_record.to_string(), now],
    ) {
        return ApiResponse::err(format!("Failed to create simulation: {}", e));
    }

    (StatusCode::ACCEPTED, ApiResponse::ok(serde_json::json!({
        "job_id": job_id, "status": "pending",
        "message": "Simulation job submitted. Use GET /simulations/{id} to check status.",
    })))
}

/// Get simulation status
pub async fn get_simulation_status(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    match conn.query_row(
        "SELECT settings_json, updated_at FROM project_settings WHERE id = ?1",
        rusqlite::params![id],
        |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
    ) {
        Ok((json_str, updated_at)) => {
            (StatusCode::OK, ApiResponse::ok(serde_json::json!({
                "job_id": id, "status": "completed", "updated_at": updated_at, "result": json_str,
            })))
        }
        Err(_) => ApiResponse::err_with_status(StatusCode::NOT_FOUND, "Simulation not found".to_string()),
    }
}

/// Get simulation result
pub async fn get_simulation_result(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    match conn.query_row(
        "SELECT settings_json FROM project_settings WHERE id = ?1",
        rusqlite::params![id],
        |row| row.get::<_, String>(0),
    ) {
        Ok(json_str) => {
            let result: serde_json::Value = serde_json::from_str(&json_str).unwrap_or_default();
            (StatusCode::OK, ApiResponse::ok(result))
        }
        Err(_) => ApiResponse::err_with_status(StatusCode::NOT_FOUND, "Simulation result not found".to_string()),
    }
}

/// Generate mesh
pub async fn generate_mesh(
    Extension(_user): Extension<AuthUser>,
    Json(req): Json<GenerateMeshRequest>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    (StatusCode::OK, ApiResponse::ok(serde_json::json!({
        "mesh_type": req.mesh_type, "config": req.config,
        "status": "mesh_generation_supported",
        "message": "Mesh generation is available through the desktop application.",
        "nodes": 0, "elements": 0,
    })))
}

/// Get user profile
pub async fn get_profile(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    match crate::commands::auth::get_user_profile_and_membership_public(&conn, &user.user_id) {
        Ok((profile, membership)) => {
            (StatusCode::OK, ApiResponse::ok(serde_json::json!({ "user": profile, "membership": membership })))
        }
        Err(e) => ApiResponse::err_with_status(StatusCode::NOT_FOUND, e),
    }
}

/// Update user profile
pub async fn update_profile(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Json(req): Json<UpdateProfileRequest>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    if let Some(nickname) = &req.nickname {
        conn.execute("UPDATE users SET nickname = ?1, updated_at = datetime('now') WHERE id = ?2", rusqlite::params![nickname, user.user_id]).ok();
    }
    if let Some(company) = &req.company {
        conn.execute("UPDATE users SET company = ?1, updated_at = datetime('now') WHERE id = ?2", rusqlite::params![company, user.user_id]).ok();
    }
    if let Some(position) = &req.position {
        conn.execute("UPDATE users SET position = ?1, updated_at = datetime('now') WHERE id = ?2", rusqlite::params![position, user.user_id]).ok();
    }
    if let Some(avatar_url) = &req.avatar_url {
        conn.execute("UPDATE users SET avatar_url = ?1, updated_at = datetime('now') WHERE id = ?2", rusqlite::params![avatar_url, user.user_id]).ok();
    }

    match crate::commands::auth::get_user_profile_and_membership_public(&conn, &user.user_id) {
        Ok((profile, membership)) => {
            (StatusCode::OK, ApiResponse::ok(serde_json::json!({ "user": profile, "membership": membership })))
        }
        Err(e) => ApiResponse::err_with_status(StatusCode::INTERNAL_SERVER_ERROR, e),
    }
}

/// Share a project
pub async fn share_project(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(project_id): Path<String>,
    Json(req): Json<ShareProjectRequest>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    if !["read", "write", "admin"].contains(&req.permission.as_str()) {
        return ApiResponse::err_with_status(StatusCode::BAD_REQUEST, "Invalid permission. Must be 'read', 'write', or 'admin'");
    }
    if req.shared_with_name.trim().is_empty() {
        return ApiResponse::err_with_status(StatusCode::BAD_REQUEST, "Member name cannot be empty");
    }

    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    let existing: Result<String, _> = conn.query_row(
        "SELECT id FROM project_shares WHERE project_id = ?1 AND shared_with_name = ?2",
        rusqlite::params![project_id, req.shared_with_name],
        |row| row.get(0),
    );
    if existing.is_ok() {
        return ApiResponse::err_with_status(StatusCode::CONFLICT, format!("Project already shared with '{}'", req.shared_with_name));
    }

    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();

    if let Err(e) = conn.execute(
        "INSERT INTO project_shares (id, project_id, shared_with_name, permission, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![id, project_id, req.shared_with_name, req.permission, now],
    ) {
        return ApiResponse::err(format!("Failed to share project: {}", e));
    }

    (StatusCode::CREATED, ApiResponse::ok(serde_json::json!({
        "id": id, "project_id": project_id, "shared_with_name": req.shared_with_name,
        "permission": req.permission, "created_at": now,
    })))
}

/// List project shares
pub async fn list_shares(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(project_id): Path<String>,
) -> (StatusCode, Json<ApiResponse<Vec<serde_json::Value>>>) {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    let mut stmt = match conn.prepare(
        "SELECT id, project_id, shared_with_name, permission, created_at FROM project_shares WHERE project_id = ?1"
    ) {
        Ok(s) => s,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    let shares: Vec<serde_json::Value> = match stmt.query_map(rusqlite::params![project_id], |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, String>(0)?, "project_id": row.get::<_, String>(1)?,
            "shared_with_name": row.get::<_, String>(2)?, "permission": row.get::<_, String>(3)?,
            "created_at": row.get::<_, String>(4)?,
        }))
    }) {
        Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    (StatusCode::OK, ApiResponse::ok(shares))
}

/// List API keys
pub async fn list_api_keys(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
) -> (StatusCode, Json<ApiResponse<Vec<serde_json::Value>>>) {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    let mut stmt = match conn.prepare(
        "SELECT id, user_id, name, key_prefix, permissions, call_count, last_used_at, created_at, expires_at FROM api_keys WHERE user_id = ?1"
    ) {
        Ok(s) => s,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    let keys: Vec<serde_json::Value> = match stmt.query_map(rusqlite::params![user.user_id], |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, String>(0)?, "user_id": row.get::<_, String>(1)?,
            "name": row.get::<_, String>(2)?, "key_prefix": row.get::<_, String>(3)?,
            "permissions": row.get::<_, String>(4)?, "call_count": row.get::<_, i64>(5)?,
            "last_used_at": row.get::<_, Option<String>>(6)?, "created_at": row.get::<_, String>(7)?,
            "expires_at": row.get::<_, Option<String>>(8)?,
        }))
    }) {
        Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    (StatusCode::OK, ApiResponse::ok(keys))
}

/// Create an API key
pub async fn create_api_key(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Json(req): Json<CreateApiKeyRequest>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let permissions = req.permissions.unwrap_or_else(|| "read".to_string());

    let random_bytes: [u8; 16] = rand::thread_rng().gen();
    let raw_key = format!("caelab_{}", hex::encode(random_bytes));
    let key_prefix = raw_key[..12].to_string();
    let key_hash = crate::commands::auth::hash_token_public(&raw_key);

    let expires_at = req.expires_days.map(|d| {
        chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(d))
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_else(|| now.clone())
    });

    if let Err(e) = conn.execute(
        "INSERT INTO api_keys (id, user_id, name, key_hash, key_prefix, permissions, call_count, created_at, expires_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0, ?7, ?8)",
        rusqlite::params![id, user.user_id, req.name, key_hash, key_prefix, permissions, now, expires_at],
    ) {
        return ApiResponse::err(format!("Failed to create API key: {}", e));
    }

    (StatusCode::CREATED, ApiResponse::ok(serde_json::json!({
        "id": id, "name": req.name, "key": raw_key, "key_prefix": key_prefix,
        "permissions": permissions, "call_count": 0, "created_at": now,
        "expires_at": expires_at,
        "warning": "Save this API key now. You will not be able to see it again.",
    })))
}

/// Delete an API key
pub async fn delete_api_key(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<String>>) {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return ApiResponse::err(e.to_string()),
    };
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(e) => return ApiResponse::err(e.to_string()),
    };

    match conn.execute(
        "DELETE FROM api_keys WHERE id = ?1 AND user_id = ?2",
        rusqlite::params![id, user.user_id],
    ) {
        Ok(rows) if rows > 0 => (StatusCode::OK, ApiResponse::ok("API key deleted".to_string())),
        _ => ApiResponse::err_with_status(StatusCode::NOT_FOUND, "API key not found".to_string()),
    }
}
