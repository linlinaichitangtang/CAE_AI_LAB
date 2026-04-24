/**
 * V2.5 仿真结果数据基础设施 - Rust 后端命令
 * - V2.5-006: 仿真结果数据库 Schema
 * - V2.5-007: 自动归档 Pipeline
 * - V2.5-008: 数据导出接口 (JSON/CSV)
 */

use serde::{Deserialize, Serialize};
use std::io::Write;
use tauri::Manager;

// ============================================================================
// 数据结构
// ============================================================================

/// 仿真结果归档记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimulationArchive {
    /// 唯一 ID
    pub id: String,
    /// 关联项目 ID
    pub project_id: Option<String>,
    /// 仿真类型 (static, modal, thermal, buckling, frequency_response, etc.)
    pub simulation_type: String,
    /// 材料名称
    pub material_name: Option<String>,
    /// 材料成分 (JSON)
    pub composition: Option<String>,
    /// 输入参数 (JSON: mesh_size, boundary_conditions, etc.)
    pub input_params: String,
    /// 输出结果 (JSON: max_stress, max_displacement, convergence, etc.)
    pub output_results: String,
    /// 网格统计 (JSON: nodes, elements, element_type)
    pub mesh_stats: Option<String>,
    /// 求解器名称
    pub solver_name: Option<String>,
    /// 求解耗时 (秒)
    pub solve_time_sec: Option<f64>,
    /// 是否收敛
    pub converged: Option<bool>,
    /// 迭代次数
    pub iterations: Option<u32>,
    /// 归档时间 (RFC3339)
    pub archived_at: String,
    /// 备注
    pub notes: Option<String>,
}

/// 归档请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArchiveRequest {
    pub project_id: Option<String>,
    pub simulation_type: String,
    pub material_name: Option<String>,
    pub composition: Option<serde_json::Value>,
    pub input_params: serde_json::Value,
    pub output_results: serde_json::Value,
    pub mesh_stats: Option<serde_json::Value>,
    pub solver_name: Option<String>,
    pub solve_time_sec: Option<f64>,
    pub converged: Option<bool>,
    pub iterations: Option<u32>,
    pub notes: Option<String>,
}

/// 查询过滤条件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArchiveFilter {
    pub project_id: Option<String>,
    pub simulation_type: Option<String>,
    pub material_name: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// 数据导出请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportRequest {
    pub format: String,  // "json" | "csv"
    pub filter: Option<ArchiveFilter>,
}

/// 导出结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportResult {
    pub format: String,
    pub record_count: u32,
    pub file_path: String,
    pub file_size_bytes: u64,
}

/// 统计信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArchiveStats {
    pub total_records: u32,
    pub by_simulation_type: std::collections::HashMap<String, u32>,
    pub by_material: std::collections::HashMap<String, u32>,
    pub latest_archive_at: Option<String>,
}

// ============================================================================
// 数据库操作
// ============================================================================

/// 创建仿真结果归档表
pub fn create_archive_table(conn: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS simulation_archives (
            id TEXT PRIMARY KEY,
            project_id TEXT,
            simulation_type TEXT NOT NULL,
            material_name TEXT,
            composition TEXT,
            input_params TEXT NOT NULL,
            output_results TEXT NOT NULL,
            mesh_stats TEXT,
            solver_name TEXT,
            solve_time_sec REAL,
            converged INTEGER,
            iterations INTEGER,
            archived_at TEXT NOT NULL,
            notes TEXT,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE SET NULL
        );

        CREATE INDEX IF NOT EXISTS idx_archives_project ON simulation_archives(project_id);
        CREATE INDEX IF NOT EXISTS idx_archives_type ON simulation_archives(simulation_type);
        CREATE INDEX IF NOT EXISTS idx_archives_material ON simulation_archives(material_name);
        CREATE INDEX IF NOT EXISTS idx_archives_date ON simulation_archives(archived_at);
        "
    )?;

    // 迁移: 检查是否需要新增列
    let has_notes = column_exists(conn, "simulation_archives", "notes");
    if !has_notes {
        conn.execute_batch("ALTER TABLE simulation_archives ADD COLUMN notes TEXT;")?;
    }

    Ok(())
}

fn column_exists(conn: &rusqlite::Connection, table: &str, column: &str) -> bool {
    conn.prepare(&format!("PRAGMA table_info({})", table))
        .ok()
        .and_then(|mut stmt| {
            let rows: Vec<String> = stmt.query_map([], |row| {
                let name: String = row.get(1)?;
                Ok(name)
            }).ok()?.filter_map(|r| r.ok()).collect();
            Some(rows.contains(&column.to_string()))
        })
        .unwrap_or(false)
}

/// 归档仿真结果
pub fn archive_result(conn: &rusqlite::Connection, req: &ArchiveRequest) -> Result<SimulationArchive, String> {
    let id = nanoid::nanoid!(12);
    let archived_at = chrono::Utc::now().to_rfc3339();

    let composition_json = req.composition.as_ref()
        .map(|c| serde_json::to_string(c).unwrap_or_default());

    let input_json = serde_json::to_string(&req.input_params).unwrap_or_default();
    let output_json = serde_json::to_string(&req.output_results).unwrap_or_default();
    let mesh_json = req.mesh_stats.as_ref()
        .map(|m| serde_json::to_string(m).unwrap_or_default());

    conn.execute(
        "INSERT INTO simulation_archives (id, project_id, simulation_type, material_name, composition, input_params, output_results, mesh_stats, solver_name, solve_time_sec, converged, iterations, archived_at, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        rusqlite::params![
            id,
            req.project_id,
            req.simulation_type,
            req.material_name,
            composition_json,
            input_json,
            output_json,
            mesh_json,
            req.solver_name,
            req.solve_time_sec,
            req.converged.map(|c| if c { 1 } else { 0 }),
            req.iterations,
            archived_at,
            req.notes,
        ],
    ).map_err(|e| format!("归档失败: {}", e))?;

    Ok(SimulationArchive {
        id,
        project_id: req.project_id.clone(),
        simulation_type: req.simulation_type.clone(),
        material_name: req.material_name.clone(),
        composition: composition_json,
        input_params: input_json,
        output_results: output_json,
        mesh_stats: mesh_json,
        solver_name: req.solver_name.clone(),
        solve_time_sec: req.solve_time_sec,
        converged: req.converged,
        iterations: req.iterations,
        archived_at,
        notes: req.notes.clone(),
    })
}

/// 查询归档记录
pub fn query_archives(conn: &rusqlite::Connection, filter: &ArchiveFilter) -> Result<Vec<SimulationArchive>, String> {
    let mut sql = String::from("SELECT * FROM simulation_archives WHERE 1=1");
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref project_id) = filter.project_id {
        sql.push_str(" AND project_id = ?");
        params.push(Box::new(project_id.clone()));
    }
    if let Some(ref sim_type) = filter.simulation_type {
        sql.push_str(" AND simulation_type = ?");
        params.push(Box::new(sim_type.clone()));
    }
    if let Some(ref material) = filter.material_name {
        sql.push_str(" AND material_name = ?");
        params.push(Box::new(material.clone()));
    }
    if let Some(ref from) = filter.date_from {
        sql.push_str(" AND archived_at >= ?");
        params.push(Box::new(from.clone()));
    }
    if let Some(ref to) = filter.date_to {
        sql.push_str(" AND archived_at <= ?");
        params.push(Box::new(to.clone()));
    }

    sql.push_str(" ORDER BY archived_at DESC");

    let limit = filter.limit.unwrap_or(100);
    let offset = filter.offset.unwrap_or(0);
    sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let mut stmt = conn.prepare(&sql).map_err(|e| format!("查询失败: {}", e))?;
    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(SimulationArchive {
            id: row.get(0)?,
            project_id: row.get(1)?,
            simulation_type: row.get(2)?,
            material_name: row.get(3)?,
            composition: row.get(4)?,
            input_params: row.get(5)?,
            output_results: row.get(6)?,
            mesh_stats: row.get(7)?,
            solver_name: row.get(8)?,
            solve_time_sec: row.get(9)?,
            converged: row.get::<_, Option<i32>>(10)?.map(|v| v != 0),
            iterations: row.get(11)?,
            archived_at: row.get(12)?,
            notes: row.get(13)?,
        })
    }).map_err(|e| format!("解析结果失败: {}", e))?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row.map_err(|e| format!("读取行失败: {}", e))?);
    }

    Ok(results)
}

/// 获取归档统计
pub fn get_archive_stats(conn: &rusqlite::Connection) -> Result<ArchiveStats, String> {
    let total: u32 = conn.query_row(
        "SELECT COUNT(*) FROM simulation_archives",
        [],
        |row| row.get(0),
    ).unwrap_or(0);

    let mut by_type = std::collections::HashMap::new();
    let mut stmt = conn.prepare(
        "SELECT simulation_type, COUNT(*) FROM simulation_archives GROUP BY simulation_type"
    ).map_err(|e| format!("统计查询失败: {}", e))?;
    let type_rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, u32>(1)?))
    }).map_err(|e| format!("统计解析失败: {}", e))?;
    for row in type_rows {
        if let Ok((t, c)) = row {
            by_type.insert(t, c);
        }
    }

    let mut by_material = std::collections::HashMap::new();
    let mut stmt2 = conn.prepare(
        "SELECT material_name, COUNT(*) FROM simulation_archives WHERE material_name IS NOT NULL GROUP BY material_name"
    ).map_err(|e| format!("材料统计查询失败: {}", e))?;
    let mat_rows = stmt2.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, u32>(1)?))
    }).map_err(|e| format!("材料统计解析失败: {}", e))?;
    for row in mat_rows {
        if let Ok((m, c)) = row {
            by_material.insert(m, c);
        }
    }

    let latest: Option<String> = conn.query_row(
        "SELECT archived_at FROM simulation_archives ORDER BY archived_at DESC LIMIT 1",
        [],
        |row| row.get(0),
    ).ok();

    Ok(ArchiveStats {
        total_records: total,
        by_simulation_type: by_type,
        by_material: by_material,
        latest_archive_at: latest,
    })
}

/// 导出归档数据
pub fn export_archives(
    conn: &rusqlite::Connection,
    req: &ExportRequest,
    app_data_dir: &std::path::Path,
) -> Result<ExportResult, String> {
    let filter = req.filter.clone().unwrap_or(ArchiveFilter {
        project_id: None,
        simulation_type: None,
        material_name: None,
        date_from: None,
        date_to: None,
        limit: Some(10000),
        offset: Some(0),
    });

    let records = query_archives(conn, &filter)?;

    let export_dir = app_data_dir.join("exports");
    std::fs::create_dir_all(&export_dir).map_err(|e| format!("创建导出目录失败: {}", e))?;

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let file_name = format!("simulation_archive_{}.{}", timestamp, req.format);
    let file_path = export_dir.join(&file_name);

    let content = match req.format.as_str() {
        "csv" => export_as_csv(&records),
        _ => export_as_json(&records),
    };

    let mut file = std::fs::File::create(&file_path).map_err(|e| format!("创建文件失败: {}", e))?;
    file.write_all(content.as_bytes()).map_err(|e| format!("写入文件失败: {}", e))?;

    let file_size = std::fs::metadata(&file_path)
        .map(|m| m.len())
        .unwrap_or(0);

    Ok(ExportResult {
        format: req.format.clone(),
        record_count: records.len() as u32,
        file_path: file_path.to_string_lossy().to_string(),
        file_size_bytes: file_size,
    })
}

fn export_as_json(records: &[SimulationArchive]) -> String {
    serde_json::to_string_pretty(&records).unwrap_or_else(|_| "[]".to_string())
}

fn export_as_csv(records: &[SimulationArchive]) -> String {
    let mut csv = String::from("id,project_id,simulation_type,material_name,solver_name,solve_time_sec,converged,iterations,archived_at,notes\n");
    for r in records {
        let material = r.material_name.as_deref().unwrap_or("");
        let project = r.project_id.as_deref().unwrap_or("");
        let solver = r.solver_name.as_deref().unwrap_or("");
        let solve_time = r.solve_time_sec.map(|t| t.to_string()).unwrap_or_default();
        let converged = r.converged.map(|c| if c { "1" } else { "0" }).unwrap_or("");
        let iters = r.iterations.map(|i| i.to_string()).unwrap_or_default();
        let notes = r.notes.as_deref().unwrap_or("").replace(',', ";");
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{}\n",
            r.id, project, r.simulation_type, material, solver, solve_time, converged, iters, r.archived_at, notes
        ));
    }
    csv
}

/// 删除归档记录
pub fn delete_archive(conn: &rusqlite::Connection, id: &str) -> Result<bool, String> {
    let affected = conn.execute(
        "DELETE FROM simulation_archives WHERE id = ?1",
        rusqlite::params![id],
    ).map_err(|e| format!("删除失败: {}", e))?;
    Ok(affected > 0)
}

// ============================================================================
// Tauri 命令 (需要 Database 状态)
// ============================================================================

use crate::db::Database;

/// 归档仿真结果
#[tauri::command]
pub async fn archive_simulation_result(
    db: tauri::State<'_, Database>,
    request: ArchiveRequest,
) -> Result<SimulationArchive, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    archive_result(&conn, &request)
}

/// 查询归档记录
#[tauri::command]
pub async fn query_simulation_archives(
    db: tauri::State<'_, Database>,
    filter: ArchiveFilter,
) -> Result<Vec<SimulationArchive>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    query_archives(&conn, &filter)
}

/// 获取归档统计
#[tauri::command]
pub async fn get_archive_statistics(
    db: tauri::State<'_, Database>,
) -> Result<ArchiveStats, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    get_archive_stats(&conn)
}

/// 导出归档数据
#[tauri::command]
pub async fn export_simulation_archives(
    app: tauri::AppHandle,
    db: tauri::State<'_, Database>,
    request: ExportRequest,
) -> Result<ExportResult, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| format!("获取应用数据目录失败: {}", e))?;
    export_archives(&conn, &request, &app_data_dir)
}

/// 删除归档记录
#[tauri::command]
pub async fn delete_simulation_archive(
    db: tauri::State<'_, Database>,
    id: String,
) -> Result<bool, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    delete_archive(&conn, &id)
}
