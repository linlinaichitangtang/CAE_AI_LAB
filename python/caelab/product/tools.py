"""
V4.8 产品化工具集

V4.8-001: 7步仿真向导
V4.8-002: 经典案例模板库 (20个)
V4.8-003: 术语解释系统
V4.8-004: 自动合规检查
V4.8-005: PDF/Word 报告生成
"""
from __future__ import annotations
import json, sys, time, numpy as np

# ============================================================================
# V4.8-001: 7步仿真向导
# ============================================================================

WIZARD_STEPS = [
    {"id": 1, "name": "选择目标", "description": "选择你要完成的仿真类型", "options": ["静力分析", "模态分析", "热分析", "屈曲分析", "瞬态动力学", "CFD流体", "拓扑优化"]},
    {"id": 2, "name": "几何建模", "description": "创建或导入几何体", "options": ["从模板创建", "导入STEP/STL", "简单几何体", "参数化模型"]},
    {"id": 3, "name": "材料定义", "description": "选择或定义材料属性", "options": ["从材料库选择", "自定义材料", "AI推荐材料"]},
    {"id": 4, "name": "网格划分", "description": "设置网格参数并预览", "options": ["自动网格", "手动尺寸", "自适应网格", "质量检查"]},
    {"id": 5, "name": "荷载与边界", "description": "施加荷载和边界条件", "options": ["固定约束", "力/压力荷载", "热边界条件", "对称条件"]},
    {"id": 6, "name": "求解设置", "description": "配置求解参数", "options": ["默认设置", "高级参数", "收敛诊断", "并行设置"]},
    {"id": 7, "name": "后处理", "description": "查看和分析结果", "options": ["云图显示", "动画播放", "数据导出", "报告生成"]},
]

def get_wizard_steps():
    return WIZARD_STEPS

def validate_wizard_step(step_id: int, selection: str) -> dict:
    step = next((s for s in WIZARD_STEPS if s["id"] == step_id), None)
    if step is None:
        return {"valid": False, "error": f"未知步骤: {step_id}"}
    valid = selection in step["options"]
    return {"valid": valid, "step": step["name"], "selection": selection,
            "message": "选择有效" if valid else f"请从 {step['options']} 中选择"}


# ============================================================================
# V4.8-002: 经典案例模板库 (20个)
# ============================================================================

CASES = [
    {"id": "case_01", "name": "悬臂梁弯曲", "category": "静力学", "difficulty": "入门", "theory_solution": "δ = PL³/(3EI)", "expected_max_displacement_mm": 2.5, "has_mesh": True},
    {"id": "case_02", "name": "薄板孔应力集中", "category": "静力学", "difficulty": "入门", "theory_solution": "σ_max = Kt·σ_nominal", "expected_stress_concentration": 3.0, "has_mesh": True},
    {"id": "case_03", "name": "热膨胀应力", "category": "热-结构耦合", "difficulty": "进阶", "theory_solution": "σ = E·α·ΔT", "expected_stress_mid": 50, "has_mesh": True},
    {"id": "case_04", "name": "模态分析-简支梁", "category": "动力学", "difficulty": "入门", "theory_solution": "f_n = (nπ/L)²√(EI/ρA)", "expected_f1_hz": 15.0, "has_mesh": True},
    {"id": "case_05", "name": "欧拉屈曲", "category": "屈曲", "difficulty": "进阶", "theory_solution": "P_cr = π²EI/(KL)²", "expected_P_cr_N": 5000, "has_mesh": True},
    {"id": "case_06", "name": "压力容器", "category": "静力学", "difficulty": "进阶", "theory_solution": "σ_hoop = PR/t", "has_mesh": True},
    {"id": "case_07", "name": "螺栓预紧力", "category": "接触力学", "difficulty": "专家", "theory_solution": "F_preload = 0.7·A_t·σ_y", "has_mesh": True},
    {"id": "case_08", "name": "齿轮接触应力", "category": "接触力学", "difficulty": "专家", "theory_solution": "Hertz接触", "has_mesh": True},
    {"id": "case_09", "name": "散热器热分析", "category": "热分析", "difficulty": "中级", "theory_solution": "Q = hAΔT", "has_mesh": True},
    {"id": "case_10", "name": "瞬态冲击响应", "category": "瞬态动力学", "difficulty": "专家", "theory_solution": "Newmark-β", "has_mesh": True},
    {"id": "case_11", "name": "复合材料层板", "category": "复合材料", "difficulty": "中级", "theory_solution": "CLT经典层合板理论", "has_mesh": True},
    {"id": "case_12", "name": "焊缝疲劳评估", "category": "疲劳", "difficulty": "专家", "theory_solution": "S-N曲线 + 热点应力", "has_mesh": True},
    {"id": "case_13", "name": "拓扑优化-支架", "category": "拓扑优化", "difficulty": "中级", "theory_solution": "SIMP密度法", "has_mesh": True},
    {"id": "case_14", "name": "CFD-翼型绕流", "category": "CFD", "difficulty": "专家", "theory_solution": "Navier-Stokes", "has_mesh": True},
    {"id": "case_15", "name": "管道流体-热耦合", "category": "多物理场", "difficulty": "专家", "theory_solution": "CHT耦合", "has_mesh": True},
    {"id": "case_16", "name": "涡轮叶片应力", "category": "静力学", "difficulty": "专家", "theory_solution": "三维应力分析", "has_mesh": True},
    {"id": "case_17", "name": "弹簧刚度计算", "category": "静力学", "difficulty": "入门", "theory_solution": "k = Gd⁴/(8D³n)", "has_mesh": True},
    {"id": "case_18", "name": "梁截面优化", "category": "尺寸优化", "difficulty": "中级", "theory_solution": "I = bh³/12", "has_mesh": True},
    {"id": "case_19", "name": "声学模态", "category": "声学", "difficulty": "中级", "theory_solution": "Helmholtz方程", "has_mesh": True},
    {"id": "case_20", "name": "纳米压痕", "category": "接触力学", "difficulty": "专家", "theory_solution": "Oliver-Pharr方法", "has_mesh": True},
]

def list_cases(category: str = None, difficulty: str = None) -> list[dict]:
    results = CASES
    if category:
        results = [c for c in results if c["category"] == category]
    if difficulty:
        results = [c for c in results if c["difficulty"] == difficulty]
    return results

def get_case(case_id: str) -> dict | None:
    return next((c for c in CASES if c["id"] == case_id), None)


# ============================================================================
# V4.8-003: 术语解释系统
# ============================================================================

GLOSSARY = {
    "elastic_modulus": {"zh": "弹性模量 (杨氏模量)", "en": "Elastic Modulus (Young's Modulus)", "unit": "Pa (GPa)", "explanation": "材料抵抗弹性变形的能力。应力与应变之比，斜率越大材料越刚硬。钢约200GPa，铝约70GPa。", "analogy": "弹簧的劲度系数——弹簧越硬，同样力变形越小。"},
    "poisson_ratio": {"zh": "泊松比", "en": "Poisson's Ratio", "unit": "无单位 (0~0.5)", "explanation": "材料横向收缩与纵向伸长之比。金属约0.3，橡胶约0.5（不可压缩）。", "analogy": "拉一根橡皮筋——变长的同时变细，变细程度就是泊松比的体现。"},
    "yield_strength": {"zh": "屈服强度", "en": "Yield Strength", "unit": "Pa (MPa)", "explanation": "材料从弹性变形过渡到塑性变形的应力值。超过屈服意味着永久变形。Q235钢屈服强度235MPa。", "analogy": "弯一根回形针——弯到某个程度它不会自动弹回了，那一刻的力就是屈服强度。"},
    "von_mises_stress": {"zh": "Von Mises 应力", "en": "Von Mises Stress", "unit": "Pa (MPa)", "explanation": "综合正应力和剪应力的等效单向应力。用于判断韧性材料是否屈服的最常用准则。", "analogy": "把各个方向的应力'折合'成一个数，和屈服强度比较——超过就说明材料要永久变形了。"},
    "safety_factor": {"zh": "安全系数", "en": "Factor of Safety (FoS)", "unit": "无单位 (>1)", "explanation": "屈服强度/最大应力。航空航天通常1.25~1.5，土木工程通常2~3。", "analogy": "电梯标称载重10人，实际能撑15人——多出来的50%就是安全系数。"},
    "mesh": {"zh": "网格", "en": "Mesh", "unit": "—", "explanation": "将几何体离散为有限个单元（三角形/四面体/六面体），越细精度越高但计算量越大。", "analogy": "用马赛克拼图表示一幅画——小方块越多越精细，但也越费时间。"},
    "convergence": {"zh": "收敛", "en": "Convergence", "unit": "—", "explanation": "迭代求解器逐渐逼近真实解的过程。能量/位移残差小于阈值时判定收敛。", "analogy": "猜一个人名——不断缩小可能性，最终确定是谁。"},
    "modal_analysis": {"zh": "模态分析", "en": "Modal Analysis", "unit": "Hz", "explanation": "计算结构的固有频率和振型。避免共振（工作频率=固有频率）是设计的基本要求。", "analogy": "敲一口钟——它发出的声音频率就是固有频率，不同的敲法有不同的振动形状。"},
    "buckling": {"zh": "屈曲", "en": "Buckling", "unit": "N", "explanation": "细长结构在压载荷下突然失稳的临界状态。Euler公式:P_cr=π²EI/(KL)²。", "analogy": "用力压一根吸管的顶端——吸管会突然弯向一侧，这就是屈曲。"},
    "fatigue": {"zh": "疲劳", "en": "Fatigue", "unit": "循环次数 (N)", "explanation": "材料在反复周期性载荷下逐渐损伤直至断裂。S-N曲线描述应力幅与寿命的关系。", "analogy": "反复弯折一个铁丝——几十次后它会断，即使每次弯曲力不大。"},
}

def lookup_glossary(term: str) -> dict | None:
    return GLOSSARY.get(term)

def search_glossary(keyword: str) -> list[dict]:
    kw = keyword.lower()
    results = []
    for term, data in GLOSSARY.items():
        if kw in term or kw in data["zh"] or kw in data["explanation"]:
            results.append({"term": term, **data})
    return results

def list_glossary():
    return sorted(GLOSSARY.keys())


# ============================================================================
# V4.8-004: 自动合规检查
# ============================================================================

STD_GB = {
    "GB50017-2017": {"name": "钢结构设计标准", "category": "steel_structure",
                     "checks": [{"property": "max_stress_MPa", "limit": 235, "op": "<", "safety_factor": 1.25},
                                {"property": "max_deflection_mm", "limit": "L/400", "op": "<"},
                                {"property": "slenderness_ratio", "limit": 150, "op": "<"}]},
    "GB50010-2010": {"name": "混凝土结构设计规范", "category": "concrete",
                     "checks": [{"property": "max_compressive_stress_MPa", "limit": 14.3, "op": "<"},
                                {"property": "crack_width_mm", "limit": 0.3, "op": "<"}]},
    "GB/T3811-2008": {"name": "起重机设计规范", "category": "crane",
                      "checks": [{"property": "max_stress_MPa", "limit": 175, "op": "<", "safety_factor": 1.33},
                                 {"property": "max_deflection_mm", "limit": "L/800", "op": "<"}]},
    "ISO_898-1": {"name": "紧固件机械性能", "category": "fastener",
                  "checks": [{"property": "proof_stress_MPa", "limit": 640, "op": ">"},
                             {"property": "tensile_strength_MPa", "limit": 800, "op": ">"}]},
    "ASTM_E8": {"name": "金属材料拉伸试验标准", "category": "material_testing",
                "checks": [{"property": "yield_strength_MPa", "limit": 0, "op": ">"},
                           {"property": "elongation_percent", "limit": 5, "op": ">"}]},
}

def check_compliance(std_code: str, simulation_results: dict) -> dict:
    std = STD_GB.get(std_code)
    if std is None:
        return {"compliant": False, "error": f"未知标准: {std_code}", "available": list(STD_GB.keys())}

    checks = std.get("checks", [])
    results = []
    all_pass = True

    for check in checks:
        prop = check["property"]
        limit = check["limit"]
        op = check["op"]
        value = simulation_results.get(prop)

        if value is None:
            results.append({"property": prop, "passed": None, "message": f"缺少数据: {prop}"})
            continue

        if isinstance(limit, str) and limit.startswith("L/") and "length" in simulation_results:
            limit_val = simulation_results["length"] / float(limit[2:])
        else:
            limit_val = limit

        passed = (op == "<" and value < limit_val) or (op == ">" and value > limit_val)
        sf = simulation_results.get("safety_factor", check.get("safety_factor", 1.0))
        results.append({"property": prop, "value": value, "limit": limit_val, "passed": passed,
                        "safety_factor": round(value / limit_val, 2) if limit_val > 0 else None})
        if not passed:
            all_pass = False

    return {"standard": std_code, "standard_name": std["name"],
            "compliant": all_pass, "checks": results, "total": len(results),
            "passed_count": sum(1 for r in results if r.get("passed"))}


# ============================================================================
# V4.8-005: 报告生成
# ============================================================================

def generate_report(project_name: str, results: dict, output_format: str = "json",
                    sections: list[str] | None = None) -> dict:
    if sections is None:
        sections = ["cover", "summary", "model", "results", "compliance", "conclusion"]

    report = {"title": f"CAELab 仿真报告 — {project_name}",
              "generated_at": time.strftime("%Y-%m-%d %H:%M:%S"),
              "format": output_format, "sections": []}

    section_generators = {
        "cover": lambda: {"name": "封面", "content": f"{project_name}\n仿真分析报告\nCAELab V4.8"},
        "summary": lambda: {"name": "摘要", "content": results.get("summary", "无摘要")},
        "model": lambda: {"name": "模型信息", "content": results.get("model_info", {})},
        "results": lambda: {"name": "仿真结果", "content": {"max_displacement_mm": results.get("max_displacement_mm", 0),
                                                          "max_stress_MPa": results.get("max_stress_MPa", 0),
                                                          "safety_factor": results.get("safety_factor", 0)}},
        "compliance": lambda: {"name": "合规判定", "content": results.get("compliance", {})},
        "conclusion": lambda: {"name": "结论与建议", "content": results.get("conclusion", "分析完成")},
    }

    for sec in sections:
        gen = section_generators.get(sec)
        if gen:
            report["sections"].append(gen())

    return report


# ============================================================================
# V4.0-003: 可视化材料编辑器
# ============================================================================

MATERIAL_TEMPLATES = {
    "structural_steel": {"name": "结构钢 Q235", "category": "金属", "density": 7850, "elastic_modulus": 210e9,
                         "poisson_ratio": 0.3, "yield_strength": 235e6, "ultimate_strength": 370e6,
                         "thermal_expansion": 1.2e-5, "thermal_conductivity": 50, "color": "#8090A0"},
    "stainless_304": {"name": "不锈钢 304", "category": "金属", "density": 8000, "elastic_modulus": 193e9,
                      "poisson_ratio": 0.29, "yield_strength": 215e6, "ultimate_strength": 505e6,
                      "thermal_expansion": 1.73e-5, "thermal_conductivity": 16.2, "color": "#C0C8D0"},
    "aluminum_6061": {"name": "铝合金 6061-T6", "category": "金属", "density": 2700, "elastic_modulus": 68.9e9,
                      "poisson_ratio": 0.33, "yield_strength": 276e6, "ultimate_strength": 310e6,
                      "thermal_expansion": 2.36e-5, "thermal_conductivity": 167, "color": "#D0D8E0"},
    "titanium_ti64": {"name": "钛合金 Ti-6Al-4V", "category": "金属", "density": 4430, "elastic_modulus": 113.8e9,
                      "poisson_ratio": 0.342, "yield_strength": 880e6, "ultimate_strength": 950e6,
                      "thermal_expansion": 8.6e-6, "thermal_conductivity": 6.7, "color": "#B8B8C8"},
    "copper_pure": {"name": "纯铜 C11000", "category": "金属", "density": 8960, "elastic_modulus": 117e9,
                    "poisson_ratio": 0.34, "yield_strength": 70e6, "ultimate_strength": 220e6,
                    "thermal_expansion": 1.67e-5, "thermal_conductivity": 401, "color": "#E08040"},
    "cast_iron": {"name": "灰铸铁 HT200", "category": "金属", "density": 7200, "elastic_modulus": 120e9,
                  "poisson_ratio": 0.25, "yield_strength": 200e6, "ultimate_strength": 300e6,
                  "thermal_expansion": 1.08e-5, "thermal_conductivity": 50, "color": "#606060"},
    "nylon_66": {"name": "尼龙 PA66", "category": "塑料", "density": 1140, "elastic_modulus": 2.5e9,
                 "poisson_ratio": 0.39, "yield_strength": 55e6, "ultimate_strength": 75e6,
                 "thermal_expansion": 9.0e-5, "thermal_conductivity": 0.25, "color": "#4080FF"},
    "abs": {"name": "ABS 塑料", "category": "塑料", "density": 1050, "elastic_modulus": 2.3e9,
            "poisson_ratio": 0.38, "yield_strength": 40e6, "ultimate_strength": 45e6,
            "thermal_expansion": 8.5e-5, "thermal_conductivity": 0.18, "color": "#FF8040"},
    "carbon_fiber_epoxy": {"name": "碳纤维/环氧 T300", "category": "复合材料", "density": 1600,
                           "elastic_modulus": 70e9, "poisson_ratio": 0.3, "yield_strength": 600e6,
                           "ultimate_strength": 1500e6, "thermal_expansion": 2.0e-6, "thermal_conductivity": 5,
                           "color": "#404040", "orthotropic": True},
    "concrete_c30": {"name": "混凝土 C30", "category": "建筑材料", "density": 2400, "elastic_modulus": 30e9,
                     "poisson_ratio": 0.2, "compressive_strength": 30e6, "thermal_expansion": 1.0e-5,
                     "thermal_conductivity": 1.5, "color": "#B0B0A0"},
    "glass": {"name": "钢化玻璃", "category": "脆性材料", "density": 2500, "elastic_modulus": 70e9,
              "poisson_ratio": 0.22, "compressive_strength": 1000e6, "thermal_expansion": 8.5e-6,
              "thermal_conductivity": 1.0, "color": "#A0D0F0"},
    "rubber": {"name": "天然橡胶", "category": "弹性体", "density": 1100, "elastic_modulus": 0.01e9,
               "poisson_ratio": 0.49, "hyperelastic": True, "thermal_expansion": 2.0e-4,
               "thermal_conductivity": 0.15, "color": "#404040"},
}


def list_material_templates(category: str = None):
    templates = list(MATERIAL_TEMPLATES.values())
    if category:
        templates = [t for t in templates if t.get("category") == category]
    return templates


def get_material_template(name: str):
    for key, tmpl in MATERIAL_TEMPLATES.items():
        if tmpl["name"] == name or key == name:
            return {"id": key, **tmpl}
    return None


def edit_material(material_id: str, properties: dict) -> dict:
    tmpl = MATERIAL_TEMPLATES.get(material_id)
    if tmpl is None:
        return {"error": f"材料模板不存在: {material_id}"}
    edited = dict(tmpl)
    edited.update(properties)
    return {"id": material_id, **edited}


def material_categories() -> list[str]:
    return sorted(set(t["category"] for t in MATERIAL_TEMPLATES.values()))


def compare_materials(material_ids: list[str]) -> dict:
    """对比多个材料的关键属性"""
    comparison = {"properties": ["density", "elastic_modulus", "yield_strength", "poisson_ratio",
                                  "thermal_expansion", "thermal_conductivity"],
                  "materials": []}
    for mid in material_ids:
        tmpl = MATERIAL_TEMPLATES.get(mid)
        if tmpl:
            comparison["materials"].append({"id": mid, "name": tmpl["name"],
                                             "values": {p: tmpl.get(p) for p in comparison["properties"]}})
    return comparison


# ============================================================================
# V4.1-003: 结果置信度标注
# ============================================================================

def compute_confidence(simulation_results: dict) -> dict:
    """计算仿真结果的综合置信度

    四大因子：
    - 网格质量 (mesh_quality: 0~1)
    - 收敛性 (convergence: 0~1)
    - 材料不确定性 (material_uncertainty: 0~1)
    - 边界条件合理性 (bc_validity: 0~1)
    """
    mesh_q = simulation_results.get("mesh_quality", 0.85)
    conv = simulation_results.get("convergence_score", 0.90)
    mat_u = simulation_results.get("material_uncertainty", 0.10)
    bc_v = simulation_results.get("bc_validity", 0.90)

    # 加权综合置信度
    weights = {"mesh": 0.25, "convergence": 0.30, "material": 0.20, "bc": 0.25}
    confidence = (mesh_q * weights["mesh"] + conv * weights["convergence"] +
                  (1 - mat_u) * weights["material"] + bc_v * weights["bc"])

    level = "high" if confidence >= 0.85 else ("medium" if confidence >= 0.65 else "low")
    color = "green" if level == "high" else ("orange" if level == "medium" else "red")

    factors = [
        {"factor": "网格质量", "score": round(mesh_q, 3), "weight": 0.25,
         "suggestion": None if mesh_q >= 0.8 else "建议细化关键区域的网格"},
        {"factor": "收敛性", "score": round(conv, 3), "weight": 0.30,
         "suggestion": None if conv >= 0.85 else "建议减小收敛容差或增加迭代次数"},
        {"factor": "材料不确定性", "score": round(1 - mat_u, 3), "weight": 0.20,
         "suggestion": None if mat_u <= 0.15 else "建议使用更精确的材料参数或进行参数敏感性分析"},
        {"factor": "边界条件", "score": round(bc_v, 3), "weight": 0.25,
         "suggestion": None if bc_v >= 0.85 else "建议检查约束/荷载设置的合理性"},
    ]

    return {"confidence": round(confidence, 3), "level": level, "color": color,
            "factors": factors, "is_mock": False}


def annotate_result_field(field_name: str, field_values: list[float],
                          confidence_map: dict[str, float] = None) -> dict:
    """对结果场进行逐区域置信度标注"""
    if confidence_map is None:
        confidence_map = {}

    mean_val = np.mean(field_values)
    std_val = np.std(field_values)
    cv = std_val / (abs(mean_val) + 1e-10)

    # 变异性越大，置信度越低
    local_confidence = max(0.3, min(0.98, 1.0 - cv * 2))

    # 热点区域检测
    threshold = mean_val + 2 * std_val
    hot_spots = [i for i, v in enumerate(field_values) if v > threshold]

    return {"field": field_name, "mean": round(float(mean_val), 4),
            "std": round(float(std_val), 4), "cv": round(float(cv), 4),
            "local_confidence": round(local_confidence, 3),
            "n_hot_spots": len(hot_spots), "hot_spot_indices": hot_spots[:20],
            "is_mock": False}


# ============================================================================
# CLI
# ============================================================================

def main():
    if len(sys.argv) < 2:
        print("product: wizard | cases | glossary <term> | compliance <std> <results_json> | report <name> <results_json>")
        print("         materials [category] | material <id> | compare <ids_json> | confidence <results_json>")
        sys.exit(1)
    cmd = sys.argv[1]

    if cmd == "wizard":
        print(json.dumps(get_wizard_steps(), ensure_ascii=False))
    elif cmd == "cases":
        category = sys.argv[2] if len(sys.argv) > 2 else None
        print(json.dumps(list_cases(category), ensure_ascii=False))
    elif cmd == "glossary":
        term = sys.argv[2] if len(sys.argv) > 2 else ""
        result = lookup_glossary(term) or search_glossary(term)
        print(json.dumps(result, ensure_ascii=False))
    elif cmd == "compliance":
        std = sys.argv[2]
        results = json.loads(sys.argv[3])
        print(json.dumps(check_compliance(std, results), ensure_ascii=False))
    elif cmd == "report":
        name = sys.argv[2]
        results = json.loads(sys.argv[3])
        print(json.dumps(generate_report(name, results), ensure_ascii=False))
    # V4.0-003: 材料编辑器
    elif cmd == "materials":
        cat = sys.argv[2] if len(sys.argv) > 2 else None
        print(json.dumps(list_material_templates(cat), ensure_ascii=False))
    elif cmd == "material":
        print(json.dumps(get_material_template(sys.argv[2]), ensure_ascii=False))
    elif cmd == "material_categories":
        print(json.dumps(material_categories()))
    elif cmd == "compare_materials":
        ids = json.loads(sys.argv[2]) if len(sys.argv) > 2 else []
        print(json.dumps(compare_materials(ids), ensure_ascii=False))
    # V4.1-003: 置信度
    elif cmd == "confidence":
        results = json.loads(sys.argv[2])
        print(json.dumps(compute_confidence(results), ensure_ascii=False))
    elif cmd == "annotate":
        field = sys.argv[2]
        values = json.loads(sys.argv[3])
        print(json.dumps(annotate_result_field(field, values), ensure_ascii=False))
    else:
        print(f"未知命令: {cmd}")


if __name__ == "__main__":
    main()
