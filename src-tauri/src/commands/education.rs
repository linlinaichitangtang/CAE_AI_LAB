#![allow(dead_code)]
/**
 * V4.3 教育市场模块
 * - V4.3-001: 课程模板管理
 * - V4.3-002: 作业发布与提交
 * - V4.3-003: 仿真实验自动批改
 * - V4.3-004: 学习进度追踪
 * - V4.3-005: 提交相似度检测
 */

use serde::{Deserialize, Serialize};
use tauri::command;

// ============================================================================
// 数据结构定义
// ============================================================================

/// 课程信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: String,
    pub name: String,
    pub description: String,
    pub instructor_id: String,
    pub semester: String,
    pub created_at: String,
    pub updated_at: String,
    pub status: CourseStatus,
    pub experiment_count: u32,
    pub enrolled_students: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CourseStatus {
    Draft,
    Active,
    Archived,
}

/// 仿真实验模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentTemplate {
    pub id: String,
    pub course_id: String,
    pub name: String,
    pub description: String,
    pub order: u32,
    pub difficulty: ExperimentDifficulty,
    pub category: String,
    pub guide_content: String,
    pub reference_model: Option<String>,
    pub grading_rubric: GradingRubric,
    pub max_score: f64,
    pub estimated_hours: f64,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExperimentDifficulty {
    Beginner,
    Intermediate,
    Advanced,
}

/// 评分标准
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradingRubric {
    pub criteria: Vec<GradingCriterion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradingCriterion {
    pub id: String,
    pub name: String,
    pub description: String,
    pub max_points: f64,
    pub check_type: CheckType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CheckType {
    MeshQuality,
    BoundaryConditions,
    MaterialProperties,
    ResultAccuracy,
    Convergence,
    FileFormat,
    Documentation,
    Custom(String),
}

/// 学生信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub id: String,
    pub user_id: String,
    pub student_number: String,
    pub name: String,
    pub email: String,
    pub enrolled_courses: Vec<String>,
    pub created_at: String,
}

/// 作业
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    pub id: String,
    pub course_id: String,
    pub experiment_id: String,
    pub title: String,
    pub description: String,
    pub due_date: String,
    pub max_score: f64,
    pub allow_late_submission: bool,
    pub late_penalty_percent: f64,
    pub status: AssignmentStatus,
    pub created_at: String,
    pub submitted_count: u32,
    pub graded_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssignmentStatus {
    Draft,
    Published,
    Closed,
}

/// 学生提交
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub id: String,
    pub assignment_id: String,
    pub student_id: String,
    pub student_name: String,
    pub submitted_at: String,
    pub status: SubmissionStatus,
    pub project_file_path: Option<String>,
    pub report_file_path: Option<String>,
    pub notes: Option<String>,
    pub grade: Option<Grade>,
    pub is_late: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SubmissionStatus {
    Submitted,
    Grading,
    Graded,
    Returned,
}

/// 评分结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grade {
    pub total_score: f64,
    pub max_score: f64,
    pub percentage: f64,
    pub criterion_scores: Vec<CriterionScore>,
    pub feedback: String,
    pub graded_at: String,
    pub graded_by: String,
    pub auto_grade: Option<AutoGrade>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriterionScore {
    pub criterion_id: String,
    pub criterion_name: String,
    pub points_earned: f64,
    pub points_possible: f64,
    pub feedback: Option<String>,
}

/// 自动评分结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoGrade {
    pub overall_score: f64,
    pub check_results: Vec<AutoCheckResult>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub computation_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoCheckResult {
    pub check_type: String,
    pub passed: bool,
    pub score: f64,
    pub max_score: f64,
    pub details: String,
}

/// 学习进度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningProgress {
    pub student_id: String,
    pub course_id: String,
    pub completed_experiments: Vec<String>,
    pub skill_levels: SkillLevels,
    pub overall_progress_percent: f64,
    pub weak_areas: Vec<String>,
    pub strong_areas: Vec<String>,
    pub last_activity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillLevels {
    pub mesh_generation: f64,
    pub material_selection: f64,
    pub boundary_conditions: f64,
    pub solver_setup: f64,
    pub post_processing: f64,
    pub result_interpretation: f64,
}

/// 相似度检测结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityResult {
    pub submission_a_id: String,
    pub submission_b_id: String,
    pub student_a_name: String,
    pub student_b_name: String,
    pub overall_similarity: f64,
    pub content_similarity: f64,
    pub structure_similarity: f64,
    pub flag: SimilarityFlag,
    pub details: Vec<SimilarityDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SimilarityFlag {
    Normal,
    Warning,
    Suspicious,
    Plagiarism,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityDetail {
    pub file_a: String,
    pub file_b: String,
    pub similarity_percent: f64,
    pub matching_sections: Vec<MatchingSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchingSection {
    pub line_start: u32,
    pub line_end: u32,
    pub content: String,
}

// ============================================================================
// Tauri Commands - 课程管理
// ============================================================================

/// 创建新课程
#[command]
pub fn create_course(
    name: String,
    description: String,
    instructor_id: String,
    semester: String,
) -> Result<Course, String> {
    if name.is_empty() {
        return Err("课程名称不能为空".to_string());
    }

    let course = Course {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        description,
        instructor_id,
        semester,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
        status: CourseStatus::Draft,
        experiment_count: 0,
        enrolled_students: 0,
    };

    Ok(course)
}

/// 获取课程列表
#[command]
pub fn list_courses(instructor_id: Option<String>) -> Result<Vec<Course>, String> {
    // Mock data for now
    let courses = vec![
        Course {
            id: "course-001".to_string(),
            name: "结构力学仿真基础".to_string(),
            description: "学习有限元分析的基础知识，包括网格划分、边界条件设置和结果分析".to_string(),
            instructor_id: instructor_id.clone().unwrap_or_default(),
            semester: "2026春季".to_string(),
            created_at: "2026-01-15T08:00:00Z".to_string(),
            updated_at: "2026-05-01T10:30:00Z".to_string(),
            status: CourseStatus::Active,
            experiment_count: 8,
            enrolled_students: 45,
        },
        Course {
            id: "course-002".to_string(),
            name: "热传导与热应力分析".to_string(),
            description: "热传导方程求解、热应力计算、热-结构耦合分析".to_string(),
            instructor_id: instructor_id.unwrap_or_default(),
            semester: "2026春季".to_string(),
            created_at: "2026-02-01T09:00:00Z".to_string(),
            updated_at: "2026-04-20T14:00:00Z".to_string(),
            status: CourseStatus::Active,
            experiment_count: 6,
            enrolled_students: 32,
        },
    ];

    Ok(courses)
}

/// 获取课程详情
#[command]
pub fn get_course(course_id: String) -> Result<Course, String> {
    Ok(Course {
        id: course_id,
        name: "结构力学仿真基础".to_string(),
        description: "学习有限元分析的基础知识".to_string(),
        instructor_id: "prof-001".to_string(),
        semester: "2026春季".to_string(),
        created_at: "2026-01-15T08:00:00Z".to_string(),
        updated_at: "2026-05-01T10:30:00Z".to_string(),
        status: CourseStatus::Active,
        experiment_count: 8,
        enrolled_students: 45,
    })
}

/// 添加实验模板到课程
#[command]
pub fn add_experiment_to_course(
    course_id: String,
    name: String,
    description: String,
    category: String,
    guide_content: String,
    difficulty: String,
) -> Result<ExperimentTemplate, String> {
    let diff = match difficulty.as_str() {
        "beginner" => ExperimentDifficulty::Beginner,
        "intermediate" => ExperimentDifficulty::Intermediate,
        "advanced" => ExperimentDifficulty::Advanced,
        _ => ExperimentDifficulty::Intermediate,
    };

    let experiment = ExperimentTemplate {
        id: uuid::Uuid::new_v4().to_string(),
        course_id,
        name,
        description,
        order: 1,
        difficulty: diff,
        category,
        guide_content,
        reference_model: None,
        grading_rubric: GradingRubric {
            criteria: vec![
                GradingCriterion {
                    id: "mesh".to_string(),
                    name: "网格质量".to_string(),
                    description: "网格划分是否合理，单元质量是否满足要求".to_string(),
                    max_points: 20.0,
                    check_type: CheckType::MeshQuality,
                },
                GradingCriterion {
                    id: "bc".to_string(),
                    name: "边界条件".to_string(),
                    description: "边界条件设置是否正确、完整".to_string(),
                    max_points: 30.0,
                    check_type: CheckType::BoundaryConditions,
                },
                GradingCriterion {
                    id: "result".to_string(),
                    name: "结果准确性".to_string(),
                    description: "数值结果与理论解的误差是否在可接受范围内".to_string(),
                    max_points: 50.0,
                    check_type: CheckType::ResultAccuracy,
                },
            ],
        },
        max_score: 100.0,
        estimated_hours: 4.0,
        tags: vec![],
    };

    Ok(experiment)
}

/// 获取课程的实验列表
#[command]
pub fn list_experiments(course_id: String) -> Result<Vec<ExperimentTemplate>, String> {
    let experiments = vec![
        ExperimentTemplate {
            id: "exp-001".to_string(),
            course_id: course_id.clone(),
            name: "悬臂梁静强度分析".to_string(),
            description: "分析悬臂梁在集中载荷作用下的应力和变形".to_string(),
            order: 1,
            difficulty: ExperimentDifficulty::Beginner,
            category: "structural".to_string(),
            guide_content: "## 实验目的\n\n学习有限元分析的基本流程...".to_string(),
            reference_model: Some("cantilever_beam.caeproj".to_string()),
            grading_rubric: GradingRubric { criteria: vec![] },
            max_score: 100.0,
            estimated_hours: 3.0,
            tags: vec!["静力学".to_string(), "梁".to_string()],
        },
        ExperimentTemplate {
            id: "exp-002".to_string(),
            course_id,
            name: "薄壁压力容器应力分析".to_string(),
            description: "分析内压作用下薄壁圆筒的环向应力和轴向应力".to_string(),
            order: 2,
            difficulty: ExperimentDifficulty::Intermediate,
            category: "structural".to_string(),
            guide_content: "## 实验目的\n\n学习轴对称问题的有限元建模...".to_string(),
            reference_model: None,
            grading_rubric: GradingRubric { criteria: vec![] },
            max_score: 100.0,
            estimated_hours: 4.0,
            tags: vec!["压力容器".to_string(), "轴对称".to_string()],
        },
    ];

    Ok(experiments)
}

// ============================================================================
// Tauri Commands - 作业管理
// ============================================================================

/// 发布作业
#[command]
pub fn create_assignment(
    course_id: String,
    experiment_id: String,
    title: String,
    description: String,
    due_date: String,
    max_score: f64,
) -> Result<Assignment, String> {
    if title.is_empty() {
        return Err("作业标题不能为空".to_string());
    }

    let assignment = Assignment {
        id: uuid::Uuid::new_v4().to_string(),
        course_id,
        experiment_id,
        title,
        description,
        due_date,
        max_score,
        allow_late_submission: true,
        late_penalty_percent: 10.0,
        status: AssignmentStatus::Published,
        created_at: chrono::Utc::now().to_rfc3339(),
        submitted_count: 0,
        graded_count: 0,
    };

    Ok(assignment)
}

/// 获取作业列表
#[command]
pub fn list_assignments(course_id: String) -> Result<Vec<Assignment>, String> {
    let assignments = vec![
        Assignment {
            id: "assign-001".to_string(),
            course_id: course_id.clone(),
            experiment_id: "exp-001".to_string(),
            title: "第一次作业：悬臂梁静强度".to_string(),
            description: "完成悬臂梁的静强度分析，提交 .caeproj 文件和实验报告".to_string(),
            due_date: "2026-06-15T23:59:59Z".to_string(),
            max_score: 100.0,
            allow_late_submission: true,
            late_penalty_percent: 10.0,
            status: AssignmentStatus::Published,
            created_at: "2026-05-01T08:00:00Z".to_string(),
            submitted_count: 38,
            graded_count: 25,
        },
        Assignment {
            id: "assign-002".to_string(),
            course_id,
            experiment_id: "exp-002".to_string(),
            title: "第二次作业：压力容器应力分析".to_string(),
            description: "完成压力容器的应力分析，注意轴对称建模".to_string(),
            due_date: "2026-07-01T23:59:59Z".to_string(),
            max_score: 100.0,
            allow_late_submission: true,
            late_penalty_percent: 10.0,
            status: AssignmentStatus::Published,
            created_at: "2026-05-15T09:00:00Z".to_string(),
            submitted_count: 15,
            graded_count: 0,
        },
    ];

    Ok(assignments)
}

/// 学生提交作业
#[command]
pub fn submit_assignment(
    assignment_id: String,
    student_id: String,
    student_name: String,
    project_file_path: Option<String>,
    report_file_path: Option<String>,
    notes: Option<String>,
) -> Result<Submission, String> {
    let now = chrono::Utc::now().to_rfc3339();

    let submission = Submission {
        id: uuid::Uuid::new_v4().to_string(),
        assignment_id,
        student_id,
        student_name,
        submitted_at: now,
        status: SubmissionStatus::Submitted,
        project_file_path,
        report_file_path,
        notes,
        grade: None,
        is_late: false,
    };

    Ok(submission)
}

/// 获取提交列表
#[command]
pub fn list_submissions(assignment_id: String) -> Result<Vec<Submission>, String> {
    let submissions = vec![
        Submission {
            id: "sub-001".to_string(),
            assignment_id: assignment_id.clone(),
            student_id: "student-001".to_string(),
            student_name: "张三".to_string(),
            submitted_at: "2026-06-10T14:30:00Z".to_string(),
            status: SubmissionStatus::Graded,
            project_file_path: Some("/uploads/zhang3/caeproj".to_string()),
            report_file_path: Some("/uploads/zhang3/report.pdf".to_string()),
            notes: Some("已完成所有要求".to_string()),
            grade: Some(Grade {
                total_score: 85.0,
                max_score: 100.0,
                percentage: 85.0,
                criterion_scores: vec![],
                feedback: "网格划分质量较好，边界条件设置正确，但应力集中区域分析不够详细".to_string(),
                graded_at: "2026-06-12T10:00:00Z".to_string(),
                graded_by: "prof-001".to_string(),
                auto_grade: None,
            }),
            is_late: false,
        },
        Submission {
            id: "sub-002".to_string(),
            assignment_id,
            student_id: "student-002".to_string(),
            student_name: "李四".to_string(),
            submitted_at: "2026-06-15T22:00:00Z".to_string(),
            status: SubmissionStatus::Submitted,
            project_file_path: Some("/uploads/li4/caeproj".to_string()),
            report_file_path: None,
            notes: None,
            grade: None,
            is_late: false,
        },
    ];

    Ok(submissions)
}

// ============================================================================
// Tauri Commands - 自动批改
// ============================================================================

/// 自动批改提交
#[command]
pub async fn auto_grade_submission(
    _submission_id: String,
    experiment_id: String,
    project_file_path: String,
) -> Result<AutoGrade, String> {
    let start = std::time::Instant::now();
    let mut check_results = Vec::new();
    let mut warnings = Vec::new();
    let mut total_score = 0.0;

    // 检查 1: 网格质量
    let mesh_score = check_mesh_quality(&project_file_path)?;
    check_results.push(AutoCheckResult {
        check_type: "mesh_quality".to_string(),
        passed: mesh_score >= 15.0,
        score: mesh_score,
        max_score: 20.0,
        details: format!("网格质量评分: {:.1}/20", mesh_score),
    });
    total_score += mesh_score;

    // 检查 2: 边界条件
    let bc_score = check_boundary_conditions(&project_file_path)?;
    check_results.push(AutoCheckResult {
        check_type: "boundary_conditions".to_string(),
        passed: bc_score >= 20.0,
        score: bc_score,
        max_score: 30.0,
        details: format!("边界条件评分: {:.1}/30", bc_score),
    });
    total_score += bc_score;

    // 检查 3: 结果准确性
    let result_score = check_result_accuracy(&project_file_path, &experiment_id)?;
    check_results.push(AutoCheckResult {
        check_type: "result_accuracy".to_string(),
        passed: result_score >= 35.0,
        score: result_score,
        max_score: 50.0,
        details: format!("结果准确性评分: {:.1}/50", result_score),
    });
    total_score += result_score;

    if total_score < 60.0 {
        warnings.push("总分低于60分，请检查实验设置".to_string());
    }

    let computation_time = start.elapsed().as_millis() as u64;

    Ok(AutoGrade {
        overall_score: total_score,
        check_results,
        warnings,
        errors: vec![],
        computation_time_ms: computation_time,
    })
}

/// 检查网格质量
fn check_mesh_quality(_project_path: &str) -> Result<f64, String> {
    // Mock: 随机生成合理分数
    Ok(18.0) // 默认给高分，实际会检查单元畸变度等
}

/// 检查边界条件
fn check_boundary_conditions(_project_path: &str) -> Result<f64, String> {
    Ok(25.0)
}

/// 检查结果准确性
fn check_result_accuracy(_project_path: &str, _experiment_id: &str) -> Result<f64, String> {
    Ok(42.0)
}

// ============================================================================
// Tauri Commands - 学习进度
// ============================================================================

/// 获取学生学习进度
#[command]
pub fn get_learning_progress(
    student_id: String,
    course_id: String,
) -> Result<LearningProgress, String> {
    Ok(LearningProgress {
        student_id,
        course_id,
        completed_experiments: vec!["exp-001".to_string()],
        skill_levels: SkillLevels {
            mesh_generation: 75.0,
            material_selection: 60.0,
            boundary_conditions: 80.0,
            solver_setup: 70.0,
            post_processing: 55.0,
            result_interpretation: 65.0,
        },
        overall_progress_percent: 45.0,
        weak_areas: vec!["后处理".to_string(), "结果解读".to_string()],
        strong_areas: vec!["边界条件设置".to_string()],
        last_activity: "2026-06-10T14:30:00Z".to_string(),
    })
}

/// 获取班级整体进度统计
#[command]
pub fn get_class_progress_stats(
    course_id: String,
) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "course_id": course_id,
        "total_students": 45,
        "active_students": 38,
        "average_progress": 52.3,
        "completion_rate": 0.28,
        "average_score": 72.5,
        "top_skills": ["boundary_conditions", "mesh_generation"],
        "weak_skills": ["post_processing", "result_interpretation"],
    }))
}

// ============================================================================
// Tauri Commands - 相似度检测
// ============================================================================

/// 检测提交相似度
#[command]
pub async fn check_submission_similarity(
    _assignment_id: String,
) -> Result<Vec<SimilarityResult>, String> {
    let mut results = Vec::new();

    // Mock: 返回示例结果
    results.push(SimilarityResult {
        submission_a_id: "sub-001".to_string(),
        submission_b_id: "sub-003".to_string(),
        student_a_name: "张三".to_string(),
        student_b_name: "王五".to_string(),
        overall_similarity: 0.92,
        content_similarity: 0.95,
        structure_similarity: 0.88,
        flag: SimilarityFlag::Suspicious,
        details: vec![],
    });

    results.push(SimilarityResult {
        submission_a_id: "sub-002".to_string(),
        submission_b_id: "sub-004".to_string(),
        student_a_name: "李四".to_string(),
        student_b_name: "赵六".to_string(),
        overall_similarity: 0.15,
        content_similarity: 0.12,
        structure_similarity: 0.18,
        flag: SimilarityFlag::Normal,
        details: vec![],
    });

    Ok(results)
}

/// 计算两个字符串的相似度 (Levenshtein distance based)
fn calculate_text_similarity(a: &str, b: &str) -> f64 {
    let len_a = a.len();
    let len_b = b.len();
    if len_a == 0 && len_b == 0 {
        return 1.0;
    }
    if len_a == 0 || len_b == 0 {
        return 0.0;
    }

    let max_len = len_a.max(len_b);
    let mut matrix = vec![vec![0usize; len_b + 1]; len_a + 1];

    for i in 0..=len_a {
        matrix[i][0] = i;
    }
    for j in 0..=len_b {
        matrix[0][j] = j;
    }

    for (i, ca) in a.chars().enumerate() {
        for (j, cb) in b.chars().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }

    let distance = matrix[len_a][len_b];
    1.0 - (distance as f64 / max_len as f64)
}
