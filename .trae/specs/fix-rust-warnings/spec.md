# 清理 CAELab V2.1 Rust 编译警告 Spec

## Why
项目当前有 128 个编译警告，影响代码质量和可维护性。清理警告可以提升代码质量，便于后续开发。

## What Changes
- **P0 必须修**：确认并处理 `_genesis_chain_hash` 未使用问题；修复 snake_case 命名问题
- **P1 应当修**：给未使用变量加 `_` 前缀；处理 dead code 警告
- **P2 可选**：清理未使用的 import；移除不必要的括号

## Impact
- Affected code: `src-tauri/src/` 下多个 Rust 源文件
- 不影响任何业务逻辑和功能

## ADDED Requirements

### Requirement: 清理未使用 import
系统 SHALL 移除所有未使用的 `use` 语句，包括：
- `warn`, `error` 日志宏
- `std::collections::HashMap`
- `serde_json::Value`
- `Deserialize`, `Serialize`
- `tauri::State`
- `std::sync::Arc`
- `body::Body`

#### Scenario: 移除未使用 import
- **WHEN** 某个 import 在模块中未被使用
- **THEN** 移除该 import 语句

### Requirement: 修复未使用变量
系统 SHALL 为所有未使用但可能有意保留的变量添加 `_` 前缀，包括但不限于：
- `min_temp`, `avg_temp`, `geometry_factor`, `T_ambient`, `cp_f`
- `min_pressure`, `rho`, `lz`, `area_ratio`, `pair_style`
- `elem`, `dy`, `total`, `kappa`, `dx`, `dz`, `iy`, `iz`
- `kpts`, `counts`, `i`, `cartesian`, `fermi_idx`, `test_data_size`
- `method`, `statuses`, `branch`, `commit`, `genesis_chain_hash`
- `domain_size`, `field_grid_size`, `db`

#### Scenario: 未使用变量处理
- **WHEN** 变量被声明但未使用
- **THEN** 添加 `_` 前缀表示有意忽略

### Requirement: 修复 snake_case 命名
系统 SHALL 将非 snake_case 命名的变量和字段改为 snake_case：
- 结构体字段：`tolerance_meV` → `tolerance_mev`, `energy_rmse_meV` → `energy_rmse_mev`, `force_rmse_meV_A` → `force_rmse_mev_a`, `stress_rmse_GPa` → `stress_rmse_gpa`
- 变量：`E` → `e`, `Pr` → `pr`, `Re` → `re`, `Gz` → `gz`, `Nu` → `nu`, `R_conv` → `r_conv`, `R_cond` → `r_cond`, `R_total` → `r_total`, `T_ambient` → `t_ambient`, `T_inlet` → `t_inlet`, `Q` → `q`, `T_base` → `t_base`, `T_max` → `t_max`, `T_min` → `t_min`, `T_avg` → `t_avg`, `C_min` → `c_min`, `Q_max` → `q_max`, `T_local` → `t_local`, `best_Rth` → `best_rth`, `best_dP` → `best_dp`, `best_Tmax` → `best_tmax`, `T_in` → `t_in`, `V_in` → `v_in`, `V_channel` → `v_channel`, `A_fin` → `a_fin`, `A_base` → `a_base`, `A_total` → `a_total`, `mL` → `m_l`, `R_fin` → `r_fin`, `dP` → `dp`, `dP_penalty` → `dp_penalty`, `D` → `d`, `L` → `l`, `M_max` → `m_max`

#### Scenario: snake_case 命名修复
- **WHEN** 变量或字段名不符合 snake_case 规范
- **THEN** 重命名为 snake_case 格式

### Requirement: 处理 dead code
系统 SHALL 对以下 dead code 进行评估和处理：
- `hex8_shape_functions` 函数
- `compute_lj_potential` 函数
- `compute_lj_force` 函数
- `xorshift64` 函数（2处）
- `format_position_line` 函数
- `generate_job_id` 函数
- `job_entry_to_task` 函数
- 未使用的结构体字段

#### Scenario: dead code 处理
- **WHEN** 代码被标记为 dead code
- **THEN** 确认是否需要保留，若保留则添加 `#[allow(dead_code)]`，否则移除

### Requirement: 移除不必要的括号
系统 SHALL 移除块返回值周围不必要的括号。

#### Scenario: 括号清理
- **WHEN** 块返回值周围有不必要的括号
- **THEN** 移除多余括号

### Requirement: 修复不必要的可变变量
系统 SHALL 移除不必要的 `mut` 关键字。

#### Scenario: 可变变量修复
- **WHEN** 变量声明为 mutable 但实际未修改
- **THEN** 移除 `mut` 关键字

### Requirement: 处理未读取的赋值
系统 SHALL 处理赋值后从未读取的变量：
- `current_material_name`
- `current_amplitude_name`
- `element_counts`
- `basis_start`

#### Scenario: 未读取赋值处理
- **WHEN** 变量被赋值但值从未被读取
- **THEN** 移除赋值或添加 `_` 前缀
