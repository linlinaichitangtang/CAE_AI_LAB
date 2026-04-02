"""
13_custom_material.py — 自定义材料

演示如何创建和配置自定义材料属性。
"""

import caelab

# 1. 创建自定义材料 — 碳纤维复合材料
cfrp = caelab.Material(
    "CFRP_Unidirectional",
    youngs_modulus=140e9,
    poisson_ratio=0.28,
    density=1600.0,
    yield_strength=1200e6,
    ultimate_strength=1500e6,
)
cfrp.set_thermal(conductivity=7.0, expansion=0.2e-6)
print(cfrp)

# 2. 创建自定义材料 — 混凝土
concrete = caelab.Material("Concrete_C30")
concrete.set_elastic(youngs_modulus=30e9, poisson_ratio=0.2)
concrete.set_density(2400.0)
concrete.set_plastic(yield_strength=30e6, ultimate_strength=40e6)
print(concrete)

# 3. 在项目中使用
project = caelab.Project("Custom Materials", "自定义材料对比")
pid = project.save()

mesh = caelab.Mesh(pid)
mesh.generate_structured(nx=10, ny=10, nz=5)

# 依次使用不同材料
for mat in [cfrp, concrete]:
    mat.assign_to_project(pid)
    sim = caelab.Simulation(pid)
    sim.set_analysis_type("static")
    sim.add_fixed_bc(node_ids=list(range(1, 12)))
    sim.add_load(node_ids=[550, 551], values=[0, 0, -5000.0])
    result = sim.run()
    print(f"\n{mat.name}:")
    print(f"  Max Disp = {result.max_displacement()*1000:.4f} mm")
    print(f"  Max Stress = {result.max_stress()/1e6:.2f} MPa")
