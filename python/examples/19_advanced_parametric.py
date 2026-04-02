"""
19_advanced_parametric.py — 高级参数化研究

演示多参数联合扫描和响应面分析。
"""

import caelab
import itertools

# 定义参数空间
thickness = [0.005, 0.01, 0.015, 0.02]       # m
load_levels = [5000, 10000, 20000, 50000]      # N
materials = ["steel", "aluminum"]               # 材料类型

material_map = {
    "steel": caelab.Material.steel(),
    "aluminum": caelab.Material.aluminum(),
}

print(f"参数组合总数: {len(thickness) * len(load_levels) * len(materials)}")
print(f"{'Material':<12} {'t(mm)':<8} {'F(N)':<10} {'Disp(mm)':<14} {'Stress(MPa)':<14}")
print("-" * 58)

results = []
for mat_name, t, F in itertools.product(materials, thickness, load_levels):
    proj = caelab.Project(
        f"AdvParam_{mat_name}_t{t*1000:.0f}_F{F}",
        "高级参数化",
    )
    pid = proj.save()

    mesh = caelab.Mesh(pid)
    mesh.generate_structured(nx=8, ny=8, nz=max(int(t * 200), 1), z_max=t)

    mat = material_map[mat_name]
    mat.assign_to_project(pid)

    sim = caelab.Simulation(pid)
    sim.set_analysis_type("static")
    sim.add_fixed_bc(node_ids=list(range(1, 10)))
    sim.add_load(node_ids=[300, 301], values=[0, 0, -F])

    result = sim.run()

    row = {
        "material": mat_name,
        "thickness": t,
        "load": F,
        "disp": result.max_displacement(),
        "stress": result.max_stress(),
    }
    results.append(row)

    print(
        f"{mat_name:<12} {t*1000:<8.1f} {F:<10} "
        f"{row['disp']*1000:<14.4f} {row['stress']/1e6:<14.2f}"
    )

# 找到最优组合
best = min(results, key=lambda r: r["disp"])
print(f"\n最小位移组合: {best['material']}, t={best['thickness']*1000:.0f}mm, F={best['load']}N")
print(f"  位移 = {best['disp']*1000:.4f} mm")
