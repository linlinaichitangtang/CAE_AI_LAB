"""
07_thermal_analysis.py — 稳态热传导分析

分析一块钢板在两侧不同温度下的温度分布和热应力。
"""

import caelab

project = caelab.Project("Thermal Analysis", "稳态热传导分析")
pid = project.save()

# 生成网格
mesh = caelab.Mesh(pid)
mesh.generate_structured(nx=20, ny=10, nz=2, element_type="DC3D8")
print(f"网格: {mesh.get_stats()}")

# 设置材料（含热属性）
steel = caelab.Material.steel()
steel.set_thermal(conductivity=50.0, expansion=12e-6)
steel.assign_to_project(pid)

# 配置热分析
sim = caelab.Simulation(pid)
sim.set_analysis_type("thermal")

# 温度边界条件
sim.add_fixed_bc(node_ids=list(range(1, 25)))  # 左侧固定
sim.add_load(
    node_ids=list(range(1, 25)),
    values=[100.0] * 24,
    load_type="temperature",
)
sim.add_load(
    node_ids=list(range(200, 224)),
    values=[25.0] * 24,
    load_type="temperature",
)

# 求解
result = sim.run()
print(result.summary())

# 可视化温度场
result.plot("displacement")
