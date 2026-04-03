//! 高通量参数扫描模块 (V1.9)
//!
//! 本模块提供高通量参数扫描与批量计算功能：
//! - 创建和管理参数扫描任务
//! - 扫描状态查询与取消
//! - 结果查询、导出与统计分析
//! - 扫描仪表盘数据

use serde::{Deserialize, Serialize};
use tauri::command;

// ============================================================================
// 数据结构定义
// ============================================================================

/// 扫描状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ScanStatus {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

/// 参数定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDef {
    pub name: String,
    pub display_name: String,
    pub data_type: String,
    pub min_value: f64,
    pub max_value: f64,
    pub step: f64,
    pub values: Option<Vec<f64>>,
}

/// 创建参数扫描请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateParameterScanRequest {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Vec<ParameterDef>,
    pub solver_type: String,
    pub base_config: Option<serde_json::Value>,
    /// 采样策略: "full_factorial" | "latin_hypercube" | "random" | "sobol"
    pub sampling_strategy: String,
    pub max_parallel: Option<u32>,
    pub timeout_per_job_sec: Option<u32>,
}

/// 参数扫描任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterScan {
    pub scan_id: String,
    pub name: String,
    pub description: String,
    pub status: ScanStatus,
    pub solver_type: String,
    pub sampling_strategy: String,
    pub total_combinations: u32,
    pub completed_combinations: u32,
    pub failed_combinations: u32,
    pub parameters: Vec<ParameterDef>,
    pub created_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub estimated_duration_sec: Option<u64>,
    pub actual_duration_sec: Option<u64>,
}

/// 扫描结果条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResultEntry {
    pub combination_id: u32,
    pub parameters: serde_json::Value,
    pub status: String,
    pub result_values: Option<serde_json::Value>,
    pub duration_ms: Option<u64>,
    pub error_message: Option<String>,
    pub completed_at: Option<String>,
}

/// 扫描统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanStatistics {
    pub scan_id: String,
    pub total_combinations: u32,
    pub completed: u32,
    pub failed: u32,
    pub running: u32,
    pub pending: u32,
    pub success_rate: f64,
    pub avg_duration_ms: f64,
    pub min_duration_ms: u64,
    pub max_duration_ms: u64,
    pub total_duration_sec: u64,
    /// 各参数对目标值的灵敏度排名
    pub sensitivity_ranking: Vec<SensitivityEntry>,
}

/// 灵敏度条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivityEntry {
    pub parameter_name: String,
    pub sensitivity_index: f64,
    pub rank: u32,
}

/// 扫描仪表盘
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanDashboard {
    pub scan_id: String,
    pub name: String,
    pub status: ScanStatus,
    pub progress_pct: f64,
    pub estimated_remaining_sec: Option<u64>,
    pub parameter_ranges: serde_json::Value,
    /// 并行度分布
    pub parallelism_history: Vec<ParallelismSnapshot>,
    /// 目标值随参数变化的趋势
    pub result_trends: serde_json::Value,
}

/// 并行度快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelismSnapshot {
    pub timestamp: String,
    pub active_jobs: u32,
    pub queued_jobs: u32,
}

/// 导出格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub scan_id: String,
    pub format: String, // "csv" | "json" | "parquet"
    pub include_metadata: Option<bool>,
    pub filter: Option<serde_json::Value>,
}

// ============================================================================
// Mock 数据生成
// ============================================================================

/// 生成50个参数组合的扫描结果
fn generate_scan_results(total: u32) -> Vec<ScanResultEntry> {
    (0..total)
        .map(|i| {
            let t = i as f64 / total as f64;
            // 模拟参数值
            let param_a = 100.0 + t * 900.0; // 100 ~ 1000
            let param_b = 0.01 + (i as f64 % 10.0) * 0.005; // 0.01 ~ 0.055
            let param_c = 1.0 + (i as f64 % 5.0) * 0.5; // 1.0 ~ 3.0

            // 模拟结果值（带一些物理意义的伪函数）
            let stress = 250.0 + param_a * 0.15 - param_b * 5000.0 + param_c * 20.0 + (i as f64 % 7.0) * 3.0;
            let strain = stress / 210000.0 * (1.0 + param_b * 10.0);
            let safety_factor = 350.0 / stress;

            let status = if i == 17 || i == 33 { "failed" } else { "completed" };

            ScanResultEntry {
                combination_id: i + 1,
                parameters: serde_json::json!({
                    "temperature_K": param_a,
                    "strain_rate": param_b,
                    "grain_size_um": param_c,
                }),
                status: status.to_string(),
                result_values: if status == "completed" {
                    Some(serde_json::json!({
                        "max_stress_mpa": stress,
                        "max_strain": strain,
                        "safety_factor": safety_factor,
                        "converged": true,
                    }))
                } else {
                    None
                },
                duration_ms: Some(12000 + (i % 20) as u64 * 3000),
                error_message: if status == "failed" {
                    Some("求解器不收敛".to_string())
                } else {
                    None
                },
                completed_at: Some(format!("2026-04-03T10:{:02}:{:02}Z", 10 + (i / 6), (i % 6) * 10)),
            }
        })
        .collect()
}

/// 生成灵敏度排名
fn generate_sensitivity_ranking() -> Vec<SensitivityEntry> {
    vec![
        SensitivityEntry {
            parameter_name: "temperature_K".to_string(),
            sensitivity_index: 0.72,
            rank: 1,
        },
        SensitivityEntry {
            parameter_name: "strain_rate".to_string(),
            sensitivity_index: 0.45,
            rank: 2,
        },
        SensitivityEntry {
            parameter_name: "grain_size_um".to_string(),
            sensitivity_index: 0.18,
            rank: 3,
        },
    ]
}

// ============================================================================
// 命令实现
// ============================================================================

/// 创建参数扫描任务
#[command]
pub fn create_parameter_scan(request: CreateParameterScanRequest) -> Result<ParameterScan, String> {
    tracing::info!("创建参数扫描: {}", request.name);

    // 计算组合数
    let total_combinations = match request.sampling_strategy.as_str() {
        "full_factorial" => request
            .parameters
            .iter()
            .map(|p| {
                if let Some(ref vals) = p.values {
                    vals.len() as u32
                } else {
                    ((p.max_value - p.min_value) / p.step).ceil() as u32 + 1
                }
            })
            .product(),
        "latin_hypercube" | "sobol" => 50,
        "random" => 50,
        _ => return Err(format!("不支持的采样策略: {}", request.sampling_strategy)),
    };

    let scan = ParameterScan {
        scan_id: format!("scan_{}", uuid::Uuid::new_v4().to_string()[..8].to_string()),
        name: request.name,
        description: request.description.unwrap_or_default(),
        status: ScanStatus::Pending,
        solver_type: request.solver_type,
        sampling_strategy: request.sampling_strategy,
        total_combinations,
        completed_combinations: 0,
        failed_combinations: 0,
        parameters: request.parameters,
        created_at: "2026-04-03T10:00:00Z".to_string(),
        started_at: None,
        completed_at: None,
        estimated_duration_sec: Some(total_combinations as u64 * 15),
        actual_duration_sec: None,
    };

    tracing::info!(
        "参数扫描创建成功: {}，共 {} 种组合",
        scan.scan_id,
        total_combinations
    );
    Ok(scan)
}

/// 获取扫描状态
#[command]
pub fn get_scan_status(scan_id: String) -> Result<ParameterScan, String> {
    tracing::info!("查询扫描状态: {}", scan_id);

    Ok(ParameterScan {
        scan_id: scan_id.clone(),
        name: "高强钢温度-应变速率参数扫描".to_string(),
        description: "扫描温度和应变速率对高强钢力学性能的影响".to_string(),
        status: ScanStatus::Completed,
        solver_type: "abaqus_standard".to_string(),
        sampling_strategy: "latin_hypercube".to_string(),
        total_combinations: 50,
        completed_combinations: 48,
        failed_combinations: 2,
        parameters: vec![
            ParameterDef {
                name: "temperature_K".to_string(),
                display_name: "温度 (K)".to_string(),
                data_type: "float".to_string(),
                min_value: 100.0,
                max_value: 1000.0,
                step: 100.0,
                values: None,
            },
            ParameterDef {
                name: "strain_rate".to_string(),
                display_name: "应变速率 (1/s)".to_string(),
                data_type: "float".to_string(),
                min_value: 0.001,
                max_value: 0.1,
                step: 0.01,
                values: None,
            },
            ParameterDef {
                name: "grain_size_um".to_string(),
                display_name: "晶粒尺寸 (um)".to_string(),
                data_type: "float".to_string(),
                min_value: 1.0,
                max_value: 5.0,
                step: 0.5,
                values: None,
            },
        ],
        created_at: "2026-04-03T10:00:00Z".to_string(),
        started_at: Some("2026-04-03T10:01:00Z".to_string()),
        completed_at: Some("2026-04-03T10:13:30Z".to_string()),
        estimated_duration_sec: Some(750),
        actual_duration_sec: Some(750),
    })
}

/// 取消扫描任务
#[command]
pub fn cancel_scan(scan_id: String) -> Result<serde_json::Value, String> {
    tracing::info!("取消扫描任务: {}", scan_id);

    Ok(serde_json::json!({
        "scan_id": scan_id,
        "status": "cancelled",
        "cancelled_at": "2026-04-03T10:05:00Z",
        "completed_combinations": 15,
        "message": "扫描任务已取消，已完成的 15 个组合结果已保留",
    }))
}

/// 查询扫描结果
#[command]
pub fn query_scan_results(
    scan_id: String,
    page: Option<u32>,
    page_size: Option<u32>,
    status_filter: Option<String>,
    sort_by: Option<String>,
    sort_order: Option<String>,
) -> Result<serde_json::Value, String> {
    tracing::info!(
        "查询扫描结果: {}, filter={:?}, sort={:?}",
        scan_id, status_filter, sort_by
    );

    let mut results = generate_scan_results(50);

    // 状态过滤
    if let Some(ref filter) = status_filter {
        results.retain(|r| r.status == *filter);
    }

    // 排序
    if let Some(ref sort_key) = sort_by {
        let desc = sort_order.as_deref() == Some("desc");
        results.sort_by(|a, b| {
            let va = a.result_values.as_ref().and_then(|v| v.get(sort_key)).and_then(|v| v.as_f64()).unwrap_or(0.0);
            let vb = b.result_values.as_ref().and_then(|v| v.get(sort_key)).and_then(|v| v.as_f64()).unwrap_or(0.0);
            if desc { vb.partial_cmp(&va) } else { va.partial_cmp(&vb) }
        }.unwrap_or(std::cmp::Ordering::Equal));
    }

    let total = results.len() as u32;
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(20);
    let start = ((page - 1) * page_size) as usize;
    let end = (start + page_size as usize).min(results.len());
    let page_results = if start < results.len() {
        &results[start..end]
    } else {
        &results[0..0]
    };

    Ok(serde_json::json!({
        "scan_id": scan_id,
        "results": page_results,
        "total": total,
        "page": page,
        "page_size": page_size,
        "total_pages": (total + page_size - 1) / page_size,
    }))
}

/// 导出扫描结果
#[command]
pub fn export_scan_results(request: ExportRequest) -> Result<serde_json::Value, String> {
    tracing::info!(
        "导出扫描结果: {}, format={}",
        request.scan_id,
        request.format
    );

    let supported_formats = vec!["csv", "json", "parquet"];
    if !supported_formats.contains(&request.format.as_str()) {
        return Err(format!(
            "不支持的导出格式: {}，支持: {}",
            request.format,
            supported_formats.join(", ")
        ));
    }

    let results = generate_scan_results(50);
    let completed_count = results.iter().filter(|r| r.status == "completed").count();

    Ok(serde_json::json!({
        "scan_id": request.scan_id,
        "export_format": request.format,
        "file_name": format!("scan_results_{}.{}", request.scan_id, request.format),
        "file_size_bytes": completed_count * 256,
        "record_count": completed_count,
        "download_url": format!("/api/v1/scans/{}/export/{}", request.scan_id, request.format),
        "expires_at": "2026-04-04T10:00:00Z",
    }))
}

/// 获取扫描统计信息
#[command]
pub fn get_scan_statistics(scan_id: String) -> Result<ScanStatistics, String> {
    tracing::info!("获取扫描统计: {}", scan_id);

    let results = generate_scan_results(50);
    let completed: Vec<_> = results.iter().filter(|r| r.status == "completed").collect();
    let failed_count = results.iter().filter(|r| r.status == "failed").count();

    let durations: Vec<u64> = completed.iter().filter_map(|r| r.duration_ms).collect();
    let avg_duration = if durations.is_empty() { 0.0 } else { durations.iter().sum::<u64>() as f64 / durations.len() as f64 };
    let min_duration = *durations.iter().min().unwrap_or(&0);
    let max_duration = *durations.iter().max().unwrap_or(&0);

    Ok(ScanStatistics {
        scan_id,
        total_combinations: 50,
        completed: completed.len() as u32,
        failed: failed_count as u32,
        running: 0,
        pending: 0,
        success_rate: completed.len() as f64 / 50.0 * 100.0,
        avg_duration_ms: avg_duration,
        min_duration_ms: min_duration,
        max_duration_ms: max_duration,
        total_duration_sec: 750,
        sensitivity_ranking: generate_sensitivity_ranking(),
    })
}

/// 获取扫描仪表盘数据
#[command]
pub fn get_scan_dashboard(scan_id: String) -> Result<ScanDashboard, String> {
    tracing::info!("获取扫描仪表盘: {}", scan_id);

    // 并行度历史快照
    let parallelism_history: Vec<ParallelismSnapshot> = (0..10)
        .map(|i| ParallelismSnapshot {
            timestamp: format!("2026-04-03T10:{:02}:00Z", i),
            active_jobs: if i < 8 { 4 } else { 2 },
            queued_jobs: if i < 7 { 46 - i * 5 } else { 0 },
        })
        .collect();

    // 结果趋势（按温度分箱的应力分布）
    let result_trends = serde_json::json!({
        "by_temperature": [
            {"bin": "100-250K", "avg_stress_mpa": 265.3, "std_stress_mpa": 12.5, "count": 10},
            {"bin": "250-400K", "avg_stress_mpa": 312.8, "std_stress_mpa": 18.3, "count": 10},
            {"bin": "400-550K", "avg_stress_mpa": 358.1, "std_stress_mpa": 22.7, "count": 10},
            {"bin": "550-700K", "avg_stress_mpa": 389.5, "std_stress_mpa": 28.4, "count": 8},
            {"bin": "700-1000K", "avg_stress_mpa": 401.2, "std_stress_mpa": 35.1, "count": 10},
        ],
        "pareto_front": [
            {"temperature_K": 300.0, "strain_rate": 0.005, "max_stress_mpa": 298.5, "safety_factor": 1.17},
            {"temperature_K": 400.0, "strain_rate": 0.01, "max_stress_mpa": 325.3, "safety_factor": 1.08},
            {"temperature_K": 500.0, "strain_rate": 0.02, "max_stress_mpa": 348.7, "safety_factor": 1.00},
        ],
    });

    Ok(ScanDashboard {
        scan_id: scan_id.clone(),
        name: "高强钢温度-应变速率参数扫描".to_string(),
        status: ScanStatus::Completed,
        progress_pct: 100.0,
        estimated_remaining_sec: None,
        parameter_ranges: serde_json::json!({
            "temperature_K": {"min": 100.0, "max": 1000.0},
            "strain_rate": {"min": 0.001, "max": 0.1},
            "grain_size_um": {"min": 1.0, "max": 5.0},
        }),
        parallelism_history,
        result_trends,
    })
}
