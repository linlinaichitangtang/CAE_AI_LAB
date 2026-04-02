"""
15_project_management.py — 项目管理

演示项目的 CRUD 操作和项目管理功能。
"""

import caelab

# 1. 创建多个项目
projects = []
for i in range(1, 6):
    proj = caelab.Project(f"Study-{i:03d}", f"第 {i} 个研究项目")
    proj.set_metadata("category", "benchmark")
    proj.set_metadata("priority", i)
    pid = proj.save()
    projects.append(proj)
    print(f"[创建] {proj.name} (ID: {pid})")

# 2. 列出所有项目
print("\n--- 所有项目 ---")
all_projects = caelab.Project.list()
for p in all_projects:
    cat = p.get_metadata("category", "N/A")
    print(f"  {p.name} | {p.description} | category={cat}")

# 3. 加载项目
loaded = caelab.Project.load(projects[0].id)
print(f"\n[加载] {loaded.name}, created_at={loaded.created_at}")

# 4. 更新项目
loaded.description = "更新后的描述"
loaded.set_metadata("status", "in_progress")
loaded.save()
print(f"[更新] {loaded.name} -> {loaded.description}")

# 5. 导出项目
projects[0].export("study_001.caelabzip")
print(f"[导出] {projects[0].name} -> study_001.caelabzip")

# 6. 删除项目
projects[-1].delete()
print(f"[删除] {projects[-1].name}")

# 7. 确认删除
remaining = caelab.Project.list()
print(f"\n剩余项目数: {len(remaining)}")
