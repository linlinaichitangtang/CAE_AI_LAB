"""
04_parametric_study.py — 参数化研究

对悬臂梁进行荷载参数化扫描，分析不同荷载水平下的结构响应。
"""

import caelab

# 创建项目
project = caelab.Project("Parametric Study", "荷载参数化扫描")
pid = project.save()

# 网格和材料（只需设置一次）
mesh = caelab.Mesh(pid)
mesh.generate_structured(nx=15, ny=5, nz=3)
caelab.Material.steel().assign_to_project(pid)

# 定义参数化荷载列表
load_levels = [1000, 2500, 5000, 7500, 10000, 15000, 20000]  # N
parameters = [{"load_magnitude": F} for F in load_levels]

# 配置仿真
sim = caelab.Simulation(pid)
sim.set_analysis_type("static")
sim.add_fixed_bc(node_ids=list(range(1, 20)))

# 运行参数化仿真
results = sim.run_parametric(parameters)

# 汇总结果
print(f"{'Load (N)':<12} {'Max Disp (mm)':<16} {'Max Stress (MPa)':<18}")
print("-" * 46)
for i, result in enumerate(results):
    F = load_levels[i]
    print(
        f"{F:<12} {result.max_displacement()*1000:<16.4f} "
        f"{result.max_stress()/1e6:<18.2f}"
    )
