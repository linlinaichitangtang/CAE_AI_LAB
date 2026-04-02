"""
06_modal_analysis.py — 模态分析

提取结构的固有频率和振型，评估动力特性。
"""

import caelab

project = caelab.Project("Modal Analysis", "自由振动模态分析")
pid = project.save()

# 生成网格
mesh = caelab.Mesh(pid)
mesh.generate_structured(nx=12, ny=8, nz=6, element_type="C3D8")
print(f"网格: {mesh.get_stats()}")

# 设置材料
steel = caelab.Material.steel()
steel.assign_to_project(pid)

# 配置模态分析
sim = caelab.Simulation(pid)
sim.set_analysis_type("modal")
sim.add_fixed_bc(node_ids=list(range(1, 20)))  # 固定底面

# 运行模态分析
result = sim.run()
print("\n模态分析结果:")
print(result.summary())

# 提取各阶频率
print("\n固有频率:")
for i in range(1, 11):
    freq = result._data.get(f"frequency_{i}", 0)
    print(f"  Mode {i:2d}: {freq:.2f} Hz")
