"""
11_topology_optimization.py — 拓扑优化

使用 CAELab 进行基础拓扑优化，在给定载荷下寻找最优材料分布。
"""

import caelab

project = caelab.Project("Topology Optimization", "悬臂梁拓扑优化")
pid = project.save()

# 设计域网格
mesh = caelab.Mesh(pid)
mesh.generate_structured(
    nx=30, ny=10, nz=5,
    x_min=0, x_max=1.0,
    y_min=0, y_max=0.3,
    z_min=0, z_max=0.15,
    element_type="C3D8",
)
print(f"设计域: {mesh.get_stats()}")

# 材料
steel = caelab.Material.steel()
steel.assign_to_project(pid)

# 配置仿真
sim = caelab.Simulation(pid)
sim.set_analysis_type("static")

# 固定端
sim.add_fixed_bc(node_ids=list(range(1, 60)))

# 荷载
sim.add_load(node_ids=[1650, 1651, 1652, 1653, 1654, 1655],
             values=[0, 0, -5000.0])

# 运行拓扑优化
result = sim.run()

# 优化结果
print(result.summary())
print(f"\n优化迭代次数: {result._data.get('iterations', 'N/A')}")
print(f"最终体积分数: {result._data.get('volume_fraction', 'N/A')}")
print(f"合规度: {result._data.get('compliance', 'N/A')}")

# 可视化密度分布
result.plot("von_mises")
