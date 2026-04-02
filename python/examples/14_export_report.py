"""
14_export_report.py — 导出仿真报告

演示如何自动生成和导出多种格式的仿真报告。
"""

import caelab
import json

# 运行仿真
project = caelab.Project("Report Export Demo", "报告导出示例")
pid = project.save()

mesh = caelab.Mesh(pid)
mesh.generate_structured(nx=12, ny=8, nz=4)
caelab.Material.aluminum().assign_to_project(pid)

sim = caelab.Simulation(pid)
sim.set_analysis_type("static")
sim.add_fixed_bc(node_ids=list(range(1, 12)))
sim.add_load(node_ids=[380, 381], values=[0, 0, -10000.0])
result = sim.run()

# 1. 导出 PDF 报告
result.export_report("report.pdf")
print("[OK] PDF 报告已导出: report.pdf")

# 2. 导出 HTML 报告
result.export_report("report.html")
print("[OK] HTML 报告已导出: report.html")

# 3. 导出 DOCX 报告
result.export_report("report.docx")
print("[OK] DOCX 报告已导出: report.docx")

# 4. 导出原始数据
result.export_data("results.json", field="all")
print("[OK] JSON 数据已导出: results.json")

result.export_data("displacement.csv", field="displacement")
print("[OK] CSV 位移数据已导出: displacement.csv")

result.export_data("stress.csv", field="stress")
print("[OK] CSV 应力数据已导出: stress.csv")

# 5. 导出项目
project.export("project_archive.caelabzip")
print("[OK] 项目已导出: project_archive.caelabzip")

print("\n所有报告和数据已导出完成。")
