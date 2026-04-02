"""
03_material_comparison.py — 材料对比分析

对比钢、铝、钛三种材料在同一载荷条件下的力学响应。
"""

import caelab

materials = {
    "Steel": caelab.Material.steel(),
    "Aluminum": caelab.Material.aluminum(),
    "Titanium": caelab.Material.titanium(),
}

results = {}

for name, mat in materials.items():
    props = mat.get_properties()
    print(f"\n{'='*50}")
    print(f"  材料: {name}")
    print(f"  E = {props['youngs_modulus']/1e9:.1f} GPa")
    print(f"  rho = {props['density']:.0f} kg/m3")
    print(f"  sigma_y = {props['yield_strength']/1e6:.0f} MPa")

    # 为每种材料创建独立项目
    proj = caelab.Project(f"Material Comparison - {name}", f"{name} analysis")
    pid = proj.save()

    mesh = caelab.Mesh(pid)
    mesh.generate_structured(nx=10, ny=10, nz=5)
    mat.assign_to_project(pid)

    sim = caelab.Simulation(pid)
    sim.set_analysis_type("static")
    sim.add_fixed_bc(node_ids=list(range(1, 12)))
    sim.add_load(node_ids=[550, 551], values=[0, 0, -5000.0])

    result = sim.run()
    results[name] = {
        "max_disp": result.max_displacement(),
        "max_stress": result.max_stress(),
    }

# 汇总对比
print(f"\n{'='*50}")
print(f"{'Material':<12} {'Max Disp (mm)':<16} {'Max Stress (MPa)':<18}")
print(f"{'-'*46}")
for name, r in results.items():
    print(f"{name:<12} {r['max_disp']*1000:<16.4f} {r['max_stress']/1e6:<18.2f}")
