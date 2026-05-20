"""
CAELab Thermo 模块冒烟测试 (V4.7)

验证材料热力学与相图模块的基本可用性：
  - 模块导入完整性
  - CALPHAD: SGTE 纯元素 Gibbs 自由能（11 种元素）
  - 二元相图计算
  - Scheil 凝固模拟
  - 扩散偶模拟
  - JMAK 动力学
  - 缺陷平衡浓度
  - 热物理属性查询
"""

import importlib

import pytest


class TestThermoImports:
    """所有 Thermo 子模块均可导入"""

    @pytest.mark.parametrize(
        "module_name",
        [
            "caelab.thermo",
            "caelab.thermo.calphad",
            "caelab.thermo.phase_diagram",
            "caelab.thermo.scheil",
            "caelab.thermo.diffusion",
            "caelab.thermo.precipitation",
            "caelab.thermo.kinetics",
            "caelab.thermo.defects",
            "caelab.thermo.thermophysical",
        ],
    )
    def test_module_importable(self, module_name: str):
        mod = importlib.import_module(module_name)
        assert mod is not None


class TestCalphad:
    """CALPHAD 纯元素 Gibbs 自由能 (SGTE) — 内置 11 种元素"""

    def test_list_elements(self):
        from caelab.thermo.calphad import list_elements

        elements = list_elements()
        assert isinstance(elements, list)
        assert len(elements) >= 10  # 内置 11 种: Al,Fe,Ni,Ti,Cu,Mg,C,Si,Mn,Cr,V
        assert "Fe" in elements
        assert "Ni" in elements
        assert "Al" in elements

    def test_list_phases(self):
        from caelab.thermo.calphad import list_phases

        phases = list_phases("Fe")
        assert isinstance(phases, list)
        assert len(phases) > 0  # Fe 有 BCC_A2, FCC_A1, LIQUID

    def test_gibbs_pure_fe(self):
        from caelab.thermo.calphad import gibbs_pure

        G = gibbs_pure("Fe", "BCC_A2", 1000.0)
        assert isinstance(G, float)
        # Gibbs energy 在高温下通常为负（相对于参考态）
        assert isinstance(G, float)

    def test_gibbs_pure_al(self):
        from caelab.thermo.calphad import gibbs_pure

        G = gibbs_pure("Al", "FCC_A1", 300.0)
        assert isinstance(G, float)

    def test_gibbs_total_feni(self):
        from caelab.thermo.calphad import gibbs_total

        G = gibbs_total(["Fe", "Ni"], "FCC_A1", [0.5, 0.5], 1000.0)
        assert isinstance(G, float)

    def test_gibbs_ideal_mixing(self):
        from caelab.thermo.calphad import gibbs_ideal_mixing

        G = gibbs_ideal_mixing([0.3, 0.7], 800.0)
        assert isinstance(G, float)
        # 理想混合熵贡献为负（稳定化）
        assert G < 0

    def test_gibbs_excess_fecr(self):
        from caelab.thermo.calphad import gibbs_excess

        G = gibbs_excess(["Fe", "Cr"], "BCC_A2", [0.2, 0.8], 1200.0)
        assert isinstance(G, float)

    def test_equilibrium_feni(self):
        from caelab.thermo.calphad import equilibrium

        result = equilibrium(["Fe", "Ni"], ["FCC_A1", "LIQUID"], 1500.0)
        assert isinstance(result, dict)
        assert "stable_phase" in result

    def test_get_gibbs_curve(self):
        from caelab.thermo.calphad import get_gibbs_curve

        curve = get_gibbs_curve(["Fe", "Ni"], "FCC_A1", 1000.0, n_points=20)
        assert isinstance(curve, dict)
        assert "x" in curve
        assert "gibbs" in curve


class TestPhaseDiagram:
    """相图计算 (V4.7-002)"""

    def test_binary_phase_diagram(self):
        from caelab.thermo.phase_diagram import binary_phase_diagram

        result = binary_phase_diagram(
            "Fe", "Ni",
            phases=["FCC_A1", "LIQUID"],
            t_range=(300.0, 2000.0)
        )
        assert isinstance(result, dict)
        assert "phase_boundaries" in result

    def test_isothermal_section(self):
        from caelab.thermo.phase_diagram import isothermal_section

        result = isothermal_section(
            "Fe", "Ni",
            phases=["FCC_A1", "LIQUID"],
            T=1000.0
        )
        assert isinstance(result, dict)
        assert "stable_phases" in result


class TestScheil:
    """Scheil 凝固模拟 (V4.7-003)"""

    def test_scheil_solidification(self):
        from caelab.thermo.scheil import scheil_solidification

        result = scheil_solidification(
            components=["Al", "Cu"],
            composition=[0.95, 0.05],
            phases=["FCC_A1", "LIQUID"],
            T_liquidus=920.0,
            T_solidus=820.0,
            k_default=0.14
        )
        assert isinstance(result, dict)
        assert "fs_curve" in result
        assert "final_fs" in result


class TestDiffusion:
    """扩散偶模拟 (V4.7-004)"""

    def test_diffusion_couple(self):
        from caelab.thermo.diffusion import diffusion_couple

        result = diffusion_couple(
            length=1e-3, n_points=100, time_total=3600,
            D=1e-12, c_left=0.05, c_right=0.0
        )
        assert isinstance(result, dict)
        assert "profiles" in result
        assert "diffusion_distance_m" in result


class TestPrecipitation:
    """KWN 析出模拟 (V4.7-005)"""

    def test_kwn_precipitation(self):
        from caelab.thermo.precipitation import kwn_precipitation

        result = kwn_precipitation(T=500.0, c0=0.05, ceq=0.01, t_total=3600.0)
        assert isinstance(result, dict)


class TestKinetics:
    """JMAK 相变动力学 (V4.7-007)"""

    def test_jmak_isothermal(self):
        from caelab.thermo.kinetics import jmak_isothermal

        result = jmak_isothermal(k=0.001, n=1.5)
        assert isinstance(result, dict)
        assert "time_s" in result
        assert "fraction" in result
        assert "t_start_s" in result

    def test_fit_avrami(self):
        from caelab.thermo.kinetics import fit_avrami

        t = [10.0, 20.0, 50.0, 100.0, 200.0, 500.0, 1000.0]
        f = [0.02, 0.05, 0.25, 0.55, 0.85, 0.97, 0.995]
        result = fit_avrami(t, f)
        assert isinstance(result, dict)
        assert "n_avrami" in result
        assert "k" in result


class TestDefects:
    """缺陷平衡浓度 (V4.7-006)"""

    def test_vacancy_concentration(self):
        from caelab.thermo.defects import vacancy_concentration

        result = vacancy_concentration("Al", T_range=(300.0, 900.0))
        assert isinstance(result, dict)


class TestThermophysical:
    """热物理属性查询 (V4.7-008)"""

    def test_list_materials(self):
        from caelab.thermo.thermophysical import list_materials

        materials = list_materials()
        assert isinstance(materials, list)
        assert len(materials) > 0

    def test_get_all_properties(self):
        from caelab.thermo.thermophysical import get_all_properties

        result = get_all_properties("Cu", 300.0)
        assert isinstance(result, dict)
