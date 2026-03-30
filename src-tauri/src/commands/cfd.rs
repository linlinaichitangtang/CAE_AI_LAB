//! CFD (Computational Fluid Dynamics) 命令处理
//! 生成OpenFOAM格式输入文件并解析结果

use std::fs;
use std::path::Path;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use tauri::State;
use crate::db::DbConnection;
use crate::errors::AppError;

/// CFD边界条件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoundaryType {
    VelocityInlet,
    PressureOutlet,
    Wall,
    Symmetry,
    Outlet,
}

/// 边界条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundaryCondition {
    pub id: u64,
    pub boundary_type: BoundaryType,
    pub velocity: Option<f64>,        // 入口速度 (m/s)
    pub turbulence_intensity: Option<f64>, // 湍流强度 (%)
    pub hydraulic_diameter: Option<f64>,  // 水力直径 (m)
    pub gauge_pressure: Option<f64>,     // 出口静压 (Pa)
    pub wall_slip: Option<bool>,         // 是否滑移壁面
    pub face_ids: Vec<u32>,             // 选中的面ID列表
}

/// 流体材料
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FluidMaterial {
    Air,
    Water,
    Oil,
    Custom { density: f64, viscosity: f64 },
}

/// 湍流模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TurbulenceModel {
    Laminar,
    KEpsilon,
    KOmegaSST,
}

/// CFD求解控制参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfdSetup {
    pub project_id: u64,
    pub domain_regions: Vec<DomainRegion>,
    pub material: FluidMaterial,
    pub boundary_conditions: Vec<BoundaryCondition>,
    pub turbulence_model: TurbulenceModel,
    pub convergence_tolerance: f64,
    pub max_iterations: u32,
}

/// 区域标记（流体域/固体域）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainRegion {
    pub region_type: RegionType,
    pub face_ids: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegionType {
    Fluid,
    Solid,
}

/// CFD结果统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfdResultStats {
    pub iterations: u32,
    pub converged: bool,
    pub cl: Option<f64>,   // 升力系数
    pub cd: Option<f64>,   // 阻力系数
    pub cm: Option<f64>,   // 力矩系数
}

/// 流速场数据点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VelocityPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub u: f64,
    pub v: f64,
    pub w: f64,
    pub magnitude: f64,
}

/// 压力场数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PressurePoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pressure: f64,
}

/// 生成OpenFOAM案例目录结构
#[tauri::command]
pub fn generate_openfoam_case(
    setup: CfdSetup,
    db: State<'_, DbConnection>,
) -> Result<String, AppError> {
    // 创建案例目录结构
    let project_dir = get_project_dir(setup.project_id)?;
    let case_dir = Path::new(&project_dir).join("openfoam_case");

    fs::create_dir_all(&case_dir)?;
    fs::create_dir_all(case_dir.join("0"))?");
    fs::create_dir_all(case_dir.join("constant"))?;
    fs::create_dir_all(case_dir.join("system"))?;

    // 生成controlDict
    generate_control_dict(&case_dir, setup.convergence_tolerance, setup.max_iterations)?;

    // 生成transportProperties（物理性质）
    generate_transport_properties(&case_dir, &setup.material)?;

    // 生成turbulenceProperties（湍流模型）
    generate_turbulence_properties(&case_dir, &setup.turbulence_model)?;

    // 生成blockMeshDict（网格）- 需要根据标记的区域
    generate_block_mesh_dict(&case_dir, &setup.domain_regions)?;

    // 生成0/U、0/p等边界条件文件
    generate_0_files(&case_dir, &setup.boundary_conditions)?;

    // 生成fvSchemes和fvSolution
    generate_fv_files(&case_dir, &setup.turbulence_model)?;

    // 保存setup到数据库
    save_cfd_setup_to_db(&db, &setup)?;

    Ok(format!("OpenFOAM case generated at: {}", case_dir.display())
}

/// 获取材料属性
fn get_material_properties(material: &FluidMaterial) -> (f64, f64) {
    match material {
        FluidMaterial::Air => (1.225, 1.81e-5),
        FluidMaterial::Water => (1000.0, 0.001),
        FluidMaterial::Oil => (850.0, 0.08),
        FluidMaterial::Custom { density, viscosity } => (*density, *viscosity),
    }
}

/// 生成controlDict
fn generate_control_dict(
    case_dir: &Path,
    tolerance: f64,
    max_iter: u32,
) -> Result<(), AppError> {
    let content = format!(r#"/*--------------------------------*- C++ -*----------------------------------*\
  =========                 |
  \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox
   \\    /   O peration     | Website:  https://openfoam.org
    \\  /    A nd           | Copyright (C) 2011-2018 OpenFOAM Foundation
     \\/     M anipulation  |
\*---------------------------------------------------------------------------*/
application     simpleFoam;

startFrom       startTime;

startTime       0;

stopAt          endTime;

endTime         1;

deltaT          1;

writeControl    timeStep;

writeInterval   100;

purgeWrite      0;

writeFormat     ascii;

writePrecision  6;

timePrecision   6;

runTimeModifiable yes;

maxCo           1;
maxDeltaT       1;

convergenceResidual {
    U           {tolerance {};}
    p           {tolerance {};}
    k           {tolerance {};}
    epsilon     {tolerance {};}
    omega       {tolerance {};}
}

maxIter        {};
"#, tolerance, tolerance, tolerance, tolerance, tolerance, max_iter);

    fs::write(case_dir.join("system/controlDict"), content)?;
    Ok(())
}

/// 生成transportProperties
fn generate_transport_properties(
    case_dir: &Path,
    material: &FluidMaterial,
) -> Result<(), AppError> {
    let (rho, nu) = get_material_properties(material);

    let content = format!(r#"/*--------------------------------*- C++ -*----------------------------------*\
  =========                 |
  \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox
   \\    /   O peration     | Website:  https://openfoam.org
    \\  /    A nd           | Copyright (C) 2011-2018 OpenFOAM Foundation
     \\/     M anipulation  |
\*---------------------------------------------------------------------------*/

transportModel  Newtonian;

nu              [0 2 -1 0 0 0 0] {};

rho             [1 -3 0 0 0 0 0] {};
"#, nu, rho);

    fs::write(case_dir.join("constant/transportProperties"), content)?;
    Ok(())
}

/// 生成湍流模型文件
fn generate_turbulence_properties(
    case_dir: &Path,
    model: &TurbulenceModel,
) -> Result<(), AppError> {
    let content = match model {
        TurbulenceModel::Laminar => r#"/*--------------------------------*- C++ -*----------------------------------*\
  =========                 |
  \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox
   \\    /   O peration     | Website:  https://openfoam.org
    \\  /    A nd           | Copyright (C) 2011-2018 OpenFOAM Foundation
     \\/     M anipulation  |
\*---------------------------------------------------------------------------*/

simulationType laminar;
"#,
        TurbulenceModel::KEpsilon => r#"/*--------------------------------*- C++ -*----------------------------------*\
  =========                 |
  \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox
   \\    /   O peration     | Website:  https://openfoam.org
    \\  /    A nd           | Copyright (C) 2011-2018 OpenFOAM Foundation
     \\/     M anipulation  |
\*---------------------------------------------------------------------------*/

simulationType RAS;

RAS
{
    RASModel        kEpsilon;

    turbulence      on;

    printCoeffs       on;
}
"#,
        TurbulenceModel::KOmegaSST => r#"/*--------------------------------*- C++ -*----------------------------------*\
  =========                 |
  \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox
   \\    /   O peration     | Website:  https://openfoam.org
    \\  /    A nd           | Copyright (C) 2011-2018 OpenFOAM Foundation
     \\/     M anipulation  |
\*---------------------------------------------------------------------------*/

simulationType RAS;

RAS
{
    RASModel        kOmegaSST;

    turbulence      on;

    printCoeffs       on;
}
"#,
    };

    fs::write(case_dir.join("constant/turbulenceProperties"), content)?;
    Ok(())
}

/// 生成网格字典（简化版）
fn generate_block_mesh_dict(
    case_dir: &Path,
    _regions: &[DomainRegion],
) -> Result<(), AppError> {
    // 对于导入的STL，这里生成snappyHexMesh配置
    let content = r#"/*--------------------------------*- C++ -*----------------------------------*\
  =========                 |
  \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox
   \\    /   O peration     | Website:  https://openfoam.org
    \\  /    A nd           | Copyright (C) 2011-2018 OpenFOAM Foundation
     \\/     M anipulation  |
\*---------------------------------------------------------------------------*/

/*--------------------------------*- C++ -*----------------------------------*\
  CAELab generated blockMeshDict
  Based on imported STL geometry
\*---------------------------------------------------------------------------*/

convertToMeters 1;

// User should import STL and the surface will be processed by snappyHexMesh
"#;

    fs::write(case_dir.join("system/blockMeshDict"), content)?;

    // Generate snappyHexMeshDict
    let shm_content = r#"/*--------------------------------*- C++ -*----------------------------------*\
  =========                 |
  \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox
   \\    /   O peration     | Website:  https://openfoam.org
    \\  /    A nd           | Copyright (C) 2011-2018 OpenFOAM Foundation
     \\/     M anipulation  |
\*---------------------------------------------------------------------------*/

castellatedMesh true;
snap            true;
addLayers       false;

geometry
{
    stlFile    constant/triSurface/geometry.stl;
}

castellatedMeshControls
{
    maxLocalCells  1000000;
    maxGlobalCells 2000000;
    minRefinementCells 10;
    nCellsBetweenLevels 3;
    resolveFeatureAngle 30;
    planarCOupling 0.001;
}

snapControls
{
    nSmoothPatch   3;
    tolerance      4.0;
    nSolveIter     300;
    nRelaxIter     5;
}
"#;

    fs::write(case_dir.join("system/snappyHexMeshDict"), shm_content)?;
    fs::create_dir_all(case_dir.join("constant/triSurface"))?;

    Ok(())
}

/// 生成0目录下的边界条件文件
fn generate_0_files(
    case_dir: &Path,
    boundaries: &[BoundaryCondition],
) -> Result<(), AppError> {
    // 生成U文件（速度）
    let mut u_content = String::from(
r#"/*--------------------------------*- C++ -*----------------------------------*\
  =========                 |
  \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox
   \\    /   O peration     | Website:  https://openfoam.org
    \\  /    A nd           | Copyright (C) 2011-2018 OpenFOAM Foundation
     \\/     M anipulation  |
\*---------------------------------------------------------------------------*/

dimensions      [0 1 -1 0 0 0 0];

internalField   uniform (0 0 0);

boundaryField
{
"#);

    // 添加每个边界条件
    for bc in boundaries {
        match bc.boundary_type {
            BoundaryType::VelocityInlet => {
                let vel = bc.velocity.unwrap_or(10.0);
                u_content.push_str(&format!(r#"
    {}
    {{
        type            fixedValue;
        value           uniform ({} 0 0);
    }}
"#, get_boundary_name(bc.id), vel));
            }
            BoundaryType::PressureOutlet => {
                u_content.push_str(&format!(r#"
    {}
    {{
        type            zeroGradient;
    }}
"#, get_boundary_name(bc.id)));
            }
            BoundaryType::Wall => {
                let slip = bc.wall_slip.unwrap_or(false);
                let wall_type = if slip { "slip" } else { "fixedValue" };
                u_content.push_str(&format!(r#"
    {}
    {{
        type            {};
        value           uniform (0 0 0);
    }}
"#, get_boundary_name(bc.id), wall_type));
            }
            BoundaryType::Symmetry => {
                u_content.push_str(&format!(r#"
    {}
    {{
        type            symmetryPlane;
    }}
"#, get_boundary_name(bc.id)));
            }
            BoundaryType::Outlet => {
                u_content.push_str(&format!(r#"
    {}
    {{
        type            inletOutlet;
        inletValue      uniform (0 0 0);
    }}
"#, get_boundary_name(bc.id)));
            }
        }
    }

    u_content.push_str("}\n");
    fs::write(case_dir.join("0/U"), u_content)?;

    // 生成p文件（压力）
    let mut p_content = String::from(
r#"/*--------------------------------*- C++ -*----------------------------------*\
  =========                 |
  \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox
   \\    /   O peration     | Website:  https://openfoam.org
    \\  /    A nd           | Copyright (C) 2011-2018 OpenFOAM Foundation
     \\/     M anipulation  |
\*---------------------------------------------------------------------------*/

dimensions      [0 2 -2 0 0 0 0];

internalField   uniform 0;

boundaryField
{
"#);

    for bc in boundaries {
        match bc.boundary_type {
            BoundaryType::VelocityInlet => {
                p_content.push_str(&format!(r#"
    {}
    {{
        type            zeroGradient;
    }}
"#, get_boundary_name(bc.id)));
            }
            BoundaryType::PressureOutlet => {
                let p = bc.gauge_pressure.unwrap_or(0.0);
                p_content.push_str(&format!(r#"
    {}
    {{
        type            fixedValue;
        value           uniform {};
    }}
"#, get_boundary_name(bc.id), p));
            }
            BoundaryType::Wall => {
                p_content.push_str(&format!(r#"
    {}
    {{
        type            zeroGradient;
    }}
"#, get_boundary_name(bc.id)));
            }
            BoundaryType::Symmetry | BoundaryType::Outlet => {
                p_content.push_str(&format!(r#"
    {}
    {{
        type            zeroGradient;
    }}
"#, get_boundary_name(bc.id)));
            }
        }
    }

    p_content.push_str("}\n");
    fs::write(case_dir.join("0/p"), p_content)?;

    // 生成k文件（湍动能）- 仅湍流模型需要
    let k_content = format!(r#"/*--------------------------------*- C++ -*----------------------------------*\
  =========                 |
  \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox
   \\    /   O peration     | Website:  https://openfoam.org
    \\  /    A nd           | Copyright (C) 2011-2018 OpenFOAM Foundation
     \\/     M anipulation  |
\*---------------------------------------------------------------------------*/

dimensions      [0 2 -2 0 0 0 0];

internalField   uniform 0.01;

boundaryField
{{
"#);

    let epsilon_content = format!(r#"/*--------------------------------*- C++ -*----------------------------------*\
  =========                 |
  \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox
   \\    /   O peration     | Website:  https://openfoam.org
    \\  /    A nd           | Copyright (C) 2011-2018 OpenFOAM Foundation
     \\/     M anipulation  |
\*---------------------------------------------------------------------------*/

dimensions      [0 2 -3 0 0 0 0];

internalField   uniform 0.01;

boundaryField
{{
"#);

    // 对于有速度入口的边界，添加湍流参数
    for bc in boundaries {
        if matches!(bc.boundary_type, BoundaryType::VelocityInlet) {
            let intensity = bc.turbulence_intensity.unwrap_or(5.0) / 100.0;
            let hydraulic_d = bc.hydraulic_diameter.unwrap_or(0.1);
            let k = 1.5 * intensity * intensity * 10.0 * 10.0; // 假设来流速度10 m/s
            let epsilon = 0.09 * k.powf(1.5) / hydraulic_d;
            
            k_content.push_str(&format!(r#"
    {}
    {{
        type            turbulentIntensityKineticEnergyInlet;
        intensity       {};
        value           uniform {};
    }}
"#, get_boundary_name(bc.id), intensity, k));

            epsilon_content.push_str(&format!(r#"
    {}
    {{
        type            turbulentDissipationRateInlet;
        dissipationRate {};
        value           uniform {};
    }}
"#, get_boundary_name(bc.id), epsilon, epsilon));
        }
    }

    k_content.push_str("}\n");
    epsilon_content.push_str("}\n");
    fs::write(case_dir.join("0/k"), k_content)?;
    fs::write(case_dir.join("0/epsilon"), epsilon_content)?;

    Ok(())
}

/// 生成fvSchemes和fvSolution文件
fn generate_fv_files(
    case_dir: &Path,
    _model: &TurbulenceModel,
) -> Result<(), AppError> {
    let schemes_content = r#"/*--------------------------------*- C++ -*----------------------------------*\
  =========                 |
  \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox
   \\    /   O peration     | Website:  https://openfoam.org
    \\  /    A nd           | Copyright (C) 2011-2018 OpenFOAM Foundation
     \\/     M anipulation  |
\*---------------------------------------------------------------------------*/

ddtSchemes      default         steadyState;

gradSchemes     default         Gauss linear;

divSchemes      default         Gauss linearUpwind grad(U);

laplacianSchemes default         Gauss linear corrected;

interpolationSchemes default     linear;

sngradSchemes    default         corrected;
"#;

    fs::write(case_dir.join("system/fvSchemes"), schemes_content)?;

    let solution_content = r#"/*--------------------------------*- C++ -*----------------------------------*\
  =========                 |
  \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox
   \\    /   O peration     | Website:  https://openfoam.org
    \\  /    A nd           | Copyright (C) 2011-2018 OpenFOAM Foundation
     \\/     M anipulation  |
\*---------------------------------------------------------------------------*/

solvers
{
    p
    {
        solver          PCG;
        preconditioner  DIC;
        tolerance       1e-6;
        relTol          0.05;
    }

    pFinal
    {
        $p;
        relTol          0;
    }

    U
    {
        solver          PBiCG;
        preconditioner  DILU;
        tolerance       1e-6;
        relTol          0.05;
    }

    UFinal
    {
        $U;
        relTol          0;
    }

    k
    {
        solver          PBiCG;
        preconditioner  DILU;
        tolerance       1e-6;
        relTol          0.05;
    }

    kFinal
    {
        $k;
        relTol          0;
    }

    epsilon
    {
        solver          PBiCG;
        preconditioner  DILU;
        tolerance       1e-6;
        relTol          0.05;
    }

    epsilonFinal
    {
        $epsilon;
        relTol          0;
    }
}

SIMPLE
{
    nNonOrthogonalCorrectors 0;
    residualControl
    {
        U           1e-4;
        p           1e-4;
        k           1e-4;
        epsilon     1e-4;
    }
}

relaxationFactors
{
    fields
    {
        p               0.3;
    }
    equations
    {
        U               0.7;
        k               0.7;
        epsilon         0.7;
    }
}
"#;

    fs::write(case_dir.join("system/fvSolution"), solution_content)?;

    // 生成Allclean脚本
    let allclean_content = r#"#!/bin/sh
cd "${0%/*}" || exit 1
. ./Allrun

cleanTimeDirectories()
{
    rm -rf [1-9]* > /dev/null 2>&1
    rm -rf 0.[0-9]* > /dev/null 2>&1
    rm -rf 0 > /dev/null 2>&1
    rm -f log.* > /dev/null 2>&1
    rm -rf processor* > /dev/null 2>&1
    rm -rf *.pvd *.vtp *.vtu > /dev/null 2>&1
    rm -f *.dat *.txt *.log *.out > /dev/null 2>&1
}
"#;

    fs::write(case_dir.join("Allclean"), allclean_content)?;

    Ok(())
}

/// 获取项目目录
fn get_project_dir(project_id: u64) -> Result<PathBuf, AppError> {
    let app_data = dirs_next().ok_or_else(|| AppError::Internal("Cannot determine app data directory".to_string()))?;
    let project_dir = app_data.join(format!("projects/{}", project_id));
    fs::create_dir_all(&project_dir)?;
    Ok(project_dir)
}

/// 获取边界条件名称
fn get_boundary_name(id: u64) -> String {
    format!("boundary_{}", id)
}

/// 保存CFD设置到数据库
fn save_cfd_setup_to_db(_db: &DbConnection, _setup: &CfdSetup) -> Result<(), AppError> {
    // TODO: 保存到数据库的cfd_setups表
    Ok(())
}

/// 解析OpenFOAM日志文件获取结果统计
#[tauri::command]
pub fn parse_openfoam_log(log_path: String) -> Result<CfdResultStats, AppError> {
    let content = fs::read_to_string(&log_path)
        .map_err(|e| AppError::Internal(format!("Failed to read log file: {}", e)))?;

    let mut iterations = 0u32;
    let mut converged = false;
    let mut cl = None;
    let mut cd = None;
    let mut cm = None;

    for line in content.lines() {
        // 解析迭代次数
        if line.contains("Time =") {
            // 提取时间步数
            if let Some(time) = line.split("Time = ").nth(1) {
                if let Ok(t) = time.trim().parse::<f64>() {
                    iterations = t as u32;
                }
            }
        }

        // 解析残差
        if line.contains("SmoothSolver:  Solving for") && line.contains("Ux") {
            if line.contains("Converged") {
                converged = true;
            }
        }

        // 解析升力和阻力系数
        if line.contains("Cd") || line.contains("Cl") || line.contains("Cm") {
            // 这里需要根据实际输出格式解析
            // 通常OpenFOAM会在forces函数中输出这些值
        }
    }

    Ok(CfdResultStats {
        iterations,
        converged,
        cl,
        cd,
        cm,
    })
}

/// 下载生成的OpenFOAM案例（打包为zip）
#[tauri::command]
pub fn download_openfoam_case(project_id: u64) -> Result<String, AppError> {
    let project_dir = get_project_dir(project_id)?;
    let case_dir = project_dir.join("openfoam_case");

    if !case_dir.exists() {
        return Err(AppError::Internal("OpenFOAM case not generated yet".to_string()));
    }

    let zip_path = project_dir.join("openfoam_case.zip");

    // 使用zip crate压缩
    // 注意：需要添加zip依赖到Cargo.toml
    use std::fs::File;
    use std::io::Write;

    let file = File::create(&zip_path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    fn add_dir_to_zip(
        zip: &mut zip::ZipWriter<File>,
        dir: &Path,
        base: &Path,
        options: zip::write::SimpleFileOptions,
    ) -> Result<(), AppError> {
        if !dir.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let relative = path.strip_prefix(base).unwrap_or(&path);

            if path.is_dir() {
                let name = relative.to_string_lossy().replace('\\", "/");
                zip.add_directory(&name, options)?;
                add_dir_to_zip(zip, &path, base, options)?;
            } else {
                let name = relative.to_string_lossy().replace('\\', "/");
                zip.start_file(&name, options)?;
                let content = fs::read(&path)?;
                zip.write_all(&content)?;
            }
        }
        Ok(())
    }

    add_dir_to_zip(&mut zip, &case_dir, &case_dir, options)?;

    zip.finish()?;

    Ok(zip_path.to_string_lossy().to_string())
}

/// 导入CFD几何文件（STL）
#[tauri::command]
pub fn import_cfd_geometry(project_id: u64, file_path: String) -> Result<String, AppError> {
    let project_dir = get_project_dir(project_id)?;
    let tri_surface_dir = project_dir.join("openfoam_case/constant/triSurface");
    fs::create_dir_all(&tri_surface_dir)?;

    let file_name = Path::new(&file_path)
        .file_name()
        .ok_or_else(|| AppError::InvalidInput("Invalid file path".to_string()))?;

    let dest_path = tri_surface_dir.join(file_name);
    fs::copy(&file_path, &dest_path)?;

    // 更新snappyHexMeshDict以引用正确的文件名
    let shm_dict_path = project_dir.join("openfoam_case/system/snappyHexMeshDict");
    if shm_dict_path.exists() {
        let content = fs::read_to_string(&shm_dict_path)?;
        let new_content = content.replace("geometry.stl", &file_name.to_string_lossy());
        fs::write(&shm_dict_path, new_content)?;
    }

    Ok(dest_path.to_string_lossy().to_string())
}

/// 生成CFD报告
#[tauri::command]
pub fn generate_cfd_report(project_id: u64) -> Result<String, AppError> {
    let project_dir = get_project_dir(project_id)?;
    let report_path = project_dir.join("cfd_report.md");

    let content = format!(r#"# CFD仿真报告

## 项目信息
- 项目ID: {}
- 生成时间: {:?}

## 仿真配置

### 材料参数
- 密度: kg/m³
- 动力粘度: Pa·s

### 边界条件
- 速度入口: m/s
- 压力出口: Pa
- 壁面条件

### 湍流模型
- 模型类型

### 求解控制
- 收敛标准
- 最大迭代次数

## 结果摘要
- 迭代次数: 
- 收敛状态: 
- 升力系数 Cl: 
- 阻力系数 Cd: 
- 力矩系数 Cm: 

## 说明
本报告由CAELab自动生成。
OpenFOAM案例已导出至: openfoam_case/
"#, project_id, chrono::Local::now());

    fs::write(&report_path, content)?;
    Ok(report_path.to_string_lossy().to_string())
}

// ========== 工具函数 ==========

// 简单的目录复制函数
fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), AppError> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(&entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// 添加必要的模块导入
use zip::ZipWriter;

/// 引入所需的crate
use std::io::Write;
use std::io::Read;
"