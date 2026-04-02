"""
18_jupyter_integration.py — Jupyter 集成

演示在 Jupyter Notebook 中使用 CAELab Python API。
注意：此脚本需在 Jupyter 环境中运行。
"""

import caelab

# 检测运行环境
from caelab._client import CAELabClient

client = CAELabClient()
print(f"运行模式: {client._mode}")
print(f"Jupyter 环境: {client._is_jupyter()}")

# 在 Jupyter 中，API 通过 HTTP 与 CAELab 桌面端通信
# 以下代码在 Jupyter 和脚本环境中均可运行

# 创建项目
project = caelab.Project("Jupyter Demo", "Jupyter 集成演示")
pid = project.save()
print(f"项目 ID: {pid}")

# 快速网格
mesh = caelab.Mesh(pid)
mesh.generate_structured(nx=10, ny=10, nz=5)

# 材料
mat = caelab.Material.aluminum()
mat.assign_to_project(pid)

# 仿真
sim = caelab.Simulation(pid)
sim.set_analysis_type("static")
sim.add_fixed_bc(node_ids=list(range(1, 12)))
sim.add_load(node_ids=[550, 551], values=[0, 0, -5000.0])

result = sim.run()

# 在 Jupyter 中 result.summary() 会格式化输出
print(result.summary())

# 在 Jupyter 中可以直接调用 plot() 显示内联图表
# result.plot("von_mises")

# 导出数据供 Jupyter 进一步分析
result.export_data("jupyter_data.json", field="all")
print("数据已导出至 jupyter_data.json，可在 Notebook 中加载分析")
