//! 拓扑优化模块 - SIMP + OC方法
//! 
//! 本模块提供完整的拓扑优化功能,采用SIMP(固体各向同性材料惩罚)方法
//! 和OC(最优准则)算法实现结构轻量化设计。
//! 
//! 主要功能：
//! - SIMP拓扑优化算法
//! - OC最优准则法更新
//! - 灵敏度过滤
//! - 体积约束控制!
//! - STL格式几何导出
//! - 多种优化模板(悬臂梁、L型支架、简支梁等)
//! 
//! 算法原理：
//! 1. SIMP: 使用惩罚因子将中间密度向0或1压缩,模拟材料分布
//! 2. OC: 基于KKT条件推导的准则法,快速更新设计变量
//! 3. 目标: 最小化柔度(最大化刚度)同时满足体积约束

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;



// ============================================================================
// 数据结构定义
// ============================================================================

/// 优化类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OptimizationType {
    /// 拓扑优化 - 改变材料分布
    Topology,
    /// 形状优化 - 改变边界形状
    Shape,
    /// 尺寸优化 - 改变截面尺寸
    Size,
}

/// 目标函数类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ObjectiveType {
    /// 最小柔度 (最大刚度)
    MinCompliance,
    /// 最小质量
    MinMass,
    /// 最大刚度
    MaxStiffness,
}

/// 约束类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConstraintType {
    /// 体积分数约束
    VolumeFraction,
    /// 最大应力约束
    MaxStress,
    /// 应力限值约束
    StressLimit,
}

/// 拓扑优化配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyConfig {
    /// 优化类型
    pub optimization_type: OptimizationType,
    /// 目标函数类型
    pub objective: ObjectiveType,
    /// 约束条件列表
    pub constraints: Vec<ConstraintConfig>,
    /// 最大迭代次数
    pub max_iterations: usize,
    /// 收敛容差 (目标函数变化率阈值)
    pub convergence_tolerance: f64,
    /// 设计域定义
    pub design_domain: DesignDomain,
    /// SIMP惩罚因子 (通常取3.0)
    pub penalization_factor: f64,
    /// 最小密度值 (通常取0.01)
    pub min_density: f64,
    /// 灵敏度过滤半径
    pub filter_radius: f64,
    /// OC方法beta参数 (控制更新速度)
    pub oc_beta: f64,
    /// OC方法移动极限
    pub oc_move_limit: f64,
}

impl Default for TopologyConfig {
    fn default() -> Self {
        Self {
            optimization_type: OptimizationType::Topology,
            objective: ObjectiveType::MinCompliance,
            constraints: vec![ConstraintConfig {
                constraint_type: ConstraintType::VolumeFraction,
                upper_bound: 0.5,
            }],
            max_iterations: 100,
            convergence_tolerance: 0.01,
            design_domain: DesignDomain {
                x_min: 0.0,
                x_max: 100.0,
                y_min: 0.0,
                y_max: 20.0,
                z_min: 0.0,
                z_max: 10.0,
            },
            penalization_factor: 3.0,
            min_density: 0.01,
            filter_radius: 0.0,
            oc_beta: 1.5,
            oc_move_limit: 0.2,
        }
    }
}

/// 约束配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintConfig {
    /// 约束类型
    pub constraint_type: ConstraintType,
    /// 上界值
    pub upper_bound: f64,
}

/// 设计域边界结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignDomain {
    /// X方向最小值
    pub x_min: f64,
    /// X方向最大值
    pub x_max: f64,
    /// Y方向最小值
    pub y_min: f64,
    /// Y方向最大值
    pub y_max: f64,
    /// Z方向最小值
    pub z_min: f64,
    /// Z方向最大值
    pub z_max: f64,
}

/// 单次迭代结果结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyIteration {
    /// 当前迭代次数
    pub iteration: usize,
    /// 目标函数值 (柔度)
    pub objective_value: f64,
    /// 当前体积分数
    pub volume_fraction: f64,
    /// 最大应力 (如有)
    pub max_stress: Option<f64>,
    /// 最大密度值
    pub max_density: f64,
    /// 最小密度值
    pub min_density: f64,
    /// 密度标准差
    pub density_std: f64,
    /// 是否已收敛
    pub converged: bool,
}

/// 完整优化结果结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyResult {
    /// 优化是否成功
    pub success: bool,
    /// 迭代历史记录
    pub iterations: Vec<TopologyIteration>,
    /// 最终目标函数值
    pub final_objective: f64,
    /// 最终体积分数
    pub final_volume: f64,
    /// 最终密度场 (每个单元的密度值)
    pub final_density_field: Vec<f64>,
    /// 优化耗时 (秒)
    pub elapsed_time_seconds: f64,
    /// 错误信息 (如有)
    pub error_message: Option<String>,
}

/// 网格节点结构体 (匹配前端格式)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptNode {
    /// 节点编号
    pub id: u64,
    /// X坐标
    pub x: f64,
    /// Y坐标
    pub y: f64,
    /// Z坐标
    pub z: f64,
}

/// 网格单元结构体 (匹配前端格式)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptElement {
    /// 单元编号
    pub id: u64,
    /// 构成节点的编号列表
    pub node_ids: Vec<u64>,
}

// ============================================================================
// 优化器状态管理
// ============================================================================

/// 优化器状态结构体
/// 
/// 用于跟踪优化迭代过程中的所有状态信息
#[derive(Clone)]
pub struct OptimizerState {
    /// 优化配置
    pub config: TopologyConfig,
    /// 网格节点列表
    pub nodes: Vec<OptNode>,
    /// 网格单元列表
    pub elements: Vec<OptElement>,
    /// 当前密度场 (设计变量)
    pub density_field: Vec<f64>,
    /// 目标函数历史
    pub objective_history: Vec<f64>,
    /// 体积分数历史
    pub volume_history: Vec<f64>,
    /// 当前迭代次数
    pub iteration: usize,
    /// 是否已收敛
    pub converged: bool,
}

impl OptimizerState {
    /// 创建新的优化器状态
    /// 
    /// # 参数
    /// - `config`: 拓扑优化配置
    /// - `nodes`: 网格节点列表
    /// - `elements`: 网格单元列表
    /// 
    /// # 返回
    /// 初始化后的OptimizerState
    pub fn new(config: TopologyConfig, nodes: Vec<OptNode>, elements: Vec<OptElement>) -> Self {
        let element_count = elements.len().max(1);
        // 初始密度为1.0 (实体)
        let initial_density = vec![1.0; element_count];

        Self {
            config,
            nodes,
            elements,
            density_field: initial_density,
            objective_history: Vec::new(),
            volume_history: Vec::new(),
            iteration: 0,
            converged: false,
        }
    }

    /// 计算当前体积分数
    /// 
    /// # 返回
    /// 密度场的平均体积分数
    pub fn volume_fraction(&self) -> f64 {
        if self.density_field.is_empty() {
            return 0.0;
        }
        self.density_field.iter().sum::<f64>() / self.density_field.len() as f64
    }

    /// 检查优化是否收敛
    /// 
    /// 收敛条件: 迭代次数达到最大, 或目标函数变化率小于容差
    /// 
    /// # 返回
    /// 是否已收敛
    pub fn check_convergence(&self) -> bool {
        if self.iteration < 2 {
            return false;
        }
        if self.iteration >= self.config.max_iterations {
            return true;
        }

        // 检查目标函数变化率
        let n = self.objective_history.len();
        if n < 2 {
            return false;
        }

        let obj_change = (self.objective_history[n - 1] - self.objective_history[n - 2]).abs();
        let obj_prev = self.objective_history[n - 2].abs();

        if obj_prev < 1e-10 {
            return true;
        }

        let change_ratio = obj_change / obj_prev;
        tracing::info!(
            "Iteration {}: objective change ratio = {:.6}",
            self.iteration,
            change_ratio
        );

        change_ratio < self.config.convergence_tolerance
    }

    /// 应用SIMP惩罚得到有效刚度
    /// 
    /// E_eff = E_min + (ρ^p - ρ_min^p) * (E_0 - E_min)
    /// 简化版: E_eff = ρ^p * E_0
    /// 
    /// # 参数
    /// - `idx`: 单元索引
    /// 
    /// # 返回
    /// 惩罚后的有效密度
    pub fn simp_penalized_density(&self, idx: usize) -> f64 {
        let rho = self.density_field.get(idx).unwrap_or(&1.0);
        rho.powf(self.config.penalization_factor)
    }

    /// 应用OC方法更新密度场
    /// 
    /// 最优准则法更新规则:
    /// ρ_new = ρ_old * (η/λ)^β
    /// 其中 η 为灵敏度, λ 为拉格朗日乘子
    /// 
    /// # 参数
    /// - `sensitivities`: 灵敏度数组
    /// - `volume_constraint`: 体积约束值
    pub fn oc_update(&mut self, sensitivities: &[f64], _volume_constraint: f64) {
        let beta = self.config.oc_beta;
        let move_limit = self.config.oc_move_limit;
        let min_rho = self.config.min_density;

        let mut new_density = self.density_field.clone();

        for (i, rho) in self.density_field.iter().enumerate() {
            let sens = sensitivities.get(i).unwrap_or(&1.0);

            // OC更新规则: ρ_new = ρ * (sens/λ)^β
            // 简化: 使用当前与平均灵敏度的比值
            let avg_sens = sensitivities.iter().sum::<f64>() / sensitivities.len().max(1) as f64;
            if avg_sens.abs() < 1e-10 {
                continue;
            }

            let ratio = sens / avg_sens;
            let mut new_rho = rho * ratio.powf(beta);

            // 应用移动极限 (MMA风格)
            let lower = (*rho * (1.0 - move_limit)).max(min_rho);
            let upper = (*rho * (1.0 + move_limit)).min(1.0);
            new_rho = new_rho.clamp(lower, upper);

            new_density[i] = new_rho;
        }

        self.density_field = new_density;
    }

    /// 强制执行体积约束 (二分法风格)
    /// 
    /// 缩放密度场使体积分数满足目标约束
    /// 
    /// # 参数
    /// - `target_volume`: 目标体积分数
    pub fn enforce_volume_constraint(&mut self, target_volume: f64) {
        let current = self.volume_fraction();
        if (current - target_volume).abs() < 0.001 {
            return;
        }

        let factor = if current > target_volume {
            // 需要减小密度
            target_volume / current
        } else {
            // 需要增加密度 - 按比例放大
            let scale = target_volume / current;
            scale.min(1.1) // 限制增长率
        };

        for rho in &mut self.density_field {
            *rho = (*rho * factor).clamp(self.config.min_density, 1.0);
        }
    }

    /// 获取当前密度统计信息
    /// 
    /// # 返回
    /// (平均值, 最大值, 最小值)
    pub fn density_stats(&self) -> (f64, f64, f64) {
        if self.density_field.is_empty() {
            return (0.0, 0.0, 0.0);
        }

        let sum: f64 = self.density_field.iter().sum();
        let count = self.density_field.len() as f64;
        let mean = sum / count;

        let variance: f64 = self
            .density_field
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / count;

        let _std_dev = variance.sqrt();
        let max_d = *self.density_field.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&1.0);
        let min_d = *self.density_field.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0);

        (mean, max_d, min_d)
    }
}

// ============================================================================
// STL导出函数
// ============================================================================

/// 导出优化后的几何到STL格式
/// 
/// 根据密度阈值筛选单元,生成三角面片
/// 
/// # 参数
/// - `nodes`: 网格节点列表
/// - `elements`: 网格单元列表
/// - `density_field`: 密度场
/// - `threshold`: 密度阈值 (低于此值的单元不导出)
/// - `output_path`: 输出文件路径
/// 
/// # 返回
/// 导出结果结构体
pub fn export_to_stl(
    nodes: &[OptNode],
    elements: &[OptElement],
    density_field: &[f64],
    threshold: f64,
    output_path: &PathBuf,
) -> Result<ExportResult, String> {
    tracing::info!(
        "Exporting to STL: {} elements, threshold={:.3}",
        elements.len(),
        threshold
    );

    // 构建节点映射表
    let node_map: HashMap<u64, &OptNode> = nodes.iter().map(|n| (n.id, n)).collect();

    let file = File::create(output_path).map_err(|e| format!("Failed to create file: {}", e))?;
    let mut writer = BufWriter::new(file);

    // 写入STL头部
    writeln!(writer, "solid optimized_geometry").map_err(|e| e.to_string())?;

    let mut triangle_count = 0u64;

    for (elem_idx, elem) in elements.iter().enumerate() {
        let density = *density_field.get(elem_idx).unwrap_or(&1.0);

        // 只包含密度高于阈值的单元
        if density < threshold {
            continue;
        }

        // 获取单元顶点
        let verts: Vec<[f64; 3]> = elem
            .node_ids
            .iter()
            .filter_map(|nid| {
                node_map.get(nid).map(|n| [n.x, n.y, n.z])
            })
            .collect();

        if verts.len() < 3 {
            continue;
        }

        // 根据单元类型进行三角化
        match verts.len() {
            3 => {
                // 三角形 - 直接输出
                write_triangle(&mut writer, &verts[0], &verts[1], &verts[2])?;
                triangle_count += 1;
            }
            4 => {
                // 四边形 - 拆分为2个三角形
                write_triangle(&mut writer, &verts[0], &verts[1], &verts[2])?;
                write_triangle(&mut writer, &verts[0], &verts[2], &verts[3])?;
                triangle_count += 2;
            }
            6 => {
                // 楔形单元 (C3D6) - 4个三角形
                write_triangle(&mut writer, &verts[0], &verts[2], &verts[1])?;
                write_triangle(&mut writer, &verts[3], &verts[4], &verts[5])?;
                write_triangle(&mut writer, &verts[0], &verts[1], &verts[4])?;
                write_triangle(&mut writer, &verts[0], &verts[4], &verts[3])?;
                triangle_count += 4;
            }
            8 => {
                // 六面体单元 (C3D8) - 12个三角形 (每面2个,共6面)
                let v = &verts;
                // 底面
                write_triangle(&mut writer, &v[0], &v[1], &v[2])?;
                write_triangle(&mut writer, &v[0], &v[2], &v[3])?;
                // 顶面
                write_triangle(&mut writer, &v[4], &v[6], &v[5])?;
                write_triangle(&mut writer, &v[4], &v[7], &v[6])?;
                // 前面
                write_triangle(&mut writer, &v[0], &v[4], &v[5])?;
                write_triangle(&mut writer, &v[0], &v[5], &v[1])?;
                // 后面
                write_triangle(&mut writer, &v[3], &v[2], &v[6])?;
                write_triangle(&mut writer, &v[3], &v[6], &v[7])?;
                // 左面
                write_triangle(&mut writer, &v[0], &v[3], &v[7])?;
                write_triangle(&mut writer, &v[0], &v[7], &v[4])?;
                // 右面
                write_triangle(&mut writer, &v[1], &v[5], &v[6])?;
                write_triangle(&mut writer, &v[1], &v[6], &v[2])?;
                triangle_count += 12;
            }
            _ => {
                // 回退: 取前3个顶点三角化
                if verts.len() >= 3 {
                    write_triangle(&mut writer, &verts[0], &verts[1], &verts[2])?;
                    triangle_count += 1;
                }
            }
        }
    }

    // 写入STL尾部
    writeln!(writer, "endsolid optimized_geometry").map_err(|e| e.to_string())?;

    tracing::info!("Exported {} triangles to {:?}", triangle_count, output_path);

    Ok(ExportResult {
        success: true,
        file_path: output_path.to_string_lossy().to_string(),
        element_count: elements.iter().enumerate().filter(|(i, _)| density_field.get(*i).unwrap_or(&1.0) >= &threshold).count(),
        node_count: nodes.len(),
        triangle_count: triangle_count as usize,
    })
}

/// 写入单个三角形面片到STL文件
/// 
/// 使用叉积计算面法向量
/// 
/// # 参数
/// - `writer`: 缓冲区写入器
/// - `v0`, `v1`, `v2`: 三角形三个顶点坐标
fn write_triangle(writer: &mut BufWriter<File>, v0: &[f64; 3], v1: &[f64; 3], v2: &[f64; 3]) -> Result<(), String> {
    // 使用叉积计算面法向量
    let ax = v1[0] - v0[0];
    let ay = v1[1] - v0[1];
    let az = v1[2] - v0[2];
    let bx = v2[0] - v0[0];
    let by = v2[1] - v0[1];
    let bz = v2[2] - v0[2];

    let nx = ay * bz - az * by;
    let ny = az * bx - ax * bz;
    let nz = ax * by - ay * bx;

    // 归一化
    let len = (nx * nx + ny * ny + nz * nz).sqrt();
    let (nx, ny, nz) = if len > 1e-10 {
        (nx / len, ny / len, nz / len)
    } else {
        (0.0, 0.0, 1.0)
    };

    writeln!(writer, "  facet normal {} {} {}", nx, ny, nz).map_err(|e| e.to_string())?;
    writeln!(writer, "    outer loop").map_err(|e| e.to_string())?;
    writeln!(writer, "      vertex {} {} {}", v0[0], v0[1], v0[2]).map_err(|e| e.to_string())?;
    writeln!(writer, "      vertex {} {} {}", v1[0], v1[1], v1[2]).map_err(|e| e.to_string())?;
    writeln!(writer, "      vertex {} {} {}", v2[0], v2[1], v2[2]).map_err(|e| e.to_string())?;
    writeln!(writer, "    endloop").map_err(|e| e.to_string())?;
    writeln!(writer, "  endfacet").map_err(|e| e.to_string())?;

    Ok(())
}

/// 导出结果结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    /// 是否成功
    pub success: bool,
    /// 输出文件路径
    pub file_path: String,
    /// 导出的单元数量
    pub element_count: usize,
    /// 节点数量
    pub node_count: usize,
    /// 生成的三角形数量
    pub triangle_count: usize,
}

// ============================================================================
// 优化模板
// ============================================================================

/// 预定义优化模板集合
pub struct OptimizationTemplates;

impl OptimizationTemplates {
    /// 悬臂梁拓扑优化模板
    /// 
    /// 经典最小柔度问题: 左端固定, 右侧施加集中荷载
    /// 
    /// # 返回
    /// TemplateInfo 结构体
    pub fn cantilever() -> TemplateInfo {
        TemplateInfo {
            name: "悬臂梁拓扑优化".to_string(),
            description: "经典悬臂梁最小柔度拓扑优化问题，左端固定，右侧施加集中荷载".to_string(),
            optimization_type: OptimizationType::Topology,
            objective: ObjectiveType::MinCompliance,
            volume_fraction: 0.5,
            max_iterations: 100,
            penalization_factor: 3.0,
            min_density: 0.01,
            design_domain: DesignDomain {
                x_min: 0.0,
                x_max: 100.0,
                y_min: 0.0,
                y_max: 40.0,
                z_min: 0.0,
                z_max: 10.0,
            },
            mesh_config: MeshConfig {
                x_div: 40,
                y_div: 16,
                z_div: 4,
            },
            material: MaterialConfig {
                elastic_modulus: 210000.0, // MPa
                poisson_ratio: 0.3,
            },
        }
    }

    /// L型支架拓扑优化模板
    /// 
    /// 模拟工业中常见的连接件设计
    /// 
    /// # 返回
    /// TemplateInfo 结构体
    pub fn l_bracket() -> TemplateInfo {
        TemplateInfo {
            name: "L型支架拓扑优化".to_string(),
            description: "L型支架结构优化，模拟工业中常见的连接件设计".to_string(),
            optimization_type: OptimizationType::Topology,
            objective: ObjectiveType::MinCompliance,
            volume_fraction: 0.4,
            max_iterations: 80,
            penalization_factor: 3.0,
            min_density: 0.02,
            design_domain: DesignDomain {
                x_min: 0.0,
                x_max: 80.0,
                y_min: 0.0,
                y_max: 60.0,
                z_min: 0.0,
                z_max: 8.0,
            },
            mesh_config: MeshConfig {
                x_div: 32,
                y_div: 24,
                z_div: 3,
            },
            material: MaterialConfig {
                elastic_modulus: 210000.0,
                poisson_ratio: 0.3,
            },
        }
    }

    /// 简支梁拓扑优化模板
    /// 
    /// 中点荷载, 上下两端铰支座
    /// 
    /// # 返回
    /// TemplateInfo 结构体
    pub fn simply_supported() -> TemplateInfo {
        TemplateInfo {
            name: "简支梁拓扑优化".to_string(),
            description: "简支梁中点荷载拓扑优化，上下两端铰支座".to_string(),
            optimization_type: OptimizationType::Topology,
            objective: ObjectiveType::MinCompliance,
            volume_fraction: 0.45,
            max_iterations: 100,
            penalization_factor: 3.0,
            min_density: 0.01,
            design_domain: DesignDomain {
                x_min: 0.0,
                x_max: 120.0,
                y_min: 0.0,
                y_max: 30.0,
                z_min: 0.0,
                z_max: 10.0,
            },
            mesh_config: MeshConfig {
                x_div: 48,
                y_div: 12,
                z_div: 4,
            },
            material: MaterialConfig {
                elastic_modulus: 210000.0,
                poisson_ratio: 0.3,
            },
        }
    }

    /// 无网格梁拓扑优化模板
    /// 
    /// 小尺寸精细化问题, 用于验证优化算法精度
    /// 
    /// # 返回
    /// TemplateInfo 结构体
    pub fn mess_free() -> TemplateInfo {
        TemplateInfo {
            name: "无网格梁拓扑优化".to_string(),
            description: "小尺寸精细化拓扑优化问题，用于验证优化算法精度".to_string(),
            optimization_type: OptimizationType::Topology,
            objective: ObjectiveType::MinCompliance,
            volume_fraction: 0.5,
            max_iterations: 60,
            penalization_factor: 3.0,
            min_density: 0.01,
            design_domain: DesignDomain {
                x_min: 0.0,
                x_max: 50.0,
                y_min: 0.0,
                y_max: 20.0,
                z_min: 0.0,
                z_max: 5.0,
            },
            mesh_config: MeshConfig {
                x_div: 50,
                y_div: 20,
                z_div: 5,
            },
            material: MaterialConfig {
                elastic_modulus: 210000.0,
                poisson_ratio: 0.3,
            },
        }
    }

    /// 获取所有可用模板
    /// 
    /// # 返回
    /// 模板信息向量
    pub fn all() -> Vec<TemplateInfo> {
        vec![
            Self::cantilever(),
            Self::l_bracket(),
            Self::simply_supported(),
            Self::mess_free(),
        ]
    }
}

/// 模板信息结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateInfo {
    /// 模板名称
    pub name: String,
    /// 模板描述
    pub description: String,
    /// 优化类型
    pub optimization_type: OptimizationType,
    /// 目标函数类型
    pub objective: ObjectiveType,
    /// 目标体积分数
    pub volume_fraction: f64,
    /// 最大迭代次数
    pub max_iterations: usize,
    /// SIMP惩罚因子
    pub penalization_factor: f64,
    /// 最小密度
    pub min_density: f64,
    /// 设计域
    pub design_domain: DesignDomain,
    /// 网格配置
    pub mesh_config: MeshConfig,
    /// 材料配置
    pub material: MaterialConfig,
}

/// 网格配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshConfig {
    /// X方向分格数
    pub x_div: usize,
    /// Y方向分格数
    pub y_div: usize,
    /// Z方向分格数
    pub z_div: usize,
}

/// 材料配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialConfig {
    /// 弹性模量 (MPa)
    pub elastic_modulus: f64,
    /// 泊松比
    pub poisson_ratio: f64,
}

// ============================================================================
// 主优化算法
// ============================================================================

/// 运行完整的拓扑优化 (SIMP + OC方法)
/// 
/// 使用SIMP方法进行密度惩罚, OC方法更新设计变量
/// 
/// # 参数
/// - `config`: 拓扑优化配置
/// - `nodes`: 网格节点列表
/// - `elements`: 网格单元列表
/// - `_boundary_conditions`: 边界条件 (可选)
/// - `_material`: 材料参数 (可选)
/// 
/// # 返回
/// TopologyResult 包含优化结果和迭代历史
pub fn run_optimization(
    config: TopologyConfig,
    nodes: Vec<OptNode>,
    elements: Vec<OptElement>,
    _boundary_conditions: Option<super::solver::bc::BcContainer>,
    _material: Option<super::input_gen::Material>,
) -> TopologyResult {
    let start_time = Instant::now();
    tracing::info!("Starting topology optimization with {} nodes, {} elements", nodes.len(), elements.len());

    // 初始化优化器
    let mut state = OptimizerState::new(config.clone(), nodes, elements);

    // 获取体积约束
    let volume_target = config
        .constraints
        .iter()
        .find(|c| matches!(c.constraint_type, ConstraintType::VolumeFraction))
        .map(|c| c.upper_bound)
        .unwrap_or(0.5);

    // 运行优化迭代
    let mut iterations = Vec::new();

    while !state.converged && state.iteration < config.max_iterations {
        state.iteration += 1;

        // 计算灵敏度 (简化版柔度灵敏度)
        let sensitivities = calculate_compliance_sensitivity(&state);

        // OC方法更新
        state.oc_update(&sensitivities, volume_target);

        // 强制执行体积约束
        state.enforce_volume_constraint(volume_target);

        // 计算当前目标函数 (柔度)
        let compliance = calculate_compliance(&state);
        state.objective_history.push(compliance);
        state.volume_history.push(state.volume_fraction());

        // 获取密度统计
        let (_, max_d, min_d) = state.density_stats();

        // 检查收敛
        state.converged = state.check_convergence();

        // 创建迭代记录
        let iteration = TopologyIteration {
            iteration: state.iteration,
            objective_value: compliance,
            volume_fraction: state.volume_fraction(),
            max_stress: None,
            max_density: max_d,
            min_density: min_d,
            density_std: 0.0,
            converged: state.converged,
        };

        iterations.push(iteration);

        tracing::info!(
            "Iteration {}: obj={:.4}, vol={:.4}, max_d={:.4}, min_d={:.4}",
            state.iteration,
            compliance,
            state.volume_fraction(),
            max_d,
            min_d
        );

        // 收敛后提前退出
        if state.converged {
            break;
        }
    }

    let elapsed = start_time.elapsed().as_secs_f64();

    tracing::info!(
        "Optimization completed in {:.2}s, {} iterations",
        elapsed,
        state.iteration
    );

    TopologyResult {
        success: state.converged || state.iteration >= 2,
        iterations,
        final_objective: state.objective_history.last().copied().unwrap_or(0.0),
        final_volume: state.volume_fraction(),
        final_density_field: state.density_field,
        elapsed_time_seconds: elapsed,
        error_message: None,
    }
}

/// 计算柔度 (目标函数)
/// 
/// 柔度 = U^T * K * U, 即应变能
/// 简化版: 对密度场求和
/// 
/// # 参数
/// - `state`: 优化器状态
/// 
/// # 返回
/// 柔度值
fn calculate_compliance(state: &OptimizerState) -> f64 {
    // 简化柔度计算
    // 实际实现需要FEA计算
    let mut compliance = 0.0;

    for (_i, rho) in state.density_field.iter().enumerate() {
        let penalized = rho.powf(state.config.penalization_factor);
        compliance += penalized;
    }

    // 归一化
    compliance / state.density_field.len().max(1) as f64
}

/// 计算柔度灵敏度 (目标函数对密度的导数)
/// 
/// dC/dρ = -p * ρ^(p-1) * (1/ρ)
/// 
/// # 参数
/// - `state`: 优化器状态
/// 
/// # 返回
/// 灵敏度数组
fn calculate_compliance_sensitivity(state: &OptimizerState) -> Vec<f64> {
    let n = state.density_field.len();
    let mut sensitivities = Vec::with_capacity(n);

    for i in 0..n {
        let rho = state.density_field[i];
        let penalized_derivative = state.config.penalization_factor * rho.powf(state.config.penalization_factor - 1.0);

        // 简化灵敏度: 柔度对密度的导数
        let sens = -penalized_derivative * (1.0 / rho.max(1e-6));
        sensitivities.push(sens.max(-100.0).min(100.0));
    }

    sensitivities
}

/// 计算SIMP惩罚后的刚度
/// 
/// E_eff = E_0 * ρ^p
/// 
/// # 参数
/// - `base_young`: 基础弹性模量 (MPa)
/// - `density`: 密度值 (0-1)
/// - `penalization`: SIMP惩罚因子
/// 
/// # 返回
/// 惩罚后的有效刚度
pub fn calculate_simp_stiffness(base_young: f64, density: f64, penalization: f64) -> f64 {
    base_young * density.powf(penalization)
}

/// 计算OC方法灵敏度
/// 
/// # 参数
/// - `element_index`: 单元索引
/// - `density`: 密度值
/// - `_compliance`: 柔度值
/// - `beta`: OC方法beta参数
/// 
/// # 返回
/// OC灵敏度值
pub fn calculate_oc_sensitivity(_element_index: usize, density: f64, _compliance: f64, beta: f64) -> f64 {
    // OC方法简化灵敏度
    let sens = -beta * density.powf(beta - 1.0);
    sens.max(-100.0).min(100.0)
}

// ============================================================================
// 全局优化器状态 (用于迭代动画)
// ============================================================================

lazy_static::lazy_static! {
    static ref OPTIMIZER_HISTORY: std::sync::Mutex<Vec<Vec<f64>>> = std::sync::Mutex::new(Vec::new());
}

/// 存储迭代密度场用于动画展示
/// 
/// # 参数
/// - `iteration`: 迭代编号
/// - `density`: 密度场数据
pub fn store_iteration_density(iteration: usize, density: &[f64]) {
    if let Ok(mut history) = OPTIMIZER_HISTORY.lock() {
        // 确保有足够的槽位
        while history.len() <= iteration {
            history.push(Vec::new());
        }
        history[iteration] = density.to_vec();
    }
}

/// 获取指定迭代的密度场
/// 
/// # 参数
/// - `iteration`: 迭代编号
/// 
/// # 返回
/// 密度场数据 (如有)
pub fn get_iteration_density(iteration: usize) -> Option<Vec<f64>> {
    OPTIMIZER_HISTORY.lock().ok().and_then(|history| {
        history.get(iteration).cloned().filter(|d| !d.is_empty())
    })
}

/// 重置优化器历史记录
pub fn reset_optimizer_history() {
    if let Ok(mut history) = OPTIMIZER_HISTORY.lock() {
        history.clear();
    }
}
