use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

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
                category TEXT DEFAULT '未分类',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // Migration: add category column if it doesn't exist (for existing databases)
        let has_category: Result<bool, _> = conn
            .prepare("SELECT COUNT(*) FROM pragma_table_info('project_files') WHERE name = 'category'")
            .and_then(|mut stmt| {
                stmt.query_row([], |row| {
                    let count: i32 = row.get(0)?;
                    Ok(count > 0)
                })
            });
        if has_category.unwrap_or(false) == false {
            conn.execute(
                "ALTER TABLE project_files ADD COLUMN category TEXT DEFAULT '未分类'",
                [],
            ).ok();
        }

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

        // CFD settings table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS cfd_settings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id INTEGER NOT NULL UNIQUE,
                config_json TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        // CFD results stats table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS cfd_stats (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id INTEGER NOT NULL UNIQUE,
                iterations INTEGER,
                converged INTEGER,
                cl REAL,
                cd REAL,
                cm REAL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        // Embed records table (embedded targets in notes)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS embed_records (
                id TEXT PRIMARY KEY,
                note_id TEXT NOT NULL,
                target_type TEXT NOT NULL,
                target_id TEXT NOT NULL,
                target_name TEXT NOT NULL,
                config TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY (note_id) REFERENCES project_files(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // Index for embed records lookups by note_id
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_embed_records_note_id ON embed_records(note_id)",
            [],
        )?;

        // ========== Auth tables ==========

        // Users table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                nickname TEXT,
                avatar_url TEXT,
                company TEXT,
                position TEXT,
                membership_tier TEXT NOT NULL DEFAULT 'free',
                membership_expires_at TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now')),
                last_login_at TEXT
            )",
            [],
        )?;

        // Verification codes table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS verification_codes (
                id TEXT PRIMARY KEY,
                email TEXT NOT NULL,
                code TEXT NOT NULL,
                purpose TEXT NOT NULL DEFAULT 'register',
                expires_at TEXT NOT NULL,
                used_at TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            [],
        )?;

        // Devices table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS devices (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL REFERENCES users(id),
                device_name TEXT,
                device_type TEXT,
                ip_address TEXT,
                last_active_at TEXT NOT NULL DEFAULT (datetime('now')),
                is_current BOOLEAN DEFAULT 0,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            [],
        )?;

        // Refresh tokens table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS refresh_tokens (
                token_hash TEXT PRIMARY KEY,
                user_id TEXT NOT NULL REFERENCES users(id),
                device_id TEXT,
                expires_at TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            [],
        )?;

        // Index for devices by user_id
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_devices_user_id ON devices(user_id)",
            [],
        )?;

        // Index for refresh_tokens by user_id
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_refresh_tokens_user_id ON refresh_tokens(user_id)",
            [],
        )?;

        // ========== Collaboration tables ==========

        // Project shares table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS project_shares (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                shared_with_name TEXT NOT NULL,
                permission TEXT NOT NULL DEFAULT 'read',
                created_at TEXT NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // Comments table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS comments (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                note_id TEXT,
                author_name TEXT NOT NULL,
                content TEXT NOT NULL,
                mention_ids TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                resolved INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // Index for project_shares by project_id
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_project_shares_project_id ON project_shares(project_id)",
            [],
        )?;

        // Index for comments by project_id
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_comments_project_id ON comments(project_id)",
            [],
        )?;

        // Index for comments by note_id
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_comments_note_id ON comments(note_id)",
            [],
        )?;

        // ========== API Keys table ==========

        conn.execute(
            "CREATE TABLE IF NOT EXISTS api_keys (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                name TEXT NOT NULL,
                key_hash TEXT NOT NULL,
                key_prefix TEXT NOT NULL,
                permissions TEXT NOT NULL DEFAULT 'read',
                call_count INTEGER NOT NULL DEFAULT 0,
                last_used_at TEXT,
                created_at TEXT NOT NULL,
                expires_at TEXT,
                FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // Index for api_keys by user_id
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_api_keys_user_id ON api_keys(user_id)",
            [],
        )?;

        Ok(())
    }

    /// Initialize built-in materials
    fn init_builtin_materials(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        // Built-in materials data: (name, description, E_GPa, nu, density_kg_m3, alpha_1_K, sy_MPa, su_MPa)
        let builtin_materials: Vec<(&str, &str, f64, f64, f64, f64, f64, Option<f64>)> = vec![
            // ========== 结构钢 (10种) ==========
            ("Q235", "Chinese standard structural steel - GB/T 700, low carbon", 206.0, 0.30, 7850.0, 1.2e-5, 235.0, Some(375.0)),
            ("Q345", "Chinese standard structural steel - GB/T 1591, low alloy", 206.0, 0.30, 7850.0, 1.2e-5, 345.0, Some(470.0)),
            ("Q420", "Chinese standard structural steel - GB/T 1591, high strength", 206.0, 0.30, 7850.0, 1.2e-5, 420.0, Some(520.0)),
            ("Q460", "Chinese standard structural steel - GB/T 1591, ultra high strength", 206.0, 0.30, 7850.0, 1.2e-5, 460.0, Some(550.0)),
            ("S235", "European structural steel - EN 10025, non-alloy", 210.0, 0.30, 7850.0, 1.2e-5, 235.0, Some(360.0)),
            ("S355", "European structural steel - EN 10025, non-alloy", 210.0, 0.30, 7850.0, 1.2e-5, 355.0, Some(470.0)),
            ("S420", "European structural steel - EN 10025, non-alloy", 210.0, 0.30, 7850.0, 1.2e-5, 420.0, Some(520.0)),
            ("A36", "American structural steel - ASTM A36, carbon steel", 200.0, 0.26, 7850.0, 1.2e-5, 250.0, Some(400.0)),
            ("A572-Gr50", "American high-strength low-alloy - ASTM A572 Grade 50", 200.0, 0.30, 7850.0, 1.2e-5, 345.0, Some(450.0)),
            ("A992", "American structural steel - ASTM A992, wide-flange shapes", 200.0, 0.30, 7850.0, 1.2e-5, 345.0, Some(450.0)),

            // ========== 不锈钢 (5种) ==========
            ("SS 304", "Austenitic stainless steel - AISI 304, general purpose", 193.0, 0.29, 8000.0, 1.73e-5, 215.0, Some(505.0)),
            ("SS 316", "Austenitic stainless steel - AISI 316, marine grade", 193.0, 0.29, 8000.0, 1.6e-5, 205.0, Some(515.0)),
            ("SS 316L", "Austenitic stainless steel - AISI 316L, low carbon", 193.0, 0.29, 8000.0, 1.6e-5, 170.0, Some(485.0)),
            ("SS 430", "Ferritic stainless steel - AISI 430, decorative", 200.0, 0.28, 7800.0, 1.04e-5, 205.0, Some(450.0)),
            ("SS 2205", "Duplex stainless steel - UNS S32205, high strength", 200.0, 0.30, 7820.0, 1.3e-5, 450.0, Some(620.0)),

            // ========== 铝合金 (5种) ==========
            ("AA 6061-T6", "Aluminum alloy - precipitation hardening, general use", 68.9, 0.33, 2700.0, 2.36e-5, 276.0, Some(310.0)),
            ("AA 7075-T6", "Aluminum alloy - highest strength, aerospace", 71.7, 0.33, 2810.0, 2.38e-5, 503.0, Some(572.0)),
            ("AA 2024-T3", "Aluminum alloy - high strength, aerospace", 73.1, 0.33, 2780.0, 2.32e-5, 345.0, Some(483.0)),
            ("AA 5052-H32", "Aluminum alloy - non-heat-treatable, corrosion resistant", 70.3, 0.33, 2680.0, 2.38e-5, 193.0, Some(255.0)),
            ("AA 1100", "Commercially pure aluminum - high corrosion resistance", 69.0, 0.33, 2710.0, 2.36e-5, 34.0, Some(90.0)),

            // ========== 钛合金 (3种) ==========
            ("Ti-6Al-4V", "Titanium alloy - alpha-beta, most widely used titanium", 113.8, 0.34, 4430.0, 8.6e-6, 880.0, Some(950.0)),
            ("Ti-6Al-7Nb", "Titanium alloy - biomedical grade, nickel-free", 105.0, 0.34, 4480.0, 8.5e-6, 795.0, Some(860.0)),
            ("CP-Ti Grade 2", "Commercially pure titanium - unalloyed, Grade 2", 105.0, 0.34, 4510.0, 8.4e-6, 275.0, Some(345.0)),

            // ========== 铜合金 (4种) ==========
            ("C11000 Copper", "Pure copper - high electrical/thermal conductivity", 117.0, 0.34, 8960.0, 1.68e-5, 33.3, Some(210.0)),
            ("C36000 Brass", "Free-cutting brass - high machinability", 97.0, 0.34, 8500.0, 2.05e-5, 124.0, Some(310.0)),
            ("C17200 Beryllium Cu", "Beryllium copper - high strength, spring alloy", 131.0, 0.30, 8250.0, 1.67e-5, 1170.0, Some(1310.0)),
            ("C51000 Phosphor Bronze", "Phosphor bronze - fatigue resistant, spring", 110.0, 0.34, 8800.0, 1.78e-5, 276.0, Some(460.0)),

            // ========== 高温合金 (3种) ==========
            ("Inconel 718", "Nickel superalloy - high temp, precipitation hardened", 200.0, 0.30, 8190.0, 1.3e-5, 1035.0, Some(1240.0)),
            ("Inconel 625", "Nickel superalloy - excellent corrosion resistance", 205.0, 0.31, 8440.0, 1.28e-5, 414.0, Some(827.0)),
            ("Hastelloy C-276", "Nickel-molybdenum alloy - extreme corrosion resistance", 205.0, 0.31, 8890.0, 1.16e-5, 283.0, Some(655.0)),

            // ========== 聚合物 (5种) ==========
            ("ABS", "Acrylonitrile Butadiene Styrene - thermoplastic", 2.3, 0.35, 1040.0, 9.0e-5, 43.0, Some(41.0)),
            ("Polycarbonate (PC)", "Polycarbonate - transparent, impact resistant", 2.4, 0.38, 1200.0, 6.5e-5, 62.0, Some(66.0)),
            ("Nylon 6/6", "Polyamide 6/6 - engineering thermoplastic", 2.9, 0.39, 1140.0, 8.0e-5, 82.7, Some(82.7)),
            ("PEEK", "Polyetheretherketone - high performance polymer", 3.6, 0.38, 1310.0, 4.7e-5, 91.0, Some(100.0)),
            ("PTFE (Teflon)", "Polytetrafluoroethylene - chemically inert", 0.5, 0.46, 2200.0, 1.0e-4, 20.0, Some(25.0)),

            // ========== 复合材料 (3种) ==========
            ("CFRP (UD)", "Carbon Fiber Reinforced Polymer - unidirectional, fiber direction", 135.0, 0.28, 1600.0, 0.2e-5, 600.0, Some(800.0)),
            ("GFRP (UD)", "Glass Fiber Reinforced Polymer - unidirectional, fiber direction", 40.0, 0.25, 2000.0, 0.6e-5, 500.0, Some(700.0)),
            ("Kevlar 49 (UD)", "Aramid fiber composite - unidirectional, fiber direction", 76.0, 0.34, 1440.0, -2.0e-6, 360.0, Some(480.0)),

            // ========== 混凝土/石材 (3种) ==========
            ("C30 Concrete", "Concrete - 30 MPa compressive strength", 30.0, 0.20, 2400.0, 1.0e-5, 30.0, Some(3.0)),
            ("C50 Concrete", "Concrete - 50 MPa compressive strength", 35.0, 0.20, 2400.0, 1.0e-5, 50.0, Some(4.0)),
            ("Granite", "Natural granite - igneous rock", 60.0, 0.25, 2650.0, 0.8e-5, 150.0, Some(10.0)),

            // ========== 木材 (2种) ==========
            ("Oak", "White oak - hardwood, structural", 12.5, 0.35, 600.0, 3.0e-6, 40.0, Some(45.0)),
            ("Pine", "Southern pine - softwood, structural", 12.0, 0.35, 510.0, 4.0e-6, 35.0, Some(40.0)),

            // ========== 橡胶 (2种) ==========
            ("Natural Rubber", "Natural rubber (vulcanized) - hyperelastic", 0.05, 0.49, 920.0, 2.2e-4, 15.0, Some(25.0)),
            ("Silicone Rubber", "Silicone rubber - high temperature range", 0.01, 0.49, 1150.0, 3.1e-4, 6.0, Some(8.0)),

            // ========== 其他 (5种) ==========
            ("GG-25 Cast Iron", "Gray cast iron - EN-GJL-250, high damping", 100.0, 0.26, 7200.0, 1.05e-5, 250.0, Some(350.0)),
            ("AZ31B Magnesium", "Magnesium alloy - lightweight structural", 45.0, 0.35, 1770.0, 2.6e-5, 200.0, Some(260.0)),
            ("ZA-27 Zinc Alloy", "Zinc-aluminum alloy - high strength casting", 78.0, 0.30, 5100.0, 2.5e-5, 370.0, Some(425.0)),
            ("Lead", "Pure lead - radiation shielding", 16.0, 0.44, 11340.0, 2.9e-5, 12.0, Some(18.0)),
            ("Soda-Lime Glass", "Common glass - transparent, brittle", 70.0, 0.22, 2500.0, 0.85e-5, 50.0, Some(33.0)),
        ];

        let now = chrono::Utc::now().to_rfc3339();
        let material_count = builtin_materials.len();
        
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

        tracing::info!("Built-in materials initialized ({} materials)", material_count);
        Ok(())
    }

    /// Get database instance from app handle
    pub fn get_handle(app: &AppHandle) -> tauri::State<'_, Database> {
        app.state::<Database>()
    }
}