"""
CAELab Python SDK 冒烟测试 (V4.2-001)

在 CI 环境中无需 CAELab 桌面端后端，验证 SDK 的基本可用性：
  - 模块导入完整性
  - 版本号格式
  - 类实例化
  - 材料工厂方法
  - 仿真配置（不触发后端调用）
  - 客户端初始化
"""

import importlib
import re
import sys

import pytest


# ========================================================================
# 模块导入
# ========================================================================

class TestImports:
    """所有 SDK 子模块均可导入"""

    @pytest.mark.parametrize(
        "module_name",
        [
            "caelab",
            "caelab.project",
            "caelab.mesh",
            "caelab.material",
            "caelab.simulation",
            "caelab.result",
            "caelab._client",
        ],
    )
    def test_module_importable(self, module_name: str):
        mod = importlib.import_module(module_name)
        assert mod is not None

    def test_top_level_exports(self):
        import caelab

        for cls_name in ("Project", "Mesh", "Material", "Simulation", "Result"):
            assert hasattr(caelab, cls_name), f"caelab.{cls_name} 未导出"

    def test_version_format(self):
        import caelab

        assert hasattr(caelab, "__version__")
        assert re.match(r"\d+\.\d+\.\d+", caelab.__version__), (
            f"版本号格式异常: {caelab.__version__}"
        )


# ========================================================================
# 类实例化（不触发后端）
# ========================================================================

class TestInstantiation:
    """实例化不依赖后端的类"""

    def test_project_creation(self):
        from caelab import Project

        p = Project("CI Test Project", "用于 CI 冒烟测试")
        assert p.name == "CI Test Project"
        assert p.description == "用于 CI 冒烟测试"
        assert len(p.id) > 0
        assert "Project" in repr(p)

    def test_project_metadata(self):
        from caelab import Project

        p = Project("Meta Test")
        p.set_metadata("solver", "calculix")
        assert p.get_metadata("solver") == "calculix"
        assert p.get_metadata("missing", "default") == "default"

    def test_mesh_creation(self):
        from caelab import Mesh

        m = Mesh("fake-project-id")
        assert m.project_id == "fake-project-id"
        assert "Mesh" in repr(m)

    def test_simulation_creation(self):
        from caelab import Simulation

        s = Simulation("fake-project-id")
        assert s.project_id == "fake-project-id"
        assert s._analysis_type == "static"
        assert "Simulation" in repr(s)

    def test_simulation_set_analysis_type(self):
        from caelab import Simulation

        s = Simulation("proj-1")
        for atype in ("static", "modal", "buckling", "thermal",
                       "frequency_response", "transient", "cfd"):
            s.set_analysis_type(atype)
            assert s._analysis_type == atype

    def test_simulation_invalid_analysis_type(self):
        from caelab import Simulation

        s = Simulation("proj-1")
        with pytest.raises(ValueError, match="Invalid analysis type"):
            s.set_analysis_type("nonexistent")

    def test_simulation_add_bc_and_load(self):
        from caelab import Simulation

        s = Simulation("proj-1")
        s.add_fixed_bc([1, 2, 3])
        s.add_displacement_bc([4, 5], ux=0.0, uy=None, uz=0.01)
        s.add_symmetry_bc("xz")
        s.add_load([10, 11], [0, 0, -1000.0])
        s.add_pressure("top_face", 1e5)
        s.add_gravity(9.81, "z")

        assert len(s._boundary_conditions) == 3
        assert len(s._loads) == 3
        assert s._boundary_conditions[0]["type"] == "fixed"
        assert s._loads[2]["type"] == "gravity"

    def test_result_creation(self):
        from caelab import Result

        data = {
            "id": "test-001",
            "max_displacement": 0.00123,
            "max_stress": 250e6,
            "max_strain": 0.001,
            "total_strain_energy": 42.0,
            "reaction_forces": {"fx": 100.0, "fy": 200.0},
        }
        r = Result(data)
        assert r.max_displacement() == pytest.approx(0.00123)
        assert r.max_stress() == pytest.approx(250e6)
        assert r.max_strain() == pytest.approx(0.001)
        assert r.total_strain_energy() == pytest.approx(42.0)
        assert r.reaction_forces()["fx"] == pytest.approx(100.0)
        assert "Result" in repr(r)

    def test_result_summary(self):
        from caelab import Result

        r = Result({"max_displacement": 1e-3, "max_stress": 200e6})
        text = r.summary()
        assert "Max Displacement" in text
        assert "Max Stress" in text


# ========================================================================
# 材料工厂方法
# ========================================================================

class TestMaterialFactory:
    """材料工厂方法返回正确的预设属性"""

    def test_steel(self):
        from caelab import Material

        m = Material.steel()
        assert m.name == "Steel_Q235"
        props = m.get_properties()
        assert props["youngs_modulus"] == pytest.approx(210e9)
        assert props["poisson_ratio"] == pytest.approx(0.3)
        assert props["density"] == pytest.approx(7850.0)

    def test_aluminum(self):
        from caelab import Material

        m = Material.aluminum()
        assert m.name == "Aluminum_6061T6"
        props = m.get_properties()
        assert props["youngs_modulus"] == pytest.approx(68.9e9)

    def test_titanium(self):
        from caelab import Material

        m = Material.titanium()
        assert m.name == "Titanium_Ti6Al4V"
        props = m.get_properties()
        assert props["yield_strength"] == pytest.approx(880e6)

    def test_custom_material(self):
        from caelab import Material

        m = Material("CustomAlloy", youngs_modulus=150e9, poisson_ratio=0.28)
        props = m.get_properties()
        assert props["youngs_modulus"] == pytest.approx(150e9)
        assert props["poisson_ratio"] == pytest.approx(0.28)
        assert "Material" in repr(m)

    def test_set_elastic(self):
        from caelab import Material

        m = Material("TestMat")
        m.set_elastic(200e9, 0.3)
        props = m.get_properties()
        assert props["youngs_modulus"] == pytest.approx(200e9)
        assert props["poisson_ratio"] == pytest.approx(0.3)

    def test_set_thermal(self):
        from caelab import Material

        m = Material("TestMat")
        m.set_thermal(50.0, 12e-6)
        props = m.get_properties()
        assert props["thermal_conductivity"] == pytest.approx(50.0)
        assert props["thermal_expansion"] == pytest.approx(12e-6)

    def test_set_plastic(self):
        from caelab import Material

        m = Material("TestMat")
        m.set_plastic(300e6, 450e6)
        props = m.get_properties()
        assert props["yield_strength"] == pytest.approx(300e6)
        assert props["ultimate_strength"] == pytest.approx(450e6)

    def test_material_str(self):
        from caelab import Material

        m = Material.steel()
        text = str(m)
        assert "Steel_Q235" in text
        assert "youngs_modulus" in text


# ========================================================================
# 客户端
# ========================================================================

class TestClient:
    """客户端初始化与模式检测"""

    def test_client_creation(self):
        from caelab._client import CAELabClient

        c = CAELabClient()
        assert c._mode in ("cli", "http", "ipc")

    def test_get_client_singleton(self):
        from caelab._client import get_client

        c1 = get_client()
        c2 = get_client()
        assert c1 is c2

    def test_client_call_without_backend(self):
        """无后端时 call 应返回 dict 而非抛异常"""
        from caelab._client import CAELabClient

        c = CAELabClient()
        resp = c.call("nonexistent_command", {})
        assert isinstance(resp, dict)
        assert "success" in resp
        assert resp["success"] is False


# ========================================================================
# Python 版本兼容性
# ========================================================================

class TestEnvironment:
    """CI 环境基本检查"""

    def test_python_version(self):
        assert sys.version_info >= (3, 9), (
            f"Python {sys.version} 低于最低要求 3.9"
        )

    def test_numpy_importable(self):
        import numpy as np

        assert np is not None

    def test_scipy_importable(self):
        import scipy

        assert scipy is not None
