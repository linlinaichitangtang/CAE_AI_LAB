"""
05_mesh_convergence.py — 网格收敛性研究

通过逐步加密网格，验证数值解的收敛性。
"""

import caelab

project = caelab.Project("Mesh Convergence", "网格收敛性验证")
pid = project.save()

caelab.Material.steel().assign_to_project(pid)

# 不同网格密度
mesh_sizes = [4, 6, 8, 12, 16, 20, 24]
results = []

for n in mesh_sizes:
    mesh = caelab.Mesh(pid)
    mesh.generate_structured(nx=n, ny=n, nz=max(n // 2, 2))
    stats = mesh.get_stats()
    elem_count = stats.get("element_count", 0)

    sim = caelab.Simulation(pid)
    sim.set_analysis_type("static")
    sim.add_fixed_bc(node_ids=list(range(1, (n + 1) * max(n // 2, 2) + 1)))
    sim.add_load(node_ids=[elem_count, elem_count - 1], values=[0, 0, -5000.0])

    result = sim.run()
    results.append({
        "n": n,
        "elements": elem_count,
        "max_disp": result.max_displacement(),
        "max_stress": result.max_stress(),
    })
    print(f"n={n:3d} | elements={elem_count:6d} | "
          f"disp={result.max_displacement()*1000:.6f} mm | "
          f"stress={result.max_stress()/1e6:.2f} MPa")

# 收敛率分析
print("\n网格收敛性分析:")
for i in range(1, len(results)):
    prev = results[i - 1]["max_disp"]
    curr = results[i]["max_disp"]
    rate = abs((curr - prev) / prev) * 100
    print(f"  {results[i]['n']:3d} -> {results[i+1]['n'] if i+1<len(results) else 'N/A':>3s}: "
          f"变化率 = {rate:.4f}%")
