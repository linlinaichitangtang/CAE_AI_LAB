# Tasks

- [ ] Task 1: 清理未使用的 import 语句
  - [ ] SubTask 1.1: 移除未使用的日志宏 (`warn`, `error`)
  - [ ] SubTask 1.2: 移除未使用的 `std::collections::HashMap`
  - [ ] SubTask 1.3: 移除未使用的 `serde_json::Value`
  - [ ] SubTask 1.4: 移除未使用的 `Deserialize`, `Serialize`
  - [ ] SubTask 1.5: 移除其他未使用 import (`tauri::State`, `std::sync::Arc`, `body::Body`)

- [ ] Task 2: 移除不必要的括号
  - [ ] SubTask 2.1: 定位并移除块返回值周围的冗余括号

- [ ] Task 3: 修复未使用变量（添加 `_` 前缀）
  - [ ] SubTask 3.1: 修复热分析相关未使用变量 (`min_temp`, `avg_temp`, `geometry_factor`, `T_ambient`, `cp_f`)
  - [ ] SubTask 3.2: 修复流体分析相关未使用变量 (`min_pressure`, `rho`, `lz`, `area_ratio`, `pair_style`)
  - [ ] SubTask 3.3: 修复网格/几何相关未使用变量 (`elem`, `dy`, `total`, `kappa`, `dx`, `dz`, `iy`, `iz`)
  - [ ] SubTask 3.4: 修复其他未使用变量 (`kpts`, `counts`, `i`, `cartesian`, `fermi_idx`, `test_data_size`, `method`, `statuses`, `branch`, `commit`, `genesis_chain_hash`, `domain_size`, `field_grid_size`, `db`)

- [ ] Task 4: 修复 snake_case 命名问题
  - [ ] SubTask 4.1: 修复结构体字段命名 (`tolerance_meV`, `energy_rmse_meV`, `force_rmse_meV_A`, `stress_rmse_GPa`)
  - [ ] SubTask 4.2: 修复热阻分析变量命名 (`E`, `Pr`, `Re`, `Gz`, `Nu`, `R_conv`, `R_cond`, `R_total`, `T_ambient`, `T_inlet`, `Q`, `T_base`, `T_max`, `T_min`, `T_avg`, `C_min`, `Q_max`, `T_local`, `best_Rth`, `best_dP`, `best_Tmax`)
  - [ ] SubTask 4.3: 修复散热器优化变量命名 (`T_in`, `V_in`, `V_channel`, `A_fin`, `A_base`, `A_total`, `mL`, `R_fin`, `dP`, `dP_penalty`, `D`, `L`, `M_max`)

- [ ] Task 5: 处理 dead code 警告
  - [ ] SubTask 5.1: 评估并处理 `hex8_shape_functions`, `compute_lj_potential`, `compute_lj_force` 函数
  - [ ] SubTask 5.2: 评估并处理 `xorshift64`, `format_position_line` 函数
  - [ ] SubTask 5.3: 评估并处理 `generate_job_id`, `job_entry_to_task` 函数
  - [ ] SubTask 5.4: 评估并处理未使用的结构体字段

- [ ] Task 6: 修复不必要的可变变量
  - [ ] SubTask 6.1: 移除不必要的 `mut` 关键字

- [ ] Task 7: 处理未读取的赋值
  - [ ] SubTask 7.1: 处理 `current_material_name`, `current_amplitude_name` 赋值
  - [ ] SubTask 7.2: 处理 `element_counts`, `basis_start` 赋值

- [ ] Task 8: 验证构建
  - [ ] SubTask 8.1: 运行 `cargo build` 验证零错误零警告
  - [ ] SubTask 8.2: 运行 `cargo build --release` 验证

# Task Dependencies
- Task 1-7 可并行执行
- Task 8 依赖 Task 1-7 全部完成
