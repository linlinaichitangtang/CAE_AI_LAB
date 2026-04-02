use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hmac::{Hmac, Mac};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

// ============================================================
// Data structures
// ============================================================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserProfile,
    pub membership: MembershipStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserProfile {
    pub id: String,
    pub email: String,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub company: Option<String>,
    pub position: Option<String>,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MembershipStatus {
    pub tier: String,
    pub expires_at: Option<String>,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeviceInfo {
    pub id: String,
    pub device_name: String,
    pub device_type: String,
    pub ip_address: String,
    pub last_active_at: String,
    pub is_current: bool,
}

// ============================================================
// JWT helpers
// ============================================================

/// JWT secret key (in production, load from config / env)
const JWT_SECRET: &[u8] = b"caelab_auth_secret_key_v0_10_change_in_prod";

/// Access token expiry: 15 minutes
const ACCESS_TOKEN_EXPIRY_SECS: u64 = 15 * 60;

/// Refresh token expiry: 7 days
const REFRESH_TOKEN_EXPIRY_SECS: u64 = 7 * 24 * 60 * 60;

fn base64_encode(input: &str) -> String {
    BASE64.encode(input.as_bytes())
}

fn base64_decode(input: &str) -> Result<String, String> {
    let bytes = BASE64
        .decode(input)
        .map_err(|e| format!("base64 decode error: {}", e))?;
    String::from_utf8(bytes).map_err(|e| format!("utf8 decode error: {}", e))
}

fn generate_access_token(user_id: &str, secret: &[u8]) -> Result<String, String> {
    let header = base64_encode(r#"{"alg":"HS256","typ":"JWT"}"#);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let exp = now + ACCESS_TOKEN_EXPIRY_SECS;
    let payload = base64_encode(&format!(
        r#"{{"sub":"{}","exp":{},"iat":{},"type":"access"}}"#,
        user_id, exp, now
    ));

    let mut mac = HmacSha256::new_from_slice(secret).map_err(|e| e.to_string())?;
    mac.update(format!("{}.{}", header, payload).as_bytes());
    let signature = BASE64.encode(mac.finalize().into_bytes());

    Ok(format!("{}.{}.{}", header, payload, signature))
}

fn generate_refresh_token(user_id: &str, secret: &[u8]) -> Result<String, String> {
    let header = base64_encode(r#"{"alg":"HS256","typ":"JWT"}"#);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let exp = now + REFRESH_TOKEN_EXPIRY_SECS;
    let payload = base64_encode(&format!(
        r#"{{"sub":"{}","exp":{},"iat":{},"type":"refresh"}}"#,
        user_id, exp, now
    ));

    let mut mac = HmacSha256::new_from_slice(secret).map_err(|e| e.to_string())?;
    mac.update(format!("{}.{}", header, payload).as_bytes());
    let signature = BASE64.encode(mac.finalize().into_bytes());

    Ok(format!("{}.{}.{}", header, payload, signature))
}

fn verify_token(token: &str, secret: &[u8], expected_type: &str) -> Result<String, String> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err("Invalid token format".to_string());
    }

    let (header, payload, signature) = (parts[0], parts[1], parts[2]);

    // Verify signature
    let mut mac = HmacSha256::new_from_slice(secret).map_err(|e| e.to_string())?;
    mac.update(format!("{}.{}", header, payload).as_bytes());
    mac.verify_slice(
        &BASE64
            .decode(signature)
            .map_err(|e| format!("base64 decode signature error: {}", e))?,
    )
    .map_err(|_| "Invalid token signature".to_string())?;

    // Decode payload
    let payload_str = base64_decode(payload)?;
    let payload_json: serde_json::Value =
        serde_json::from_str(&payload_str).map_err(|e| format!("Invalid payload JSON: {}", e))?;

    // Check token type
    let token_type = payload_json
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if token_type != expected_type {
        return Err(format!(
            "Invalid token type: expected {}, got {}",
            expected_type, token_type
        ));
    }

    // Check expiration
    let exp = payload_json
        .get("exp")
        .and_then(|v| v.as_u64())
        .ok_or("Missing exp claim")?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    if now > exp {
        return Err("Token has expired".to_string());
    }

    // Extract user_id
    let user_id = payload_json
        .get("sub")
        .and_then(|v| v.as_str())
        .ok_or("Missing sub claim")?
        .to_string();

    Ok(user_id)
}

/// Hash a refresh token for storage (SHA-256)
fn hash_token(token: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(token.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

// ============================================================
// Validation helpers
// ============================================================

fn validate_email(email: &str) -> Result<(), String> {
    if email.is_empty() {
        return Err("Email cannot be empty".to_string());
    }
    // Basic email regex
    let re = regex::Regex::new(
        r"^[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}$",
    )
    .map_err(|e| format!("Regex error: {}", e))?;
    if !re.is_match(email) {
        return Err("Invalid email format".to_string());
    }
    Ok(())
}

fn validate_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("Password must be at least 8 characters".to_string());
    }
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    if !has_upper {
        return Err("Password must contain at least one uppercase letter".to_string());
    }
    if !has_lower {
        return Err("Password must contain at least one lowercase letter".to_string());
    }
    if !has_digit {
        return Err("Password must contain at least one digit".to_string());
    }
    Ok(())
}

// ============================================================
// Helper: build MembershipStatus
// ============================================================

fn build_membership_status(
    tier: &str,
    expires_at: Option<String>,
) -> MembershipStatus {
    let is_active = if tier == "free" {
        true
    } else {
        expires_at.as_ref().map_or(false, |exp| {
            chrono::DateTime::parse_from_rfc3339(exp)
                .map_or(false, |dt| dt > chrono::Utc::now())
        })
    };
    MembershipStatus {
        tier: tier.to_string(),
        expires_at,
        is_active,
    }
}

fn get_user_profile_and_membership(
    conn: &rusqlite::Connection,
    user_id: &str,
) -> Result<(UserProfile, MembershipStatus), String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, email, nickname, avatar_url, company, position, created_at,
                    membership_tier, membership_expires_at
             FROM users WHERE id = ?1",
        )
        .map_err(|e| e.to_string())?;

    let result = stmt
        .query_row(rusqlite::params![user_id], |row| {
            let profile = UserProfile {
                id: row.get(0)?,
                email: row.get(1)?,
                nickname: row.get(2)?,
                avatar_url: row.get(3)?,
                company: row.get(4)?,
                position: row.get(5)?,
                created_at: row.get(6)?,
            };
            let tier: String = row.get(7).unwrap_or_else(|_| "free".to_string());
            let expires_at: Option<String> = row.get(8).unwrap_or(None);
            let membership = build_membership_status(&tier, expires_at);
            Ok::<(UserProfile, MembershipStatus), rusqlite::Error>((profile, membership))
        })
        .map_err(|e| e.to_string())?;

    Ok(result)
}

// ============================================================
// Tauri commands
// ============================================================

/// Register a new user
#[tauri::command]
pub async fn register(
    email: String,
    password: String,
    nickname: Option<String>,
    db: tauri::State<'_, crate::db::Database>,
) -> Result<AuthResponse, String> {
    // 1. Validate email
    validate_email(&email)?;

    // 2. Validate password strength
    validate_password(&password)?;

    // 3. Hash password
    let password_hash = bcrypt::hash(&password, bcrypt::DEFAULT_COST)
        .map_err(|e| format!("Password hashing error: {}", e))?;

    // 4. Generate user ID
    let user_id = nanoid::nanoid!(21);

    // 5. Insert into database
    let conn = db.conn.lock().unwrap();

    // Check if email already exists
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM users WHERE email = ?1",
            rusqlite::params![email],
            |row| row.get::<_, i32>(0),
        )
        .map(|count| count > 0)
        .unwrap_or(false);

    if exists {
        return Err("Email already registered".to_string());
    }

    conn.execute(
        "INSERT INTO users (id, email, password_hash, nickname, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, datetime('now'), datetime('now'))",
        rusqlite::params![user_id, email, password_hash, nickname],
    )
    .map_err(|e| format!("Failed to create user: {}", e))?;

    // 6. Generate JWT tokens
    let access_token = generate_access_token(&user_id, JWT_SECRET)?;
    let refresh_token = generate_refresh_token(&user_id, JWT_SECRET)?;

    // 7. Store refresh token hash
    let refresh_token_hash = hash_token(&refresh_token);

    conn.execute(
        "INSERT INTO refresh_tokens (token_hash, user_id, expires_at, created_at)
         VALUES (?1, ?2, datetime('now', '+7 days'), datetime('now'))",
        rusqlite::params![refresh_token_hash, user_id],
    )
    .map_err(|e| format!("Failed to store refresh token: {}", e))?;

    drop(conn);

    // 8. Build response
    let conn = db.conn.lock().unwrap();
    let (user, membership) = get_user_profile_and_membership(&conn, &user_id)?;

    tracing::info!("User registered: {} ({})", email, user_id);

    Ok(AuthResponse {
        access_token,
        refresh_token,
        user,
        membership,
    })
}

/// Login with email and password
#[tauri::command]
pub async fn login(
    email: String,
    password: String,
    db: tauri::State<'_, crate::db::Database>,
) -> Result<AuthResponse, String> {
    let conn = db.conn.lock().unwrap();

    // 1. Find user by email
    let (user_id, password_hash): (String, String) = conn
        .query_row(
            "SELECT id, password_hash FROM users WHERE email = ?1",
            rusqlite::params![email],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|_| "Invalid email or password".to_string())?;

    // 2. Verify password
    let valid = bcrypt::verify(&password, &password_hash)
        .map_err(|e| format!("Password verification error: {}", e))?;

    if !valid {
        return Err("Invalid email or password".to_string());
    }

    // 3. Update last_login_at
    conn.execute(
        "UPDATE users SET last_login_at = datetime('now'), updated_at = datetime('now') WHERE id = ?1",
        rusqlite::params![user_id],
    )
    .map_err(|e| e.to_string())?;

    // 4. Generate JWT tokens
    let access_token = generate_access_token(&user_id, JWT_SECRET)?;
    let refresh_token = generate_refresh_token(&user_id, JWT_SECRET)?;

    // 5. Store refresh token hash
    let refresh_token_hash = hash_token(&refresh_token);
    conn.execute(
        "INSERT INTO refresh_tokens (token_hash, user_id, expires_at, created_at)
         VALUES (?1, ?2, datetime('now', '+7 days'), datetime('now'))",
        rusqlite::params![refresh_token_hash, user_id],
    )
    .map_err(|e| format!("Failed to store refresh token: {}", e))?;

    // 6. Record device (simulated)
    let device_id = nanoid::nanoid!(21);
    conn.execute(
        "INSERT INTO devices (id, user_id, device_name, device_type, ip_address, is_current, created_at, last_active_at)
         VALUES (?1, ?2, ?3, ?4, ?5, 1, datetime('now'), datetime('now'))",
        rusqlite::params![device_id, user_id, "Local Device", "Desktop", "127.0.0.1"],
    )
    .map_err(|e| format!("Failed to record device: {}", e))?;

    // 7. Build response
    let (user, membership) = get_user_profile_and_membership(&conn, &user_id)?;

    tracing::info!("User logged in: {} ({})", email, user_id);

    Ok(AuthResponse {
        access_token,
        refresh_token,
        user,
        membership,
    })
}

/// Refresh access token using refresh token
#[tauri::command]
pub async fn refresh_token(
    refresh_token: String,
    db: tauri::State<'_, crate::db::Database>,
) -> Result<AuthResponse, String> {
    // 1. Verify refresh token and extract user_id
    let user_id = verify_token(&refresh_token, JWT_SECRET, "refresh")?;

    let conn = db.conn.lock().unwrap();

    // 2. Check if refresh token hash exists in database and is not expired
    let refresh_token_hash = hash_token(&refresh_token);
    let token_valid: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM refresh_tokens
             WHERE token_hash = ?1 AND user_id = ?2 AND expires_at > datetime('now')",
            rusqlite::params![refresh_token_hash, user_id],
            |row| row.get::<_, i32>(0),
        )
        .map(|count| count > 0)
        .unwrap_or(false);

    if !token_valid {
        return Err("Invalid or expired refresh token".to_string());
    }

    // 3. Delete old refresh token (one-time use)
    conn.execute(
        "DELETE FROM refresh_tokens WHERE token_hash = ?1",
        rusqlite::params![refresh_token_hash],
    )
    .map_err(|e| e.to_string())?;

    // 4. Generate new tokens
    let new_access_token = generate_access_token(&user_id, JWT_SECRET)?;
    let new_refresh_token = generate_refresh_token(&user_id, JWT_SECRET)?;

    // 5. Store new refresh token hash
    let new_refresh_token_hash = hash_token(&new_refresh_token);
    conn.execute(
        "INSERT INTO refresh_tokens (token_hash, user_id, expires_at, created_at)
         VALUES (?1, ?2, datetime('now', '+7 days'), datetime('now'))",
        rusqlite::params![new_refresh_token_hash, user_id],
    )
    .map_err(|e| format!("Failed to store refresh token: {}", e))?;

    // 6. Build response
    let (user, membership) = get_user_profile_and_membership(&conn, &user_id)?;

    tracing::info!("Token refreshed for user: {}", user_id);

    Ok(AuthResponse {
        access_token: new_access_token,
        refresh_token: new_refresh_token,
        user,
        membership,
    })
}

/// Get user profile
#[tauri::command]
pub async fn get_profile(
    user_id: String,
    db: tauri::State<'_, crate::db::Database>,
) -> Result<UserProfile, String> {
    let conn = db.conn.lock().unwrap();
    let (profile, _) = get_user_profile_and_membership(&conn, &user_id)?;
    Ok(profile)
}

/// Update user profile
#[tauri::command]
pub async fn update_profile(
    user_id: String,
    nickname: Option<String>,
    company: Option<String>,
    position: Option<String>,
    avatar_url: Option<String>,
    db: tauri::State<'_, crate::db::Database>,
) -> Result<UserProfile, String> {
    let conn = db.conn.lock().unwrap();

    // Build dynamic UPDATE statement
    let mut updates = vec!["updated_at = datetime('now')".to_string()];
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = vec![];

    if let Some(ref nick) = nickname {
        updates.push("nickname = ?".to_string());
        params.push(Box::new(nick.clone()));
    }
    if let Some(ref comp) = company {
        updates.push("company = ?".to_string());
        params.push(Box::new(comp.clone()));
    }
    if let Some(ref pos) = position {
        updates.push("position = ?".to_string());
        params.push(Box::new(pos.clone()));
    }
    if let Some(ref avatar) = avatar_url {
        updates.push("avatar_url = ?".to_string());
        params.push(Box::new(avatar.clone()));
    }

    if updates.len() <= 1 {
        // No fields to update
        let (profile, _) = get_user_profile_and_membership(&conn, &user_id)?;
        return Ok(profile);
    }

    params.push(Box::new(user_id.clone()));
    let sql = format!(
        "UPDATE users SET {} WHERE id = ?",
        updates.join(", ")
    );

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    conn.execute(&sql, param_refs.as_slice())
        .map_err(|e| format!("Failed to update profile: {}", e))?;

    let (profile, _) = get_user_profile_and_membership(&conn, &user_id)?;

    tracing::info!("Profile updated for user: {}", user_id);
    Ok(profile)
}

/// Change password
#[tauri::command]
pub async fn change_password(
    user_id: String,
    old_password: String,
    new_password: String,
    db: tauri::State<'_, crate::db::Database>,
) -> Result<(), String> {
    // Validate new password
    validate_password(&new_password)?;

    let conn = db.conn.lock().unwrap();

    // Get current password hash
    let current_hash: String = conn
        .query_row(
            "SELECT password_hash FROM users WHERE id = ?1",
            rusqlite::params![user_id],
            |row| row.get(0),
        )
        .map_err(|_| "User not found".to_string())?;

    // Verify old password
    let valid = bcrypt::verify(&old_password, &current_hash)
        .map_err(|e| format!("Password verification error: {}", e))?;

    if !valid {
        return Err("Current password is incorrect".to_string());
    }

    // Hash new password
    let new_hash = bcrypt::hash(&new_password, bcrypt::DEFAULT_COST)
        .map_err(|e| format!("Password hashing error: {}", e))?;

    // Update password
    conn.execute(
        "UPDATE users SET password_hash = ?1, updated_at = datetime('now') WHERE id = ?2",
        rusqlite::params![new_hash, user_id],
    )
    .map_err(|e| format!("Failed to update password: {}", e))?;

    // Invalidate all refresh tokens for this user
    conn.execute(
        "DELETE FROM refresh_tokens WHERE user_id = ?1",
        rusqlite::params![user_id],
    )
    .map_err(|e| e.to_string())?;

    tracing::info!("Password changed for user: {}", user_id);
    Ok(())
}

/// List devices for a user
#[tauri::command]
pub async fn list_devices(
    user_id: String,
    db: tauri::State<'_, crate::db::Database>,
) -> Result<Vec<DeviceInfo>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT id, device_name, device_type, ip_address, last_active_at, is_current
             FROM devices WHERE user_id = ?1 ORDER BY last_active_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let devices = stmt
        .query_map(rusqlite::params![user_id], |row| {
            Ok(DeviceInfo {
                id: row.get(0)?,
                device_name: row.get::<_, Option<String>>(1).unwrap_or(None).unwrap_or_default(),
                device_type: row.get::<_, Option<String>>(2).unwrap_or(None).unwrap_or_default(),
                ip_address: row.get::<_, Option<String>>(3).unwrap_or(None).unwrap_or_default(),
                last_active_at: row.get::<_, Option<String>>(4).unwrap_or(None).unwrap_or_default(),
                is_current: row.get::<_, bool>(5).unwrap_or(false),
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(devices)
}

/// Logout a specific device
#[tauri::command]
pub async fn logout_device(
    user_id: String,
    device_id: String,
    db: tauri::State<'_, crate::db::Database>,
) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();

    // Delete the device
    let affected = conn
        .execute(
            "DELETE FROM devices WHERE id = ?1 AND user_id = ?2",
            rusqlite::params![device_id, user_id],
        )
        .map_err(|e| format!("Failed to logout device: {}", e))?;

    if affected == 0 {
        return Err("Device not found".to_string());
    }

    // Also delete associated refresh tokens for this device
    conn.execute(
        "DELETE FROM refresh_tokens WHERE device_id = ?1 AND user_id = ?2",
        rusqlite::params![device_id, user_id],
    )
    .ok();

    tracing::info!(
        "Device {} logged out for user: {}",
        device_id,
        user_id
    );
    Ok(())
}

/// Get membership status
#[tauri::command]
pub async fn get_membership_status(
    user_id: String,
    db: tauri::State<'_, crate::db::Database>,
) -> Result<MembershipStatus, String> {
    let conn = db.conn.lock().unwrap();
    let (_, membership) = get_user_profile_and_membership(&conn, &user_id)?;
    Ok(membership)
}

/// Update membership tier
#[tauri::command]
pub async fn update_membership(
    user_id: String,
    tier: String,
    expires_at: String,
    db: tauri::State<'_, crate::db::Database>,
) -> Result<MembershipStatus, String> {
    // Validate tier
    let valid_tiers = ["free", "pro", "enterprise"];
    if !valid_tiers.contains(&tier.as_str()) {
        return Err(format!(
            "Invalid membership tier: {}. Must be one of: free, pro, enterprise",
            tier
        ));
    }

    let conn = db.conn.lock().unwrap();

    // Check user exists
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM users WHERE id = ?1",
            rusqlite::params![user_id],
            |row| row.get::<_, i32>(0),
        )
        .map(|count| count > 0)
        .unwrap_or(false);

    if !exists {
        return Err("User not found".to_string());
    }

    // Update membership
    let expires_at_value: Option<String> = if tier == "free" {
        None
    } else {
        Some(expires_at)
    };

    conn.execute(
        "UPDATE users SET membership_tier = ?1, membership_expires_at = ?2, updated_at = datetime('now') WHERE id = ?3",
        rusqlite::params![tier, expires_at_value, user_id],
    )
    .map_err(|e| format!("Failed to update membership: {}", e))?;

    let (_, membership) = get_user_profile_and_membership(&conn, &user_id)?;

    tracing::info!(
        "Membership updated for user {}: tier={}",
        user_id,
        tier
    );
    Ok(membership)
}

/// Send verification code (simulated - logs to console)
#[tauri::command]
pub async fn send_verification_code(
    email: String,
    purpose: String,
    db: tauri::State<'_, crate::db::Database>,
) -> Result<(), String> {
    // Validate email
    validate_email(&email)?;

    // Validate purpose
    let valid_purposes = ["register", "reset_password", "change_email"];
    if !valid_purposes.contains(&purpose.as_str()) {
        return Err(format!(
            "Invalid purpose: {}. Must be one of: register, reset_password, change_email",
            purpose
        ));
    }

    // Generate 6-digit code
    let code: String = rand::thread_rng().gen_range(100_000..999_999).to_string();

    let code_id = nanoid::nanoid!(21);

    let conn = db.conn.lock().unwrap();

    // Invalidate previous codes for this email+purpose
    conn.execute(
        "UPDATE verification_codes SET used_at = datetime('now') WHERE email = ?1 AND purpose = ?2 AND used_at IS NULL",
        rusqlite::params![email, purpose],
    )
    .ok();

    // Insert new code
    conn.execute(
        "INSERT INTO verification_codes (id, email, code, purpose, expires_at, created_at)
         VALUES (?1, ?2, ?3, ?4, datetime('now', '+10 minutes'), datetime('now'))",
        rusqlite::params![code_id, email, code, purpose],
    )
    .map_err(|e| format!("Failed to store verification code: {}", e))?;

    // In production, send email here. For now, log the code.
    tracing::info!(
        "[SIMULATED] Verification code for {} ({}): {}",
        email,
        purpose,
        code
    );

    Ok(())
}

/// Verify a code
#[tauri::command]
pub async fn verify_code(
    email: String,
    code: String,
    purpose: String,
    db: tauri::State<'_, crate::db::Database>,
) -> Result<bool, String> {
    let conn = db.conn.lock().unwrap();

    // Find matching unused, non-expired code
    let result = conn.query_row(
        "SELECT id FROM verification_codes
         WHERE email = ?1 AND code = ?2 AND purpose = ?3
           AND used_at IS NULL AND expires_at > datetime('now')
         ORDER BY created_at DESC LIMIT 1",
        rusqlite::params![email, code, purpose],
        |row| row.get::<_, String>(0),
    );

    match result {
        Ok(code_id) => {
            // Mark as used
            conn.execute(
                "UPDATE verification_codes SET used_at = datetime('now') WHERE id = ?1",
                rusqlite::params![code_id],
            )
            .map_err(|e| e.to_string())?;

            tracing::info!(
                "Verification code verified for {} ({})",
                email,
                purpose
            );
            Ok(true)
        }
        Err(_) => {
            tracing::warn!(
                "Invalid verification code attempt for {} ({})",
                email,
                purpose
            );
            Ok(false)
        }
    }
}

/// Verify access token (utility command for frontend middleware)
#[tauri::command]
pub async fn verify_access_token_cmd(
    token: String,
) -> Result<String, String> {
    let user_id = verify_token(&token, JWT_SECRET, "access")?;
    Ok(user_id)
}
