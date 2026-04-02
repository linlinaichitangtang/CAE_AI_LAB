"""
10_result_visualization.py — 结果可视化

演示多种结果可视化方式。
"""

import caelab

project = caelab.Project("Result Visualization", "可视化示例")
pid = project.save()

mesh = caelab.Mesh(pid)
mesh.generate_structured(nx=15, ny=10, nz=5)
caelab.Material.steel().assign_to_project(pid)

sim = caelab.Simulation(pid)
sim.set_analysis_type("static")
sim.add_fixed_bc(node_ids=list(range(1, 12)))
sim.add_load(node_ids=[750, 751, 752], values=[0, 0, -8000.0])

result = sim.run()

# 1. Von Mises 应力云图
print("绘制 Von Mises 应力云图...")
result.plot("von_mises")

# 2. 位移场
print("绘制位移场...")
result.plot("displacement")

# 3. 应力分量 Sxx
print("绘制 Sxx 应力分量...")
result.plot("sxx")

# 4. 导出数据用于外部绘图
result.export_data("displacement.csv", field="displacement")
print("位移数据已导出至 displacement.csv")

result.export_data("stress.csv", field="stress")
print("应力数据已导出至 stress.csv")

# 5. 导出完整报告
result.export_report("simulation_report.pdf")
print("PDF 报告已导出")
