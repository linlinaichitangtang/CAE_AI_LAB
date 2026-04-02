"""
02_cantilever_beam.py — 悬臂梁静力学分析

模拟一端固定的悬臂梁在自由端承受集中力时的变形和应力分布。
梁尺寸: 1m x 0.1m x 0.05m
材料: 结构钢 Q235
荷载: 自由端集中力 10 kN
"""

import caelab

# 创建项目
project = caelab.Project("Cantilever Beam", "悬臂梁静力学分析")
pid = project.save()

# 生成网格 — 沿长度方向加密
mesh = caelab.Mesh(pid)
mesh.generate_structured(
    nx=20, ny=4, nz=2,
    x_min=0, x_max=1.0,
    y_min=0, y_max=0.1,
    z_min=0, z_max=0.05,
    element_type="C3D8",
)
stats = mesh.get_stats()
print(f"网格: {stats.get('node_count', '?')} 节点, {stats.get('element_count', '?')} 单元")

# 设置材料
steel = caelab.Material.steel()
steel.assign_to_project(pid)
print(f"材料: E={steel.get_properties()['youngs_modulus']/1e9:.1f} GPa")

# 配置仿真
sim = caelab.Simulation(pid)
sim.set_analysis_type("static")

# 固定端 (x=0 面)
sim.add_fixed_bc(node_ids=list(range(1, 15)))

# 自由端集中荷载 (x=1m 面, Fz = -10kN)
sim.add_load(node_ids=[180, 181, 182, 183], values=[0, 0, -10000.0])

# 求解
result = sim.run()
print(result.summary())

# 可视化
result.plot("von_mises")
