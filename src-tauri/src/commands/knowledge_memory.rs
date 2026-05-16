/**
 * V3.9 Agent 记忆系统 — Rust 后端命令
 * - 短期记忆: messageHistory + ContextCompressor (TypeScript 前端)
 * - 工作记忆: stateTracker + toolCallHistory (TypeScript 前端)
 * - 项目记忆: versionHistory + embedRecords (已有 simulation_archive)
 * - 跨会话记忆: UserProfile + ProjectHistory (本模块)
 * - 长期记忆: MaterialKnowledge + DesignStandard + FailureModeGraph (本模块)
 *
 * 核心能力:
 * - 材料知识库查询 (MaterialKnowledge CRUD)
 * - 设计规范库管理 (DesignStandard CRUD)
 * - 失效模式图谱查询 (FailureModeGraph CRUD)
 * - 用户画像管理 (UserProfile CRUD)
 * - 知识召回 (Knowledge Recall) — 基于关键词匹配
 */

use rusqlite::{Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};

// ============================================================================
// V3.9-001: 材料知识库 (MaterialKnowledge)
// ============================================================================

/// 材料知识记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MaterialKnowledgeRecord {
    pub id: String,
    pub material_id: String,
    pub name: String,
    pub category: String,
    pub elastic_modulus: f64,
    pub poissons_ratio: f64,
    pub density: f64,
    pub yield_strength: f64,
    pub ultimate_strength: Option<f64>,
    pub fatigue_params_json: Option<String>,
    pub thermal_params_json: Option<String>,
    pub mesh_guidelines_json: Option<String>,
    pub solver_settings_json: Option<String>,
    pub common_errors_json: Option<String>,
    pub last_analysis: Option<i64>,
    pub project_ids_json: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// 写入材料知识请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MaterialKnowledgeWriteRequest {
    pub material_id: String,
    pub name: String,
    pub category: String,
    pub elastic_modulus: f64,
    pub poissons_ratio: f64,
    pub density: f64,
    pub yield_strength: f64,
    pub ultimate_strength: Option<f64>,
    pub fatigue_params_json: Option<String>,
    pub thermal_params_json: Option<String>,
    pub mesh_guidelines_json: Option<String>,
    pub solver_settings_json: Option<String>,
    pub common_errors_json: Option<String>,
    pub project_ids_json: Option<String>,
    pub notes: Option<String>,
}

// ============================================================================
// V3.9-002: 设计规范库 (DesignStandard)
// ============================================================================

/// 设计规范记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DesignStandardRecord {
    pub id: String,
    pub name: String,
    pub version: String,
    pub source: String,
    pub scope: String,
    pub safety_factor: f64,
    pub allowable_stress: Option<f64>,
    pub applicable_load_cases_json: Option<String>,
    pub rules_json: Option<String>,
    pub industry: Option<String>,
    pub issue_date: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// 写入设计规范请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DesignStandardWriteRequest {
    pub name: String,
    pub version: String,
    pub source: String,
    pub scope: String,
    pub safety_factor: f64,
    pub allowable_stress: Option<f64>,
    pub applicable_load_cases_json: Option<String>,
    pub rules_json: Option<String>,
    pub industry: Option<String>,
    pub issue_date: Option<String>,
    pub notes: Option<String>,
}

// ============================================================================
// V3.9-003: 失效模式图谱 (FailureModeGraph)
// ============================================================================

/// 失效模式记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FailureModeRecord {
    pub id: String,
    pub mode_type: String,
    pub description: String,
    pub applicable_materials_json: Option<String>,
    pub temperature_range_json: Option<String>,
    pub mechanisms_json: String,
    pub uncertainty: String,
    pub related_standards_json: Option<String>,
    pub case_study_ids_json: Option<String>,
    pub priority: i32,
    pub created_at: String,
    pub updated_at: String,
}

/// 写入失效模式请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FailureModeWriteRequest {
    pub mode_type: String,
    pub description: String,
    pub applicable_materials_json: Option<String>,
    pub temperature_range_json: Option<String>,
    pub mechanisms_json: String,
    pub uncertainty: String,
    pub related_standards_json: Option<String>,
    pub case_study_ids_json: Option<String>,
    pub priority: i32,
}

// ============================================================================
// V3.9-004: 用户画像 (UserProfile - 跨会话记忆)
// ============================================================================

/// 用户偏好
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPreferenceRecord {
    pub unit_system: String,
    pub language: String,
    pub theme: Option<String>,
    pub frequent_materials_json: Option<String>,
    pub frequent_simulation_types_json: Option<String>,
    pub preferred_postprocessor: Option<String>,
}

/// 项目历史条目
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectHistoryRecord {
    pub project_id: String,
    pub project_name: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub last_accessed_at: i64,
    pub tags_json: Option<String>,
    pub completed_analyses: Option<i32>,
}

/// 用户画像记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfileRecord {
    pub user_id: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub company: Option<String>,
    pub position: Option<String>,
    pub preferences_json: String,
    pub project_history_json: String,
    pub expertise_tags_json: Option<String>,
    pub created_at: i64,
    pub last_login_at: i64,
}

/// 写入用户画像请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfileWriteRequest {
    pub user_id: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub company: Option<String>,
    pub position: Option<String>,
    pub preferences_json: String,
    pub project_history_json: String,
    pub expertise_tags_json: Option<String>,
}

// ============================================================================
// V3.9-005: 知识召回 (Knowledge Recall)
// ============================================================================

/// 知识召回请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KnowledgeRecallRequest {
    pub keywords: Vec<String>,
    pub context_type: Option<String>,
    pub project_id: Option<String>,
    pub simulation_type: Option<String>,
    pub limit: Option<i32>,
}

/// 知识召回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KnowledgeRecallResult {
    pub materials_json: Option<String>,
    pub standards_json: Option<String>,
    pub failure_modes_json: Option<String>,
    pub confidence: f64,
    pub hit_description: String,
    pub source: String,
}

// ============================================================================
// 数据库初始化
// ============================================================================

/// 创建知识库表
pub fn create_knowledge_tables(conn: &Connection) -> SqliteResult<()> {
    // 材料知识库表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS material_knowledge (
            id TEXT PRIMARY KEY,
            material_id TEXT NOT NULL,
            name TEXT NOT NULL,
            category TEXT NOT NULL,
            elastic_modulus REAL NOT NULL,
            poissons_ratio REAL NOT NULL,
            density REAL NOT NULL,
            yield_strength REAL NOT NULL,
            ultimate_strength REAL,
            fatigue_params_json TEXT,
            thermal_params_json TEXT,
            mesh_guidelines_json TEXT,
            solver_settings_json TEXT,
            common_errors_json TEXT,
            last_analysis INTEGER,
            project_ids_json TEXT,
            notes TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    // 设计规范库表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS design_standards (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            version TEXT NOT NULL,
            source TEXT NOT NULL,
            scope TEXT NOT NULL,
            safety_factor REAL NOT NULL,
            allowable_stress REAL,
            applicable_load_cases_json TEXT,
            rules_json TEXT,
            industry TEXT,
            issue_date TEXT,
            notes TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    // 失效模式图谱表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS failure_modes (
            id TEXT PRIMARY KEY,
            mode_type TEXT NOT NULL,
            description TEXT NOT NULL,
            applicable_materials_json TEXT,
            temperature_range_json TEXT,
            mechanisms_json TEXT NOT NULL,
            uncertainty TEXT NOT NULL,
            related_standards_json TEXT,
            case_study_ids_json TEXT,
            priority INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    // 用户画像表 (跨会话记忆)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_profiles (
            user_id TEXT PRIMARY KEY,
            nickname TEXT,
            email TEXT,
            company TEXT,
            position TEXT,
            preferences_json TEXT NOT NULL,
            project_history_json TEXT NOT NULL,
            expertise_tags_json TEXT,
            created_at INTEGER NOT NULL,
            last_login_at INTEGER NOT NULL
        )",
        [],
    )?;

    // 创建索引
    conn.execute("CREATE INDEX IF NOT EXISTS idx_matknow_id ON material_knowledge(material_id)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_matknow_name ON material_knowledge(name)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_design_name ON design_standards(name)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_failure_type ON failure_modes(mode_type)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_user_profile_id ON user_profiles(user_id)", [])?;

    // 初始化内置材料知识
    init_builtin_materials(conn)?;

    // 初始化内置设计规范
    init_builtin_standards(conn)?;

    // 初始化内置失效模式
    init_builtin_failure_modes(conn)?;

    Ok(())
}

/// 初始化内置材料知识
fn init_builtin_materials(conn: &Connection) -> SqliteResult<()> {
    let now = chrono::Utc::now().to_rfc3339();

    let materials = vec![
        // 钛合金
        ("TC4", "Ti-6Al-4V", "titanium", 113.8, 0.34, 4430.0, 880.0, 950.0,
         r#"[{"material":"Ti-6Al-4V","stressRatio":-1,"curveType":"basquin","basquinB":900,"basquinM":-0.12,"fatigueLimit":620,"cutoffCycles":1e7,"standardSource":"BMS7-368E"}]"#,
         r#"{"thermalConductivity":6.7,"specificHeat":560,"thermalExpansion":8.6e-6,"maxTemperature":350}"#,
         r#"{"elementType":"tet10","minSize":0.5,"maxSize":3.0,"localRefinement":3,"qualityTarget":0.7,"boundaryLayerRatio":0.1}"#,
         r#"{"solverType":"direct","maxIterations":1000,"tolerance":1e-6,"preconditioner":"amg","linearSolver":"pardiso"}"#),
        // 铝合金
        ("AA7075", "AA 7075-T6", "aluminum", 71.7, 0.33, 2810.0, 503.0, 572.0,
         r#"[{"material":"AA 7075-T6","stressRatio":-1,"curveType":"basquin","basquinB":540,"basquinM":-0.11,"fatigueLimit":160,"cutoffCycles":1e7,"standardSource":"VDA"}]"#,
         r#"{"thermalConductivity":130,"specificHeat":960,"thermalExpansion":2.38e-5,"maxTemperature":200}"#,
         r#"{"elementType":"tet10","minSize":0.3,"maxSize":2.0,"localRefinement":4,"qualityTarget":0.75}"#,
         r#"{"solverType":"direct","linearSolver":"mumps","nonlinearStrategy":"newton"}"#),
        // 结构钢
        ("Q355", "Q355", "steel", 206.0, 0.30, 7850.0, 355.0, 470.0,
         r#"[{"material":"Q355","stressRatio":-1,"curveType":"basquin","basquinB":620,"basquinM":-0.15,"fatigueLimit":200,"cutoffCycles":2e6,"standardSource":"GB/T 6398"}]"#,
         r#"{"thermalConductivity":50,"specificHeat":460,"thermalExpansion":1.2e-5,"maxTemperature":400}"#,
         r#"{"elementType":"hex20","minSize":1.0,"maxSize":10.0,"localRefinement":2,"qualityTarget":0.7}"#,
         r#"{"solverType":"iterative","maxIterations":500,"tolerance":1e-5,"preconditioner":"ilu"}"#),
        // 不锈钢
        ("SS316L", "SS 316L", "steel", 193.0, 0.29, 8000.0, 170.0, 485.0,
         r#"[{"material":"SS 316L","stressRatio":-1,"curveType":"basquin","basquinB":310,"basquinM":-0.18,"fatigueLimit":120,"cutoffCycles":1e7}]"#,
         r#"{"thermalConductivity":16.3,"specificHeat":500,"thermalExpansion":1.6e-5,"maxTemperature":600}"#,
         r#"{"elementType":"tet10","minSize":0.5,"maxSize":4.0,"localRefinement":3,"qualityTarget":0.7}"#,
         r#"{"solverType":"direct","linearSolver":"pardiso","nonlinearStrategy":"modified_newton"}"#),
    ];

    for (mid, name, cat, em, pr, dens, ys, us, fp, tp, mg, ss) in materials {
        conn.execute(
            "INSERT OR IGNORE INTO material_knowledge
             (id, material_id, name, category, elastic_modulus, poissons_ratio, density, yield_strength, ultimate_strength, fatigue_params_json, thermal_params_json, mesh_guidelines_json, solver_settings_json, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?14)",
            rusqlite::params![nanoid::nanoid!(12), mid, name, cat, em, pr, dens, ys, us, fp, tp, mg, ss, &now],
        ).ok();
    }

    Ok(())
}

/// 初始化内置设计规范
fn init_builtin_standards(conn: &Connection) -> SqliteResult<()> {
    let now = chrono::Utc::now().to_rfc3339();

    let standards = vec![
        ("BMS7-368E", "BMS7-368E", "1.0", "BMS7-368E", "航空航天结构件疲劳设计", 1.5, 620.0,
         r#"[{"name":"轴向疲劳","type":"axial"},{"name":"弯曲疲劳","type":"bending"},{"name":"扭转疲劳","type":"torsion"}]"#,
         r#"["高周疲劳设计应基于 S-N 曲线，安全系数取 1.5", "关键结构件需进行应变控制疲劳分析", "表面处理工艺对疲劳极限有显著影响"]"#,
         " aerospace", "2023-01-01"),
        ("VDA-2300", "VDA 2300", "2.0", "VDA", "汽车零部件疲劳耐久设计", 1.35, 350.0,
         r#"[{"name":"道路疲劳","type":"dynamic"},{"name":"振动疲劳","type":"combined"}]"#,
         r#"["汽车零部件采用雨流计数法进行疲劳损伤计算", "考虑载荷谱的统计特性", "多轴应力状态下采用等效效应力幅"]"#,
         "automotive", "2022-06-01"),
        ("GB/T 6398", "GB/T 6398-2024", "2024", "GB/T", "金属材料疲劳试验方法", 1.4, 0.0,
         r#"[{"name":"旋转弯曲","type":"axial"},{"name":"平面弯曲","type":"bending"}]"#,
         r#"["标准规定了金属材料旋转弯曲和平面弯曲疲劳试验方法", "S-N 曲线应采用双对数坐标表示", "疲劳极限取 1e7 次循环对应的应力幅"]"#,
         "general", "2024-03-01"),
    ];

    for (id, name, ver, src, scope, sf, als, alc, rules, ind, date) in standards {
        conn.execute(
            "INSERT OR IGNORE INTO design_standards
             (id, name, version, source, scope, safety_factor, allowable_stress, applicable_load_cases_json, rules_json, industry, issue_date, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?12)",
            rusqlite::params![id, name, ver, src, scope, sf, als, alc, rules, ind, date, &now],
        ).ok();
    }

    Ok(())
}

/// 初始化内置失效模式
fn init_builtin_failure_modes(conn: &Connection) -> SqliteResult<()> {
    let now = chrono::Utc::now().to_rfc3339();

    let modes = vec![
        ("HCF-001", "HCF", "高周疲劳失效 — 高循环次数、低应力水平下的疲劳裂纹萌生与扩展",
         r#"["Ti-6Al-4V","TC4","AA 7075-T6","SS 316L"]"#,
         r#"{"min":-50,"max":300}"#,
         r#"[{"name":"裂纹萌生","drivingFactors":["表面应力集中","残余拉应力","晶界弱化"],"predictors":["最大主应力幅","应力比R","表面粗糙度"],"mitigationStrategies":["喷丸强化","表面抛光","减小应力集中"]}]"#,
         "medium",
         r#"["BMS7-368E"]"#,
         r#"[]"#,
         1),
        ("LCF-001", "LCF", "低周疲劳失效 — 低循环次数、高应变幅下的塑性累积损伤",
         r#"["Ti-6Al-4V","TC4","AA 7075-T6"]"#,
         r#"{"min":300,"max":600}"#,
         r#"[{"name":"塑性疲劳","drivingFactors":["应变幅","平均应力","温度"],"predictors":["总应变范围","塑性应变能","软化系数"],"mitigationStrategies":["应变控制设计","热处理优化","降低平均应力"]}]"#,
         "low",
         r#"[]"#,
         r#"[]"#,
         2),
        ("TMF-001", "TMF", "热机械疲劳失效 — 温度循环与机械载荷耦合作用下的失效",
         r#"["Ti-6Al-4V","SS 316L","Inconel 718"]"#,
         r#"{"min":400,"max":700}"#,
         r#"[{"name":"热疲劳","drivingFactors":["温度范围","循环速率","相变","氧化"],"predictors":["ΔT","热应变幅","氧化层厚度"],"mitigationStrategies":["梯度涂层","冷却优化","材料升级"]}]"#,
         "high",
         r#"[]"#,
         r#"[]"#,
         2),
        ("CREEP-001", "CREEP", "蠕变失效 — 高温长时间载荷下的塑性流动",
         r#"["Inconel 718","Hastelloy C-276","SS 316L"]"#,
         r#"{"min":500,"max":900}"#,
         r#"[{"name":"蠕变","drivingFactors":["温度","应力","时间","晶界滑移"],"predictors":["Larson-Miller参数","稳态蠕变速率","最小蠕变速率"],"mitigationStrategies":["降低工作温度","选用更高等级材料","减少应力集中"]}]"#,
         "high",
         r#"[]"#,
         r#"[]"#,
         3),
        ("OVERLOAD-001", "OVERLOAD", "静力过载失效 — 一次性超载导致的塑性失稳或断裂",
         r#"["Ti-6Al-4V","TC4","AA 7075-T6","Q355"]"#,
         r#"{"min":-50,"max":400}"#,
         r#"[{"name":"过载断裂","drivingFactors":["过载倍数","应力状态","缺口效应"],"predictors":["von Mises应力","最大主应力","应力梯度"],"mitigationStrategies":["提高安全系数","避免应力集中","增加截面"]}]"#,
         "low",
         r#"["BMS7-368E","GB/T 6398"]"#,
         r#"[]"#,
         1),
        ("FOD-001", "FOD", "外来物损伤失效 — 硬物撞击导致的表面损伤和裂纹萌生",
         r#"["Ti-6Al-4V","TC4","AA 7075-T6","AA 6061-T6"]"#,
         r#"{"min":-50,"max":400}"#,
         r#"[{"name":"撞击损伤","drivingFactors":["撞击能量","撞击角度","靶材硬度","表面状态"],"predictors":["冲击坑深度","裂纹长度","剩余疲劳极限"],"mitigationStrategies":["表面强化","添加保护层","定期检测"]}]"#,
         "high",
         r#"["BMS7-368E"]"#,
         r#"[]"#,
         2),
    ];

    for (id, mt, desc, mats, tr, mech, unc, std, cs, pri) in modes {
        conn.execute(
            "INSERT OR IGNORE INTO failure_modes
             (id, mode_type, description, applicable_materials_json, temperature_range_json, mechanisms_json, uncertainty, related_standards_json, case_study_ids_json, priority, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?11)",
            rusqlite::params![id, mt, desc, mats, tr, mech, unc, std, cs, pri, &now],
        ).ok();
    }

    Ok(())
}

// ============================================================================
// 数据库操作函数
// ============================================================================

/// 写入材料知识
pub fn write_material_knowledge(conn: &Connection, req: &MaterialKnowledgeWriteRequest) -> Result<MaterialKnowledgeRecord, String> {
    let id = nanoid::nanoid!(12);
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO material_knowledge
         (id, material_id, name, category, elastic_modulus, poissons_ratio, density, yield_strength, ultimate_strength, fatigue_params_json, thermal_params_json, mesh_guidelines_json, solver_settings_json, common_errors_json, project_ids_json, notes, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
        rusqlite::params![
            &id, &req.material_id, &req.name, &req.category, req.elastic_modulus, req.poissons_ratio, req.density, req.yield_strength,
            &req.ultimate_strength, &req.fatigue_params_json, &req.thermal_params_json, &req.mesh_guidelines_json,
            &req.solver_settings_json, &req.common_errors_json, &req.project_ids_json, &req.notes, &now, &now
        ],
    ).map_err(|e| format!("写入材料知识失败: {}", e))?;

    Ok(MaterialKnowledgeRecord {
        id,
        material_id: req.material_id.clone(),
        name: req.name.clone(),
        category: req.category.clone(),
        elastic_modulus: req.elastic_modulus,
        poissons_ratio: req.poissons_ratio,
        density: req.density,
        yield_strength: req.yield_strength,
        ultimate_strength: req.ultimate_strength,
        fatigue_params_json: req.fatigue_params_json.clone(),
        thermal_params_json: req.thermal_params_json.clone(),
        mesh_guidelines_json: req.mesh_guidelines_json.clone(),
        solver_settings_json: req.solver_settings_json.clone(),
        common_errors_json: req.common_errors_json.clone(),
        last_analysis: None,
        project_ids_json: req.project_ids_json.clone(),
        notes: req.notes.clone(),
        created_at: now.clone(),
        updated_at: now,
    })
}

/// 查询材料知识
pub fn query_material_knowledge(conn: &Connection, material_id: Option<&str>, name: Option<&str>, limit: i32) -> Result<Vec<MaterialKnowledgeRecord>, String> {
    let mut sql = String::from("SELECT * FROM material_knowledge WHERE 1=1");
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(mid) = material_id {
        sql.push_str(" AND material_id LIKE ?");
        params.push(Box::new(format!("%{}%", mid)));
    }
    if let Some(n) = name {
        sql.push_str(" AND name LIKE ?");
        params.push(Box::new(format!("%{}%", n)));
    }

    sql.push_str(&format!(" ORDER BY updated_at DESC LIMIT {}", limit));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let mut stmt = conn.prepare(&sql).map_err(|e| format!("查询失败: {}", e))?;

    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(MaterialKnowledgeRecord {
            id: row.get(0)?,
            material_id: row.get(1)?,
            name: row.get(2)?,
            category: row.get(3)?,
            elastic_modulus: row.get(4)?,
            poissons_ratio: row.get(5)?,
            density: row.get(6)?,
            yield_strength: row.get(7)?,
            ultimate_strength: row.get(8)?,
            fatigue_params_json: row.get(9)?,
            thermal_params_json: row.get(10)?,
            mesh_guidelines_json: row.get(11)?,
            solver_settings_json: row.get(12)?,
            common_errors_json: row.get(13)?,
            last_analysis: row.get(14)?,
            project_ids_json: row.get(15)?,
            notes: row.get(16)?,
            created_at: row.get(17)?,
            updated_at: row.get(18)?,
        })
    }).map_err(|e| format!("解析失败: {}", e))?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row.map_err(|e| format!("读取行失败: {}", e))?);
    }
    Ok(results)
}

/// 写入设计规范
pub fn write_design_standard(conn: &Connection, req: &DesignStandardWriteRequest) -> Result<DesignStandardRecord, String> {
    let id = nanoid::nanoid!(12);
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO design_standards
         (id, name, version, source, scope, safety_factor, allowable_stress, applicable_load_cases_json, rules_json, industry, issue_date, notes, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        rusqlite::params![
            &id, &req.name, &req.version, &req.source, &req.scope, req.safety_factor,
            &req.allowable_stress, &req.applicable_load_cases_json, &req.rules_json,
            &req.industry, &req.issue_date, &req.notes, &now, &now
        ],
    ).map_err(|e| format!("写入设计规范失败: {}", e))?;

    Ok(DesignStandardRecord {
        id,
        name: req.name.clone(),
        version: req.version.clone(),
        source: req.source.clone(),
        scope: req.scope.clone(),
        safety_factor: req.safety_factor,
        allowable_stress: req.allowable_stress,
        applicable_load_cases_json: req.applicable_load_cases_json.clone(),
        rules_json: req.rules_json.clone(),
        industry: req.industry.clone(),
        issue_date: req.issue_date.clone(),
        notes: req.notes.clone(),
        created_at: now.clone(),
        updated_at: now,
    })
}

/// 查询设计规范
pub fn query_design_standards(conn: &Connection, name: Option<&str>, source: Option<&str>, limit: i32) -> Result<Vec<DesignStandardRecord>, String> {
    let mut sql = String::from("SELECT * FROM design_standards WHERE 1=1");
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(n) = name {
        sql.push_str(" AND name LIKE ?");
        params.push(Box::new(format!("%{}%", n)));
    }
    if let Some(s) = source {
        sql.push_str(" AND source = ?");
        params.push(Box::new(s.to_string()));
    }

    sql.push_str(&format!(" ORDER BY updated_at DESC LIMIT {}", limit));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let mut stmt = conn.prepare(&sql).map_err(|e| format!("查询失败: {}", e))?;

    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(DesignStandardRecord {
            id: row.get(0)?,
            name: row.get(1)?,
            version: row.get(2)?,
            source: row.get(3)?,
            scope: row.get(4)?,
            safety_factor: row.get(5)?,
            allowable_stress: row.get(6)?,
            applicable_load_cases_json: row.get(7)?,
            rules_json: row.get(8)?,
            industry: row.get(9)?,
            issue_date: row.get(10)?,
            notes: row.get(11)?,
            created_at: row.get(12)?,
            updated_at: row.get(13)?,
        })
    }).map_err(|e| format!("解析失败: {}", e))?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row.map_err(|e| format!("读取行失败: {}", e))?);
    }
    Ok(results)
}

/// 写入失效模式
pub fn write_failure_mode(conn: &Connection, req: &FailureModeWriteRequest) -> Result<FailureModeRecord, String> {
    let id = nanoid::nanoid!(12);
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO failure_modes
         (id, mode_type, description, applicable_materials_json, temperature_range_json, mechanisms_json, uncertainty, related_standards_json, case_study_ids_json, priority, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        rusqlite::params![
            &id, &req.mode_type, &req.description, &req.applicable_materials_json,
            &req.temperature_range_json, &req.mechanisms_json, &req.uncertainty,
            &req.related_standards_json, &req.case_study_ids_json, req.priority, &now, &now
        ],
    ).map_err(|e| format!("写入失效模式失败: {}", e))?;

    Ok(FailureModeRecord {
        id,
        mode_type: req.mode_type.clone(),
        description: req.description.clone(),
        applicable_materials_json: req.applicable_materials_json.clone(),
        temperature_range_json: req.temperature_range_json.clone(),
        mechanisms_json: req.mechanisms_json.clone(),
        uncertainty: req.uncertainty.clone(),
        related_standards_json: req.related_standards_json.clone(),
        case_study_ids_json: req.case_study_ids_json.clone(),
        priority: req.priority,
        created_at: now.clone(),
        updated_at: now,
    })
}

/// 查询失效模式
pub fn query_failure_modes(conn: &Connection, mode_type: Option<&str>, material: Option<&str>, limit: i32) -> Result<Vec<FailureModeRecord>, String> {
    let mut sql = String::from("SELECT * FROM failure_modes WHERE 1=1");
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(mt) = mode_type {
        sql.push_str(" AND mode_type = ?");
        params.push(Box::new(mt.to_string()));
    }
    if let Some(mat) = material {
        sql.push_str(" AND applicable_materials_json LIKE ?");
        params.push(Box::new(format!("%{}%", mat)));
    }

    sql.push_str(&format!(" ORDER BY priority ASC, updated_at DESC LIMIT {}", limit));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let mut stmt = conn.prepare(&sql).map_err(|e| format!("查询失败: {}", e))?;

    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(FailureModeRecord {
            id: row.get(0)?,
            mode_type: row.get(1)?,
            description: row.get(2)?,
            applicable_materials_json: row.get(3)?,
            temperature_range_json: row.get(4)?,
            mechanisms_json: row.get(5)?,
            uncertainty: row.get(6)?,
            related_standards_json: row.get(7)?,
            case_study_ids_json: row.get(8)?,
            priority: row.get(9)?,
            created_at: row.get(10)?,
            updated_at: row.get(11)?,
        })
    }).map_err(|e| format!("解析失败: {}", e))?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row.map_err(|e| format!("读取行失败: {}", e))?);
    }
    Ok(results)
}

/// 写入用户画像
pub fn write_user_profile(conn: &Connection, req: &UserProfileWriteRequest) -> Result<UserProfileRecord, String> {
    let now = chrono::Utc::now().timestamp();

    conn.execute(
        "INSERT OR REPLACE INTO user_profiles
         (user_id, nickname, email, company, position, preferences_json, project_history_json, expertise_tags_json, created_at, last_login_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9)",
        rusqlite::params![
            &req.user_id, &req.nickname, &req.email, &req.company, &req.position,
            &req.preferences_json, &req.project_history_json, &req.expertise_tags_json, now
        ],
    ).map_err(|e| format!("写入用户画像失败: {}", e))?;

    Ok(UserProfileRecord {
        user_id: req.user_id.clone(),
        nickname: req.nickname.clone(),
        email: req.email.clone(),
        company: req.company.clone(),
        position: req.position.clone(),
        preferences_json: req.preferences_json.clone(),
        project_history_json: req.project_history_json.clone(),
        expertise_tags_json: req.expertise_tags_json.clone(),
        created_at: now,
        last_login_at: now,
    })
}

/// 获取用户画像
pub fn db_get_user_profile(conn: &Connection, user_id: &str) -> Result<Option<UserProfileRecord>, String> {
    let mut stmt = conn.prepare("SELECT * FROM user_profiles WHERE user_id = ?")
        .map_err(|e| format!("查询失败: {}", e))?;

    let result = stmt.query_row([user_id], |row| {
        Ok(UserProfileRecord {
            user_id: row.get(0)?,
            nickname: row.get(1)?,
            email: row.get(2)?,
            company: row.get(3)?,
            position: row.get(4)?,
            preferences_json: row.get(5)?,
            project_history_json: row.get(6)?,
            expertise_tags_json: row.get(7)?,
            created_at: row.get(8)?,
            last_login_at: row.get(9)?,
        })
    });

    match result {
        Ok(profile) => Ok(Some(profile)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(format!("读取用户画像失败: {}", e)),
    }
}

/// 知识召回
pub fn db_recall_knowledge(conn: &Connection, req: &KnowledgeRecallRequest) -> Result<KnowledgeRecallResult, String> {
    let limit = req.limit.unwrap_or(5) as i32;
    let mut all_materials = Vec::new();
    let mut all_standards = Vec::new();
    let mut all_failure_modes = Vec::new();

    // 搜索材料知识
    if req.context_type.is_none() || req.context_type.as_ref().map(|s| s.as_str()) == Some("material") || req.context_type.as_ref().map(|s| s.as_str()) == Some("all") {
        for kw in &req.keywords {
            let results = query_material_knowledge(conn, Some(kw), Some(kw), limit)?;
            all_materials.extend(results);
        }
    }

    // 搜索设计规范
    if req.context_type.is_none() || req.context_type.as_ref().map(|s| s.as_str()) == Some("standard") || req.context_type.as_ref().map(|s| s.as_str()) == Some("all") {
        for kw in &req.keywords {
            let results = query_design_standards(conn, Some(kw), None, limit)?;
            all_standards.extend(results);
        }
    }

    // 搜索失效模式
    if req.context_type.is_none() || req.context_type.as_ref().map(|s| s.as_str()) == Some("failure_mode") || req.context_type.as_ref().map(|s| s.as_str()) == Some("all") {
        for kw in &req.keywords {
            let results = query_failure_modes(conn, None, Some(kw), limit)?;
            all_failure_modes.extend(results);
        }
    }

    // 去重
    all_materials.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    all_materials.dedup_by(|a, b| a.id == b.id);
    all_standards.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    all_standards.dedup_by(|a, b| a.id == b.id);
    all_failure_modes.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    all_failure_modes.dedup_by(|a, b| a.id == b.id);

    // 计算置信度
    let hit_count = all_materials.len() + all_standards.len() + all_failure_modes.len();
    let confidence = if hit_count == 0 { 0.0 } else { (hit_count as f64 / (req.keywords.len() as f64 * 3.0)).min(1.0) };
    let hit_description = format!("命中 {} 个材料知识、{} 个设计规范、{} 个失效模式",
        all_materials.len(), all_standards.len(), all_failure_modes.len());

    Ok(KnowledgeRecallResult {
        materials_json: if all_materials.is_empty() { None } else { serde_json::to_string(&all_materials).ok() },
        standards_json: if all_standards.is_empty() { None } else { serde_json::to_string(&all_standards).ok() },
        failure_modes_json: if all_failure_modes.is_empty() { None } else { serde_json::to_string(&all_failure_modes).ok() },
        confidence,
        hit_description,
        source: "local".to_string(),
    })
}

// ============================================================================
// Tauri 命令
// ============================================================================

use crate::db::Database;

/// 获取材料知识列表
#[tauri::command]
pub async fn get_material_knowledge_list(
    db: tauri::State<'_, Database>,
    material_id: Option<String>,
    name: Option<String>,
    limit: Option<i32>,
) -> Result<Vec<MaterialKnowledgeRecord>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    query_material_knowledge(&conn, material_id.as_deref(), name.as_deref(), limit.unwrap_or(20))
}

/// 写入材料知识
#[tauri::command]
pub async fn save_material_knowledge(
    db: tauri::State<'_, Database>,
    request: MaterialKnowledgeWriteRequest,
) -> Result<MaterialKnowledgeRecord, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    write_material_knowledge(&conn, &request)
}

/// 获取设计规范列表
#[tauri::command]
pub async fn get_design_standards_list(
    db: tauri::State<'_, Database>,
    name: Option<String>,
    source: Option<String>,
    limit: Option<i32>,
) -> Result<Vec<DesignStandardRecord>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    query_design_standards(&conn, name.as_deref(), source.as_deref(), limit.unwrap_or(20))
}

/// 写入设计规范
#[tauri::command]
pub async fn save_design_standard(
    db: tauri::State<'_, Database>,
    request: DesignStandardWriteRequest,
) -> Result<DesignStandardRecord, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    write_design_standard(&conn, &request)
}

/// 获取失效模式列表
#[tauri::command]
pub async fn get_failure_modes_list(
    db: tauri::State<'_, Database>,
    mode_type: Option<String>,
    material: Option<String>,
    limit: Option<i32>,
) -> Result<Vec<FailureModeRecord>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    query_failure_modes(&conn, mode_type.as_deref(), material.as_deref(), limit.unwrap_or(20))
}

/// 写入失效模式
#[tauri::command]
pub async fn save_failure_mode(
    db: tauri::State<'_, Database>,
    request: FailureModeWriteRequest,
) -> Result<FailureModeRecord, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    write_failure_mode(&conn, &request)
}

/// 获取用户画像
#[tauri::command]
pub async fn get_user_profile(
    db: tauri::State<'_, Database>,
    user_id: String,
) -> Result<Option<UserProfileRecord>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db_get_user_profile(&conn, &user_id)
}

/// 写入用户画像
#[tauri::command]
pub async fn save_user_profile(
    db: tauri::State<'_, Database>,
    request: UserProfileWriteRequest,
) -> Result<UserProfileRecord, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    write_user_profile(&conn, &request)
}

/// 知识召回
#[tauri::command]
pub async fn recall_knowledge(
    db: tauri::State<'_, Database>,
    request: KnowledgeRecallRequest,
) -> Result<KnowledgeRecallResult, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db_recall_knowledge(&conn, &request)
}