//! 疲劳分析命令模块
//! 
//! 本模块提供高周疲劳(S-N曲线)、低周疲劳(E-N曲线)和随机振动疲劳分析功能。
//! 支持多种平均应力修正方法(Goodman, Gerber, Soderberg)和表面处理效应计算。
//! 
//! 主要功能：
//! - S-N曲线拟合与疲劳寿命预测
//! - E-N曲线低周疲劳分析
//! - 随机振动疲劳(PSD)分析
//! - Rainflow循环计数
//! - Miner线性损伤累积
//! - CalculiX输入文件生成

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 疲劳分析类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FatigueAnalysisType {
    /// 高周疲劳 - 基于S-N曲线
    SN,
    /// 低周疲劳 - 基于E-N曲线(应变-寿命)
    EN,
    /// 随机振动疲劳 - 基于功率谱密度(PSD)
    PSD,
}

/// 平均应力修正方法枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MeanStressCorrection {
    /// 无修正
    None,
    /// Goodman修正 - 适用于脆性材料
    Goodman,
    /// Gerber修正 - 适用于韧性材料
    Gerber,
    /// Soderberg修正 - 保守方法
    Soderberg,
}

/// 表面处理类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SurfaceTreatment {
    /// 机加工表面
    Machined,
    /// 铸态表面
    AsCast,
    /// 抛光表面
    Polished,
    /// 喷丸处理(产生压应力,提高疲劳性能)
    ShotPeened,
}

/// S-N曲线数据点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SNDataPoint {
    /// 应力幅值 (MPa)
    pub stress: f64,
    /// 失效循环次数
    pub cycles: f64,
}

/// S-N曲线参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SNParams {
    /// 应力比R (R = Sa/Sm)
    pub stress_ratio: f64,
    /// S-N曲线数据点列表
    pub data_points: Vec<SNDataPoint>,
    /// 疲劳极限 (MPa) - 低于此应力可认为无限寿命
    pub fatigue_limit: f64,
    /// 平均应力修正方法
    pub mean_stress_correction: String,
}

/// E-N曲线参数 (应变-寿命曲线)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ENParams {
    /// 应变幅值
    pub strain_amplitude: f64,
    /// 循环强度系数 (MPa)
    pub cyclic_stress_coeff: f64,
    /// 循环应变硬化指数
    pub cyclic_exponent: f64,
    /// 是否使用Neuber修正
    pub use_neuber: bool,
}

/// PSD功率谱密度参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PSDParams {
    /// PSD数据(频率-功率对)
    pub psd_data: String,
    /// RMS应力 (MPa)
    pub rms_stress: f64,
    /// 目标使用寿命 (小时)
    pub target_life: f64,
}

/// 荷载参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadParams {
    /// 应力幅值 (MPa)
    pub stress_amplitude: f64,
}

/// 组合疲劳参数 (从前端接收)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FatigueParams {
    /// 分析类型: "sn", "en", "psd"
    pub analysis_type: String,
    /// S-N曲线参数
    pub sn_params: Option<SNParams>,
    /// E-N曲线参数
    pub en_params: Option<ENParams>,
    /// PSD参数
    pub psd_params: Option<PSDParams>,
    /// 荷载类型: "constant"(恒幅), "variable"(变幅)
    pub load_type: String,
    /// 荷载参数
    pub load: LoadParams,
    /// 应力集中系数Kt
    pub kt: f64,
    /// 表面处理类型
    pub surface_treatment: String,
}

/// 疲劳分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FatigueResults {
    /// 损伤累积值D (Miner法则: D = Σ ni/Ni)
    pub damage: f64,
    /// 预测寿命 (循环次数)
    pub life_cycles: f64,
    /// 安全系数 (基于疲劳极限)
    pub safety_factor: f64,
}

/// S-N曲线模型
#[derive(Debug, Clone)]
pub struct SNCurve {
    /// S-N曲线数据点
    pub data_points: Vec<SNDataPoint>,
    /// 疲劳极限 (MPa)
    pub fatigue_limit: f64,
    /// 对数空间截距 log(a) (Basquin定律: N = a * S^(-m))
    pub log_a: f64,
    /// 对数空间斜率 (-m)
    pub m: f64,
}

impl SNCurve {
    /// 创建新的S-N曲线模型
    /// 
    /// 使用最小二乘法在双对数空间拟合: log(N) = log(a) - m * log(S)
    /// 
    /// # 参数
    /// - `data_points`: S-N曲线实验数据点
    /// - `fatigue_limit`: 疲劳极限 (MPa)
    /// 
    /// # 返回
    /// 拟合后的SNCurve对象
    pub fn new(data_points: Vec<SNDataPoint>, fatigue_limit: f64) -> Self {
        // 拟合S-N曲线: log(N) = log(a) - m * log(S)
        // 即: N = a * S^(-m)
        let mut m = 3.0;  // 默认斜率
        let mut log_a = 12.0;  // 默认截距

        if data_points.len() >= 2 {
            // 简单线性回归在双对数空间
            let n = data_points.len() as f64;
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            let mut sum_xx = 0.0;
            let mut sum_xy = 0.0;

            for point in &data_points {
                let x = point.stress.ln();
                let y = point.cycles.ln();
                sum_x += x;
                sum_y += y;
                sum_xx += x * x;
                sum_xy += x * y;
            }

            let denom = n * sum_xx - sum_x * sum_x;
            if denom.abs() > 1e-10 {
                m = (n * sum_xy - sum_x * sum_y) / denom;
                log_a = (sum_y - m * sum_x) / n;
            }
        }

        SNCurve {
            data_points,
            fatigue_limit,
            log_a,
            m,
        }
    }

    /// 根据给定应力幅值计算失效循环次数
    /// 
    /// # 参数
    /// - `stress`: 应力幅值 (MPa)
    /// 
    /// # 返回
    /// 失效循环次数, 若应力低于疲劳极限则返回无穷大(无限寿命)
    pub fn get_cycles(&self, stress: f64) -> f64 {
        if stress <= self.fatigue_limit {
            return f64::INFINITY;  // 无限寿命
        }
        
        let log_n = self.log_a - self.m * stress.ln();
        (-log_n).exp()
    }

    /// 根据给定循环次数计算允许的应力幅值
    /// 
    /// # 参数
    /// - `cycles`: 目标循环次数
    /// 
    /// # 返回
    /// 允许的应力幅值 (不低于疲劳极限)
    pub fn get_stress(&self, cycles: f64) -> f64 {
        if cycles <= 0.0 {
            return 0.0;
        }
        
        let log_n = cycles.ln();
        let stress = ((self.log_a - log_n) / self.m).exp();
        stress.max(self.fatigue_limit)
    }
}

/// Rainflow循环 (从载荷谱中提取)
#[derive(Debug, Clone, Serialize)]
pub struct RainflowCycle {
    /// 循环幅值
    pub amplitude: f64,
    /// 循环均值
    pub mean: f64,
    /// 循环计数
    pub count: i32,
}

/// Rainflow循环计数算法 (ASTM E1049标准)
/// 
/// 将变幅载荷谱分解为一系列等效的恒幅循环
/// 
/// # 参数
/// - `loads`: 载荷时间历程数组
/// 
/// # 返回
/// 提取的Rainflow循环列表
pub fn rainflow_count(loads: &[f64]) -> Vec<RainflowCycle> {
    let n = loads.len();
    if n < 4 {
        return vec![];
    }

    // 简化的Rainflow计数
    let mut cycles: Vec<RainflowCycle> = vec![];
    let mut residuals: Vec<f64> = vec![];

    // 半循环提取 (简化版)
    for i in 0..(n - 1) {
        let range = (loads[i + 1] - loads[i]).abs();
        let mean = (loads[i] + loads[i + 1]) / 2.0;
        
        // 存储半循环
        residuals.push(range);
        residuals.push(mean);
    }

    // 合并半循环为完整循环
    let mut i = 0;
    while i + 1 < residuals.len() {
        let amp = (residuals[i] + residuals[i + 1]) / 2.0;
        let mean = (residuals[i] - residuals[i + 1]) / 2.0;
        
        if amp > 0.0 {
            cycles.push(RainflowCycle {
                amplitude: amp,
                mean: mean,
                count: 1,
            });
        }
        i += 2;
    }

    cycles
}

/// 应用平均应力修正
/// 
/// 将含有平均应力的应力幅值修正为等效的R=-1(对称循环)应力幅值
/// 
/// # 参数
/// - `stress_amplitude`: 应力幅值 Sa (MPa)
/// - `mean_stress`: 平均应力 Sm (MPa)
/// - `correction_type`: 修正方法类型 ("goodman", "gerber", "soderberg")
/// - `ultimate_tensile`: 抗拉强度 Su (MPa)
/// - `yield_strength`: 屈服强度 Sy (MPa)
/// 
/// # 返回
/// 修正后的等效应力幅值 (MPa)
pub fn mean_stress_correction(
    stress_amplitude: f64,
    mean_stress: f64,
    correction_type: &str,
    ultimate_tensile: f64,  // Su (MPa)
    yield_strength: f64,   // Sy (MPa)
) -> f64 {
    let r = stress_amplitude / (stress_amplitude + mean_stress.abs());
    
    match correction_type {
        "goodman" => {
            // Goodman: Sa = Su / (Su / Sa + Sm)
            stress_amplitude * ultimate_tensile / (ultimate_tensile + mean_stress.abs())
        },
        "gerber" => {
            // Gerber: Sa = Su * (1 - (Sm/Su)^2)
            stress_amplitude * (1.0 - (mean_stress / ultimate_tensile).powi(2))
        },
        "soderberg" => {
            // Soderberg: Sa = Sy / (Sy / Sa + Sm)
            stress_amplitude * yield_strength / (yield_strength + mean_stress.abs())
        },
        _ => stress_amplitude,  // 无修正
    }
}

/// 计算表面处理因子
/// 
/// 考虑不同表面加工状态对疲劳强度的影响
/// 
/// # 参数
/// - `treatment`: 表面处理类型
/// 
/// # 返回
/// 表面处理因子 (≥1.0表示提高疲劳性能)
pub fn surface_factor(treatment: &str) -> f64 {
    match treatment {
        "polished" => 1.0,      // 抛光表面
        "machined" => 0.9,     // 机加工表面
        "as_cast" => 0.5,      // 铸态(最差)
        "shot_peened" => 1.2,  // 喷丸(压应力提高疲劳)
        _ => 1.0,
    }
}

/// Miner线性损伤累积法则
/// 
/// 假设各循环产生的损伤可以线性叠加, 当D≥1时发生疲劳破坏
/// D = Σ (ni / Ni) = n1/N1 + n2/N2 + ...
/// 
/// # 参数
/// - `cycles`: 各应力水平下的实际循环次数数组
/// - `sn_curve`: S-N曲线模型
/// 
/// # 返回
/// 总损伤累积值D
pub fn minER_rule(cycles: &[f64], sn_curve: &SNCurve) -> f64 {
    let mut damage = 0.0;
    
    for &n in cycles {
        let allowable = sn_curve.get_cycles(n);
        if allowable.is_finite() && allowable > 0.0 {
            damage += n / allowable;
        }
    }
    
    damage
}

/// 主疲劳分析函数
/// 
/// 根据指定的分析类型执行完整的疲劳寿命预测
/// 
/// # 参数
/// - `params`: 疲劳分析参数
/// 
/// # 返回
/// 疲劳分析结果 (损伤、寿命、安全系数)
pub fn analyze_fatigue(params: &FatigueParams) -> Result<FatigueResults, String> {
    let stress_amplitude = params.load.stress_amplitude;
    let kt = params.kt;
    let effective_stress = stress_amplitude * kt;
    
    // 表面处理因子
    let surface_fact = surface_factor(&params.surface_treatment);
    
    // 表面处理修正后的应力
    let adjusted_stress = effective_stress / surface_fact;
    
    let (damage, life_cycles, safety_factor) = match params.analysis_type.as_str() {
        "sn" => {
            // S-N曲线分析 (高周疲劳)
            let sn_params = params.sn_params.as_ref()
                .ok_or("S-N parameters required")?;
            
            let sn_curve = SNCurve::new(
                sn_params.data_points.clone(),
                sn_params.fatigue_limit,
            );
            
            // 计算寿命
            let life = sn_curve.get_cycles(adjusted_stress);
            
            // 计算损伤 (使用Miner法则)
            let damage = if life.is_finite() {
                stress_amplitude / life
            } else {
                0.0
            };
            
            // 基于疲劳极限的安全系数
            let sf = sn_params.fatigue_limit / adjusted_stress;
            
            (damage, life, sf)
        },
        "en" => {
            // E-N曲线分析 (低周疲劳)
            let en_params = params.en_params.as_ref()
                .ok_or("E-N parameters required")?;
            
            // Manson-Coffin关系 (简化版)
            // Δε/2 = (σf'/2E) * (2N)^b + (εf') * (2N)^c
            // 简化: 应变-寿命关系
            let cyclic_stress_coeff = en_params.cyclic_stress_coeff;
            let cyclic_exponent = en_params.cyclic_exponent;
            
            // 计算塑性应变幅值
            let elastic_strain = adjusted_stress / (2.0 * 1e5);  // E = 200 GPa
            let plastic_strain = (cyclic_stress_coeff / adjusted_stress).powf(1.0 / cyclic_exponent);
            
            let total_strain = (elastic_strain.powi(2) + plastic_strain.powi(2)).sqrt();
            
            // 估计失效循环次数
            let life = if total_strain > 0.0 {
                ((cyclic_stress_coeff / total_strain).powf(1.0 / cyclic_exponent) / 2.0).floor()
            } else {
                f64::INFINITY
            };
            
            let damage = if life.is_finite() && life > 0.0 {
                stress_amplitude / life
            } else {
                0.0
            };
            
            let sf = if en_params.strain_amplitude > 0.0 {
                en_params.strain_amplitude / total_strain
            } else {
                1.0
            };
            
            (damage, life, sf)
        },
        "psd" => {
            // PSD随机振动疲劳
            let psd_params = params.psd_params.as_ref()
                .ok_or("PSD parameters required")?;
            
            let rms = psd_params.rms_stress;
            let target_life = psd_params.target_life;
            
            // 简化: 使用RMS应力估计疲劳损伤
            // D = n / N 其中n是实际循环次数, N是允许循环次数
            let effective_rms = adjusted_stress * rms / 100.0;
            
            // 估计每小时循环次数 (简化)
            let cycles_per_hour = 3600.0 * 10.0;  // 假设10 Hz带宽
            let total_cycles = cycles_per_hour * target_life;
            
            // 使用Basquin定律计算随机载荷损伤
            let m = 5.0;  // 钢材典型斜率
            let damage = total_cycles * effective_rms.powf(m) / 1e9;
            let life = if damage > 0.0 && damage < 1.0 {
                target_life / damage
            } else {
                target_life
            };
            
            let sf = if damage > 0.0 {
                1.0 / damage
            } else {
                f64::INFINITY
            };
            
            (damage, life, sf)
        },
        _ => return Err("Unknown analysis type".to_string()),
    };
    
    Ok(FatigueResults {
        damage,
        life_cycles,
        safety_factor: safety_factor.min(10.0),  // 上限为10
    })
}

/// Tauri命令封装 - 执行疲劳分析
/// 
/// # 参数
/// - `params`: 疲劳分析参数结构体
/// 
/// # 返回
/// Result<FatigueResults, String>: 分析成功返回结果, 失败返回错误信息
#[tauri::command]
pub fn fatigue_analysis(params: FatigueParams) -> Result<FatigueResults, String> {
    analyze_fatigue(&params)
}

/// Tauri命令 - Rainflow循环计数
/// 
/// # 参数
/// - `loads`: 载荷时间历程数组
/// 
/// # 返回
/// 提取的Rainflow循环列表
#[tauri::command]
pub fn rainflow_analysis(loads: Vec<f64>) -> Vec<RainflowCycle> {
    rainflow_count(&loads)
}

/// Tauri命令 - 从实验数据拟合S-N曲线
/// 
/// # 参数
/// - `data_points`: S-N曲线实验数据点
/// - `fatigue_limit`: 疲劳极限 (MPa)
/// 
/// # 返回
/// 包含拟合参数的HashMap (log_a, m, fatigue_limit)
#[tauri::command]
pub fn fit_sn_curve(data_points: Vec<SNDataPoint>, fatigue_limit: f64) -> HashMap<String, f64> {
    let sn_curve = SNCurve::new(data_points, fatigue_limit);
    
    let mut result = HashMap::new();
    result.insert("log_a".to_string(), sn_curve.log_a);
    result.insert("m".to_string(), sn_curve.m);
    result.insert("fatigue_limit".to_string(), sn_curve.fatigue_limit);
    result
}

/// Tauri命令 - 计算每个节点的损伤值
/// 
/// 从应力结果计算疲劳损伤分布
/// 
/// # 参数
/// - `stress_results`: 应力结果数组 (每个节点一个值)
/// - `sn_curve_params`: S-N曲线参数
/// 
/// # 返回
/// 每个节点对应的损伤值数组
#[tauri::command]
pub fn calculate_node_damage(
    stress_results: Vec<f64>,
    sn_curve_params: SNParams,
) -> Vec<f64> {
    let sn_curve = SNCurve::new(
        sn_curve_params.data_points,
        sn_curve_params.fatigue_limit,
    );
    
    stress_results
        .iter()
        .map(|&s| {
            let cycles = sn_curve.get_cycles(s);
            if cycles.is_finite() && cycles > 0.0 {
                1.0 / cycles
            } else {
                0.0
            }
        })
        .collect()
}

/// 疲劳分析模板结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FatigueTemplate {
    /// 模板名称
    pub name: String,
    /// 模板描述
    pub description: String,
    /// 模板类型
    pub template_type: String,
    /// 预定义的疲劳参数
    pub params: FatigueParams,
}

/// Tauri命令 - 获取可用的疲劳分析模板
/// 
/// # 返回
/// 预定义的疲劳分析模板列表
#[tauri::command]
pub fn get_fatigue_templates() -> Vec<FatigueTemplate> {
    vec![
        FatigueTemplate {
            name: "悬臂梁循环加载".to_string(),
            description: "悬臂梁端部循环加载疲劳分析".to_string(),
            template_type: "cantilever".to_string(),
            params: FatigueParams {
                analysis_type: "sn".to_string(),
                sn_params: Some(SNParams {
                    stress_ratio: -1.0,
                    data_points: vec![
                        SNDataPoint { stress: 400.0, cycles: 1e4 },
                        SNDataPoint { stress: 300.0, cycles: 1e5 },
                        SNDataPoint { stress: 220.0, cycles: 1e6 },
                        SNDataPoint { stress: 180.0, cycles: 1e7 },
                    ],
                    fatigue_limit: 150.0,
                    mean_stress_correction: "goodman".to_string(),
                }),
                en_params: None,
                psd_params: None,
                load_type: "constant".to_string(),
                load: LoadParams { stress_amplitude: 200.0 },
                kt: 1.0,
                surface_treatment: "machined".to_string(),
            },
        },
        FatigueTemplate {
            name: "压力容器疲劳".to_string(),
            description: "压力容器内压循环疲劳分析".to_string(),
            template_type: "pressure_vessel".to_string(),
            params: FatigueParams {
                analysis_type: "sn".to_string(),
                sn_params: Some(SNParams {
                    stress_ratio: 0.0,
                    data_points: vec![
                        SNDataPoint { stress: 350.0, cycles: 1e4 },
                        SNDataPoint { stress: 280.0, cycles: 1e5 },
                        SNDataPoint { stress: 200.0, cycles: 1e6 },
                        SNDataPoint { stress: 160.0, cycles: 1e7 },
                    ],
                    fatigue_limit: 120.0,
                    mean_stress_correction: "gerber".to_string(),
                }),
                en_params: None,
                psd_params: None,
                load_type: "constant".to_string(),
                load: LoadParams { stress_amplitude: 150.0 },
                kt: 2.5,
                surface_treatment: "as_cast".to_string(),
            },
        },
        FatigueTemplate {
            name: "焊接结构疲劳".to_string(),
            description: "焊接接头循环加载疲劳分析".to_string(),
            template_type: "weld".to_string(),
            params: FatigueParams {
                analysis_type: "sn".to_string(),
                sn_params: Some(SNParams {
                    stress_ratio: -1.0,
                    data_points: vec![
                        SNDataPoint { stress: 200.0, cycles: 1e4 },
                        SNDataPoint { stress: 140.0, cycles: 1e5 },
                        SNDataPoint { stress: 100.0, cycles: 1e6 },
                        SNDataPoint { stress: 80.0, cycles: 2e6 },
                    ],
                    fatigue_limit: 60.0,
                    mean_stress_correction: "none".to_string(),
                }),
                en_params: None,
                psd_params: None,
                load_type: "constant".to_string(),
                load: LoadParams { stress_amplitude: 80.0 },
                kt: 3.0,
                surface_treatment: "as_cast".to_string(),
            },
        },
    ]
}

/// Tauri命令 - 生成疲劳分析的CalculiX输入文件
/// 
/// # 参数
/// - `mesh_data`: 网格数据 (节点和单元)
/// - `bc_data`: 边界条件数据
/// - `load_data`: 荷载数据
/// - `fatigue_params`: 疲劳分析参数
/// 
/// # 返回
/// Result<String, String>: 生成的INP文件内容, 失败返回错误信息
#[tauri::command]
pub fn generate_fatigue_inp_file(
    mesh_data: HashMap<String, serde_json::Value>,
    bc_data: HashMap<String, serde_json::Value>,
    load_data: HashMap<String, serde_json::Value>,
    fatigue_params: FatigueParams,
) -> Result<String, String> {
    let mut inp_content = String::new();
    
    // 头部
    inp_content.push_str("*HEADING\n");
    inp_content.push_str("Fatigue Analysis - CAELab\n");
    inp_content.push_str("*PREPRINT, ECHO=NO, MODEL=NO\n");
    
    // 材料定义 (简化)
    inp_content.push_str("*MATERIAL, NAME=Steel\n");
    inp_content.push_str("*ELASTIC\n");
    inp_content.push_str("200000.0, 0.3\n");
    
    // 循环加载使用 *Cyclic Hardening (如需要)
    inp_content.push_str("*DENSITY\n");
    inp_content.push_str("7850.0\n");
    
    // 从mesh_data获取节点
    let nodes = mesh_data.get("nodes")
        .and_then(|v| v.as_array())
        .map(|arr| arr.clone())
        .unwrap_or_default();
    
    let elements = mesh_data.get("elements")
        .and_then(|v| v.as_array())
        .map(|arr| arr.clone())
        .unwrap_or_default();
    
    // 生成节点
    inp_content.push_str("*NODE, NSET=NALL\n");
    for node in nodes.iter() {
        if let (Some(id), Some(x), Some(y), Some(z)) = (
            node.get("id").and_then(|v| v.as_i64()),
            node.get("x").and_then(|v| v.as_f64()),
            node.get("y").and_then(|v| v.as_f64()),
            node.get("z").and_then(|v| v.as_f64())
        ) {
            inp_content.push_str(&format!("{}, {}, {}, {}\n", id, x, y, z));
        }
    }
    
    // 生成单元
    let element_type = mesh_data.get("element_type")
        .and_then(|v| v.as_str())
        .unwrap_or("C3D8");
    
    inp_content.push_str(&format!("*ELEMENT, TYPE={}, ELSET=EALL\n", element_type));
    for elem in elements.iter() {
        if let (Some(id), Some(node_ids)) = (
            elem.get("id").and_then(|v| v.as_i64()),
            elem.get("node_ids").and_then(|v| v.as_array())
        ) {
            let node_str: Vec<String> = node_ids.iter()
                .filter_map(|n| n.as_i64().map(|id| id.to_string()))
                .collect();
            if node_str.len() >= 4 {
                inp_content.push_str(&format!("{}, {}\n", id, node_str.join(", ")));
            }
        }
    }
    
    // 边界条件
    if let Some(bc_type) = bc_data.get("type").and_then(|v| v.as_str()) {
        match bc_type {
            "cantilever" => {
                if let Some(face_nodes) = bc_data.get("fixed_face_nodes")
                    .and_then(|v| v.as_array())
                {
                    inp_content.push_str("*BOUNDARY, OP=NEW\n");
                    for node in face_nodes.iter().take(10) {
                        if let Some(nid) = node.as_i64() {
                            inp_content.push_str(&format!("{}, 1, 6, 0\n", nid));
                        }
                    }
                }
            },
            "fixed" => {
                inp_content.push_str("*BOUNDARY, OP=NEW\n");
                inp_content.push_str("NALL, 1, 6, 0\n");
            },
            _ => {}
        }
    }
    
    // 荷载工况 (循环加载)
    let num_cycles = load_data.get("num_cycles")
        .and_then(|v| v.as_i64())
        .unwrap_or(1000) as i32;
    
    let load_amplitude = fatigue_params.load.stress_amplitude;
    
    // Step 1: 初始荷载
    inp_content.push_str("*STEP, NAME=LoadUp\n");
    inp_content.push_str("*STATIC\n");
    inp_content.push_str("0.1, 1.0\n");
    inp_content.push_str("*CLOAD\n");
    
    if let Some(load_nodes) = load_data.get("load_nodes")
        .and_then(|v| v.as_array())
    {
        for node in load_nodes.iter().take(5) {
            if let Some(nid) = node.as_i64() {
                inp_content.push_str(&format!("{}, 2, {:.1}\n", nid, load_amplitude));
            }
        }
    }
    inp_content.push_str("*END STEP\n");
    
    // Step 2: 循环加载 (简化 - 在后处理中计算损伤)
    inp_content.push_str("*STEP, NAME=CyclicLoad\n");
    inp_content.push_str("*STATIC\n");
    inp_content.push_str("0.1, 1.0\n");
    inp_content.push_str("*CLOAD\n");
    
    if let Some(load_nodes) = load_data.get("load_nodes")
        .and_then(|v| v.as_array())
    {
        for node in load_nodes.iter().take(5) {
            if let Some(nid) = node.as_i64() {
                inp_content.push_str(&format!("{}, 2, {:.1}\n", nid, -load_amplitude));
            }
        }
    }
    inp_content.push_str("*END STEP\n");
    
    // 输出请求
    inp_content.push_str("*EL PRINT, ELSET=EALL, FREQUENCY=1\n");
    inp_content.push_str("S\n");
    inp_content.push_str("*EL PRINT, ELSET=EALL, FREQUENCY=1\n");
    inp_content.push_str("E\n");
    inp_content.push_str("*NODE PRINT, NSET=NALL, FREQUENCY=1\n");
    inp_content.push_str("U\n");
    
    Ok(inp_content)
}

/// Tauri命令 - 运行疲劳仿真
/// 
/// 写入INP文件并调用CalculiX求解器
/// 
/// # 参数
/// - `inp_content`: CalculiX INP文件内容
/// - `work_dir`: 工作目录路径
/// 
/// # 返回
/// Result<HashMap, String>: 仿真结果, 包含状态、输出文件、结果数据等
#[tauri::command]
pub fn run_fatigue_simulation(
    inp_content: String,
    work_dir: String,
) -> Result<HashMap<String, serde_json::Value>, String> {
    use std::process::Command;
    use std::fs;
    
    let inp_path = format!("{}/fatigue.inp", work_dir);
    let out_path = format!("{}/fatigue.out", work_dir);
    
    // 写入输入文件
    fs::write(&inp_path, &inp_content)
        .map_err(|e| format!("Failed to write input file: {}", e))?;
    
    // 运行CalculiX (ccx)
    let result = Command::new("ccx")
        .arg(&inp_path)
        .arg("-o")
        .arg(&out_path)
        .output();
    
    let mut response = HashMap::new();
    
    match result {
        Ok(output) => {
            if output.status.success() {
                response.insert("status".to_string(), "success".to_json());
                response.insert("output_file".to_string(), out_path.to_json());
                
                // 解析结果 (如可用)
                if let Ok(content) = fs::read_to_string(&out_path) {
                    response.insert("result_data".to_string(), content.to_json());
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                response.insert("status".to_string(), "error".to_json());
                response.insert("message".to_string(), format!("CalculiX error: {}", stderr).to_json());
            }
        },
        Err(_) => {
            // CalculiX不可用, 返回演示结果
            response.insert("status".to_string(), "demo".to_json());
            response.insert("message".to_string(), "CalculiX not installed, showing demo results".to_json());
            
            // 演示损伤结果
            let mut demo_results = HashMap::new();
            demo_results.insert("damage".to_string(), 0.65f64.to_json());
            demo_results.insert("life_cycles".to_string(), 150000f64.to_json());
            demo_results.insert("safety_factor".to_string(), 1.2f64.to_json());
            demo_results.insert("max_stress".to_string(), 180f64.to_json());
            demo_results.insert("damage_at_nodes".to_string(), vec![0.1, 0.3, 0.5, 0.65, 0.4].to_json());
            
            response.insert("results".to_string(), demo_results.to_json());
        }
    }
    
    Ok(response)
}

// ============ 辅助trait用于JSON转换 ============

/// JSON转换辅助trait
trait ToJson {
    fn to_json(&self) -> serde_json::Value;
}

impl ToJson for String {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::String(self.clone())
    }
}

impl ToJson for &str {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::String(self.to_string())
    }
}

impl ToJson for f64 {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::Number(serde_json::Number::from_f64(*self).unwrap_or(serde_json::Number::from(0)))
    }
}

impl ToJson for i64 {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::Number(serde_json::Number::from(*self))
    }
}

impl<T: ToJson> ToJson for Vec<T> {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::Array(self.iter().map(|v| v.to_json()).collect())
    }
}

impl ToJson for HashMap<String, serde_json::Value> {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::Object(self.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
    }
}
