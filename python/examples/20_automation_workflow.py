"""
20_automation_workflow.py — 自动化工作流

演示完整的自动化仿真工作流，包含条件判断和错误处理。
"""

import caelab
import sys

# 配置
CONFIG = {
    "design_cases": [
        {"name": "Case-A", "load": 10000, "material": "steel"},
        {"name": "Case-B", "load": 20000, "material": "aluminum"},
        {"name": "Case-C", "load": 30000, "material": "titanium"},
    ],
    "max_allowable_stress": 300e6,  # Pa
    "max_allowable_disp": 0.5e-3,   # m
}

material_factory = {
    "steel": caelab.Material.steel,
    "aluminum": caelab.Material.aluminum,
    "titanium": caelab.Material.titanium,
}

print("=" * 60)
print("  CAELab 自动化工作流")
print("=" * 60)

passed = []
failed = []

for case in CONFIG["design_cases"]:
    print(f"\n--- {case['name']} ---")
    try:
        # 创建项目
        proj = caelab.Project(case["name"], f"自动化 - {case['material']}")
        pid = proj.save()

        # 网格
        mesh = caelab.Mesh(pid)
        mesh.generate_structured(nx=12, ny=8, nz=4)

        # 材料
        mat = material_factory[case["material"]]()
        mat.assign_to_project(pid)

        # 仿真
        sim = caelab.Simulation(pid)
        sim.set_analysis_type("static")
        sim.add_fixed_bc(node_ids=list(range(1, 12)))
        sim.add_load(node_ids=[380, 381], values=[0, 0, -case["load"]])

        result = sim.run()

        # 校核
        max_stress = result.max_stress()
        max_disp = result.max_displacement()

        stress_ok = max_stress <= CONFIG["max_allowable_stress"]
        disp_ok = max_disp <= CONFIG["max_allowable_disp"]

        status = "PASS" if (stress_ok and disp_ok) else "FAIL"
        print(f"  Stress: {max_stress/1e6:.1f} MPa {'[OK]' if stress_ok else '[FAIL]'}")
        print(f"  Disp:   {max_disp*1000:.4f} mm {'[OK]' if disp_ok else '[FAIL]'}")
        print(f"  Status: {status}")

        if status == "PASS":
            passed.append(case["name"])
            result.export_report(f"{case['name']}_report.pdf")
        else:
            failed.append(case["name"])

    except Exception as e:
        print(f"  ERROR: {e}")
        failed.append(case["name"])

# 汇总
print(f"\n{'='*60}")
print(f"  工作流完成: {len(passed)} 通过, {len(failed)} 失败")
if failed:
    print(f"  失败案例: {', '.join(failed)}")
    sys.exit(1)
else:
    print("  所有设计案例均通过校核!")
