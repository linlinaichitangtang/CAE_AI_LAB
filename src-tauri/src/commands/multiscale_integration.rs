//! 多尺度集成流水线模块 (V1.9)
//!
//! 本模块提供端到端多尺度集成流水线功能：
//! - 运行集成流水线（多步骤自动串联）
//! - 获取流水线状态与步骤结果
//! - 端到端测试用例管理
//! - 与文献结果对比验证

use serde::{Deserialize, Serialize};
use tauri::command;

// ============================================================================
// 数据结构定义
// ============================================================================

/// 流水线步骤状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StepStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
}

/// 流水线步骤结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub step_id: String,
    pub step_name: String,
    pub status: StepStatus,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub duration_ms: Option<u64>,
    pub output_summary: Option<String>,
    pub output_data: Option<serde_json::Value>,
    pub error_message: Option<String>,
}

/// 流水线状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PipelineStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// 流水线运行信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineRun {
    pub pipeline_id: String,
    pub scenario_id: String,
    pub scenario_name: String,
    pub status: PipelineStatus,
    pub current_step: usize,
    pub total_steps: usize,
    pub steps: Vec<StepResult>,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub total_duration_ms: Option<u64>,
}

/// 运行流水线请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunPipelineRequest {
    pub scenario_id: String,
    pub parameters: Option<serde_json::Value>,
    pub skip_steps: Option<Vec<String>>,
}

/// 端到端测试用例
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndToEndCase {
    pub case_id: String,
    pub name: String,
    pub description: String,
    pub scenario_id: String,
    pub expected_results: serde_json::Value,
    pub tolerance: f64,
    pub last_run_status: Option<String>,
    pub last_run_at: Option<String>,
}

/// 文献对比结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteratureComparison {
    pub metric_name: String,
    pub simulation_value: f64,
    pub literature_value: f64,
    pub relative_error_pct: f64,
    pub literature_source: String,
    pub passed: bool,
}

/// 文献对比报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteratureComparisonReport {
    pub pipeline_id: String,
    pub scenario_name: String,
    pub comparisons: Vec<LiteratureComparison>,
    pub overall_passed: bool,
    pub summary: String,
}

// ============================================================================
// Mock 数据
// ============================================================================

/// 场景定义
fn get_scenarios() -> Vec<(String, String, String)> {
    vec![
        ("scenario_001".to_string(), "高强钢析出相分析".to_string(), "precipitation_analysis".to_string()),
        ("scenario_002".to_string(), "镁合金蠕变预测".to_string(), "magnesium_creep".to_string()),
        ("scenario_003".to_string(), "陶瓷增材制造模拟".to_string(), "ceramic_am_simulation".to_string()),
    ]
}

/// 生成4步流水线结果
fn generate_pipeline_steps(scenario_id: &str) -> Vec<StepResult> {
    let step_names = match scenario_id {
        "scenario_001" => vec![
            ("step_1", "DFT电子结构计算"),
            ("step_2", "CALPHAD相图计算"),
            ("step_3", "相场析出动力学模拟"),
            ("step_4", "宏观力学性能预测"),
        ],
        "scenario_002" => vec![
            ("step_1", "晶界扩散系数计算"),
            ("step_2", "位错攀移速率参数化"),
            ("step_3", "晶体塑性有限元模拟"),
            ("step_4", "蠕变寿命预测"),
        ],
        "scenario_003" => vec![
            ("step_1", "粉末床热力学模拟"),
            ("step_2", "温度场-应力场耦合分析"),
            ("step_3", "微观组织演化模拟"),
            ("step_4", "残余应力与变形预测"),
        ],
        _ => vec![
            ("step_1", "步骤1：微观尺度计算"),
            ("step_2", "步骤2：介观尺度传递"),
            ("step_3", "步骤3：宏观尺度模拟"),
            ("step_4", "步骤4：结果验证与输出"),
        ],
    };

    let base_time = "2026-04-03T10:00:00Z";

    step_names
        .into_iter()
        .enumerate()
        .map(|(idx, (sid, sname))| {
            let duration = 120000 + (idx as u64) * 80000; // 120s, 200s, 280s, 360s
            StepResult {
                step_id: sid.to_string(),
                step_name: sname.to_string(),
                status: StepStatus::Completed,
                started_at: Some(base_time.to_string()),
                completed_at: Some(base_time.to_string()),
                duration_ms: Some(duration),
                output_summary: Some(format!("{} 完成，输出 {} 个数据点", sname, 1000 + idx * 500)),
                output_data: Some(serde_json::json!({
                    "data_points": 1000 + idx * 500,
                    "convergence_achieved": true,
                    "key_value": format!("{:.4}", 0.85 + idx as f64 * 0.05),
                })),
                error_message: None,
            }
        })
        .collect()
}

/// 生成端到端测试用例
fn get_end_to_end_cases() -> Vec<EndToEndCase> {
    vec![
        EndToEndCase {
            case_id: "e2e_001".to_string(),
            name: "高强钢析出相 - 标准验证".to_string(),
            description: "验证高强钢析出相分析流水线在标准参数下的输出精度".to_string(),
            scenario_id: "scenario_001".to_string(),
            expected_results: serde_json::json!({"yield_strength_mpa": 1200.0, "precipitate_volume_fraction": 0.05}),
            tolerance: 5.0,
            last_run_status: Some("passed".to_string()),
            last_run_at: Some("2026-04-02T14:30:00Z".to_string()),
        },
        EndToEndCase {
            case_id: "e2e_002".to_string(),
            name: "高强钢析出相 - 高温工况".to_string(),
            description: "验证高温条件下析出相稳定性预测".to_string(),
            scenario_id: "scenario_001".to_string(),
            expected_results: serde_json::json!({"critical_temperature_k": 873.0}),
            tolerance: 3.0,
            last_run_status: Some("passed".to_string()),
            last_run_at: Some("2026-04-01T09:15:00Z".to_string()),
        },
        EndToEndCase {
            case_id: "e2e_003".to_string(),
            name: "镁合金蠕变 - 短期预测".to_string(),
            description: "验证100小时短期蠕变应变预测精度".to_string(),
            scenario_id: "scenario_002".to_string(),
            expected_results: serde_json::json!({"strain_at_100h": 0.012}),
            tolerance: 10.0,
            last_run_status: Some("passed".to_string()),
            last_run_at: Some("2026-04-02T16:00:00Z".to_string()),
        },
        EndToEndCase {
            case_id: "e2e_004".to_string(),
            name: "镁合金蠕变 - 长期外推".to_string(),
            description: "验证1000小时长期蠕变寿命外推精度".to_string(),
            scenario_id: "scenario_002".to_string(),
            expected_results: serde_json::json!({"rupture_time_h": 1500.0}),
            tolerance: 15.0,
            last_run_status: Some("failed".to_string()),
            last_run_at: Some("2026-04-01T11:20:00Z".to_string()),
        },
        EndToEndCase {
            case_id: "e2e_005".to_string(),
            name: "陶瓷增材制造 - 热应力验证".to_string(),
            description: "验证陶瓷3D打印过程中的热应力分布预测".to_string(),
            scenario_id: "scenario_003".to_string(),
            expected_results: serde_json::json!({"max_residual_stress_mpa": 350.0}),
            tolerance: 8.0,
            last_run_status: Some("passed".to_string()),
            last_run_at: Some("2026-04-03T08:00:00Z".to_string()),
        },
        EndToEndCase {
            case_id: "e2e_006".to_string(),
            name: "陶瓷增材制造 - 变形预测".to_string(),
            description: "验证烧结后翘曲变形量预测精度".to_string(),
            scenario_id: "scenario_003".to_string(),
            expected_results: serde_json::json!({"max_warpage_mm": 0.45}),
            tolerance: 12.0,
            last_run_status: Some("passed".to_string()),
            last_run_at: Some("2026-04-02T10:45:00Z".to_string()),
        },
    ]
}

// ============================================================================
// 命令实现
// ============================================================================

/// 运行集成流水线
#[command]
pub fn run_integration_pipeline(request: RunPipelineRequest) -> Result<PipelineRun, String> {
    tracing::info!("启动集成流水线，场景: {}", request.scenario_id);

    let scenarios = get_scenarios();
    let scenario = scenarios
        .iter()
        .find(|s| s.0 == request.scenario_id)
        .ok_or_else(|| format!("未找到场景: {}", request.scenario_id))?;

    let steps = generate_pipeline_steps(&request.scenario_id);
    let total_duration: u64 = steps.iter().filter_map(|s| s.duration_ms).sum();

    let pipeline_run = PipelineRun {
        pipeline_id: format!("pipeline_{}", uuid::Uuid::new_v4().to_string()[..8].to_string()),
        scenario_id: scenario.0.clone(),
        scenario_name: scenario.1.clone(),
        status: PipelineStatus::Completed,
        current_step: steps.len(),
        total_steps: steps.len(),
        steps,
        started_at: "2026-04-03T10:00:00Z".to_string(),
        completed_at: Some("2026-04-03T10:16:00Z".to_string()),
        total_duration_ms: Some(total_duration),
    };

    tracing::info!(
        "流水线 {} 完成，共 {} 步，总耗时 {}ms",
        pipeline_run.pipeline_id,
        pipeline_run.total_steps,
        total_duration
    );

    Ok(pipeline_run)
}

/// 获取流水线状态
#[command]
pub fn get_pipeline_status(pipeline_id: String) -> Result<PipelineRun, String> {
    tracing::info!("查询流水线状态: {}", pipeline_id);

    // Mock: 返回一个示例流水线状态
    let scenarios = get_scenarios();
    let scenario = &scenarios[0];
    let steps = generate_pipeline_steps(&scenario.0);

    Ok(PipelineRun {
        pipeline_id: pipeline_id.clone(),
        scenario_id: scenario.0.clone(),
        scenario_name: scenario.1.clone(),
        status: PipelineStatus::Completed,
        current_step: steps.len(),
        total_steps: steps.len(),
        steps,
        started_at: "2026-04-03T10:00:00Z".to_string(),
        completed_at: Some("2026-04-03T10:16:00Z".to_string()),
        total_duration_ms: Some(960000),
    })
}

/// 获取步骤结果
#[command]
pub fn get_step_result(pipeline_id: String, step_id: String) -> Result<StepResult, String> {
    tracing::info!("查询流水线 {} 的步骤 {} 结果", pipeline_id, step_id);

    let steps = generate_pipeline_steps("scenario_001");
    let step = steps
        .into_iter()
        .find(|s| s.step_id == step_id)
        .ok_or_else(|| format!("未找到步骤: {}", step_id))?;

    Ok(step)
}

/// 重试失败的步骤
#[command]
pub fn retry_step(pipeline_id: String, step_id: String) -> Result<StepResult, String> {
    tracing::info!("重试流水线 {} 的步骤 {}", pipeline_id, step_id);

    let mut steps = generate_pipeline_steps("scenario_001");
    let step = steps
        .iter_mut()
        .find(|s| s.step_id == step_id)
        .ok_or_else(|| format!("未找到步骤: {}", step_id))?;

    step.status = StepStatus::Completed;
    step.duration_ms = Some(150000);
    step.error_message = None;
    step.output_summary = Some(format!("步骤 {} 重试成功", step_id));

    Ok(step.clone())
}

/// 列出端到端测试用例
#[command]
pub fn list_end_to_end_cases() -> Result<Vec<EndToEndCase>, String> {
    tracing::info!("列出所有端到端测试用例");

    let cases = get_end_to_end_cases();
    tracing::info!("共 {} 个端到端测试用例", cases.len());
    Ok(cases)
}

/// 运行端到端测试用例
#[command]
pub fn run_end_to_end_case(case_id: String) -> Result<serde_json::Value, String> {
    tracing::info!("运行端到端测试用例: {}", case_id);

    let cases = get_end_to_end_cases();
    let case = cases
        .iter()
        .find(|c| c.case_id == case_id)
        .ok_or_else(|| format!("未找到测试用例: {}", case_id))?;

    // Mock: 模拟测试结果
    let passed = case.case_id != "e2e_004"; // e2e_004 模拟失败

    Ok(serde_json::json!({
        "case_id": case.case_id,
        "name": case.name,
        "status": if passed { "passed" } else { "failed" },
        "actual_results": {
            "yield_strength_mpa": 1185.0,
            "precipitate_volume_fraction": 0.048,
        },
        "expected_results": case.expected_results,
        "tolerance": case.tolerance,
        "max_deviation_pct": 1.25,
        "ran_at": "2026-04-03T10:30:00Z",
        "duration_ms": 245000,
    }))
}

/// 与文献结果对比
#[command]
pub fn compare_with_literature(pipeline_id: String) -> Result<LiteratureComparisonReport, String> {
    tracing::info!("对比流水线 {} 结果与文献数据", pipeline_id);

    let comparisons = vec![
        LiteratureComparison {
            metric_name: "屈服强度 (MPa)".to_string(),
            simulation_value: 1185.0,
            literature_value: 1200.0,
            relative_error_pct: 1.25,
            literature_source: "Zhang et al., Acta Materialia, 2024".to_string(),
            passed: true,
        },
        LiteratureComparison {
            metric_name: "析出相体积分数".to_string(),
            simulation_value: 0.048,
            literature_value: 0.050,
            relative_error_pct: 4.0,
            literature_source: "Li et al., Materials Science & Engineering A, 2023".to_string(),
            passed: true,
        },
        LiteratureComparison {
            metric_name: "蠕变应变 (100h)".to_string(),
            simulation_value: 0.0115,
            literature_value: 0.0120,
            relative_error_pct: 4.17,
            literature_source: "Wang et al., Journal of Magnesium and Alloys, 2024".to_string(),
            passed: true,
        },
        LiteratureComparison {
            metric_name: "残余应力 (MPa)".to_string(),
            simulation_value: 365.0,
            literature_value: 350.0,
            relative_error_pct: 4.29,
            literature_source: "Chen et al., Ceramics International, 2023".to_string(),
            passed: true,
        },
    ];

    let all_passed = comparisons.iter().all(|c| c.passed);

    Ok(LiteratureComparisonReport {
        pipeline_id,
        scenario_name: "高强钢析出相分析".to_string(),
        comparisons,
        overall_passed: all_passed,
        summary: if all_passed {
            "所有指标均在容差范围内，与文献吻合良好".to_string()
        } else {
            "部分指标超出容差范围，建议检查参数设置".to_string()
        },
    })
}
