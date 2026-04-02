"""
12_contact_analysis.py — 接触分析

模拟两个部件之间的接触行为。
"""

import caelab

project = caelab.Project("Contact Analysis", "二维接触分析")
pid = project.save()

# 生成网格 — 两个部件
mesh = caelab.Mesh(pid)
mesh.generate_structured(
    nx=20, ny=20, nz=4,
    x_min=0, x_max=0.2,
    y_min=0, y_max=0.2,
    z_min=0, z_max=0.02,
    element_type="C3D8",
)
print(f"网格: {mesh.get_stats()}")

# 材料
steel = caelab.Material.steel()
steel.assign_to_project(pid)

# 配置仿真
sim = caelab.Simulation(pid)
sim.set_analysis_type("static")

# 底部固定
sim.add_fixed_bc(node_ids=list(range(1, 85)))

# 接触面荷载
sim.add_pressure("contact_surface_top", 1e6)  # 1 MPa

# 添加重力
sim.add_gravity(magnitude=9.81, direction="z")

# 求解
result = sim.run()
print(result.summary())

# 检查接触状态
contact_data = result._data.get("contact", {})
print(f"\n接触状态:")
print(f"  接触节点数: {contact_data.get('contact_nodes', 'N/A')}")
print(f"  最大接触压力: {contact_data.get('max_pressure', 0)/1e6:.2f} MPa")
print(f"  总接触力: {contact_data.get('total_force', 0):.2f} N")
