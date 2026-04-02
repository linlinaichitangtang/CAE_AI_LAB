"""
09_batch_processing.py — 批处理仿真

演示如何使用 Python 循环批量运行多个仿真任务。
"""

import caelab
import time

# 定义仿真任务列表
tasks = [
    {"name": "Beam-10kN", "load": 10000},
    {"name": "Beam-20kN", "load": 20000},
    {"name": "Beam-30kN", "load": 30000},
    {"name": "Beam-40kN", "load": 40000},
    {"name": "Beam-50kN", "load": 50000},
]

print(f"开始批处理仿真，共 {len(tasks)} 个任务\n")

all_results = []
total_start = time.time()

for idx, task in enumerate(tasks, 1):
    print(f"[{idx}/{len(tasks)}] 运行: {task['name']}...")

    task_start = time.time()

    # 创建项目
    proj = caelab.Project(task["name"], f"批处理 - 荷载 {task['load']}N")
    pid = proj.save()

    # 网格 + 材料
    mesh = caelab.Mesh(pid)
    mesh.generate_structured(nx=10, ny=10, nz=5)
    caelab.Material.steel().assign_to_project(pid)

    # 仿真
    sim = caelab.Simulation(pid)
    sim.set_analysis_type("static")
    sim.add_fixed_bc(node_ids=list(range(1, 12)))
    sim.add_load(node_ids=[550, 551], values=[0, 0, -task["load"]])

    result = sim.run()
    elapsed = time.time() - task_start

    all_results.append({
        "name": task["name"],
        "load": task["load"],
        "max_disp": result.max_displacement(),
        "max_stress": result.max_stress(),
        "time": elapsed,
    })
    print(f"  完成: {elapsed:.2f}s, "
          f"max_disp={result.max_displacement()*1000:.4f}mm, "
          f"max_stress={result.max_stress()/1e6:.2f}MPa")

total_time = time.time() - total_start
print(f"\n批处理完成! 总耗时: {total_time:.2f}s")
