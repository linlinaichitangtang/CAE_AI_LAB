use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::AppHandle;

/// Database manager for SQLite operations
pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    /// Initialize database with the given path
    pub fn new(db_path: PathBuf) -> Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        
        let conn = Connection::open(&db_path)?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.init_tables()?;
        db.init_builtin_materials()?;
        Ok(db)
    }

    /// Initialize database tables
    fn init_tables(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        // Projects table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        // Project files table (notes, modeling data, code files)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS project_files (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                file_type TEXT NOT NULL,
                file_name TEXT NOT NULL,
                content TEXT,
                file_path TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // Project settings table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS project_settings (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL UNIQUE,
                settings_json TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // Materials table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS materials (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                youngs_modulus REAL NOT NULL,
                poissons_ratio REAL NOT NULL,
                density REAL NOT NULL,
                thermal_expansion REAL NOT NULL,
                yield_strength REAL NOT NULL,
                ultimate_strength REAL,
                description TEXT,
                is_builtin INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    /// Initialize built-in materials
    fn init_builtin_materials(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        // Built-in materials data
        let builtin_materials = vec![
            ("Steel (Structural)", "Structural steel - carbon steel for general use", 200.0, 0.3, 7850.0, 1.2e-5, 250.0, Some(460.0)),
            ("Aluminum", "Aluminum alloy - lightweight and corrosion resistant", 70.0, 0.33, 2700.0, 2.3e-5, 270.0, Some(310.0)),
            ("Copper", "Pure copper - excellent electrical and thermal conductor", 110.0, 0.34, 8960.0, 1.7e-5, 33.0, Some(210.0)),
            ("Concrete", "Plain concrete - compressive strength 30MPa", 30.0, 0.2, 2400.0, 1.0e-5, 30.0, Some(3.0)),
            ("Titanium", "Titanium alloy - high strength, light weight", 110.0, 0.34, 4500.0, 8.6e-6, 880.0, Some(950.0)),
        ];

        let now = chrono::Utc::now().to_rfc3339();
        
        for (name, description, e, nu, density, alpha, sy, su) in builtin_materials {
            conn.execute(
                "INSERT OR IGNORE INTO materials 
                 (id, name, youngs_modulus, poissons_ratio, density, thermal_expansion, 
                  yield_strength, ultimate_strength, description, is_builtin, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 1, ?10, ?10)",
                rusqlite::params![
                    uuid::Uuid::new_v4().to_string(),
                    name,
                    e,
                    nu,
                    density,
                    alpha,
                    sy,
                    su,
                    description,
                    &now
                ],
            ).ok();
        }

        tracing::info!("Built-in materials initialized");
        Ok(())
    }

    /// Get database instance from app handle
    pub fn get_handle(app: &AppHandle) -> tauri::State<'_, Database> {
        app.state::<Database>()
    }
}