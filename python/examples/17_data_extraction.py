"""
17_data_extraction.py — 数据提取与分析

从仿真结果中提取数据并进行后处理分析。
"""

import caelab

# 运行仿真
project = caelab.Project("Data Extraction", "数据提取示例")
pid = project.save()

mesh = caelab.Mesh(pid)
mesh.generate_structured(nx=15, ny=10, nz=5)
caelab.Material.steel().assign_to_project(pid)

sim = caelab.Simulation(pid)
sim.set_analysis_type("static")
sim.add_fixed_bc(node_ids=list(range(1, 12)))
sim.add_load(node_ids=[750, 751, 752], values=[0, 0, -8000.0])
result = sim.run()

# 1. 提取位移场
disp = result.get_displacement_field()
print("位移场统计:")
print(f"  节点数: {len(disp.get('node_ids', []))}")
if disp.get("magnitude"):
    mags = disp["magnitude"]
    print(f"  最大位移: {max(mags)*1000:.6f} mm")
    print(f"  最小位移: {min(mags)*1000:.6f} mm")
    print(f"  平均位移: {sum(mags)/len(mags)*1000:.6f} mm")

# 2. 提取应力场
stress = result.get_stress_field()
print("\n应力场统计:")
if stress.get("von_mises"):
    vm = stress["von_mises"]
    print(f"  单元数: {len(vm)}")
    print(f"  最大 von Mises: {max(vm)/1e6:.2f} MPa")
    print(f"  最小 von Mises: {min(vm)/1e6:.2f} MPa")

# 3. 反力分析
rf = result.reaction_forces()
print(f"\n反力:")
for direction, force in rf.items():
    print(f"  {direction}: {force:.4f} N")

# 4. 应变能
print(f"\n总应变能: {result.total_strain_energy():.6e} J")

# 5. 安全系数估算
yield_stress = 235e6  # Q235
max_vm = result.max_stress()
safety_factor = yield_stress / max_vm if max_vm > 0 else float("inf")
print(f"\n安全系数 (Q235): {safety_factor:.2f}")
