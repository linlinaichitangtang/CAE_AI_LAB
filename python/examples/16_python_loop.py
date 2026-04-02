"""
16_python_loop.py — Python 循环控制

演示使用 Python 原生循环进行参数扫描和自动化分析。
"""

import caelab

# 参数定义
thickness_values = [0.005, 0.01, 0.015, 0.02, 0.025, 0.03]  # m
load_value = 10000  # N

print("=" * 60)
print("  板厚度参数化分析")
print("=" * 60)
print(f"{'Thickness(mm)':<16} {'Max Disp(mm)':<16} {'Max Stress(MPa)':<18} {'Mass(kg)':<12}")
print("-" * 62)

for t in thickness_values:
    # 创建项目
    proj = caelab.Project(
        f"Plate_t{t*1000:.0f}mm",
        f"钢板厚度 {t*1000:.0f}mm",
    )
    pid = proj.save()

    # 生成网格
    mesh = caelab.Mesh(pid)
    mesh.generate_structured(
        nx=10, ny=10, nz=max(int(t * 200), 1),
        z_max=t,
    )

    # 材料
    steel = caelab.Material.steel()
    steel.assign_to_project(pid)

    # 仿真
    sim = caelab.Simulation(pid)
    sim.set_analysis_type("static")
    sim.add_fixed_bc(node_ids=list(range(1, 12)))
    sim.add_load(node_ids=[500, 501], values=[0, 0, -load_value])

    result = sim.run()

    # 计算质量
    area = 0.1 * 0.1  # 0.01 m2
    mass = area * t * steel.get_properties()["density"]

    print(
        f"{t*1000:<16.1f} "
        f"{result.max_displacement()*1000:<16.4f} "
        f"{result.max_stress()/1e6:<18.2f} "
        f"{mass:<12.4f}"
    )
