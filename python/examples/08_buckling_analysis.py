"""
08_buckling_analysis.py — 屈曲分析

分析薄壁柱在轴向压力下的屈曲载荷和屈曲模态。
"""

import caelab

project = caelab.Project("Buckling Analysis", "薄壁柱屈曲分析")
pid = project.save()

# 生成网格 — 细长柱
mesh = caelab.Mesh(pid)
mesh.generate_structured(
    nx=4, ny=4, nz=30,
    x_min=-0.05, x_max=0.05,
    y_min=-0.05, y_max=0.05,
    z_min=0, z_max=1.0,
    element_type="C3D8",
)
print(f"网格: {mesh.get_stats()}")

# 设置材料
steel = caelab.Material.steel()
steel.assign_to_project(pid)

# 配置屈曲分析
sim = caelab.Simulation(pid)
sim.set_analysis_type("buckling")

# 底部固定
sim.add_fixed_bc(node_ids=list(range(1, 25)))

# 顶部施加单位轴向压力
sim.add_load(node_ids=list(range(577, 601)), values=[0, 0, -1.0])

# 求解
result = sim.run()
print(result.summary())

# 屈曲载荷因子
print("\n屈曲分析结果:")
for i in range(1, 6):
    factor = result._data.get(f"buckling_factor_{i}", 0)
    print(f"  Mode {i}: 载荷因子 = {factor:.4f}")
