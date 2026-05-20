"""
CAELab DFT 与多尺度模块冒烟测试 (V4.5)

验证 DFT 与多尺度模块的基本可用性（无需 VASP/LAMMPS）：
  - DFT 工作流 (DFTWorkflow / create_workflow)
  - OpenKIM 原子间势 (search_potentials / verify_potential)
  - 微观结构模拟 (PottsModel / PhaseFieldSim / CPFEMSimulator)
  - FE² 多尺度 (RVEComputer / FE2Solver)
  - 不确定性量化 (check_kpoint_convergence / bayesian_uq / parameter_sensitivity)
"""

import importlib
import os
import tempfile

import pytest

# 临时 work_dir（避免 macOS 沙箱 /tmp 权限问题）
_WORK_DIR = os.path.join(os.environ.get("TMPDIR", tempfile.gettempdir()), "caelab_dft_test")
_WORK_DIR2 = os.path.join(os.environ.get("TMPDIR", tempfile.gettempdir()), "caelab_dft_test2")


class TestDftImports:
    """所有 DFT 子模块均可导入"""

    @pytest.mark.parametrize(
        "module_name",
        [
            "caelab.dft",
            "caelab.dft.workflow",
            "caelab.dft.openkim",
            "caelab.dft.microstructure",
            "caelab.dft.fe2",
            "caelab.dft.uq",
        ],
    )
    def test_module_importable(self, module_name: str):
        mod = importlib.import_module(module_name)
        assert mod is not None


class TestDFTWorkflow:
    """DFT 工作流 (V4.5-001)"""

    def test_dft_workflow_class(self):
        from caelab.dft.workflow import DFTWorkflow

        wf = DFTWorkflow(code="vasp", work_dir=_WORK_DIR)
        assert wf is not None
        assert wf.code == "vasp"

    def test_list_workflows(self):
        from caelab.dft.workflow import list_workflows

        workflows = list_workflows()
        assert isinstance(workflows, list)

    def test_create_workflow(self):
        from caelab.dft.workflow import create_workflow

        wf = create_workflow(code="vasp", work_dir=_WORK_DIR2)
        assert wf is not None
        assert hasattr(wf, "run_full_chain")


class TestOpenKIM:
    """OpenKIM 原子间势 (V4.5-002)"""

    def test_search_potentials(self):
        from caelab.dft.openkim import search_potentials

        results = search_potentials(elements=["Al"])
        assert isinstance(results, list)

    def test_get_potential_detail(self):
        from caelab.dft.openkim import get_potential_detail

        result = get_potential_detail("EAM_Dynamo_ErcolessiAdams_1994_Al__MO_123629422045_005")
        assert result is None or isinstance(result, dict)

    def test_list_model_types(self):
        from caelab.dft.openkim import list_model_types

        types = list_model_types()
        assert isinstance(types, list)
        assert len(types) >= 1

    def test_verify_potential(self):
        from caelab.dft.openkim import verify_potential

        result = verify_potential(
            "EAM_Dynamo_ErcolessiAdams_1994_Al__MO_123629422045_005",
            ["Al"],
        )
        assert isinstance(result, dict)

    def test_generate_lammps_script(self):
        from caelab.dft.openkim import generate_lammps_script

        result = generate_lammps_script(
            "EAM_Dynamo_ErcolessiAdams_1994_Al__MO_123629422045_005",
            {
                "positions": [[0, 0, 0], [0.5, 0.5, 0.5]],
                "numbers": [13, 13],
                "cell": [[4.05, 0, 0], [0, 4.05, 0], [0, 0, 4.05]],
                "pbc": [True, True, True],
            },
        )
        assert isinstance(result, dict)


class TestMicrostructure:
    """微观结构模拟 (V4.5-003)"""

    def test_potts_model(self):
        from caelab.dft.microstructure import PottsModel

        model = PottsModel(nx=32, ny=32, n_grains=10)
        assert model is not None
        assert model.nx == 32

    def test_potts_run(self):
        from caelab.dft.microstructure import PottsModel

        model = PottsModel(nx=16, ny=16, n_grains=5)
        result = model.run(temperature=0.5, n_steps=50)
        assert isinstance(result, dict)
        assert "final_grain_stats" in result

    def test_phase_field(self):
        from caelab.dft.microstructure import PhaseFieldSim

        sim = PhaseFieldSim(nx=32, ny=32, n_phases=2)
        assert sim is not None

    def test_cpfem_simulator(self):
        from caelab.dft.microstructure import CPFEMSimulator

        sim = CPFEMSimulator()
        assert sim is not None

    def test_run_mc_pf_cpfem_chain(self):
        from caelab.dft.microstructure import run_mc_pf_cpfem_chain

        result = run_mc_pf_cpfem_chain({
            "mc": {"nx": 16, "ny": 16, "n_grains": 5, "n_steps": 10},
            "pf": {"nx": 16, "ny": 16, "n_steps": 10},
        })
        assert isinstance(result, dict)


class TestFE2:
    """FE² 多尺度 (V4.5-004)"""

    def test_rve_computer(self):
        from caelab.dft.fe2 import RVEComputer

        rve = RVEComputer(rve_id=0, method="md", size=8)
        assert rve is not None
        assert rve.method == "md"

    def test_fe2_solver(self):
        from caelab.dft.fe2 import FE2Solver

        solver = FE2Solver(n_rve=4, method="md", n_workers=2)
        assert solver is not None

    def test_fe2_run(self):
        from caelab.dft.fe2 import FE2Solver

        solver = FE2Solver(n_rve=4, method="mock", n_workers=2)
        result = solver.run(macro_steps=3, max_strain=0.01)
        assert isinstance(result, dict)
        assert "stress_strain_curve" in result


class TestUQ:
    """不确定性量化 (V4.5-005)"""

    def test_check_kpoint_convergence(self):
        from caelab.dft.uq import check_kpoint_convergence

        result = check_kpoint_convergence(
            energies=[-5.0, -5.001, -5.0011, -5.0012],
            kpoint_grids=[(2, 2, 2), (4, 4, 4), (6, 6, 6), (8, 8, 8)],
        )
        assert isinstance(result, dict)
        assert "converged" in result

    def test_bayesian_uq(self):
        from caelab.dft.uq import bayesian_uq

        result = bayesian_uq(
            predictions=[4.04, 4.05, 4.06, 4.05, 4.04],
            reference=4.05,
        )
        assert isinstance(result, dict)
        assert "mean" in result
        assert "confidence_95" in result

    def test_parameter_sensitivity(self):
        from caelab.dft.uq import parameter_sensitivity

        result = parameter_sensitivity(
            param_ranges={"encut": [400, 500, 600], "kpoints": [2, 4, 6]},
            evaluation_fn_result=[4.04, 4.05, 4.06, 4.05, 4.04, 4.055],
        )
        assert isinstance(result, dict)
        assert "sensitivities" in result
