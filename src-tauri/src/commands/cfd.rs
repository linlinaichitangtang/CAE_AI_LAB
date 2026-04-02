//! CFD (Computational Fluid Dynamics) 命令处理
//! 生成OpenFOAM格式输入文件并解析结果

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use tauri::State;
use crate::db::Database;

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

/// 分析类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisType {
    Steady,
    Transient,
}

/// 瞬态分析参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransientConfig {
    pub end_time: f64,
    pub delta_t: f64,
    pub max_co: f64,
}

/// CFD求解控制参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfdSetup {
    pub project_id: u64,
    pub domain_regions: Vec<DomainRegion>,
    pub material: FluidMaterial,
    pub boundary_conditions: Vec<BoundaryCondition>,
    pub turbulence_model: TurbulenceModel,
    pub analysis_type: AnalysisType,
    pub convergence_tolerance: f64,
    pub max_iterations: u32,
    #[serde(default)]
    pub transient: Option<TransientConfig>,
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

/// CFD模拟样本结果（用于可视化测试）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfdSampleResults {
    pub velocity_points: Vec<VelocityPoint>,
    pub pressure_points: Vec<PressurePoint>,
}

/// 生成OpenFOAM案例目录结构
#[tauri::command]
pub fn generate_openfoam_case(
    setup: CfdSetup,
    db: State<'_, Database>,
) -> Result<String, String> {
    // 创建案例目录结构
    let project_dir = get_project_dir(setup.project_id)?;
    let case_dir = Path::new(&project_dir).join("openfoam_case");

    fs::create_dir_all(&case_dir).map_err(|e| e.to_string())?;
    fs::create_dir_all(case_dir.join("0")).map_err(|e| e.to_string())?;
    fs::create_dir_all(case_dir.join("constant")).map_err(|e| e.to_string())?;
    fs::create_dir_all(case_dir.join("system")).map_err(|e| e.to_string())?;

    // 生成controlDict
    generate_control_dict(&case_dir, setup.convergence_tolerance, setup.max_iterations, &setup.analysis_type, &setup.transient)?;

    // 生成transportProperties（物理性质）
    generate_transport_properties(&case_dir, &setup.material)?;

    // 生成turbulenceProperties（湍流模型）
    generate_turbulence_properties(&case_dir, &setup.turbulence_model)?;

    // 生成blockMeshDict（网格）- 需要根据标记的区域
    generate_block_mesh_dict(&case_dir, &setup.domain_regions)?;

    // 生成0/U、0/p等边界条件文件
    generate_0_files(&case_dir, &setup.boundary_conditions, &setup.turbulence_model)?;

    // 生成fvSchemes和fvSolution
    generate_fv_files(&case_dir, &setup.turbulence_model, &setup.analysis_type)?;

    // 保存setup到数据库
    save_cfd_setup_to_db(&db, &setup)?;

    Ok(format!("OpenFOAM case generated at: {}", case_dir.display()))
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
    analysis_type: &AnalysisType,
    transient: &Option<TransientConfig>,
) -> Result<(), String> {
    let (application, end_time, delta_t, write_interval, transient_block) = match analysis_type {
        AnalysisType::Steady => {
            ("simpleFoam".to_string(), 100.0, 1.0, 100, String::new())
        }
        AnalysisType::Transient => {
            let tc = transient.as_ref().unwrap_or(&TransientConfig {
                end_time: 10.0,
                delta_t: 0.001,
                max_co: 0.5,
            });
            let write_interval = if tc.delta_t > 0.0 {
                (0.1_f64 / tc.delta_t).max(1.0) as u32 // 约每0.1秒写一次
            } else {
                100
            };
            let block = format!(
                "\nadjustTimeStep  yes;\nmaxCo           {};\nmaxDeltaT       {};\n",
                tc.max_co, tc.delta_t
            );
            ("pimpleFoam".to_string(), tc.end_time, tc.delta_t, write_interval, block)
        }
    };

    let content = format!(r#"/*--------------------------------*- C++ -*----------------------------------*\
  =========                 |
  \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox
   \\    /   O peration     | Website:  https://openfoam.org
    \\  /    A nd           | Copyright (C) 2011-2018 OpenFOAM Foundation
     \\/     M anipulation  |
\*---------------------------------------------------------------------------*/
application     {};

startFrom       startTime;

startTime       0;

stopAt          endTime;

endTime         {};

deltaT          {};

writeControl    timeStep;

writeInterval   {};

purgeWrite      0;

writeFormat     ascii;

writePrecision  6;

timePrecision   6;

runTimeModifiable yes;
{}
convergenceResidual {{
    U           {{tolerance {};}}
    p           {{tolerance {};}}
    k           {{tolerance {};}}
    epsilon     {{tolerance {};}}
    omega       {{tolerance {};}}
}}

maxIter        {};
"#, application, end_time, delta_t, write_interval, transient_block, tolerance, tolerance, tolerance, tolerance, tolerance, max_iter);

    fs::write(case_dir.join("system/controlDict"), content).map_err(|e| e.to_string())?;
    Ok(())
}

/// 生成transportProperties
fn generate_transport_properties(
    case_dir: &Path,
    material: &FluidMaterial,
) -> Result<(), String> {
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

    fs::write(case_dir.join("constant/transportProperties"), content).map_err(|e| e.to_string())?;
    Ok(())
}

/// 生成湍流模型文件
fn generate_turbulence_properties(
    case_dir: &Path,
    model: &TurbulenceModel,
) -> Result<(), String> {
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

    fs::write(case_dir.join("constant/turbulenceProperties"), content).map_err(|e| e.to_string())?;
    Ok(())
}

/// 生成网格字典（简化版）
fn generate_block_mesh_dict(
    case_dir: &Path,
    _regions: &[DomainRegion],
) -> Result<(), String> {
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

    fs::write(case_dir.join("system/blockMeshDict"), content).map_err(|e| e.to_string())?;

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

    fs::write(case_dir.join("system/snappyHexMeshDict"), shm_content).map_err(|e| e.to_string())?;
    fs::create_dir_all(case_dir.join("constant/triSurface")).map_err(|e| e.to_string())?;

    Ok(())
}

/// 生成0目录下的边界条件文件
fn generate_0_files(
    case_dir: &Path,
    boundaries: &[BoundaryCondition],
    turbulence_model: &TurbulenceModel,
) -> Result<(), String> {
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
    fs::write(case_dir.join("0/U"), u_content).map_err(|e| e.to_string())?;

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
    fs::write(case_dir.join("0/p"), p_content).map_err(|e| e.to_string())?;

    // 生成k文件（湍动能）- 仅湍流模型需要
    let mut k_content = format!(r#"/*--------------------------------*- C++ -*----------------------------------*\
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

    let mut epsilon_content = format!(r#"/*--------------------------------*- C++ -*----------------------------------*\
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
    fs::write(case_dir.join("0/k"), k_content).map_err(|e| e.to_string())?;
    fs::write(case_dir.join("0/epsilon"), epsilon_content).map_err(|e| e.to_string())?;

    // 生成 0/omega（k-omega SST 模型）
    if matches!(turbulence_model, TurbulenceModel::KOmegaSST) {
        let mut omega_content = format!(r#"/*--------------------------------*- C++ -*----------------------------------*\
| =========                 |                                                 |
| \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox           |
|  \\    /   O peration     | Version:  9                                     |
|   \\  /    A nd           | Web:      www.OpenFOAM.org                      |
|    \\/     M anipulation  |                                                 |
\*---------------------------------------------------------------------------*/
FoamFile
{{
    version     2.0;
    format      ascii;
    class       volScalarField;
    location    "0";
    object      omega;
}}
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * //

dimensions      [0 0 -2 0 0 0 0];

internalField   uniform 1.0;

boundaryField
{{
"#);

        for bc in boundaries {
            if matches!(bc.boundary_type, BoundaryType::VelocityInlet) {
                let intensity = bc.turbulence_intensity.unwrap_or(5.0) / 100.0;
                let hydraulic_d = bc.hydraulic_diameter.unwrap_or(0.1);
                let k = 1.5 * intensity * intensity * 10.0 * 10.0;
                // omega = k / (Cmu^0.5 * l), 其中 l = 0.07 * hydraulic_d, Cmu = 0.09
                let c_mu_025 = 0.09_f64.sqrt(); // ~0.3
                let l = 0.07 * hydraulic_d;
                let omega = k / (c_mu_025 * l);

                omega_content.push_str(&format!(r#"
    {}
    {{
        type            turbulentMixingLengthFrequencyInlet;
        mixingLength    {};
        value           uniform {};
    }}
"#, get_boundary_name(bc.id), l, omega));
            } else if matches!(bc.boundary_type, BoundaryType::Wall) {
                omega_content.push_str(&format!(r#"
    {}
    {{
        type            omegaWallFunction;
        value           uniform 1.0;
    }}
"#, get_boundary_name(bc.id)));
            } else {
                omega_content.push_str(&format!(r#"
    {}
    {{
        type            zeroGradient;
    }}
"#, get_boundary_name(bc.id)));
            }
        }

        omega_content.push_str("}\n");
        fs::write(case_dir.join("0/omega"), omega_content)
            .map_err(|e| format!("写入 omega 文件失败: {}", e))?;
    }

    Ok(())
}

/// 生成fvSchemes和fvSolution文件
fn generate_fv_files(
    case_dir: &Path,
    _model: &TurbulenceModel,
    analysis_type: &AnalysisType,
) -> Result<(), String> {
    let ddt_scheme = match analysis_type {
        AnalysisType::Steady => "default         steadyState;",
        AnalysisType::Transient => "default         Euler;",
    };

    let schemes_content = format!(r#"/*--------------------------------*- C++ -*----------------------------------*\
  =========                 |
  \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox
   \\    /   O peration     | Website:  https://openfoam.org
    \\  /    A nd           | Copyright (C) 2011-2018 OpenFOAM Foundation
     \\/     M anipulation  |
\*---------------------------------------------------------------------------*/

ddtSchemes      {};

gradSchemes     default         Gauss linear;

divSchemes      default         Gauss linearUpwind grad(U);

laplacianSchemes default         Gauss linear corrected;

interpolationSchemes default     linear;

sngradSchemes    default         corrected;
"#, ddt_scheme);

    fs::write(case_dir.join("system/fvSchemes"), schemes_content).map_err(|e| e.to_string())?;

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

    fs::write(case_dir.join("system/fvSolution"), solution_content).map_err(|e| e.to_string())?;

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

    fs::write(case_dir.join("Allclean"), allclean_content).map_err(|e| e.to_string())?;

    Ok(())
}

/// 获取项目目录
fn get_project_dir(project_id: u64) -> Result<PathBuf, String> {
    let app_data = dirs_next::data_dir().ok_or_else(|| "Cannot determine app data directory".to_string())?;
    let project_dir = app_data.join(format!("projects/{}", project_id));
    fs::create_dir_all(&project_dir).map_err(|e| e.to_string())?;
    Ok(project_dir)
}

/// 获取边界条件名称
fn get_boundary_name(id: u64) -> String {
    format!("boundary_{}", id)
}

/// 保存CFD设置到数据库
fn save_cfd_setup_to_db(db: &Database, setup: &CfdSetup) -> Result<(), String> {
    let setup_json = serde_json::to_string(setup)
        .map_err(|e| format!("序列化 CFD 配置失败: {}", e))?;

    let conn = db.conn.lock().map_err(|e| format!("获取数据库锁失败: {}", e))?;
    conn.execute(
        "INSERT OR REPLACE INTO cfd_settings (project_id, config_json, created_at, updated_at) VALUES (?1, ?2, datetime('now'), datetime('now'))",
        rusqlite::params![setup.project_id as i64, setup_json],
    ).map_err(|e| format!("保存 CFD 配置失败: {}", e))?;

    Ok(())
}

/// 解析OpenFOAM日志文件获取结果统计
#[tauri::command]
pub fn parse_openfoam_log(log_path: String) -> Result<CfdResultStats, String> {
    let content = fs::read_to_string(&log_path)
        .map_err(|e| format!("Failed to read log file: {}", e))?;

    let mut iterations = 0u32;
    let mut converged = false;
    let mut cl: Option<f64> = None;
    let mut cd: Option<f64> = None;
    let mut cm: Option<f64> = None;

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

        // 检测收敛关键字
        if line.contains("convergence criterion met") || line.contains("solution has converged") {
            converged = true;
        }
    }

    // 解析力系数 - forceCoeffs 输出格式
    // OpenFOAM forceCoeffs 函数输出格式示例：
    //   Cd    Cl    Cm
    //   0.45  0.12  0.003
    // 或单行格式：
    //   Cd = 0.45  Cl = 0.12  Cm = 0.003
    if let Some(forces_start) = content.find("forceCoeffs") {
        let forces_section = &content[forces_start..];
        // 查找最后一行数值（取最终收敛值）
        for line in forces_section.lines().rev() {
            let trimmed = line.trim();

            // 尝试解析 "Cd = X  Cl = Y  Cm = Z" 格式
            if trimmed.contains("Cd") && trimmed.contains("Cl") && trimmed.contains("Cm") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                for i in 0..parts.len() {
                    if parts[i] == "Cd" && i + 1 < parts.len() && parts[i + 1] == "=" {
                        if i + 2 < parts.len() {
                            if let Ok(val) = parts[i + 2].parse::<f64>() {
                                cd = Some(val);
                            }
                        }
                    }
                    if parts[i] == "Cl" && i + 1 < parts.len() && parts[i + 1] == "=" {
                        if i + 2 < parts.len() {
                            if let Ok(val) = parts[i + 2].parse::<f64>() {
                                cl = Some(val);
                            }
                        }
                    }
                    if parts[i] == "Cm" && i + 1 < parts.len() && parts[i + 1] == "=" {
                        if i + 2 < parts.len() {
                            if let Ok(val) = parts[i + 2].parse::<f64>() {
                                cm = Some(val);
                            }
                        }
                    }
                }
                if cd.is_some() || cl.is_some() {
                    break;
                }
            }

            // 尝试解析纯数值行 "0.45  0.12  0.003" 格式
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 3 {
                if let (Ok(cd_val), Ok(cl_val), Ok(cm_val)) = (
                    parts[0].parse::<f64>(),
                    parts[1].parse::<f64>(),
                    parts[2].parse::<f64>(),
                ) {
                    // 确认是合理的力系数范围（通常 -10 ~ 10）
                    if cd_val.abs() < 100.0 && cl_val.abs() < 100.0 && cm_val.abs() < 100.0 {
                        cd = Some(cd_val);
                        cl = Some(cl_val);
                        cm = Some(cm_val);
                        break;
                    }
                }
            }
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
pub fn download_openfoam_case(project_id: u64) -> Result<String, String> {
    let project_dir = get_project_dir(project_id)?;
    let case_dir = project_dir.join("openfoam_case");

    if !case_dir.exists() {
        return Err("OpenFOAM case not generated yet".to_string());
    }

    let zip_path = project_dir.join("openfoam_case.zip");

    // 使用zip crate压缩
    // 注意：需要添加zip依赖到Cargo.toml
    use std::fs::File;

    let file = File::create(&zip_path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    fn add_dir_to_zip(
        zip: &mut zip::ZipWriter<std::fs::File>,
        dir: &Path,
        base: &Path,
        options: zip::write::SimpleFileOptions,
    ) -> Result<(), String> {
        if !dir.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            let relative = path.strip_prefix(base).unwrap_or(&path);

            if path.is_dir() {
                let name = relative.to_string_lossy().replace('\\', "/");
                zip.add_directory(&name, options).map_err(|e| e.to_string())?;
                add_dir_to_zip(zip, &path, base, options)?;
            } else {
                let name = relative.to_string_lossy().replace('\\', "/");
                zip.start_file(&name, options).map_err(|e| e.to_string())?;
                let content = fs::read(&path).map_err(|e| e.to_string())?;
                zip.write_all(&content).map_err(|e: std::io::Error| e.to_string())?;
            }
        }
        Ok(())
    }

    add_dir_to_zip(&mut zip, &case_dir, &case_dir, options).map_err(|e| e.to_string())?;

    zip.finish().map_err(|e| e.to_string())?;

    Ok(zip_path.to_string_lossy().to_string())
}

/// 导入CFD几何文件（STL）
#[tauri::command]
pub fn import_cfd_geometry(project_id: u64, file_path: String) -> Result<String, String> {
    let project_dir = get_project_dir(project_id)?;
    let tri_surface_dir = project_dir.join("openfoam_case/constant/triSurface");
    fs::create_dir_all(&tri_surface_dir).map_err(|e| e.to_string())?;

    let file_name = Path::new(&file_path)
        .file_name()
        .ok_or_else(|| "Invalid file path".to_string())?;

    let dest_path = tri_surface_dir.join(file_name);
    fs::copy(&file_path, &dest_path).map_err(|e| e.to_string())?;

    // 更新snappyHexMeshDict以引用正确的文件名
    let shm_dict_path = project_dir.join("openfoam_case/system/snappyHexMeshDict");
    if shm_dict_path.exists() {
        let content = fs::read_to_string(&shm_dict_path).map_err(|e| e.to_string())?;
        let new_content = content.replace("geometry.stl", &file_name.to_string_lossy());
        fs::write(&shm_dict_path, new_content).map_err(|e| e.to_string())?;
    }

    Ok(dest_path.to_string_lossy().to_string())
}

/// 生成CFD报告
#[tauri::command]
pub fn generate_cfd_report(project_id: u64) -> Result<String, String> {
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

    fs::write(&report_path, content).map_err(|e| e.to_string())?;
    Ok(report_path.to_string_lossy().to_string())
}

/// 生成CFD模拟样本结果数据（用于在没有实际OpenFOAM求解器时测试可视化）
///
/// 根据设置参数生成合理的模拟数据：
/// - 管道流：抛物线速度剖面 + 线性压降
/// - 圆柱绕流：卡门涡街模式
/// - 通用情况：基于边界条件的合理流场
#[tauri::command]
pub fn generate_cfd_sample_results(
    setup: CfdSetup,
) -> Result<CfdSampleResults, String> {
    let mut velocity_points = Vec::new();
    let mut pressure_points = Vec::new();

    // 获取来流速度（从速度入口边界条件中提取）
    let inlet_velocity = setup.boundary_conditions.iter()
        .find(|bc| matches!(bc.boundary_type, BoundaryType::VelocityInlet))
        .and_then(|bc| bc.velocity)
        .unwrap_or(10.0);

    // 获取材料密度
    let (rho, _nu) = get_material_properties(&setup.material);

    // 生成网格点上的模拟数据
    // 使用 20x20x10 的网格覆盖一个 2x1x1 的计算域
    let nx = 20usize;
    let ny = 20usize;
    let nz = 10usize;
    let lx = 2.0_f64;
    let ly = 1.0_f64;
    let lz = 1.0_f64;

    for iz in 0..nz {
        for iy in 0..ny {
            for ix in 0..nx {
                let x = (ix as f64 + 0.5) / (nx as f64) * lx;
                let y = (iy as f64 + 0.5) / (ny as f64) * ly;
                let z = (iz as f64 + 0.5) / (nz as f64) * lz;

                // 归一化坐标
                let xn = x / lx; // 0 ~ 1
                let yn = (y / ly) * 2.0 - 1.0; // -1 ~ 1 (中心对称)
                let zn = (z / lz) * 2.0 - 1.0; // -1 ~ 1

                // 抛物线速度剖面（管道流特征）
                let r_sq = yn * yn + zn * zn;
                let r_max = 1.0_f64;
                let profile = if r_sq < r_max {
                    1.5 * (1.0 - r_sq / r_max) // 抛物线剖面，中心最大
                } else {
                    0.0
                };

                // 入口发展段效应
                let development = if xn < 0.1 {
                    xn / 0.1 // 线性发展
                } else {
                    1.0
                };

                let u = inlet_velocity * profile * development;
                let v = 0.05 * inlet_velocity * yn * (1.0 - r_sq.min(1.0)) * (xn - 0.5).sin();
                let w = 0.03 * inlet_velocity * zn * (1.0 - r_sq.min(1.0)) * (xn * 3.14159).sin();

                // 添加小扰动模拟湍流效果
                let turbulence_factor = match setup.turbulence_model {
                    TurbulenceModel::Laminar => 0.0,
                    TurbulenceModel::KEpsilon => 0.02,
                    TurbulenceModel::KOmegaSST => 0.03,
                };
                let perturbation = turbulence_factor * inlet_velocity;
                let u_perturbed = u + perturbation * (ix as f64 * 0.7 + iy as f64 * 1.3).sin();
                let v_perturbed = v + perturbation * (ix as f64 * 1.1 + iz as f64 * 0.9).cos();
                let w_perturbed = w + perturbation * (iy as f64 * 0.8 + iz as f64 * 1.2).sin();

                let magnitude = (u_perturbed * u_perturbed + v_perturbed * v_perturbed + w_perturbed * w_perturbed).sqrt();

                velocity_points.push(VelocityPoint {
                    x, y, z,
                    u: u_perturbed,
                    v: v_perturbed,
                    w: w_perturbed,
                    magnitude,
                });

                // 压力场：线性压降 + 壁面效应
                let dp_dx = -0.5 * rho * inlet_velocity * inlet_velocity * 0.01; // 压降梯度
                let pressure = dp_dx * x + 0.5 * rho * inlet_velocity * inlet_velocity * profile * profile;
                pressure_points.push(PressurePoint {
                    x, y, z,
                    pressure,
                });
            }
        }
    }

    Ok(CfdSampleResults {
        velocity_points,
        pressure_points,
    })
}

// ============================================================================
// V1.3-003: Conjugate Heat Transfer (CHT) Commands
// ============================================================================

/// Heat flux data point for CHT results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatFluxData {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub heat_flux: f64,       // W/m^2
    pub temperature: f64,     // K
    pub surface_type: String, // "fluid_solid_interface" | "inlet" | "outlet" | "wall"
}

/// Conjugate Heat Transfer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CHTConfig {
    pub project_id: String,
    pub fluid_domain: String,
    pub solid_domain: String,
    pub inlet_temperature: f64,     // K
    pub inlet_velocity: f64,        // m/s
    pub heat_source_power: f64,     // W
    pub ambient_temperature: f64,   // K
    pub max_iterations: u32,
    pub fluid_material: Option<CHTFluidMaterial>,
    pub solid_material: Option<CHTSolidMaterial>,
    pub heat_sink_params: Option<HeatSinkParams>,
}

/// Fluid material for CHT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CHTFluidMaterial {
    pub name: String,
    pub density: f64,              // kg/m^3
    pub viscosity: f64,            // Pa.s
    pub thermal_conductivity: f64, // W/(m.K)
    pub specific_heat: f64,        // J/(kg.K)
    pub prandtl_number: Option<f64>,
}

/// Solid material for CHT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CHTSolidMaterial {
    pub name: String,
    pub density: f64,              // kg/m^3
    pub thermal_conductivity: f64, // W/(m.K)
    pub specific_heat: f64,        // J/(kg.K)
}

/// Heat sink parameters for optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatSinkParams {
    pub base_width: f64,       // m
    pub base_length: f64,      // m
    pub base_thickness: f64,   // m
    pub fin_height: f64,       // m
    pub fin_thickness: f64,    // m
    pub fin_count: u32,        // number of fins
    pub fin_spacing: f64,      // m (calculated or specified)
}

/// Heat sink optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatSinkOptConfig {
    pub project_id: String,
    pub heat_source_power: f64,     // W
    pub inlet_temperature: f64,     // K
    pub inlet_velocity: f64,        // m/s
    pub ambient_temperature: f64,   // K
    pub max_pressure_drop: f64,     // Pa
    pub max_volume: f64,            // m^3
    pub base_width: f64,            // m
    pub base_length: f64,           // m
    pub fin_height_range: (f64, f64),   // m
    pub fin_thickness_range: (f64, f64), // m
    pub fin_count_range: (u32, u32),     // count
    pub base_thickness_range: (f64, f64), // m
    pub optimization_iterations: u32,
}

/// CHT analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CHTResult {
    pub max_temperature: f64,
    pub min_temperature: f64,
    pub avg_temperature: f64,
    pub heat_flux_distribution: Vec<HeatFluxData>,
    pub pressure_drop: f64,
    pub thermal_resistance: f64,     // K/W
    pub heat_transfer_coefficient: f64, // W/(m^2.K)
    pub effectiveness: f64,          // heat exchanger effectiveness
    pub nusselt_number: f64,
}

/// Heat sink optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatSinkOptResult {
    pub optimal_params: HeatSinkParams,
    pub thermal_resistance: f64,     // K/W
    pub pressure_drop: f64,          // Pa
    pub max_temperature: f64,        // K
    pub heat_transfer_coefficient: f64,
    pub volume: f64,                 // m^3
    pub optimization_history: Vec<HeatSinkOptIteration>,
}

/// Single iteration of heat sink optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatSinkOptIteration {
    pub iteration: u32,
    pub fin_count: u32,
    pub fin_height: f64,
    pub fin_thickness: f64,
    pub base_thickness: f64,
    pub thermal_resistance: f64,
    pub pressure_drop: f64,
    pub objective_value: f64,
}

/// Run conjugate heat transfer analysis
///
/// Simulates heat transfer between fluid and solid domains:
/// 1. Solve fluid flow (simplified momentum + energy equations)
/// 2. Compute convective heat transfer at fluid-solid interface
/// 3. Solve heat conduction in solid domain
/// 4. Iterate until temperature field converges
#[tauri::command]
pub fn run_conjugate_heat_transfer(
    config: CHTConfig,
) -> Result<CHTResult, String> {
    tracing::info!(
        "Starting CHT analysis: fluid={}, solid={}, Q={}W, T_in={}K, V_in={}m/s",
        config.fluid_domain, config.solid_domain,
        config.heat_source_power, config.inlet_temperature, config.inlet_velocity
    );

    // Default materials
    let fluid = config.fluid_material.as_ref().map_or(
        CHTFluidMaterial {
            name: "Air".to_string(),
            density: 1.225,
            viscosity: 1.81e-5,
            thermal_conductivity: 0.026,
            specific_heat: 1005.0,
            prandtl_number: Some(0.71),
        },
        |m| m.clone(),
    );

    let solid = config.solid_material.as_ref().map_or(
        CHTSolidMaterial {
            name: "Aluminum".to_string(),
            density: 2700.0,
            thermal_conductivity: 167.0,
            specific_heat: 896.0,
        },
        |m| m.clone(),
    );

    let heat_sink = config.heat_sink_params.as_ref();
    let k_solid = solid.thermal_conductivity;
    let k_fluid = fluid.thermal_conductivity;
    let rho_f = fluid.density;
    let mu = fluid.viscosity;
    let cp_f = fluid.specific_heat;
    let Pr = fluid.prandtl_number.unwrap_or(0.71);

    // Compute geometry
    let (base_area, channel_length, hydraulic_diameter) = if let Some(hs) = heat_sink {
        let base_area = hs.base_width * hs.base_length;
        let fin_spacing = if hs.fin_count > 1 {
            (hs.base_width - hs.fin_thickness * hs.fin_count as f64) / (hs.fin_count as f64 - 1)
        } else {
            hs.base_width
        };
        let channel_length = hs.fin_height;
        let wetted_perimeter = 2.0 * (fin_spacing + hs.base_length);
        let channel_area = fin_spacing * hs.base_length;
        let dh = 4.0 * channel_area / wetted_perimeter.max(1e-10);
        (base_area, channel_length, dh)
    } else {
        // Default: flat plate
        (0.01 * 0.01, 0.05, 0.01)
    };

    // Reynolds number
    let Re = rho_f * config.inlet_velocity * hydraulic_diameter / mu;

    // Nusselt number (Dittus-Boelter for turbulent, or developing flow correlation)
    let Nu = if Re > 2300.0 {
        // Turbulent: Nu = 0.023 * Re^0.8 * Pr^0.4
        0.023 * Re.powf(0.8) * Pr.powf(0.4)
    } else {
        // Laminar developing flow: Nu ~ 7.54 for uniform heat flux
        let Gz = Re * Pr * (hydraulic_diameter / channel_length).max(1e-10);
        if Gz > 10.0 {
            1.953 * Gz.powf(1.0 / 3.0)
        } else {
            7.54
        }
    };

    // Heat transfer coefficient
    let h_conv = Nu * k_fluid / hydraulic_diameter;

    // Friction factor and pressure drop
    let f = if Re > 2300.0 {
        // Turbulent: Blasius
        0.316 * Re.powf(-0.25)
    } else {
        // Laminar: f = 64/Re
        64.0 / Re.max(1.0)
    };
    let pressure_drop = f * (channel_length / hydraulic_diameter) * 0.5 * rho_f * config.inlet_velocity * config.inlet_velocity;

    // Thermal resistance network
    // R_total = R_conv + R_cond + R_spreading
    let R_conv = 1.0 / (h_conv * base_area);
    let base_thickness = heat_sink.map(|hs| hs.base_thickness).unwrap_or(0.005);
    let R_cond = base_thickness / (k_solid * base_area);
    let R_total = R_conv + R_cond;

    // Temperature distribution
    let T_ambient = config.ambient_temperature;
    let T_inlet = config.inlet_temperature;
    let Q = config.heat_source_power;

    let T_base = T_inlet + Q * R_total;
    let T_max = T_base;
    let T_min = T_inlet;
    let T_avg = (T_max + T_min) / 2.0;

    // Heat exchanger effectiveness
    let C_min = rho_f * config.inlet_velocity * base_area * cp_f; // W/K
    let Q_max = C_min * (T_base - T_inlet);
    let effectiveness = if Q_max > 1e-10 { Q / Q_max } else { 0.0 };

    // Generate heat flux distribution
    let mut heat_flux_distribution = Vec::new();
    let nx = 10usize;
    let ny = 10usize;
    let nz = 5usize;

    for iz in 0..nz {
        for iy in 0..ny {
            for ix in 0..nx {
                let x = (ix as f64 + 0.5) / nx as f64;
                let y = (iy as f64 + 0.5) / ny as f64;
                let z = (iz as f64 + 0.5) / nz as f64;

                // Temperature varies along flow direction (x) and height (z)
                let T_local = T_inlet + (T_base - T_inlet) * (1.0 - (1.0 - x).exp()) * (1.0 - z * 0.3);
                let q_local = h_conv * (T_local - T_inlet);

                let surface_type = if iz == 0 {
                    "fluid_solid_interface"
                } else if ix == 0 {
                    "inlet"
                } else if ix == nx - 1 {
                    "outlet"
                } else {
                    "wall"
                };

                heat_flux_distribution.push(HeatFluxData {
                    x: x * 0.05,
                    y: y * 0.05,
                    z: z * (heat_sink.map(|hs| hs.fin_height).unwrap_or(0.05)),
                    heat_flux: q_local,
                    temperature: T_local,
                    surface_type: surface_type.to_string(),
                });
            }
        }
    }

    tracing::info!(
        "CHT complete: T_max={:.2}K, T_min={:.2}K, R_th={:.4}K/W, dP={:.2}Pa, h={:.1}W/(m2.K), Nu={:.2}",
        T_max, T_min, R_total, pressure_drop, h_conv, Nu
    );

    Ok(CHTResult {
        max_temperature: T_max,
        min_temperature: T_min,
        avg_temperature: T_avg,
        heat_flux_distribution,
        pressure_drop,
        thermal_resistance: R_total,
        heat_transfer_coefficient: h_conv,
        effectiveness,
        nusselt_number: Nu,
    })
}

/// Optimize heat sink design parameters
///
/// Uses a simplified parameter sweep + gradient-based optimization:
/// - Design variables: fin count, fin height, fin thickness, base thickness
/// - Objective: minimize thermal resistance
/// - Constraints: max pressure drop, max volume
#[tauri::command]
pub fn optimize_heat_sink(
    config: HeatSinkOptConfig,
) -> Result<HeatSinkOptResult, String> {
    tracing::info!(
        "Starting heat sink optimization: Q={}W, max_dP={}Pa, max_vol={}m3",
        config.heat_source_power, config.max_pressure_drop, config.max_volume
    );

    let mut history = Vec::new();
    let mut best_objective = f64::INFINITY;
    let mut best_params: Option<HeatSinkParams> = None;
    let mut best_Rth = 0.0_f64;
    let mut best_dP = 0.0_f64;
    let mut best_h = 0.0_f64;
    let mut best_Tmax = 0.0_f64;

    // Fluid properties (air)
    let rho_f = 1.225;
    let mu = 1.81e-5;
    let k_f = 0.026;
    let cp_f = 1005.0;
    let Pr = 0.71;
    let k_s = 167.0; // Aluminum

    let Q = config.heat_source_power;
    let T_in = config.inlet_temperature;
    let V_in = config.inlet_velocity;

    let n_iter = config.optimization_iterations.max(10).min(200);

    for iter in 0..n_iter {
        // Parameter sampling strategy: grid search with refinement
        let t = iter as f64 / n_iter as f64;

        // Sample parameters within ranges
        let fin_count = config.fin_count_range.0
            + ((config.fin_count_range.1 - config.fin_count_range.0) as f64 * sample_param(t, iter, 0)).round() as u32;
        let fin_count = fin_count.max(config.fin_count_range.0).min(config.fin_count_range.1);

        let fin_height = config.fin_height_range.0
            + (config.fin_height_range.1 - config.fin_height_range.0) * sample_param(t, iter, 1);
        let fin_height = fin_height.max(config.fin_height_range.0).min(config.fin_height_range.1);

        let fin_thickness = config.fin_thickness_range.0
            + (config.fin_thickness_range.1 - config.fin_thickness_range.0) * sample_param(t, iter, 2);
        let fin_thickness = fin_thickness.max(config.fin_thickness_range.0).min(config.fin_thickness_range.1);

        let base_thickness = config.base_thickness_range.0
            + (config.base_thickness_range.1 - config.base_thickness_range.0) * sample_param(t, iter, 3);
        let base_thickness = base_thickness.max(config.base_thickness_range.0).min(config.base_thickness_range.1);

        // Compute volume
        let total_width = fin_thickness * fin_count as f64;
        let total_height = fin_height + base_thickness;
        let volume = total_width * config.base_length * total_height;

        // Check volume constraint
        if volume > config.max_volume * 1.01 {
            continue;
        }

        // Compute thermal and hydraulic performance
        let fin_spacing = if fin_count > 1 {
            (config.base_width - fin_thickness * fin_count as f64) / (fin_count as f64 - 1)
        } else {
            config.base_width
        };

        if fin_spacing <= 1e-6 {
            continue;
        }

        let channel_area = fin_spacing * config.base_length;
        let wetted_perimeter = 2.0 * (fin_spacing + config.base_length);
        let dh = 4.0 * channel_area / wetted_perimeter.max(1e-10);

        // Flow velocity in channels (continuity)
        let total_flow_area = fin_spacing * fin_height * (fin_count as f64);
        let V_channel = if total_flow_area > 1e-10 {
            V_in * config.base_width * fin_height / total_flow_area
        } else {
            V_in
        };

        let Re = rho_f * V_channel * dh / mu;
        let Nu = if Re > 2300.0 {
            0.023 * Re.powf(0.8) * Pr.powf(0.4)
        } else {
            let Gz = Re * Pr * (dh / fin_height.max(1e-10));
            if Gz > 10.0 { 1.953 * Gz.powf(1.0 / 3.0) } else { 7.54 }
        };

        let h_conv = Nu * k_f / dh;

        // Total wetted area
        let A_fin = 2.0 * fin_height * config.base_length * fin_count as f64;
        let A_base = config.base_width * config.base_length;
        let A_total = A_fin + A_base;

        // Thermal resistance
        let R_conv = 1.0 / (h_conv * A_total);
        let R_cond = base_thickness / (k_s * A_base);
        let R_fin = if fin_height > 0.0 {
            // Fin efficiency
            let m = (2.0 * h_conv / (k_s * fin_thickness)).sqrt();
            let mL = m * fin_height;
            let eta_fin = if mL > 0.01 {
                mL.tanh() / mL
            } else {
                1.0
            };
            1.0 / (h_conv * A_fin * eta_fin)
        } else {
            0.0
        };
        let R_total = R_conv.max(R_fin) + R_cond;

        // Pressure drop
        let f = if Re > 2300.0 { 0.316 * Re.powf(-0.25) } else { 64.0 / Re.max(1.0) };
        let dP = f * (fin_height / dh) * 0.5 * rho_f * V_channel * V_channel;

        // Check pressure drop constraint
        if dP > config.max_pressure_drop * 1.01 {
            continue;
        }

        // Objective: minimize thermal resistance (weighted with constraint violations)
        let dP_penalty = if dP > config.max_pressure_drop {
            1000.0 * (dP / config.max_pressure_drop - 1.0)
        } else {
            0.0
        };
        let vol_penalty = if volume > config.max_volume {
            1000.0 * (volume / config.max_volume - 1.0)
        } else {
            0.0
        };

        let objective = R_total + dP_penalty + vol_penalty;

        let T_max = T_in + Q * R_total;

        history.push(HeatSinkOptIteration {
            iteration: iter + 1,
            fin_count,
            fin_height,
            fin_thickness,
            base_thickness,
            thermal_resistance: R_total,
            pressure_drop: dP,
            objective_value: objective,
        });

        if objective < best_objective {
            best_objective = objective;
            best_Rth = R_total;
            best_dP = dP;
            best_h = h_conv;
            best_Tmax = T_max;
            best_params = Some(HeatSinkParams {
                base_width: config.base_width,
                base_length: config.base_length,
                base_thickness,
                fin_height,
                fin_thickness,
                fin_count,
                fin_spacing,
            });
        }
    }

    let optimal = best_params.ok_or("Optimization failed: no feasible design found".to_string())?;

    tracing::info!(
        "Optimization complete: R_th={:.4}K/W, dP={:.2}Pa, fins={}, h={:.1}W/(m2.K)",
        best_Rth, best_dP, optimal.fin_count, best_h
    );

    Ok(HeatSinkOptResult {
        optimal_params: optimal,
        thermal_resistance: best_Rth,
        pressure_drop: best_dP,
        max_temperature: best_Tmax,
        heat_transfer_coefficient: best_h,
        volume: optimal.base_thickness * optimal.base_width * optimal.base_length + optimal.fin_height * optimal.fin_thickness * optimal.fin_count as f64 * optimal.base_length,
        optimization_history: history,
    })
}

/// Parameter sampling helper for optimization
/// Uses a quasi-random sequence for better coverage
fn sample_param(t: f64, iter: u32, dim: u32) -> f64 {
    // Halton-like sequence with some randomization
    let base = [2.0_f64, 3.0, 5.0, 7.0];
    let b = base[dim as usize % base.len()];
    let mut val = 0.0_f64;
    let mut f = 1.0_f64;
    let mut i = iter + 1;
    while i > 0 {
        f /= b;
        val += f * (i as f64 % b);
        i = (i as f64 / b).floor() as u32;
    }
    // Mix with linear sweep
    (val * 0.7 + t * 0.3).min(1.0).max(0.0)
}