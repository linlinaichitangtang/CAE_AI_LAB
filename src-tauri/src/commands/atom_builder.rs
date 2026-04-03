//! Atom Structure Builder Module Commands
//! V1.5: Crystal structure generation, defect creation, and nanotube building

use serde::{Deserialize, Serialize};
use tauri::command;

// ============================================================================
// 数据结构
// ============================================================================

/// 单个原子数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtomData {
    pub element: String,
    pub position: [f64; 3],
    pub id: u32,
}

/// 原子构建结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtomBuilderResult {
    pub atoms: Vec<AtomData>,
    pub total_atoms: u32,
    pub box_size: [f64; 3],
    pub cell_vectors: [[f64; 3]; 3],
    pub density: f64,
    pub build_time_ms: f64,
}

/// 超胞构建配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupercellConfig {
    pub crystal_structure: String,
    pub lattice_constant: f64,
    pub repetitions: [u32; 3],
    pub element: String,
    pub basis_atoms: Vec<[f64; 3]>,
}

/// 非晶结构构建配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmorphousConfig {
    pub element: String,
    pub num_atoms: u32,
    pub box_size: [f64; 3],
    pub density: f64,
    pub seed: Option<u64>,
}

/// 界面构建配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceConfig {
    pub crystal_structure: String,
    pub lattice_constant: f64,
    pub element: String,
    pub orientation: String,
    pub vacuum_gap: f64,
    pub thickness: [f64; 3],
}

/// 缺陷构建配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefectConfig {
    pub crystal_structure: String,
    pub lattice_constant: f64,
    pub element: String,
    pub defect_type: String,
    pub defect_position: Option<[f64; 3]>,
    pub defect_count: u32,
    pub substitution_element: Option<String>,
}

/// 碳纳米管构建配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CntConfig {
    pub chirality_n: u32,
    pub chirality_m: u32,
    pub length_nm: f64,
    pub element: String,
}

/// 团簇构建配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {
    pub element: String,
    pub cluster_type: String,
    pub num_shells: u32,
    pub lattice_constant: f64,
}

/// 元素信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementInfo {
    pub symbol: String,
    pub name: String,
    pub atomic_number: u32,
    pub mass: f64,
    pub covalent_radius: f64,
    pub lattice_constant: Option<f64>,
    pub crystal_structure: Option<String>,
}

/// 原子构建模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtomBuilderTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub crystal_structure: String,
    pub element: String,
    pub lattice_constant: f64,
    pub repetitions: [u32; 3],
    pub category: String,
}

// ============================================================================
// 辅助函数
// ============================================================================

/// 获取晶体结构的基矢原子（分数坐标）
fn get_basis_atoms(crystal_structure: &str, custom_basis: &[[f64; 3]]) -> Vec<[f64; 3]> {
    if !custom_basis.is_empty() {
        return custom_basis.to_vec();
    }
    match crystal_structure {
        "FCC" => vec![
            [0.0, 0.0, 0.0],
            [0.5, 0.5, 0.0],
            [0.5, 0.0, 0.5],
            [0.0, 0.5, 0.5],
        ],
        "BCC" => vec![
            [0.0, 0.0, 0.0],
            [0.5, 0.5, 0.5],
        ],
        "SC" => vec![
            [0.0, 0.0, 0.0],
        ],
        "HCP" => vec![
            [0.0, 0.0, 0.0],
            [1.0 / 3.0, 2.0 / 3.0, 0.5],
        ],
        "Diamond" => vec![
            [0.0, 0.0, 0.0],
            [0.5, 0.5, 0.0],
            [0.5, 0.0, 0.5],
            [0.0, 0.5, 0.5],
            [0.25, 0.25, 0.25],
            [0.75, 0.75, 0.25],
            [0.75, 0.25, 0.75],
            [0.25, 0.75, 0.75],
        ],
        _ => vec![
            [0.0, 0.0, 0.0],
        ],
    }
}

/// 简单伪随机数生成器（线性同余法）
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        SimpleRng {
            state: if seed == 0 { 12345 } else { seed },
        }
    }

    fn next_f64(&mut self) -> f64 {
        // LCG parameters from Numerical Recipes
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        (self.state >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// 计算盒子体积
fn box_volume(size: &[f64; 3]) -> f64 {
    size[0] * size[1] * size[2]
}

/// 查找元素质量（简化版）
fn element_mass(symbol: &str) -> f64 {
    match symbol {
        "H" => 1.008,
        "C" => 12.011,
        "N" => 14.007,
        "O" => 15.999,
        "Al" => 26.982,
        "Si" => 28.086,
        "Fe" => 55.845,
        "Cu" => 63.546,
        "Ni" => 58.693,
        "Ti" => 47.867,
        "Mg" => 24.305,
        "Au" => 196.967,
        "Ag" => 107.868,
        "Pt" => 195.085,
        "W" => 183.841,
        "Mo" => 95.95,
        "Ta" => 180.948,
        "Zr" => 91.224,
        "Na" => 22.990,
        "Ca" => 40.078,
        _ => 12.011,
    }
}

// ============================================================================
// 命令实现
// ============================================================================

/// 构建超胞结构
///
/// 根据晶体结构类型和重复次数生成超胞原子位置。
/// 支持 FCC、BCC、HCP、Diamond、SC 五种晶体结构。
#[command]
pub fn build_supercell(config: SupercellConfig) -> Result<AtomBuilderResult, String> {
    tracing::info!(
        "构建超胞: 结构={}, 晶格常数={:.4} A, 重复={:?}, 元素={}",
        config.crystal_structure,
        config.lattice_constant,
        config.repetitions,
        config.element
    );

    let start = std::time::Instant::now();

    let basis = get_basis_atoms(&config.crystal_structure, &config.basis_atoms);
    let a = config.lattice_constant;
    let [nx, ny, nz] = config.repetitions;

    // HCP 特殊处理：c/a 比约为 1.633
    let (ax, ay, az) = if config.crystal_structure == "HCP" {
        (a, a * (3.0_f64).sqrt() / 2.0, a * 1.633)
    } else {
        (a, a, a)
    };

    let mut atoms = Vec::new();
    let mut atom_id: u32 = 0;

    for ix in 0..nx {
        for iy in 0..ny {
            for iz in 0..nz {
                for basis_atom in &basis {
                    let fx = (ix as f64 + basis_atom[0]) / nx as f64;
                    let fy = (iy as f64 + basis_atom[1]) / ny as f64;
                    let fz = (iz as f64 + basis_atom[2]) / nz as f64;

                    let position = [fx * ax * nx as f64, fy * ay * ny as f64, fz * az * nz as f64];

                    atoms.push(AtomData {
                        element: config.element.clone(),
                        position,
                        id: atom_id,
                    });
                    atom_id += 1;
                }
            }
        }
    }

    let box_size = [ax * nx as f64, ay * ny as f64, az * nz as f64];
    let cell_vectors = [
        [ax * nx as f64, 0.0, 0.0],
        [0.0, ay * ny as f64, 0.0],
        [0.0, 0.0, az * nz as f64],
    ];

    let vol = box_volume(&box_size);
    let mass = element_mass(&config.element);
    // 密度 = (原子数 * 质量) / (体积 * 1e-27 * 阿伏伽德罗常数)
    let density = (atoms.len() as f64 * mass) / (vol * 1e-27 * 6.022e23);

    let build_time_ms = start.elapsed().as_secs_f64() * 1000.0;

    tracing::info!(
        "超胞构建完成: 总原子数={}, 盒子尺寸={:.2}x{:.2}x{:.2} A, 密度={:.4} g/cm3, 耗时={:.2} ms",
        atoms.len(),
        box_size[0],
        box_size[1],
        box_size[2],
        density,
        build_time_ms
    );

    Ok(AtomBuilderResult {
        total_atoms: atoms.len() as u32,
        atoms,
        box_size,
        cell_vectors,
        density,
        build_time_ms,
    })
}

/// 构建非晶结构
///
/// 在指定盒子中随机生成原子位置，模拟非晶态结构。
/// 使用可重复的伪随机数生成器确保结果可复现。
#[command]
pub fn build_amorphous(config: AmorphousConfig) -> Result<AtomBuilderResult, String> {
    tracing::info!(
        "构建非晶结构: 元素={}, 原子数={}, 盒子={:.2}x{:.2}x{:.2} A, 密度={:.4} g/cm3",
        config.element,
        config.num_atoms,
        config.box_size[0],
        config.box_size[1],
        config.box_size[2],
        config.density
    );

    let start = std::time::Instant::now();

    let seed = config.seed.unwrap_or(42);
    let mut rng = SimpleRng::new(seed);

    let mut atoms = Vec::new();

    for i in 0..config.num_atoms {
        let position = [
            rng.next_f64() * config.box_size[0],
            rng.next_f64() * config.box_size[1],
            rng.next_f64() * config.box_size[2],
        ];

        atoms.push(AtomData {
            element: config.element.clone(),
            position,
            id: i,
        });
    }

    let cell_vectors = [
        [config.box_size[0], 0.0, 0.0],
        [0.0, config.box_size[1], 0.0],
        [0.0, 0.0, config.box_size[2]],
    ];

    let build_time_ms = start.elapsed().as_secs_f64() * 1000.0;

    tracing::info!(
        "非晶结构构建完成: 总原子数={}, 耗时={:.2} ms",
        atoms.len(),
        build_time_ms
    );

    Ok(AtomBuilderResult {
        total_atoms: atoms.len() as u32,
        atoms,
        box_size: config.box_size,
        cell_vectors,
        density: config.density,
        build_time_ms,
    })
}

/// 构建界面结构
///
/// 生成两个晶体平板（slab）并在中间加入真空层，用于表面/界面模拟。
#[command]
pub fn build_interface(config: InterfaceConfig) -> Result<AtomBuilderResult, String> {
    tracing::info!(
        "构建界面结构: 结构={}, 晶格常数={:.4} A, 元素={}, 取向={}, 真空间距={:.2} A, 厚度={:?}",
        config.crystal_structure,
        config.lattice_constant,
        config.element,
        config.orientation,
        config.vacuum_gap,
        config.thickness
    );

    let start = std::time::Instant::now();

    let a = config.lattice_constant;
    let basis = get_basis_atoms(&config.crystal_structure, &[]);

    // 根据厚度计算每个方向的重复次数
    let nx = (config.thickness[0] / a).ceil().max(1.0) as u32;
    let ny = (config.thickness[1] / a).ceil().max(1.0) as u32;
    let nz = (config.thickness[2] / a).ceil().max(1.0) as u32;

    let mut atoms = Vec::new();
    let mut atom_id: u32 = 0;

    // 构建第一个平板
    for ix in 0..nx {
        for iy in 0..ny {
            for iz in 0..nz {
                for basis_atom in &basis {
                    let position = [
                        (ix as f64 + basis_atom[0]) * a,
                        (iy as f64 + basis_atom[1]) * a,
                        (iz as f64 + basis_atom[2]) * a,
                    ];
                    atoms.push(AtomData {
                        element: config.element.clone(),
                        position,
                        id: atom_id,
                    });
                    atom_id += 1;
                }
            }
        }
    }

    let slab_height = nz as f64 * a;
    let total_z = 2.0 * slab_height + config.vacuum_gap;

    // 构建第二个平板（沿 z 方向偏移）
    let z_offset = slab_height + config.vacuum_gap;
    for ix in 0..nx {
        for iy in 0..ny {
            for iz in 0..nz {
                for basis_atom in &basis {
                    let position = [
                        (ix as f64 + basis_atom[0]) * a,
                        (iy as f64 + basis_atom[1]) * a,
                        (iz as f64 + basis_atom[2]) * a + z_offset,
                    ];
                    atoms.push(AtomData {
                        element: config.element.clone(),
                        position,
                        id: atom_id,
                    });
                    atom_id += 1;
                }
            }
        }
    }

    let box_size = [nx as f64 * a, ny as f64 * a, total_z];
    let cell_vectors = [
        [box_size[0], 0.0, 0.0],
        [0.0, box_size[1], 0.0],
        [0.0, 0.0, box_size[2]],
    ];

    let vol = box_volume(&box_size);
    let mass = element_mass(&config.element);
    let density = (atoms.len() as f64 * mass) / (vol * 1e-27 * 6.022e23);

    let build_time_ms = start.elapsed().as_secs_f64() * 1000.0;

    tracing::info!(
        "界面结构构建完成: 总原子数={}, 盒子={:.2}x{:.2}x{:.2} A, 密度={:.4} g/cm3, 耗时={:.2} ms",
        atoms.len(),
        box_size[0],
        box_size[1],
        box_size[2],
        density,
        build_time_ms
    );

    Ok(AtomBuilderResult {
        total_atoms: atoms.len() as u32,
        atoms,
        box_size,
        cell_vectors,
        density,
        build_time_ms,
    })
}

/// 构建缺陷结构
///
/// 在完美晶体中引入缺陷，包括空位、间隙原子、替位原子、刃位错、螺位错和晶界。
#[command]
pub fn build_defect(config: DefectConfig) -> Result<AtomBuilderResult, String> {
    tracing::info!(
        "构建缺陷结构: 结构={}, 晶格常数={:.4} A, 元素={}, 缺陷类型={}, 缺陷数量={}",
        config.crystal_structure,
        config.lattice_constant,
        config.element,
        config.defect_type,
        config.defect_count
    );

    let start = std::time::Instant::now();

    let a = config.lattice_constant;
    let basis = get_basis_atoms(&config.crystal_structure, &[]);

    // 构建基础超胞 (5x5x5)
    let (nx, ny, nz) = (5u32, 5u32, 5u32);
    let mut atoms = Vec::new();
    let mut atom_id: u32 = 0;

    for ix in 0..nx {
        for iy in 0..ny {
            for iz in 0..nz {
                for basis_atom in &basis {
                    let position = [
                        (ix as f64 + basis_atom[0]) * a,
                        (iy as f64 + basis_atom[1]) * a,
                        (iz as f64 + basis_atom[2]) * a,
                    ];
                    atoms.push(AtomData {
                        element: config.element.clone(),
                        position,
                        id: atom_id,
                    });
                    atom_id += 1;
                }
            }
        }
    }

    let box_size = [nx as f64 * a, ny as f64 * a, nz as f64 * a];
    let center = [box_size[0] / 2.0, box_size[1] / 2.0, box_size[2] / 2.0];

    // 缺陷位置（默认在中心附近）
    let defect_pos = config.defect_position.unwrap_or(center);

    match config.defect_type.as_str() {
        "vacancy" => {
            // 空位：移除距离缺陷位置最近的原子
            tracing::info!("创建 {} 个空位缺陷", config.defect_count);
            let mut removed = 0u32;
            let mut to_remove = Vec::new();
            for (idx, atom) in atoms.iter().enumerate() {
                if removed >= config.defect_count {
                    break;
                }
                let dx = atom.position[0] - defect_pos[0];
                let dy = atom.position[1] - defect_pos[1];
                let dz = atom.position[2] - defect_pos[2];
                let dist = (dx * dx + dy * dy + dz * dz).sqrt();
                if dist < a * 0.6 {
                    to_remove.push(idx);
                    removed += 1;
                }
            }
            // 从后往前删除以保持索引正确
            to_remove.sort_unstable_by(|a, b| b.cmp(a));
            for idx in to_remove {
                atoms.remove(idx);
            }
        }
        "interstitial" => {
            // 间隙原子：在缺陷位置附近添加额外原子
            tracing::info!("创建 {} 个间隙原子缺陷", config.defect_count);
            for i in 0..config.defect_count {
                let offset = i as f64 * a * 0.3;
                atoms.push(AtomData {
                    element: config.element.clone(),
                    position: [
                        defect_pos[0] + offset,
                        defect_pos[1],
                        defect_pos[2],
                    ],
                    id: atom_id,
                });
                atom_id += 1;
            }
        }
        "substitution" => {
            // 替位原子：将最近原子替换为指定元素
            tracing::info!("创建 {} 个替位缺陷", config.defect_count);
            let sub_element = config
                .substitution_element
                .clone()
                .unwrap_or_else(|| "Si".to_string());
            let mut substituted = 0u32;
            for atom in atoms.iter_mut() {
                if substituted >= config.defect_count {
                    break;
                }
                let dx = atom.position[0] - defect_pos[0];
                let dy = atom.position[1] - defect_pos[1];
                let dz = atom.position[2] - defect_pos[2];
                let dist = (dx * dx + dy * dy + dz * dz).sqrt();
                if dist < a * 0.6 {
                    atom.element = sub_element.clone();
                    substituted += 1;
                }
            }
        }
        "dislocation_edge" => {
            // 刃位错：沿 y 轴插入半原子面
            tracing::info!("创建刃位错缺陷");
            let dislocation_x = defect_pos[0];
            let mut new_atoms = Vec::new();
            for atom in &atoms {
                let dx = atom.position[0] - dislocation_x;
                let _dy = atom.position[1] - defect_pos[1];
                let dz = atom.position[2] - defect_pos[2];
                let r = (dx * dx + dz * dz).sqrt().max(0.01);
                let theta = dz.atan2(dx);
                let burgers = a / 2.0_f64.sqrt();
                let displacement = burgers * theta / (2.0 * std::f64::consts::PI);
                new_atoms.push(AtomData {
                    element: atom.element.clone(),
                    position: [
                        atom.position[0] + displacement * (-dz / r) * 0.3,
                        atom.position[1],
                        atom.position[2] + displacement * (dx / r) * 0.3,
                    ],
                    id: atom.id,
                });
            }
            atoms = new_atoms;
        }
        "dislocation_screw" => {
            // 螺位错：沿 z 轴的螺旋位移场
            tracing::info!("创建螺位错缺陷");
            let mut new_atoms = Vec::new();
            for atom in &atoms {
                let dx = atom.position[0] - defect_pos[0];
                let dy = atom.position[1] - defect_pos[1];
                let r = (dx * dx + dy * dy).sqrt().max(0.01);
                let theta = dy.atan2(dx);
                let burgers = a / 2.0_f64.sqrt();
                let displacement = burgers * theta / (2.0 * std::f64::consts::PI);
                new_atoms.push(AtomData {
                    element: atom.element.clone(),
                    position: [
                        atom.position[0] + displacement * (-dy / r) * 0.3,
                        atom.position[1] + displacement * (dx / r) * 0.3,
                        atom.position[2],
                    ],
                    id: atom.id,
                });
            }
            atoms = new_atoms;
        }
        "grain_boundary" => {
            // 晶界：将上半部分旋转一个小角度
            tracing::info!("创建晶界缺陷");
            let angle = 5.0 * std::f64::consts::PI / 180.0; // 5 度旋转
            let cos_a = angle.cos();
            let sin_a = angle.sin();
            let mid_y = box_size[1] / 2.0;
            for atom in atoms.iter_mut() {
                if atom.position[1] > mid_y {
                    let dy = atom.position[1] - mid_y;
                    let dz = atom.position[2] - defect_pos[2];
                    atom.position[1] = mid_y + dy * cos_a - dz * sin_a;
                    atom.position[2] = defect_pos[2] + dy * sin_a + dz * cos_a;
                }
            }
        }
        _ => {
            return Err(format!("未知的缺陷类型: {}", config.defect_type));
        }
    }

    let cell_vectors = [
        [box_size[0], 0.0, 0.0],
        [0.0, box_size[1], 0.0],
        [0.0, 0.0, box_size[2]],
    ];

    let vol = box_volume(&box_size);
    let mass = element_mass(&config.element);
    let density = (atoms.len() as f64 * mass) / (vol * 1e-27 * 6.022e23);

    let build_time_ms = start.elapsed().as_secs_f64() * 1000.0;

    tracing::info!(
        "缺陷结构构建完成: 总原子数={}, 缺陷类型={}, 耗时={:.2} ms",
        atoms.len(),
        config.defect_type,
        build_time_ms
    );

    Ok(AtomBuilderResult {
        total_atoms: atoms.len() as u32,
        atoms,
        box_size,
        cell_vectors,
        density,
        build_time_ms,
    })
}

/// 构建碳纳米管
///
/// 根据 (n,m) 手性指数生成碳纳米管原子位置。
/// 使用标准 C-C 键长 1.421 A。
#[command]
pub fn build_cnt(config: CntConfig) -> Result<AtomBuilderResult, String> {
    tracing::info!(
        "构建碳纳米管: 手性=({},{}), 长度={:.2} nm, 元素={}",
        config.chirality_n,
        config.chirality_m,
        config.length_nm,
        config.element
    );

    let start = std::time::Instant::now();

    let n = config.chirality_n as f64;
    let m = config.chirality_m as f64;
    let cc_bond = 1.421; // C-C 键长 (A)
    let a_lat = cc_bond * (3.0_f64).sqrt(); // 石墨烯晶格常数

    // 计算手性向量 |Ch| = a * sqrt(n^2 + nm + m^2)
    let ch_length = a_lat * (n * n + n * m + m * m).sqrt();

    // 管径 R = |Ch| / (2 * pi)
    let radius = ch_length / (2.0 * std::f64::consts::PI);

    // 管长 (nm -> A)
    let tube_length = config.length_nm * 10.0;

    // 生成纳米管原子：沿圆周和轴向排列
    let circumference = 2.0 * std::f64::consts::PI * radius;
    let n_circum = (circumference / cc_bond).round() as u32;
    let n_axial = (tube_length / (cc_bond * (3.0_f64).sqrt() / 2.0)).round() as u32;

    let mut atoms = Vec::new();
    let mut atom_id: u32 = 0;

    for i in 0..n_circum {
        for j in 0..n_axial {
            let theta = 2.0 * std::f64::consts::PI * i as f64 / n_circum as f64;
            let z_base = j as f64 * cc_bond * (3.0_f64).sqrt() / 2.0;

            // A 子晶格
            let x_a = radius * theta.cos();
            let y_a = radius * theta.sin();
            let z_a = z_base;

            if z_a <= tube_length {
                atoms.push(AtomData {
                    element: config.element.clone(),
                    position: [x_a, y_a, z_a],
                    id: atom_id,
                });
                atom_id += 1;
            }

            // B 子晶格（沿轴向偏移）
            let theta_b =
                2.0 * std::f64::consts::PI * (i as f64 + 0.5) / n_circum as f64;
            let x_b = radius * theta_b.cos();
            let y_b = radius * theta_b.sin();
            let z_b = z_base + cc_bond / (3.0_f64).sqrt();

            if z_b <= tube_length {
                atoms.push(AtomData {
                    element: config.element.clone(),
                    position: [x_b, y_b, z_b],
                    id: atom_id,
                });
                atom_id += 1;
            }
        }
    }

    // 将纳米管居中
    let x_min = atoms.iter().map(|a| a.position[0]).fold(f64::INFINITY, f64::min);
    let x_max = atoms.iter().map(|a| a.position[0]).fold(f64::NEG_INFINITY, f64::max);
    let y_min = atoms.iter().map(|a| a.position[1]).fold(f64::INFINITY, f64::min);
    let y_max = atoms.iter().map(|a| a.position[1]).fold(f64::NEG_INFINITY, f64::max);

    let cx = (x_min + x_max) / 2.0;
    let cy = (y_min + y_max) / 2.0;

    for atom in atoms.iter_mut() {
        atom.position[0] -= cx;
        atom.position[1] -= cy;
    }

    let box_size = [
        (x_max - x_min) + 5.0,
        (y_max - y_min) + 5.0,
        tube_length,
    ];

    let cell_vectors = [
        [box_size[0], 0.0, 0.0],
        [0.0, box_size[1], 0.0],
        [0.0, 0.0, box_size[2]],
    ];

    let vol = box_volume(&box_size);
    let mass = element_mass(&config.element);
    let density = (atoms.len() as f64 * mass) / (vol * 1e-27 * 6.022e23);

    let build_time_ms = start.elapsed().as_secs_f64() * 1000.0;

    tracing::info!(
        "碳纳米管构建完成: 手性=({},{}), 半径={:.3} A, 总原子数={}, 耗时={:.2} ms",
        config.chirality_n,
        config.chirality_m,
        radius,
        atoms.len(),
        build_time_ms
    );

    Ok(AtomBuilderResult {
        total_atoms: atoms.len() as u32,
        atoms,
        box_size,
        cell_vectors,
        density,
        build_time_ms,
    })
}

/// 构建纳米团簇
///
/// 生成不同对称性的纳米团簇，包括二十面体、十面体、立方体和球形。
#[command]
pub fn build_cluster(config: ClusterConfig) -> Result<AtomBuilderResult, String> {
    tracing::info!(
        "构建纳米团簇: 元素={}, 类型={}, 壳层数={}, 晶格常数={:.4} A",
        config.element,
        config.cluster_type,
        config.num_shells,
        config.lattice_constant
    );

    let start = std::time::Instant::now();

    let a = config.lattice_constant;
    let shells = config.num_shells;
    let mut atoms = Vec::new();
    let mut atom_id: u32 = 0;

    match config.cluster_type.as_str() {
        "icosahedron" => {
            // 二十面体团簇：Mackay 二十面体
            // 第一层：1 个中心原子
            // 后续层：每层 10*k^2 + 2 个原子（k 为层序号）
            atoms.push(AtomData {
                element: config.element.clone(),
                position: [0.0, 0.0, 0.0],
                id: atom_id,
            });
            atom_id += 1;

            // 二十面体的 12 个顶点方向
            let phi = (1.0 + (5.0_f64).sqrt()) / 2.0; // 黄金比例
            let ico_vertices: [[f64; 3]; 12] = [
                [0.0, 1.0, phi],
                [0.0, -1.0, phi],
                [0.0, 1.0, -phi],
                [0.0, -1.0, -phi],
                [1.0, phi, 0.0],
                [-1.0, phi, 0.0],
                [1.0, -phi, 0.0],
                [-1.0, -phi, 0.0],
                [phi, 0.0, 1.0],
                [-phi, 0.0, 1.0],
                [phi, 0.0, -1.0],
                [-phi, 0.0, -1.0],
            ];

            // 归一化顶点
            let ico_norm: Vec<[f64; 3]> = ico_vertices
                .iter()
                .map(|v| {
                    let norm = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
                    [v[0] / norm, v[1] / norm, v[2] / norm]
                })
                .collect();

            for layer in 1..=shells {
                let r = layer as f64 * a;
                // 沿 12 个顶点方向放置原子
                for vertex in &ico_norm {
                    let pos = [
                        vertex[0] * r,
                        vertex[1] * r,
                        vertex[2] * r,
                    ];
                    atoms.push(AtomData {
                        element: config.element.clone(),
                        position: pos,
                        id: atom_id,
                    });
                    atom_id += 1;
                }
                // 在顶点之间插值放置原子
                for i in 0..12 {
                    for j in (i + 1)..12 {
                        let vi = ico_norm[i];
                        let vj = ico_norm[j];
                        let dot =
                            vi[0] * vj[0] + vi[1] * vj[1] + vi[2] * vj[2];
                        // 只连接相邻顶点（点积约为 1/phi^2 ≈ 0.382）
                        if dot > 0.3 && dot < 0.5 {
                            let steps = layer as u32;
                            for s in 1..steps {
                                let t = s as f64 / steps as f64;
                                let pos = [
                                    (vi[0] * (1.0 - t) + vj[0] * t) * r,
                                    (vi[1] * (1.0 - t) + vj[1] * t) * r,
                                    (vi[2] * (1.0 - t) + vj[2] * t) * r,
                                ];
                                atoms.push(AtomData {
                                    element: config.element.clone(),
                                    position: pos,
                                    id: atom_id,
                                });
                                atom_id += 1;
                            }
                        }
                    }
                }
            }
        }
        "decahedron" => {
            // 十面体团簇：5 个四面体共享一个公共边
            atoms.push(AtomData {
                element: config.element.clone(),
                position: [0.0, 0.0, 0.0],
                id: atom_id,
            });
            atom_id += 1;

            for layer in 1..=shells {
                let r = layer as f64 * a;
                // 5 重对称轴沿 z 方向
                for k in 0..5 {
                    let theta = 2.0 * std::f64::consts::PI * k as f64 / 5.0;
                    // 顶部锥面
                    for s in 0..=layer {
                        let frac = s as f64 / layer as f64;
                        let pos = [
                            r * frac * theta.cos(),
                            r * frac * theta.sin(),
                            r * (1.0 - frac),
                        ];
                        atoms.push(AtomData {
                            element: config.element.clone(),
                            position: pos,
                            id: atom_id,
                        });
                        atom_id += 1;
                    }
                    // 底部锥面
                    for s in 0..=layer {
                        let frac = s as f64 / layer as f64;
                        let pos = [
                            r * frac * theta.cos(),
                            r * frac * theta.sin(),
                            -r * (1.0 - frac),
                        ];
                        atoms.push(AtomData {
                            element: config.element.clone(),
                            position: pos,
                            id: atom_id,
                        });
                        atom_id += 1;
                    }
                }
            }
        }
        "fcc_cube" => {
            // FCC 立方团簇
            let half = shells as f64;
            for ix in -(shells as i32)..=(shells as i32) {
                for iy in -(shells as i32)..=(shells as i32) {
                    for iz in -(shells as i32)..=(shells as i32) {
                        let dist = ((ix as f64).powi(2)
                            + (iy as f64).powi(2)
                            + (iz as f64).powi(2))
                        .sqrt();
                        if dist <= half {
                            let pos = [ix as f64 * a, iy as f64 * a, iz as f64 * a];
                            atoms.push(AtomData {
                                element: config.element.clone(),
                                position: pos,
                                id: atom_id,
                            });
                            atom_id += 1;
                            // FCC 额外原子
                            atoms.push(AtomData {
                                element: config.element.clone(),
                                position: [
                                    pos[0] + a * 0.5,
                                    pos[1] + a * 0.5,
                                    pos[2],
                                ],
                                id: atom_id,
                            });
                            atom_id += 1;
                            atoms.push(AtomData {
                                element: config.element.clone(),
                                position: [
                                    pos[0] + a * 0.5,
                                    pos[1],
                                    pos[2] + a * 0.5,
                                ],
                                id: atom_id,
                            });
                            atom_id += 1;
                            atoms.push(AtomData {
                                element: config.element.clone(),
                                position: [
                                    pos[0],
                                    pos[1] + a * 0.5,
                                    pos[2] + a * 0.5,
                                ],
                                id: atom_id,
                            });
                            atom_id += 1;
                        }
                    }
                }
            }
        }
        "bcc_cube" => {
            // BCC 立方团簇
            let half = shells as f64;
            for ix in -(shells as i32)..=(shells as i32) {
                for iy in -(shells as i32)..=(shells as i32) {
                    for iz in -(shells as i32)..=(shells as i32) {
                        let dist = ((ix as f64).powi(2)
                            + (iy as f64).powi(2)
                            + (iz as f64).powi(2))
                        .sqrt();
                        if dist <= half {
                            let pos = [ix as f64 * a, iy as f64 * a, iz as f64 * a];
                            atoms.push(AtomData {
                                element: config.element.clone(),
                                position: pos,
                                id: atom_id,
                            });
                            atom_id += 1;
                            // BCC 体心原子
                            atoms.push(AtomData {
                                element: config.element.clone(),
                                position: [
                                    pos[0] + a * 0.5,
                                    pos[1] + a * 0.5,
                                    pos[2] + a * 0.5,
                                ],
                                id: atom_id,
                            });
                            atom_id += 1;
                        }
                    }
                }
            }
        }
        "sphere" => {
            // 球形团簇：在球内填充原子
            let radius = shells as f64 * a;
            let spacing = a * 0.8;
            let n_steps = (radius / spacing).ceil() as i32;

            for ix in -n_steps..=n_steps {
                for iy in -n_steps..=n_steps {
                    for iz in -n_steps..=n_steps {
                        let x = ix as f64 * spacing;
                        let y = iy as f64 * spacing;
                        let z = iz as f64 * spacing;
                        let dist = (x * x + y * y + z * z).sqrt();
                        if dist <= radius {
                            atoms.push(AtomData {
                                element: config.element.clone(),
                                position: [x, y, z],
                                id: atom_id,
                            });
                            atom_id += 1;
                        }
                    }
                }
            }
        }
        _ => {
            return Err(format!("未知的团簇类型: {}", config.cluster_type));
        }
    }

    // 计算包围盒
    let x_min = atoms.iter().map(|a| a.position[0]).fold(f64::INFINITY, f64::min);
    let x_max = atoms.iter().map(|a| a.position[0]).fold(f64::NEG_INFINITY, f64::max);
    let y_min = atoms.iter().map(|a| a.position[1]).fold(f64::INFINITY, f64::min);
    let y_max = atoms.iter().map(|a| a.position[1]).fold(f64::NEG_INFINITY, f64::max);
    let z_min = atoms.iter().map(|a| a.position[2]).fold(f64::INFINITY, f64::min);
    let z_max = atoms.iter().map(|a| a.position[2]).fold(f64::NEG_INFINITY, f64::max);

    let box_size = [
        x_max - x_min + 2.0 * a,
        y_max - y_min + 2.0 * a,
        z_max - z_min + 2.0 * a,
    ];

    let cell_vectors = [
        [box_size[0], 0.0, 0.0],
        [0.0, box_size[1], 0.0],
        [0.0, 0.0, box_size[2]],
    ];

    let vol = box_volume(&box_size);
    let mass = element_mass(&config.element);
    let density = (atoms.len() as f64 * mass) / (vol * 1e-27 * 6.022e23);

    let build_time_ms = start.elapsed().as_secs_f64() * 1000.0;

    tracing::info!(
        "团簇构建完成: 类型={}, 总原子数={}, 盒子={:.2}x{:.2}x{:.2} A, 耗时={:.2} ms",
        config.cluster_type,
        atoms.len(),
        box_size[0],
        box_size[1],
        box_size[2],
        build_time_ms
    );

    Ok(AtomBuilderResult {
        total_atoms: atoms.len() as u32,
        atoms,
        box_size,
        cell_vectors,
        density,
        build_time_ms,
    })
}

/// 获取元素库
///
/// 返回至少 20 种常见元素的基本信息，包括符号、名称、原子序数、质量、共价半径等。
#[command]
pub fn get_element_library() -> Result<Vec<ElementInfo>, String> {
    tracing::info!("获取元素库");

    let elements = vec![
        ElementInfo {
            symbol: "H".to_string(),
            name: "Hydrogen".to_string(),
            atomic_number: 1,
            mass: 1.008,
            covalent_radius: 0.31,
            lattice_constant: None,
            crystal_structure: None,
        },
        ElementInfo {
            symbol: "C".to_string(),
            name: "Carbon".to_string(),
            atomic_number: 6,
            mass: 12.011,
            covalent_radius: 0.76,
            lattice_constant: Some(3.567),
            crystal_structure: Some("Diamond".to_string()),
        },
        ElementInfo {
            symbol: "N".to_string(),
            name: "Nitrogen".to_string(),
            atomic_number: 7,
            mass: 14.007,
            covalent_radius: 0.71,
            lattice_constant: None,
            crystal_structure: None,
        },
        ElementInfo {
            symbol: "O".to_string(),
            name: "Oxygen".to_string(),
            atomic_number: 8,
            mass: 15.999,
            covalent_radius: 0.66,
            lattice_constant: None,
            crystal_structure: None,
        },
        ElementInfo {
            symbol: "Na".to_string(),
            name: "Sodium".to_string(),
            atomic_number: 11,
            mass: 22.990,
            covalent_radius: 1.66,
            lattice_constant: Some(4.291),
            crystal_structure: Some("BCC".to_string()),
        },
        ElementInfo {
            symbol: "Mg".to_string(),
            name: "Magnesium".to_string(),
            atomic_number: 12,
            mass: 24.305,
            covalent_radius: 1.41,
            lattice_constant: Some(3.209),
            crystal_structure: Some("HCP".to_string()),
        },
        ElementInfo {
            symbol: "Al".to_string(),
            name: "Aluminium".to_string(),
            atomic_number: 13,
            mass: 26.982,
            covalent_radius: 1.21,
            lattice_constant: Some(4.046),
            crystal_structure: Some("FCC".to_string()),
        },
        ElementInfo {
            symbol: "Si".to_string(),
            name: "Silicon".to_string(),
            atomic_number: 14,
            mass: 28.086,
            covalent_radius: 1.11,
            lattice_constant: Some(5.431),
            crystal_structure: Some("Diamond".to_string()),
        },
        ElementInfo {
            symbol: "Ca".to_string(),
            name: "Calcium".to_string(),
            atomic_number: 20,
            mass: 40.078,
            covalent_radius: 1.76,
            lattice_constant: Some(5.588),
            crystal_structure: Some("FCC".to_string()),
        },
        ElementInfo {
            symbol: "Ti".to_string(),
            name: "Titanium".to_string(),
            atomic_number: 22,
            mass: 47.867,
            covalent_radius: 1.60,
            lattice_constant: Some(2.951),
            crystal_structure: Some("HCP".to_string()),
        },
        ElementInfo {
            symbol: "Fe".to_string(),
            name: "Iron".to_string(),
            atomic_number: 26,
            mass: 55.845,
            covalent_radius: 1.32,
            lattice_constant: Some(2.870),
            crystal_structure: Some("BCC".to_string()),
        },
        ElementInfo {
            symbol: "Ni".to_string(),
            name: "Nickel".to_string(),
            atomic_number: 28,
            mass: 58.693,
            covalent_radius: 1.24,
            lattice_constant: Some(3.524),
            crystal_structure: Some("FCC".to_string()),
        },
        ElementInfo {
            symbol: "Cu".to_string(),
            name: "Copper".to_string(),
            atomic_number: 29,
            mass: 63.546,
            covalent_radius: 1.32,
            lattice_constant: Some(3.615),
            crystal_structure: Some("FCC".to_string()),
        },
        ElementInfo {
            symbol: "Zr".to_string(),
            name: "Zirconium".to_string(),
            atomic_number: 40,
            mass: 91.224,
            covalent_radius: 1.75,
            lattice_constant: Some(3.232),
            crystal_structure: Some("HCP".to_string()),
        },
        ElementInfo {
            symbol: "Mo".to_string(),
            name: "Molybdenum".to_string(),
            atomic_number: 42,
            mass: 95.95,
            covalent_radius: 1.54,
            lattice_constant: Some(3.147),
            crystal_structure: Some("BCC".to_string()),
        },
        ElementInfo {
            symbol: "Ag".to_string(),
            name: "Silver".to_string(),
            atomic_number: 47,
            mass: 107.868,
            covalent_radius: 1.45,
            lattice_constant: Some(4.086),
            crystal_structure: Some("FCC".to_string()),
        },
        ElementInfo {
            symbol: "Ta".to_string(),
            name: "Tantalum".to_string(),
            atomic_number: 73,
            mass: 180.948,
            covalent_radius: 1.70,
            lattice_constant: Some(3.303),
            crystal_structure: Some("BCC".to_string()),
        },
        ElementInfo {
            symbol: "W".to_string(),
            name: "Tungsten".to_string(),
            atomic_number: 74,
            mass: 183.841,
            covalent_radius: 1.62,
            lattice_constant: Some(3.165),
            crystal_structure: Some("BCC".to_string()),
        },
        ElementInfo {
            symbol: "Pt".to_string(),
            name: "Platinum".to_string(),
            atomic_number: 78,
            mass: 195.085,
            covalent_radius: 1.36,
            lattice_constant: Some(3.924),
            crystal_structure: Some("FCC".to_string()),
        },
        ElementInfo {
            symbol: "Au".to_string(),
            name: "Gold".to_string(),
            atomic_number: 79,
            mass: 196.967,
            covalent_radius: 1.36,
            lattice_constant: Some(4.079),
            crystal_structure: Some("FCC".to_string()),
        },
    ];

    tracing::info!("元素库返回 {} 个元素", elements.len());

    Ok(elements)
}

/// 计算密度
///
/// 根据原子数、盒子尺寸和平均原子质量计算材料密度。
/// 公式: density = (num_atoms * avg_mass) / (box_volume * 1e-27 * 6.022e23)
/// 其中 box_volume 单位为 A^3，结果单位为 g/cm^3。
#[command]
pub fn calculate_density(num_atoms: u32, box_size: [f64; 3], avg_mass: f64) -> Result<f64, String> {
    tracing::info!(
        "计算密度: 原子数={}, 盒子={:.2}x{:.2}x{:.2} A, 平均质量={:.4} u",
        num_atoms,
        box_size[0],
        box_size[1],
        box_size[2],
        avg_mass
    );

    let vol = box_volume(&box_size);
    if vol <= 0.0 {
        return Err("盒子体积必须大于零".to_string());
    }

    let density = (num_atoms as f64 * avg_mass) / (vol * 1e-27 * 6.022e23);

    tracing::info!("计算密度结果: {:.4} g/cm3", density);

    Ok(density)
}

/// 获取原子构建模板
///
/// 返回 5 个预定义的常用晶体结构构建模板，方便用户快速开始。
#[command]
pub fn get_atom_builder_templates() -> Result<Vec<AtomBuilderTemplate>, String> {
    tracing::info!("获取原子构建模板");

    let templates = vec![
        AtomBuilderTemplate {
            id: "fcc_al_bulk".to_string(),
            name: "FCC Al Bulk".to_string(),
            description: "面心立方铝块体结构，适用于金属铝的力学和热学性质模拟".to_string(),
            crystal_structure: "FCC".to_string(),
            element: "Al".to_string(),
            lattice_constant: 4.046,
            repetitions: [5, 5, 5],
            category: "metal".to_string(),
        },
        AtomBuilderTemplate {
            id: "bcc_fe_bulk".to_string(),
            name: "BCC Fe Bulk".to_string(),
            description: "体心立方铁块体结构，适用于铁基合金和磁性材料研究".to_string(),
            crystal_structure: "BCC".to_string(),
            element: "Fe".to_string(),
            lattice_constant: 2.870,
            repetitions: [5, 5, 5],
            category: "metal".to_string(),
        },
        AtomBuilderTemplate {
            id: "hcp_ti_bulk".to_string(),
            name: "HCP Ti Bulk".to_string(),
            description: "密排六方钛块体结构，适用于轻质合金和生物医用材料研究".to_string(),
            crystal_structure: "HCP".to_string(),
            element: "Ti".to_string(),
            lattice_constant: 2.951,
            repetitions: [5, 5, 5],
            category: "metal".to_string(),
        },
        AtomBuilderTemplate {
            id: "si_diamond".to_string(),
            name: "Si Diamond".to_string(),
            description: "金刚石结构硅晶体，适用于半导体和电子器件模拟".to_string(),
            crystal_structure: "Diamond".to_string(),
            element: "Si".to_string(),
            lattice_constant: 5.431,
            repetitions: [3, 3, 3],
            category: "semiconductor".to_string(),
        },
        AtomBuilderTemplate {
            id: "graphene_sheet".to_string(),
            name: "Graphene Sheet".to_string(),
            description: "石墨烯单层结构，适用于二维材料性质和纳米器件研究".to_string(),
            crystal_structure: "HCP".to_string(),
            element: "C".to_string(),
            lattice_constant: 2.461,
            repetitions: [8, 8, 1],
            category: "2d_material".to_string(),
        },
    ];

    tracing::info!("返回 {} 个构建模板", templates.len());

    Ok(templates)
}
