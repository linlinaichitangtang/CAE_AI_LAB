"""
CAELab Product 模块冒烟测试 (V4.8)

验证产品化模块的基本可用性：
  - 仿真向导 (7 步)
  - 案例库 (20 模板)
  - 术语表 (10 术语)
  - 合规检查 (5 标准)
  - 报告生成
  - 材料编辑器 (12 模板)
  - 置信度标注 (4 因子)
"""

import importlib

import pytest


class TestProductImports:
    """所有 Product 子模块均可导入"""

    def test_module_importable(self):
        mod = importlib.import_module("caelab.product")
        assert mod is not None

    def test_tools_importable(self):
        mod = importlib.import_module("caelab.product.tools")
        assert mod is not None


class TestWizard:
    """仿真向导 (V4.8-001)"""

    def test_get_wizard_steps(self):
        from caelab.product.tools import get_wizard_steps

        steps = get_wizard_steps()
        assert isinstance(steps, list)
        assert len(steps) > 0
        # 每个步骤应有 id/name/description
        step = steps[0]
        assert isinstance(step, dict)
        assert "id" in step or "name" in step

    def test_validate_wizard_step(self):
        from caelab.product.tools import validate_wizard_step

        result = validate_wizard_step(0, "thermal")
        assert isinstance(result, dict)


class TestCases:
    """案例库 (V4.8-002)"""

    def test_list_cases_all(self):
        from caelab.product.tools import list_cases

        cases = list_cases()
        assert isinstance(cases, list)
        assert len(cases) > 0
        # 每个案例至少要有 id 和 name
        case = cases[0]
        assert isinstance(case, dict)

    def test_list_cases_by_category(self):
        from caelab.product.tools import list_cases

        cases = list_cases(category="thermal")
        assert isinstance(cases, list)

    def test_get_case(self):
        from caelab.product.tools import list_cases, get_case

        cases = list_cases()
        if cases:
            first_case = cases[0]
            case_id = first_case.get("id", first_case.get("name", ""))
            if case_id:
                detail = get_case(case_id)
                assert isinstance(detail, dict)


class TestGlossary:
    """术语表 (V4.8-003)"""

    def test_lookup_glossary(self):
        from caelab.product.tools import lookup_glossary

        result = lookup_glossary("FEM")
        assert result is None or isinstance(result, dict)

    def test_search_glossary(self):
        from caelab.product.tools import search_glossary

        results = search_glossary("stress")
        assert isinstance(results, list)

    def test_list_glossary(self):
        from caelab.product.tools import list_glossary

        terms = list_glossary()
        assert isinstance(terms, list)
        assert len(terms) > 0


class TestCompliance:
    """合规检查 (V4.8-004)"""

    def test_check_compliance(self):
        from caelab.product.tools import check_compliance

        result = check_compliance("ASME_VIII", {
            "max_stress": 250e6,
            "yield_strength": 300e6,
        })
        assert isinstance(result, dict)
        assert "passed" in result or "status" in result or "compliant" in result

    def test_check_compliance_iso(self):
        from caelab.product.tools import check_compliance

        result = check_compliance("ISO_19902", {
            "max_displacement": 0.01,
            "allowable": 0.015,
        })
        assert isinstance(result, dict)


class TestReport:
    """报告生成 (V4.8-005)"""

    def test_generate_report(self):
        from caelab.product.tools import generate_report

        result = generate_report("Test Project", {
            "solver": "calculix",
            "max_stress": 200e6,
            "max_displacement": 0.001,
        })
        assert isinstance(result, dict)


class TestMaterialEditor:
    """材料编辑器 (V4.0-003)"""

    def test_list_material_templates(self):
        from caelab.product.tools import list_material_templates

        templates = list_material_templates()
        assert isinstance(templates, list)
        assert len(templates) > 0

    def test_list_material_templates_by_category(self):
        from caelab.product.tools import list_material_templates

        templates = list_material_templates(category="metal")
        assert isinstance(templates, list)

    def test_get_material_template(self):
        from caelab.product.tools import list_material_templates, get_material_template

        templates = list_material_templates()
        if templates:
            first = templates[0]
            mat_id = first.get("id", first.get("name", ""))
            if mat_id:
                mat = get_material_template(mat_id)
                assert isinstance(mat, dict)

    def test_material_categories(self):
        from caelab.product.tools import material_categories

        cats = material_categories()
        assert isinstance(cats, list)
        assert len(cats) > 0

    def test_compare_materials(self):
        from caelab.product.tools import list_material_templates, compare_materials

        templates = list_material_templates()
        if len(templates) >= 2:
            ids = [t.get("id", t.get("name", "")) for t in templates[:2]]
            result = compare_materials(ids)
            assert isinstance(result, dict)

    def test_edit_material(self):
        from caelab.product.tools import edit_material

        result = edit_material("test_mat", {"youngs_modulus": 210e9})
        assert isinstance(result, dict)


class TestConfidence:
    """置信度标注 (V4.1-003)"""

    def test_compute_confidence(self):
        from caelab.product.tools import compute_confidence

        result = compute_confidence({
            "max_stress": 250e6,
            "max_displacement": 0.001,
            "mesh_quality": 0.95,
            "solver": "calculix",
        })
        assert isinstance(result, dict)
        assert "confidence" in result or "score" in result or "level" in result

    def test_annotate_result_field(self):
        from caelab.product.tools import annotate_result_field

        result = annotate_result_field("max_stress", [200e6, 250e6, 300e6, 280e6, 260e6])
        assert isinstance(result, dict)
