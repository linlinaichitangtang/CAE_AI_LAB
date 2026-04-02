"""
01_quick_start.py — CAELab Python API 快速开始

本脚本演示最基本的 CAELab 工作流：
  创建项目 -> 生成网格 -> 设置材料 -> 施加荷载 -> 求解 -> 查看结果
"""

import caelab

# 1. 创建项目
project = caelab.Project("Quick Start Demo", "CAELab Python API 入门示例")
project_id = project.save()
print(f"[OK] 项目已创建: {project.name} (ID: {project_id})")

# 2. 生成网格
mesh = caelab.Mesh(project_id)
mesh_info = mesh.generate_structured(nx=10, ny=10, nz=5, element_type="C3D8")
print(f"[OK] 网格已生成: {mesh_info}")

# 3. 设置材料
material = caelab.Material.steel()
material.assign_to_project(project_id)
print(f"[OK] 材料已设置: {material.name}")

# 4. 配置仿真
sim = caelab.Simulation(project_id)
sim.set_analysis_type("static")
sim.add_fixed_bc(node_ids=list(range(1, 12)))  # 固定底面节点
sim.add_load(node_ids=[110, 111, 112], values=[0, 0, -1000.0])
print("[OK] 边界条件和荷载已设置")

# 5. 运行求解
result = sim.run()
print("[OK] 求解完成")

# 6. 查看结果
print(result.summary())
