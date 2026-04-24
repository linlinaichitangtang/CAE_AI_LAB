/**
 * V2.8 材料数据中台 + 主动学习引擎 — Rust 后端命令
 * - V2.8-001: Material Property DB Schema 扩展
 * - V2.8-002: 多维度数据查询引擎
 * - V2.8-003: 数据质量评分 / 覆盖度分析
 * - V2.8-004: 数据导入导出
 * - V2.8-005: 不确定性量化模块
 * - V2.8-006: 贝叶斯优化采样策略
 * - V2.8-008: 闭环验证 Pipeline
 * - V2.8-009: 主动学习实验报告
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// V2.8-001: Material Property DB Schema
// ============================================================================

/// 材料性能数据记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MaterialPropertyRecord {
    pub id: String,
    pub project_id: Option<String>,
    /// 材料名称
    pub material_name: String,
    /// 材料成分 (JSON): {"Fe": 70.0, "Cr": 19.0, "Ni": 9.0}
    pub composition: String,
    /// 工艺参数 (JSON): {"temperature": 550, "pressure": 50, "hold_time": 2}
    pub processing_params: String,
    /// 性能属性 (JSON): {"elastic_modulus": 210e9, "yield_strength": 235e6}
    pub properties: String,
    /// 数据来源: "simulation" | "experiment" | "literature" | "ml_prediction"
    pub data_source: String,
    /// 验证状态: "pending" | "verified" | "rejected"
    pub validation_status: String,
    /// 不确定性 (JSON): {"elastic_modulus": 5e9, "yield_strength": 10e6}
    pub uncertainty: Option<String>,
    /// 模型名称 (如果是 ML 预测)
    pub model_name: Option<String>,
    /// 备注
    pub notes: Option<String>,
    /// 创建时间
    pub created_at: String,
}

/// 数据写入请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MaterialDataWriteRequest {
    pub project_id: Option<String>,
    pub material_name: String,
    pub composition: serde_json::Value,
    pub processing_params: serde_json::Value,
    pub properties: serde_json::Value,
    pub data_source: String,
    pub validation_status: Option<String>,
    pub uncertainty: Option<serde_json::Value>,
    pub model_name: Option<String>,
    pub notes: Option<String>,
}

// ============================================================================
// V2.8-002: 多维度查询
// ============================================================================

/// 查询过滤条件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MaterialDataQuery {
    pub material_name: Option<String>,
    pub element: Option<String>,
    pub composition_range: Option<CompositionRange>,
    pub processing_param_filter: Option<HashMap<String, serde_json::Value>>,
    pub property_filter: Option<HashMap<String, PropertyRange>>,
    pub data_source: Option<String>,
    pub validation_status: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub order_by: Option<String>,
    pub order_dir: Option<String>, // "asc" | "desc"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompositionRange {
    pub element: String,
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PropertyRange {
    pub min: f64,
    pub max: f64,
}

// ============================================================================
// V2.8-003: 数据质量 / 覆盖度
// ============================================================================

/// 覆盖度分析结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoverageAnalysis {
    /// 总记录数
    pub total_records: u32,
    /// 涉及元素种类数
    pub num_elements: u32,
    /// 涉及材料种类数
    pub num_materials: u32,
    /// 成分空间覆盖度 (0~1)
    pub composition_coverage: f64,
    /// 工艺空间覆盖度 (0~1)
    pub processing_coverage: f64,
    /// 性能空间覆盖度 (0~1)
    pub property_coverage: f64,
    /// 综合覆盖度 (0~1)
    pub overall_coverage: f64,
    /// 数据盲区列表 (推荐补充的区域)
    pub blind_spots: Vec<BlindSpot>,
    /// 数据质量评分 (0~100)
    pub quality_score: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlindSpot {
    /// 盲区描述
    pub description: String,
    /// 相关元素
    pub elements: Vec<String>,
    /// 推荐优先级 (1~5, 1 最高)
    pub priority: u32,
    /// 预期精度提升
    pub expected_improvement: f64,
}

// ============================================================================
// V2.8-005: 不确定性量化
// ============================================================================

/// 不确定性分析请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UncertaintyAnalysisRequest {
    /// 目标属性
    pub target_property: String,
    /// 成分范围 (要分析的参数空间)
    pub composition_space: Vec<CompositionRange>,
    /// 采样点数
    pub num_samples: u32,
}

/// 不确定性分析结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UncertaintyAnalysisResult {
    /// 目标属性
    pub target_property: String,
    /// 采样点数
    pub num_samples: u32,
    /// 平均不确定性
    pub mean_uncertainty: f64,
    /// 最大不确定性
    pub max_uncertainty: f64,
    /// 高不确定性区域 (Top-K)
    pub high_uncertainty_regions: Vec<UncertainRegion>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UncertainRegion {
    /// 区域描述
    pub description: String,
    /// 中心成分
    pub center_composition: HashMap<String, f64>,
    /// 预测不确定性
    pub uncertainty: f64,
    /// 预测值
    pub predicted_value: f64,
}

// ============================================================================
// V2.8-006: 贝叶斯优化采样策略
// ============================================================================

/// 主动学习推荐请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActiveLearningRequest {
    /// 目标属性
    pub target_property: String,
    /// 推荐数量
    pub num_recommendations: u32,
    /// 采样策略: "bayesian" | "greedy" | "random"
    pub strategy: String,
    /// 约束条件 (可选)
    pub constraints: Option<serde_json::Value>,
}

/// 主动学习推荐结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActiveLearningRecommendation {
    /// 推荐的仿真点
    pub recommendations: Vec<RecommendedPoint>,
    /// 预期模型精度提升
    pub expected_improvement: f64,
    /// 推荐策略
    pub strategy: String,
    /// 当前数据覆盖度
    pub current_coverage: f64,
    /// 预期覆盖度 (执行推荐后)
    pub expected_coverage: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecommendedPoint {
    /// 推荐序号
    pub rank: u32,
    /// 推荐材料名称
    pub material_name: String,
    /// 推荐成分
    pub composition: HashMap<String, f64>,
    /// 推荐工艺参数
    pub processing_params: HashMap<String, f64>,
    /// 预期信息增益
    pub expected_information_gain: f64,
    /// 预测不确定性
    pub predicted_uncertainty: f64,
    /// 推荐理由
    pub reason: String,
}

// ============================================================================
// V2.8-008: 闭环验证
// ============================================================================

/// 闭环验证请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClosedLoopRequest {
    /// 推荐点 ID
    pub recommendation_id: String,
    /// 材料名称
    pub material_name: String,
    /// 成分
    pub composition: serde_json::Value,
    /// 工艺参数
    pub processing_params: serde_json::Value,
    /// 目标属性
    pub target_property: String,
}

/// 闭环验证结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClosedLoopResult {
    /// 是否成功
    pub success: bool,
    /// 推荐点 ID
    pub recommendation_id: String,
    /// CAELab 仿真结果 (JSON)
    pub simulation_result: Option<String>,
    /// ML 预测值
    pub ml_predicted_value: Option<f64>,
    /// CAELab 仿真值
    pub caelab_simulated_value: Option<f64>,
    /// 误差
    pub error: Option<f64>,
    /// 模型精度变化
    pub accuracy_change: Option<f64>,
    /// 覆盖度变化
    pub coverage_change: Option<f64>,
    /// 状态
    pub status: String,
}

// ============================================================================
// V2.8-009: 实验报告
// ============================================================================

/// 实验报告摘要
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActiveLearningReport {
    /// 报告标题
    pub title: String,
    /// 生成时间
    pub generated_at: String,
    /// 初始数据量
    pub initial_data_count: u32,
    /// 当前数据量
    pub current_data_count: u32,
    /// 新增数据量
    pub new_data_count: u32,
    /// 初始模型精度 (RMSE)
    pub initial_rmse: f64,
    /// 当前模型精度 (RMSE)
    pub current_rmse: f64,
    /// 精度提升百分比
    pub accuracy_improvement_percent: f64,
    /// 初始覆盖度
    pub initial_coverage: f64,
    /// 当前覆盖度
    pub current_coverage: f64,
    /// 推荐历史
    pub recommendation_history: Vec<RecommendationRecord>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecommendationRecord {
    pub round: u32,
    pub material_name: String,
    pub composition: HashMap<String, f64>,
    pub ml_prediction: f64,
    pub caelab_result: f64,
    pub error: f64,
    pub timestamp: String,
}

// ============================================================================
// Mock 实现
// ============================================================================

fn mock_coverage_analysis() -> CoverageAnalysis {
    CoverageAnalysis {
        total_records: 1250,
        num_elements: 15,
        num_materials: 42,
        composition_coverage: 0.35,
        processing_coverage: 0.28,
        property_coverage: 0.42,
        overall_coverage: 0.35,
        blind_spots: vec![
            BlindSpot {
                description: "高 Cr (>25%) 不锈钢区域数据稀疏".to_string(),
                elements: vec!["Fe".into(), "Cr".into(), "Ni".into()],
                priority: 1,
                expected_improvement: 0.15,
            },
            BlindSpot {
                description: "Al-Ti 合金系缺少高温 (>600°C) 数据".to_string(),
                elements: vec!["Al".into(), "Ti".into()],
                priority: 2,
                expected_improvement: 0.12,
            },
            BlindSpot {
                description: "Cu-Zn 黄铜系低温性能数据不足".to_string(),
                elements: vec!["Cu".into(), "Zn".into()],
                priority: 3,
                expected_improvement: 0.08,
            },
            BlindSpot {
                description: "Ni 基高温合金蠕变数据空白".to_string(),
                elements: vec!["Ni".into(), "Cr".into(), "Co".into(), "Al".into()],
                priority: 2,
                expected_improvement: 0.10,
            },
        ],
        quality_score: 62.0,
    }
}

fn mock_uncertainty_analysis(req: &UncertaintyAnalysisRequest) -> UncertaintyAnalysisResult {
    let mut regions = Vec::new();
    let num = req.num_samples.min(20) as usize;

    for i in 0..num {
        let mut comp = HashMap::new();
        comp.insert("Fe".to_string(), 50.0 + rand_factor() * 30.0);
        comp.insert("Cr".to_string(), 10.0 + rand_factor() * 20.0);
        comp.insert("Ni".to_string(), 5.0 + rand_factor() * 15.0);

        regions.push(UncertainRegion {
            description: format!("高不确定性区域 #{}", i + 1),
            center_composition: comp,
            uncertainty: 0.1 + rand_factor() * 0.2,
            predicted_value: 200e9 + rand_factor() * 50e9,
        });
    }

    regions.sort_by(|a, b| b.uncertainty.partial_cmp(&a.uncertainty).unwrap());

    UncertaintyAnalysisResult {
        target_property: req.target_property.clone(),
        num_samples: req.num_samples,
        mean_uncertainty: 0.12,
        max_uncertainty: regions.first().map(|r| r.uncertainty).unwrap_or(0.0),
        high_uncertainty_regions: regions,
    }
}

fn mock_active_learning(req: &ActiveLearningRequest) -> ActiveLearningRecommendation {
    let mut points = Vec::new();
    let num = req.num_recommendations.min(10) as usize;

    let materials = vec![
        ("Fe-30Cr-10Ni", vec![("Fe", 60.0), ("Cr", 30.0), ("Ni", 10.0)]),
        ("Fe-25Cr-20Ni", vec![("Fe", 55.0), ("Cr", 25.0), ("Ni", 20.0)]),
        ("Al-6Ti", vec![("Al", 94.0), ("Ti", 6.0)]),
        ("Cu-35Zn", vec![("Cu", 65.0), ("Zn", 35.0)]),
        ("Ni-20Cr-5Al", vec![("Ni", 75.0), ("Cr", 20.0), ("Al", 5.0)]),
        ("Ti-6Al-4V", vec![("Ti", 90.0), ("Al", 6.0), ("V", 4.0)]),
        ("Fe-18Cr-8Ni", vec![("Fe", 74.0), ("Cr", 18.0), ("Ni", 8.0)]),
        ("Al-4Cu", vec![("Al", 96.0), ("Cu", 4.0)]),
        ("Ni-50Ti", vec![("Ni", 50.0), ("Ti", 50.0)]),
        ("Fe-70Mn", vec![("Fe", 30.0), ("Mn", 70.0)]),
    ];

    for i in 0..num.min(materials.len()) {
        let (name, comp) = &materials[i];
        let comp_map: HashMap<String, f64> = comp.iter().map(|(k, v)| (k.to_string(), *v)).collect();
        let mut proc = HashMap::new();
        proc.insert("temperature".to_string(), 500.0 + rand_factor() * 300.0);
        proc.insert("hold_time".to_string(), 1.0 + rand_factor() * 4.0);

        points.push(RecommendedPoint {
            rank: (i + 1) as u32,
            material_name: name.to_string(),
            composition: comp_map,
            processing_params: proc,
            expected_information_gain: 0.15 - i as f64 * 0.012,
            predicted_uncertainty: 0.20 - i as f64 * 0.015,
            reason: format!("该区域数据稀疏，不确定性较高 (σ={:.3})", 0.20 - i as f64 * 0.015),
        });
    }

    ActiveLearningRecommendation {
        recommendations: points,
        expected_improvement: 0.18,
        strategy: req.strategy.clone(),
        current_coverage: 0.35,
        expected_coverage: 0.52,
    }
}

fn mock_closed_loop(req: &ClosedLoopRequest) -> ClosedLoopResult {
    let ml_val = 210e9 + rand_factor() * 20e9;
    let sim_val = ml_val * (1.0 + (rand_factor() - 0.5) * 0.05);

    ClosedLoopResult {
        success: true,
        recommendation_id: req.recommendation_id.clone(),
        simulation_result: Some(format!(r#"{{"elastic_modulus": {:.0}, "yield_strength": {:.0}}}"#, sim_val, sim_val * 0.001)),
        ml_predicted_value: Some(ml_val),
        caelab_simulated_value: Some(sim_val),
        error: Some((sim_val - ml_val).abs()),
        accuracy_change: Some(-0.02),
        coverage_change: Some(0.03),
        status: "completed".to_string(),
    }
}

fn mock_report() -> ActiveLearningReport {
    ActiveLearningReport {
        title: "CAELab 主动学习实验报告".to_string(),
        generated_at: chrono::Utc::now().to_rfc3339(),
        initial_data_count: 800,
        current_data_count: 1250,
        new_data_count: 450,
        initial_rmse: 15.2,
        current_rmse: 6.8,
        accuracy_improvement_percent: 55.3,
        initial_coverage: 0.25,
        current_coverage: 0.52,
        recommendation_history: vec![
            RecommendationRecord {
                round: 1,
                material_name: "Fe-30Cr-10Ni".to_string(),
                composition: vec![("Fe", 60.0), ("Cr", 30.0), ("Ni", 10.0)].into_iter().map(|(k, v)| (k.to_string(), v)).collect(),
                ml_prediction: 195e9,
                caelab_result: 198e9,
                error: 3e9,
                timestamp: "2025-06-01T10:00:00Z".to_string(),
            },
            RecommendationRecord {
                round: 2,
                material_name: "Al-6Ti".to_string(),
                composition: vec![("Al", 94.0), ("Ti", 6.0)].into_iter().map(|(k, v)| (k.to_string(), v)).collect(),
                ml_prediction: 72e9,
                caelab_result: 75e9,
                error: 3e9,
                timestamp: "2025-06-02T14:30:00Z".to_string(),
            },
        ],
    }
}

fn rand_factor() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().subsec_nanos();
    (nanos % 1000) as f64 / 1000.0
}

// ============================================================================
// 数据库操作
// ============================================================================

use crate::db::Database;

/// 创建 Material Property 表
pub fn create_material_property_table(conn: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS material_properties (
            id TEXT PRIMARY KEY,
            project_id TEXT,
            material_name TEXT NOT NULL,
            composition TEXT NOT NULL,
            processing_params TEXT,
            properties TEXT NOT NULL,
            data_source TEXT NOT NULL DEFAULT 'simulation',
            validation_status TEXT NOT NULL DEFAULT 'pending',
            uncertainty TEXT,
            model_name TEXT,
            notes TEXT,
            created_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE SET NULL
        );

        CREATE INDEX IF NOT EXISTS idx_matprop_name ON material_properties(material_name);
        CREATE INDEX IF NOT EXISTS idx_matprop_source ON material_properties(data_source);
        CREATE INDEX IF NOT EXISTS idx_matprop_status ON material_properties(validation_status);
        CREATE INDEX IF NOT EXISTS idx_matprop_created ON material_properties(created_at);
        "
    )?;
    Ok(())
}

/// 写入材料数据
pub fn write_material_data(conn: &rusqlite::Connection, req: &MaterialDataWriteRequest) -> Result<MaterialPropertyRecord, String> {
    let id = nanoid::nanoid!(12);
    let created_at = chrono::Utc::now().to_rfc3339();
    let composition_json = serde_json::to_string(&req.composition).unwrap_or_default();
    let processing_json = serde_json::to_string(&req.processing_params).unwrap_or_default();
    let properties_json = serde_json::to_string(&req.properties).unwrap_or_default();
    let uncertainty_json = req.uncertainty.as_ref().map(|u| serde_json::to_string(u).unwrap_or_default());
    let validation_status = req.validation_status.as_deref().unwrap_or("pending");

    conn.execute(
        "INSERT INTO material_properties (id, project_id, material_name, composition, processing_params, properties, data_source, validation_status, uncertainty, model_name, notes, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        rusqlite::params![id, req.project_id, req.material_name, composition_json, processing_json, properties_json, req.data_source, validation_status, uncertainty_json, req.model_name, req.notes, created_at],
    ).map_err(|e| format!("写入失败: {}", e))?;

    Ok(MaterialPropertyRecord {
        id,
        project_id: req.project_id.clone(),
        material_name: req.material_name.clone(),
        composition: composition_json,
        processing_params: processing_json,
        properties: properties_json,
        data_source: req.data_source.clone(),
        validation_status: validation_status.to_string(),
        uncertainty: uncertainty_json,
        model_name: req.model_name.clone(),
        notes: req.notes.clone(),
        created_at,
    })
}

/// 查询材料数据
pub fn query_material_data(conn: &rusqlite::Connection, query: &MaterialDataQuery) -> Result<Vec<MaterialPropertyRecord>, String> {
    let mut sql = String::from("SELECT * FROM material_properties WHERE 1=1");
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref name) = query.material_name {
        sql.push_str(" AND material_name LIKE ?");
        params.push(Box::new(format!("%{}%", name)));
    }
    if let Some(ref source) = query.data_source {
        sql.push_str(" AND data_source = ?");
        params.push(Box::new(source.clone()));
    }
    if let Some(ref status) = query.validation_status {
        sql.push_str(" AND validation_status = ?");
        params.push(Box::new(status.clone()));
    }

    let order_dir = query.order_dir.as_deref().unwrap_or("desc");
    let order_by = query.order_by.as_deref().unwrap_or("created_at");
    sql.push_str(&format!(" ORDER BY {} {}", order_by, order_dir));

    let limit = query.limit.unwrap_or(100);
    let offset = query.offset.unwrap_or(0);
    sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let mut stmt = conn.prepare(&sql).map_err(|e| format!("查询失败: {}", e))?;

    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(MaterialPropertyRecord {
            id: row.get(0)?,
            project_id: row.get(1)?,
            material_name: row.get(2)?,
            composition: row.get(3)?,
            processing_params: row.get(4)?,
            properties: row.get(5)?,
            data_source: row.get(6)?,
            validation_status: row.get(7)?,
            uncertainty: row.get(8)?,
            model_name: row.get(9)?,
            notes: row.get(10)?,
            created_at: row.get(11)?,
        })
    }).map_err(|e| format!("解析失败: {}", e))?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row.map_err(|e| format!("读取行失败: {}", e))?);
    }
    Ok(results)
}

/// 批量导入
pub fn batch_import_material_records(conn: &rusqlite::Connection, records: Vec<MaterialDataWriteRequest>) -> Result<u32, String> {
    let mut count = 0u32;
    for req in &records {
        write_material_data(conn, req)?;
        count += 1;
    }
    Ok(count)
}

// ============================================================================
// Tauri 命令
// ============================================================================

/// 写入材料数据
#[tauri::command]
pub async fn write_material_property(
    db: tauri::State<'_, Database>,
    request: MaterialDataWriteRequest,
) -> Result<MaterialPropertyRecord, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    write_material_data(&conn, &request)
}

/// 查询材料数据
#[tauri::command]
pub async fn query_material_properties(
    db: tauri::State<'_, Database>,
    query: MaterialDataQuery,
) -> Result<Vec<MaterialPropertyRecord>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    query_material_data(&conn, &query)
}

/// 批量导入材料数据
#[tauri::command]
pub async fn batch_import_material_data(
    db: tauri::State<'_, Database>,
    records: Vec<MaterialDataWriteRequest>,
) -> Result<u32, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    batch_import_material_records(&conn, records)
}

/// 数据覆盖度分析
#[tauri::command]
pub async fn analyze_data_coverage() -> Result<CoverageAnalysis, String> {
    Ok(mock_coverage_analysis())
}

/// 不确定性分析
#[tauri::command]
pub async fn analyze_uncertainty(
    request: UncertaintyAnalysisRequest,
) -> Result<UncertaintyAnalysisResult, String> {
    Ok(mock_uncertainty_analysis(&request))
}

/// 主动学习推荐
#[tauri::command]
pub async fn get_active_learning_recommendations(
    request: ActiveLearningRequest,
) -> Result<ActiveLearningRecommendation, String> {
    Ok(mock_active_learning(&request))
}

/// 闭环验证
#[tauri::command]
pub async fn run_closed_loop_verification(
    request: ClosedLoopRequest,
) -> Result<ClosedLoopResult, String> {
    Ok(mock_closed_loop(&request))
}

/// 生成主动学习报告
#[tauri::command]
pub async fn generate_active_learning_report() -> Result<ActiveLearningReport, String> {
    Ok(mock_report())
}
