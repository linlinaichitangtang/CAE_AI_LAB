//! 工作流模板管理模块 (V1.9)
//!
//! 本模块提供工作流模板的浏览、下载、上传、评分等功能：
//! - 模板列表与详情查询
//! - 模板下载与上传
//! - 模板评分与评论
//! - 从流水线创建模板

use serde::{Deserialize, Serialize};
use tauri::command;

// ============================================================================
// 数据结构定义
// ============================================================================

/// 模板类别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TemplateCategory {
    /// 多尺度集成
    MultiscaleIntegration,
    /// 参数化分析
    ParametricAnalysis,
    /// 优化设计
    OptimizationDesign,
    /// 材料建模
    MaterialModeling,
    /// 热力学分析
    ThermalAnalysis,
    /// 结构分析
    StructuralAnalysis,
}

/// 模板难度
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

/// 工作流模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTemplate {
    pub template_id: String,
    pub name: String,
    pub description: String,
    pub category: TemplateCategory,
    pub difficulty: DifficultyLevel,
    pub author: String,
    pub version: String,
    pub tags: Vec<String>,
    pub download_count: u32,
    pub rating_avg: f64,
    pub rating_count: u32,
    pub created_at: String,
    pub updated_at: String,
    pub steps_count: u32,
    pub estimated_time_min: u32,
    pub is_official: bool,
    pub thumbnail_url: Option<String>,
}

/// 模板详情（包含步骤信息）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateDetail {
    pub template: WorkflowTemplate,
    pub steps: Vec<TemplateStep>,
    pub required_modules: Vec<String>,
    pub input_parameters: Vec<ParameterDef>,
    pub output_description: String,
}

/// 模板步骤
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateStep {
    pub step_index: u32,
    pub name: String,
    pub description: String,
    pub module: String,
    pub estimated_time_sec: u32,
    pub input_keys: Vec<String>,
    pub output_keys: Vec<String>,
}

/// 参数定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDef {
    pub key: String,
    pub name: String,
    pub data_type: String,
    pub default_value: serde_json::Value,
    pub description: String,
    pub required: bool,
}

/// 模板评论
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateReview {
    pub review_id: String,
    pub user_id: String,
    pub username: String,
    pub rating: u32,
    pub comment: String,
    pub created_at: String,
}

/// 模板评分请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateTemplateRequest {
    pub template_id: String,
    pub rating: u32,
    pub comment: Option<String>,
}

/// 上传模板请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadTemplateRequest {
    pub name: String,
    pub description: String,
    pub category: TemplateCategory,
    pub difficulty: DifficultyLevel,
    pub tags: Vec<String>,
    pub steps: Vec<TemplateStep>,
    pub required_modules: Vec<String>,
    pub input_parameters: Vec<ParameterDef>,
}

/// 从流水线创建模板请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFromPipelineRequest {
    pub pipeline_id: String,
    pub name: String,
    pub description: String,
    pub category: TemplateCategory,
    pub tags: Vec<String>,
}

// ============================================================================
// Mock 数据
// ============================================================================

fn get_mock_templates() -> Vec<WorkflowTemplate> {
    vec![
        WorkflowTemplate {
            template_id: "tpl_001".to_string(),
            name: "高强钢多尺度力学性能预测".to_string(),
            description: "从DFT电子结构到宏观力学性能的完整多尺度流水线，涵盖析出相强化、位错交互和晶体塑性模拟".to_string(),
            category: TemplateCategory::MultiscaleIntegration,
            difficulty: DifficultyLevel::Expert,
            author: "CAELab Official".to_string(),
            version: "1.2.0".to_string(),
            tags: vec!["多尺度".to_string(), "高强钢".to_string(), "析出强化".to_string()],
            download_count: 342,
            rating_avg: 4.8,
            rating_count: 56,
            created_at: "2026-01-15T08:00:00Z".to_string(),
            updated_at: "2026-03-20T10:30:00Z".to_string(),
            steps_count: 4,
            estimated_time_min: 180,
            is_official: true,
            thumbnail_url: Some("/assets/templates/multiscale_steel.png".to_string()),
        },
        WorkflowTemplate {
            template_id: "tpl_002".to_string(),
            name: "镁合金蠕变寿命评估".to_string(),
            description: "基于晶体塑性有限元和蠕变本构模型的镁合金高温蠕变寿命预测模板".to_string(),
            category: TemplateCategory::MultiscaleIntegration,
            difficulty: DifficultyLevel::Advanced,
            author: "CAELab Official".to_string(),
            version: "1.0.0".to_string(),
            tags: vec!["镁合金".to_string(), "蠕变".to_string(), "寿命预测".to_string()],
            download_count: 198,
            rating_avg: 4.5,
            rating_count: 32,
            created_at: "2026-02-10T09:00:00Z".to_string(),
            updated_at: "2026-03-15T14:00:00Z".to_string(),
            steps_count: 4,
            estimated_time_min: 240,
            is_official: true,
            thumbnail_url: Some("/assets/templates/mg_creep.png".to_string()),
        },
        WorkflowTemplate {
            template_id: "tpl_003".to_string(),
            name: "陶瓷增材制造工艺优化".to_string(),
            description: "陶瓷3D打印全流程模拟，从粉末铺展到烧结致密化的多物理场耦合分析".to_string(),
            category: TemplateCategory::MultiscaleIntegration,
            difficulty: DifficultyLevel::Expert,
            author: "CAELab Official".to_string(),
            version: "1.1.0".to_string(),
            tags: vec!["陶瓷".to_string(), "增材制造".to_string(), "多物理场".to_string()],
            download_count: 156,
            rating_avg: 4.6,
            rating_count: 28,
            created_at: "2026-02-20T11:00:00Z".to_string(),
            updated_at: "2026-03-18T16:00:00Z".to_string(),
            steps_count: 4,
            estimated_time_min: 300,
            is_official: true,
            thumbnail_url: Some("/assets/templates/ceramic_am.png".to_string()),
        },
        WorkflowTemplate {
            template_id: "tpl_004".to_string(),
            name: "复合材料参数化刚度分析".to_string(),
            description: "纤维增强复合材料层合板刚度参数化扫描，自动生成设计空间图谱".to_string(),
            category: TemplateCategory::ParametricAnalysis,
            difficulty: DifficultyLevel::Intermediate,
            author: "Dr. Zhang".to_string(),
            version: "1.0.0".to_string(),
            tags: vec!["复合材料".to_string(), "参数化".to_string(), "刚度".to_string()],
            download_count: 267,
            rating_avg: 4.3,
            rating_count: 41,
            created_at: "2026-01-20T10:00:00Z".to_string(),
            updated_at: "2026-03-10T12:00:00Z".to_string(),
            steps_count: 3,
            estimated_time_min: 60,
            is_official: false,
            thumbnail_url: None,
        },
        WorkflowTemplate {
            template_id: "tpl_005".to_string(),
            name: "拓扑优化-悬臂梁轻量化".to_string(),
            description: "基于SIMP方法的悬臂梁拓扑优化，支持多种约束条件和目标函数配置".to_string(),
            category: TemplateCategory::OptimizationDesign,
            difficulty: DifficultyLevel::Beginner,
            author: "CAELab Official".to_string(),
            version: "2.0.0".to_string(),
            tags: vec!["拓扑优化".to_string(), "轻量化".to_string(), "SIMP".to_string()],
            download_count: 891,
            rating_avg: 4.9,
            rating_count: 134,
            created_at: "2025-11-01T08:00:00Z".to_string(),
            updated_at: "2026-03-25T09:00:00Z".to_string(),
            steps_count: 3,
            estimated_time_min: 30,
            is_official: true,
            thumbnail_url: Some("/assets/templates/topology_beam.png".to_string()),
        },
        WorkflowTemplate {
            template_id: "tpl_006".to_string(),
            name: "CALPHAD相图计算与验证".to_string(),
            description: "二元/三元合金体系相图计算，支持多种热力学数据库".to_string(),
            category: TemplateCategory::MaterialModeling,
            difficulty: DifficultyLevel::Intermediate,
            author: "Prof. Li".to_string(),
            version: "1.0.0".to_string(),
            tags: vec!["CALPHAD".to_string(), "相图".to_string(), "热力学".to_string()],
            download_count: 178,
            rating_avg: 4.4,
            rating_count: 25,
            created_at: "2026-02-05T14:00:00Z".to_string(),
            updated_at: "2026-03-12T11:00:00Z".to_string(),
            steps_count: 2,
            estimated_time_min: 45,
            is_official: false,
            thumbnail_url: None,
        },
        WorkflowTemplate {
            template_id: "tpl_007".to_string(),
            name: "电子封装热-结构耦合分析".to_string(),
            description: "BGA封装热应力分析，涵盖焊点疲劳寿命预测和热膨胀失配评估".to_string(),
            category: TemplateCategory::ThermalAnalysis,
            difficulty: DifficultyLevel::Advanced,
            author: "CAELab Official".to_string(),
            version: "1.3.0".to_string(),
            tags: vec!["电子封装".to_string(), "热应力".to_string(), "焊点疲劳".to_string()],
            download_count: 423,
            rating_avg: 4.7,
            rating_count: 67,
            created_at: "2025-12-10T09:00:00Z".to_string(),
            updated_at: "2026-03-22T15:00:00Z".to_string(),
            steps_count: 3,
            estimated_time_min: 90,
            is_official: true,
            thumbnail_url: Some("/assets/templates/electronics_pkg.png".to_string()),
        },
        WorkflowTemplate {
            template_id: "tpl_008".to_string(),
            name: "CFD翼型气动优化".to_string(),
            description: "二维翼型气动性能参数化分析与优化，自动寻找最优翼型几何".to_string(),
            category: TemplateCategory::OptimizationDesign,
            difficulty: DifficultyLevel::Advanced,
            author: "Dr. Wang".to_string(),
            version: "1.0.0".to_string(),
            tags: vec!["CFD".to_string(), "翼型".to_string(), "气动优化".to_string()],
            download_count: 234,
            rating_avg: 4.2,
            rating_count: 38,
            created_at: "2026-01-28T13:00:00Z".to_string(),
            updated_at: "2026-03-08T10:00:00Z".to_string(),
            steps_count: 4,
            estimated_time_min: 120,
            is_official: false,
            thumbnail_url: None,
        },
        WorkflowTemplate {
            template_id: "tpl_009".to_string(),
            name: "分子动力学-相场桥接".to_string(),
            description: "MD模拟结果到相场模型参数的自动化传递，实现跨尺度无缝衔接".to_string(),
            category: TemplateCategory::MultiscaleIntegration,
            difficulty: DifficultyLevel::Expert,
            author: "CAELab Official".to_string(),
            version: "1.0.0".to_string(),
            tags: vec!["MD".to_string(), "相场".to_string(), "跨尺度".to_string()],
            download_count: 145,
            rating_avg: 4.5,
            rating_count: 22,
            created_at: "2026-03-01T10:00:00Z".to_string(),
            updated_at: "2026-03-20T14:00:00Z".to_string(),
            steps_count: 3,
            estimated_time_min: 150,
            is_official: true,
            thumbnail_url: Some("/assets/templates/md_phasefield.png".to_string()),
        },
        WorkflowTemplate {
            template_id: "tpl_010".to_string(),
            name: "结构模态分析与验证".to_string(),
            description: "结构固有频率和振型计算，支持实验模态数据对比验证".to_string(),
            category: TemplateCategory::StructuralAnalysis,
            difficulty: DifficultyLevel::Beginner,
            author: "CAELab Official".to_string(),
            version: "1.1.0".to_string(),
            tags: vec!["模态分析".to_string(), "固有频率".to_string(), "振型".to_string()],
            download_count: 567,
            rating_avg: 4.6,
            rating_count: 89,
            created_at: "2025-10-15T08:00:00Z".to_string(),
            updated_at: "2026-03-15T09:00:00Z".to_string(),
            steps_count: 2,
            estimated_time_min: 20,
            is_official: true,
            thumbnail_url: Some("/assets/templates/modal_analysis.png".to_string()),
        },
        WorkflowTemplate {
            template_id: "tpl_011".to_string(),
            name: "形状优化-应力集中缓解".to_string(),
            description: "基于形状优化的孔洞/缺口应力集中缓解，自动生成光滑过渡几何".to_string(),
            category: TemplateCategory::OptimizationDesign,
            difficulty: DifficultyLevel::Intermediate,
            author: "Dr. Chen".to_string(),
            version: "1.0.0".to_string(),
            tags: vec!["形状优化".to_string(), "应力集中".to_string(), "疲劳".to_string()],
            download_count: 189,
            rating_avg: 4.3,
            rating_count: 30,
            created_at: "2026-02-18T11:00:00Z".to_string(),
            updated_at: "2026-03-14T16:00:00Z".to_string(),
            steps_count: 3,
            estimated_time_min: 75,
            is_official: false,
            thumbnail_url: None,
        },
        WorkflowTemplate {
            template_id: "tpl_012".to_string(),
            name: "生物力学关节接触分析".to_string(),
            description: "人体膝关节接触力学分析，涵盖软骨力学、韧带约束和运动学仿真".to_string(),
            category: TemplateCategory::StructuralAnalysis,
            difficulty: DifficultyLevel::Advanced,
            author: "CAELab Official".to_string(),
            version: "1.0.0".to_string(),
            tags: vec!["生物力学".to_string(), "关节".to_string(), "接触".to_string()],
            download_count: 112,
            rating_avg: 4.4,
            rating_count: 18,
            created_at: "2026-03-05T09:00:00Z".to_string(),
            updated_at: "2026-03-19T13:00:00Z".to_string(),
            steps_count: 3,
            estimated_time_min: 100,
            is_official: true,
            thumbnail_url: Some("/assets/templates/biomech_joint.png".to_string()),
        },
    ]
}

fn get_mock_reviews(template_id: &str) -> Vec<TemplateReview> {
    match template_id {
        "tpl_001" => vec![
            TemplateReview {
                review_id: "rev_001".to_string(),
                user_id: "user_001".to_string(),
                username: "张教授".to_string(),
                rating: 5,
                comment: "非常完整的多尺度流水线，从DFT到宏观性能预测一气呵成，结果与实验吻合很好。".to_string(),
                created_at: "2026-03-18T10:00:00Z".to_string(),
            },
            TemplateReview {
                review_id: "rev_002".to_string(),
                user_id: "user_002".to_string(),
                username: "李博士".to_string(),
                rating: 4,
                comment: "步骤清晰，参数设置合理。建议增加更多中间结果可视化选项。".to_string(),
                created_at: "2026-03-15T14:30:00Z".to_string(),
            },
            TemplateReview {
                review_id: "rev_003".to_string(),
                user_id: "user_003".to_string(),
                username: "王工".to_string(),
                rating: 5,
                comment: "帮我们团队节省了大量时间，直接用这个模板完成了项目验证。".to_string(),
                created_at: "2026-03-12T09:15:00Z".to_string(),
            },
        ],
        _ => vec![
            TemplateReview {
                review_id: "rev_010".to_string(),
                user_id: "user_010".to_string(),
                username: "匿名用户".to_string(),
                rating: 4,
                comment: "模板质量不错，文档清晰。".to_string(),
                created_at: "2026-03-10T08:00:00Z".to_string(),
            },
        ],
    }
}

// ============================================================================
// 命令实现
// ============================================================================

/// 列出所有工作流模板
#[command]
pub fn list_templates(
    category: Option<String>,
    difficulty: Option<String>,
    search: Option<String>,
    page: Option<u32>,
    page_size: Option<u32>,
) -> Result<serde_json::Value, String> {
    tracing::info!(
        "列出工作流模板，category={:?}, difficulty={:?}, search={:?}",
        category, difficulty, search
    );

    let mut templates = get_mock_templates();

    // 按类别过滤
    if let Some(ref cat) = category {
        templates.retain(|t| format!("{:?}", t.category).to_lowercase() == cat.to_lowercase());
    }

    // 按难度过滤
    if let Some(ref diff) = difficulty {
        templates.retain(|t| format!("{:?}", t.difficulty).to_lowercase() == diff.to_lowercase());
    }

    // 按搜索关键词过滤
    if let Some(ref kw) = search {
        let kw_lower = kw.to_lowercase();
        templates.retain(|t| {
            t.name.to_lowercase().contains(&kw_lower)
                || t.description.to_lowercase().contains(&kw_lower)
                || t.tags.iter().any(|tag| tag.to_lowercase().contains(&kw_lower))
        });
    }

    let total = templates.len() as u32;
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(12);
    let start = ((page - 1) * page_size) as usize;
    let end = (start + page_size as usize).min(templates.len());
    let page_templates = if start < templates.len() {
        &templates[start..end]
    } else {
        &templates[0..0]
    };

    Ok(serde_json::json!({
        "templates": page_templates,
        "total": total,
        "page": page,
        "page_size": page_size,
        "total_pages:": (total + page_size - 1) / page_size,
    }))
}

/// 获取模板详情
#[command]
pub fn get_template_detail(template_id: String) -> Result<TemplateDetail, String> {
    tracing::info!("获取模板详情: {}", template_id);

    let templates = get_mock_templates();
    let template = templates
        .into_iter()
        .find(|t| t.template_id == template_id)
        .ok_or_else(|| format!("未找到模板: {}", template_id))?;

    let steps = vec![
        TemplateStep {
            step_index: 1,
            name: "参数初始化与预处理".to_string(),
            description: "设置材料参数、边界条件和网格参数".to_string(),
            module: "input_gen".to_string(),
            estimated_time_sec: 30,
            input_keys: vec!["material_params".to_string(), "geometry_config".to_string()],
            output_keys: vec!["mesh_data".to_string(), "bc_config".to_string()],
        },
        TemplateStep {
            step_index: 2,
            name: "求解计算".to_string(),
            description: "执行核心求解器计算".to_string(),
            module: "solver".to_string(),
            estimated_time_sec: 300,
            input_keys: vec!["mesh_data".to_string(), "bc_config".to_string()],
            output_keys: vec!["raw_results".to_string()],
        },
        TemplateStep {
            step_index: 3,
            name: "后处理分析".to_string(),
            description: "结果提取、可视化和统计分析".to_string(),
            module: "postprocess".to_string(),
            estimated_time_sec: 60,
            input_keys: vec!["raw_results".to_string()],
            output_keys: vec!["analysis_report".to_string(), "charts".to_string()],
        },
    ];

    let input_params = vec![
        ParameterDef {
            key: "temperature".to_string(),
            name: "温度 (K)".to_string(),
            data_type: "float".to_string(),
            default_value: serde_json::json!(298.0),
            description: "分析温度".to_string(),
            required: true,
        },
        ParameterDef {
            key: "strain_rate".to_string(),
            name: "应变速率 (1/s)".to_string(),
            data_type: "float".to_string(),
            default_value: serde_json::json!(0.001),
            description: "加载应变速率".to_string(),
            required: false,
        },
    ];

    Ok(TemplateDetail {
        template,
        steps,
        required_modules: vec!["input_gen".to_string(), "solver".to_string(), "postprocess".to_string()],
        input_parameters: input_params,
        output_description: "生成包含应力、应变、位移场的完整分析报告".to_string(),
    })
}

/// 下载模板
#[command]
pub fn download_template(template_id: String) -> Result<serde_json::Value, String> {
    tracing::info!("下载模板: {}", template_id);

    let templates = get_mock_templates();
    let template = templates
        .iter()
        .find(|t| t.template_id == template_id)
        .ok_or_else(|| format!("未找到模板: {}", template_id))?;

    Ok(serde_json::json!({
        "template_id": template.template_id,
        "name": template.name,
        "download_url": format!("/api/v1/templates/{}/download", template_id),
        "file_size_kb": 128,
        "format": "yaml",
        "version": template.version,
        "downloaded_at": "2026-04-03T10:00:00Z",
    }))
}

/// 上传模板
#[command]
pub fn upload_template(request: UploadTemplateRequest) -> Result<WorkflowTemplate, String> {
    tracing::info!("上传模板: {}", request.name);

    let template = WorkflowTemplate {
        template_id: format!("tpl_{}", uuid::Uuid::new_v4().to_string()[..8].to_string()),
        name: request.name,
        description: request.description,
        category: request.category,
        difficulty: request.difficulty,
        author: "current_user".to_string(),
        version: "1.0.0".to_string(),
        tags: request.tags,
        download_count: 0,
        rating_avg: 0.0,
        rating_count: 0,
        created_at: "2026-04-03T10:00:00Z".to_string(),
        updated_at: "2026-04-03T10:00:00Z".to_string(),
        steps_count: request.steps.len() as u32,
        estimated_time_min: request.steps.iter().map(|s| s.estimated_time_sec).sum::<u32>() / 60,
        is_official: false,
        thumbnail_url: None,
    };

    tracing::info!("模板上传成功: {}", template.template_id);
    Ok(template)
}

/// 评分模板
#[command]
pub fn rate_template(request: RateTemplateRequest) -> Result<serde_json::Value, String> {
    tracing::info!(
        "评分模板: {}, rating={}",
        request.template_id,
        request.rating
    );

    if request.rating < 1 || request.rating > 5 {
        return Err("评分必须在 1-5 之间".to_string());
    }

    Ok(serde_json::json!({
        "template_id": request.template_id,
        "rating": request.rating,
        "comment": request.comment,
        "new_avg_rating": 4.6,
        "new_rating_count": 58,
        "rated_at": "2026-04-03T10:00:00Z",
    }))
}

/// 获取模板评论
#[command]
pub fn get_template_reviews(
    template_id: String,
    page: Option<u32>,
    page_size: Option<u32>,
) -> Result<serde_json::Value, String> {
    tracing::info!("获取模板评论: {}", template_id);

    let reviews = get_mock_reviews(&template_id);
    let total = reviews.len() as u32;
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(10);
    let start = ((page - 1) * page_size) as usize;
    let end = (start + page_size as usize).min(reviews.len());
    let page_reviews = if start < reviews.len() {
        &reviews[start..end]
    } else {
        &reviews[0..0]
    };

    Ok(serde_json::json!({
        "reviews": page_reviews,
        "total": total,
        "page": page,
        "page_size": page_size,
    }))
}

/// 从流水线创建模板
#[command]
pub fn create_template_from_pipeline(request: CreateFromPipelineRequest) -> Result<WorkflowTemplate, String> {
    tracing::info!(
        "从流水线 {} 创建模板: {}",
        request.pipeline_id,
        request.name
    );

    let template = WorkflowTemplate {
        template_id: format!("tpl_{}", uuid::Uuid::new_v4().to_string()[..8].to_string()),
        name: request.name,
        description: request.description,
        category: request.category,
        difficulty: DifficultyLevel::Intermediate,
        author: "current_user".to_string(),
        version: "1.0.0".to_string(),
        tags: request.tags,
        download_count: 0,
        rating_avg: 0.0,
        rating_count: 0,
        created_at: "2026-04-03T10:00:00Z".to_string(),
        updated_at: "2026-04-03T10:00:00Z".to_string(),
        steps_count: 4,
        estimated_time_min: 120,
        is_official: false,
        thumbnail_url: None,
    };

    tracing::info!("从流水线创建模板成功: {}", template.template_id);
    Ok(template)
}
