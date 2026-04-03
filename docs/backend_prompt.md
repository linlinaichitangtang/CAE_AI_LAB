# CAELab Phase 3 后端实现任务书

**项目路径**：`/Users/gongzhaolin/.openclaw/workspace-chief/projects/cae`  
**前端 API 路径**：`src/api/`（对应后端命令的调用方）  
**后端源码路径**：`src-tauri/src/commands/`  
**版本**：V2.0.1（Phase 3 后端实现，承接前端已完成的所有 invoke 调用）

---

## 0. 背景

CAELab 是科研与工程创作一体化工作台，核心壁垒是**多尺度贯通**——第一性原理(DFT) → 分子动力学(MD) → 相场(Phase Field) → 有限元(FE)。

前端已完成 Phase 3 所有 UI 和 API 接口（V1.5~V2.0 前端模块全部 git），但前端 `invoke()` 调用的 Rust 命令尚未实现。

**你的任务**：实现所有 Phase 3 相关的 Rust Tauri 命令，补全前后端对接，让 Phase 3 真正可运行。

---

## 1. 架构约定

### 1.1 命令注册
- 所有新命令放在 `src-tauri/src/commands/` 下对应的 `mod.rs` 文件
- 命令用 `#[tauri::command]` 注解
- 在 `src-tauri/src/lib.rs` 中注册到 `app.manage()` 或 `generate_handler!` 宏

### 1.2 Rust 命名规范
- 命令函数：`snake_case`（如 `run_lammps_simulation`）
- 类型结构：`PascalCase`（与前端 TypeScript 类型对应）
- 模块文件名：`snake_case.rs`

### 1.3 前后端类型对齐
- 前端 TypeScript 类型在 `src/api/*.ts` 中定义
- 后端 Rust 结构体需要与前端类型完全对齐（字段名、嵌套结构一致）
- 使用 `serde` 的 `Serialize`/`Deserialize` 进行 JSON 序列化

### 1.4 错误处理
- 命令返回 `Result<T, String>`，错误信息返回可读字符串
- 不要 panic，妥善处理所有 edge case

---

## 2. Phase 3 后端任务清单

### 2.1 MD 模块（V1.5，对应 `src/api/molecularDynamics.ts`）

**必须实现的 Tauri 命令：**

```rust
// 检查 LAMMPS 是否可用
#[tauri::command]
async fn check_lammps_available() -> Result<LmpsAvailability, String>

// 运行 MD 模拟
#[tauri::command]
async fn run_lammps_simulation(config: MdConfig) -> Result<MdResult, String>

// 生成 LAMMPS 输入文件（不运行）
#[tauri::command]
async fn generate_lammps_input(config: MdConfig) -> Result<String, String>

// 解析 LAMMPS 输出
#[tauri::command]
async fn parse_lammps_output(output_path: String) -> Result<MdResult, String>

// 校验 MD 配置
#[tauri::command]
async fn validate_md_config(config: MdConfig) -> Result<MdValidationResult, String>

// 估算内存需求
#[tauri::command]
async fn estimate_memory(config: MdConfig) -> Result<MemoryEstimate, String>
```

**MdConfig 关键字段**（对应前端 `MdConfig` 类型）：
- `ensemble`: NVE | NVT | NPT | NPH | UM
- `potential`: lj | eam | meam | tersoff | reaxff | morse | buckingham
- `timestep_fs`: f64
- `num_steps`: u64
- `atoms`: Vec<MdAtom>
- `thermostat`: nosé_hoover | berendsen | velocity_rescaling | andersen
- `barostat`: parrinello_rahman | berendsen | andersen

---

### 2.2 原子结构建模器（V1.5，对应 `src/api/atomBuilder.ts`）

**必须实现的 Tauri 命令：**

```rust
#[tauri::command]
async fn build_supercell(config: SupercellConfig) -> Result<AtomBuilderResult, String>

#[tauri::command]
async fn build_amorphous(config: AmorphousConfig) -> Result<AtomBuilderResult, String>

#[tauri::command]
async fn build_interface(config: InterfaceConfig) -> Result<AtomBuilderResult, String>

#[tauri::command]
async fn build_defect(config: DefectConfig) -> Result<AtomBuilderResult, String>

#[tauri::command]
async fn build_cnt(config: CntConfig) -> Result<AtomBuilderResult, String>

#[tauri::command]
async fn build_cluster(config: ClusterConfig) -> Result<AtomBuilderResult, String>

#[tauri::command]
async fn build_dislocation(config: DislocationConfig) -> Result<AtomBuilderResult, String>
```

**支持的晶体结构**：FCC, BCC, HCP, 金刚石, SC  
**特殊结构**：碳纳米管（手性向量 `(n,m)`）, 团簇（二十面体/十面体/FCC立方/BCC立方/球形）, 位错线（刃型/螺型/混合）

---

### 2.3 MD 后处理（V1.5，对应 `src/api/mdPostProcess.ts`）

**必须实现的 Tauri 命令：**

```rust
#[tauri::command]
async fn calculate_rdf(positions: Vec<[f64;3]>, r_max: f64, num_bins: u32) -> Result<Vec<f64>, String>

#[tauri::command]
async fn calculate_msd(positions: Vec<Vec<[f64;3]>>, dt: f64) -> Result<Vec<f64>, String>

#[tauri::command]
async fn calculate_vacf(velocities: Vec<Vec<[f64;3]>>, dt: f64) -> Result<Vec<f64>, String>

#[tauri::command]
async fn calculate_diffusion_coefficient(msd: Vec<f64>, dt: f64, num_atoms: u32) -> Result<f64, String>

#[tauri::command]
async fn calculate_virial_stress(positions: Vec<[f64;3]>, forces: Vec<[f64;3]>, volumes: Vec<f64>, num_steps: u32) -> Result<Vec<VirialStress>, String>

#[tauri::command]
async fn coarse_grain_md_to_phase_field(config: MdToPhaseConfig) -> Result<CoarseGrainResult, String>

#[tauri::command]
async fn coarse_grain_md_to_fe(config: MdToFeConfig) -> Result<FeLoadCase, String>
```

---

### 2.4 相场求解器（V1.6，对应 `src/api/phaseField.ts`）

**必须实现的 Tauri 命令：**

```rust
#[tauri::command]
async fn run_phase_field_simulation(config: PhaseFieldConfig) -> Result<PhaseFieldResult, String>

#[tauri::command]
async fn initialize_phase_field(config: PfInitConfig) -> Result<Vec<f64>, String>

#[tauri::command]
async fn couple_phase_field_temperature(config: ThermalCouplingConfig) -> Result<CoupledResult, String>

#[tauri::command]
async fn validate_phase_field_config(config: PhaseFieldConfig) -> Result<ValidationResult, String>
```

**方程类型**：Cahn-Hilliard (CH), Allen-Cahn (AC), Phase Field Crystal (PFC), Karma  
**自由能类型**：double_well, polynomial, landau, flory_huggins  
**时间积分**：explicit, implicit, semi_implicit, rk4

---

### 2.5 相场后处理（V1.6，对应 `src/api/phaseFieldPostProcess.ts`）

```rust
#[tauri::command]
async fn calculate_von_mises_stress(field_data: Vec<f64>, nx: u32, ny: u32, nz: u32) -> Result<Vec<f64>, String>

#[tauri::command]
async fn extract_grain_statistics(field_data: Vec<f64>, threshold: f64) -> Result<GrainStats, String>

#[tauri::command]
async fn calculate_elastic_energy_drive(phase_field: Vec<f64>, stress: Vec<f64>, mobility: f64) -> Result<f64, String>
```

---

### 2.6 GPU 加速（V1.6，对应 `src/api/phaseFieldGpu.ts`）

```rust
#[tauri::command]
async fn run_gpu_phase_field(config: PhaseFieldConfig, backend: GpuBackend) -> Result<PhaseFieldResult, String>

#[tauri::command]
async fn get_gpu_info() -> Result<Vec<GpuDevice>, String>

#[tauri::command]
async fn benchmark_gpu(backend: GpuBackend, grid_size: u32) -> Result<BenchmarkResult, String>
```

**后端选项**：cuda, opencl, webgpu, wasm_simd

---

### 2.7 尺度桥接（V1.6，对应 `src/api/phaseFieldBridge.ts`）

```rust
#[tauri::command]
async fn bridge_md_to_phase_field(atom_config: MdToPhaseConfig) -> Result<PhaseFieldInitData, String>

#[tauri::command]
async fn homogenize_phase_field_to_fe(homogenization_config: HomogenizeConfig) -> Result<HomogenizedMaterial, String>

#[tauri::command]
async fn calculate_homogenized_elastic_tensor(phase_data: Vec<f64>, voigt_reuss: String) -> Result<[f64;36], String>
```

**均匀化方法**：voigt, reuss, mori_tanaka, self_consistent, numerical

---

### 2.8 DFT 输入生成（V1.7，对应 `src/api/dftInput.ts`）

```rust
#[tauri::command]
async fn generate_vasp_input(config: VaspInputConfig) -> Result<VaspInputFiles, String>

#[tauri::command]
async fn generate_qe_input(config: QeInputConfig) -> Result<QeInputFiles, String>

#[tauri::command]
async fn validate_dft_config(config: DftConfig) -> Result<ValidationResult, String>
```

**VASP**：POSCAR, INCAR, KPOINTS 自动生成  
**QE**：PW/CP/PP 类型，Control/System/Electrons 块  
**模板类型**：scf, relax, md, bands, dos

---

### 2.9 DFT 任务管理（V1.7，对应 `src/api/dftTask.ts`）

```rust
#[tauri::command]
async fn submit_dft_job(config: DftJobConfig) -> Result<JobId, String>

#[tauri::command]
async fn get_dft_job_status(job_id: String) -> Result<JobStatus, String>

#[tauri::command]
async fn generate_slurm_script(config: DftJobConfig) -> Result<String, String>

#[tauri::command]
async fn generate_pbs_script(config: DftJobConfig) -> Result<String, String>
```

---

### 2.10 DFT 后处理（V1.7，对应 `src/api/dftPostProcess.ts`）

```rust
#[tauri::command]
async fn parse_vasp_output(output_dir: String) -> Result<DftResult, String>

#[tauri::command]
async fn parse_qe_output(output_dir: String) -> Result<DftResult, String>

#[tauri::command]
async fn validate_dft_energy_forces(energy: f64, forces: Vec<[f64;3]>, benchmark: String) -> Result<ValidationResult, String>

#[tauri::command]
async fn generate_bandstructure_plot(data: BandStructureData) -> Result<String, String>

#[tauri::command]
async fn generate_dos_plot(data: DosData) -> Result<String, String>
```

**标准算例**：energy < 1 meV/atom 精度验证

---

### 2.11 DFT 桥接（V1.7，对应 `src/api/dftBridge.ts`）

```rust
#[tauri::command]
async fn train_nep_potential(training_config: NepTrainingConfig) -> Result<TrainedPotential, String>

#[tauri::command]
async fn train_mtp_potential(training_config: MtpTrainingConfig) -> Result<TrainedPotential, String>

#[tauri::command]
async fn extract_gl_parameters(dft_result: DftResult) -> Result<GlParameters, String>

#[tauri::command]
async fn calculate_chemical_potential(dft_result: DftResult, temperature: f64) -> Result<f64, String>
```

---

### 2.12 多尺度集成（V1.8/V1.9）

```rust
#[tauri::command]
async fn run_multiscale_workflow(workflow_config: WorkflowConfig) -> Result<WorkflowResult, String>

#[tauri::command]
async fn transfer_data_between_scales(from_scale: String, to_scale: String, data: ScaleData) -> Result<ScaleData, String>

#[tauri::command]
async fn map_parameters_across_scales(param_map: ParamMapConfig) -> Result<MappedParams, String>
```

---

## 3. 实现优先级建议

| 优先级 | 模块 | 理由 |
|--------|------|------|
| P0 | MD LAMMPS 命令 | 最成熟的前端，先打通 |
| P0 | 原子结构建模器 | MD 的前置依赖 |
| P0 | MD 后处理（RDF/MSD） | 演示必需 |
| P1 | DFT 输入生成 | 相对独立 |
| P1 | 相场求解器 | 核心介观尺度 |
| P2 | GPU 加速 | 可 later fallback 到 CPU |
| P2 | 工作流编排器 | 依赖前后端都完成 |

---

## 4. 注意事项

- **不要破坏现有功能**：修改 `lib.rs` 时，确保已注册的 Phase 1~2 命令不受影响
- **编译通过**：修改完成后运行 `cargo build` 验证零错误
- **前后端对齐**：实现前先读对应前端 `src/api/*.ts` 文件，确认类型定义
- **代码质量**：遵守 Rust 编码规范，使用 `cargo clippy` 检查
- **模块化**：每个尺度（MD/相场/DFT）单独一个 `mod.rs` 子模块

---

## 5. 成功标准

1. `cargo build --release` 零错误
2. Phase 3 所有前端 `invoke()` 调用都有对应的 Rust 命令
3. 可以启动 `npm run dev` + `cargo tauri dev`，打开 `/md` 路由不报 "command not found"
4. 基础 MD 任务（单原子盒 NVE 模拟）可以从 UI 创建到拿到结果
