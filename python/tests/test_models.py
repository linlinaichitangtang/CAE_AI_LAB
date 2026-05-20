"""
CAELab AI/ML 模块冒烟测试 (V4.4)

验证 AI/ML 模块的基本可用性（无需 PyTorch）：
  - 模型注册表 (get_model_def / list_all_models)
  - 推理框架 (get_model_status, list_model_status)
  - 训练管理器 (get_default_config, load_training_data, TrainingEngine)
  - 实验追踪器 (create_tracker, WandbTracker, MLflowTracker)
  - 代理模型 (SurrogateMLP, SegmentationModel)
  - 主动学习 (compute_mc_dropout_uncertainty, select_candidates)
"""

import importlib

import pytest


class TestModelsImports:
    """所有 Models 子模块均可导入"""

    @pytest.mark.parametrize(
        "module_name",
        [
            "caelab.models",
            "caelab.models.registry",
            "caelab.models.inference",
            "caelab.models.training",
            "caelab.models.experiment_tracker",
            "caelab.models.surrogate",
            "caelab.models.active_learning",
        ],
    )
    def test_module_importable(self, module_name: str):
        mod = importlib.import_module(module_name)
        assert mod is not None


class TestRegistry:
    """模型注册表 (V4.4-002)"""

    def test_list_all_models(self):
        from caelab.models.registry import list_all_models

        models = list_all_models()
        assert isinstance(models, list)
        assert len(models) > 0

    def test_get_model_def(self):
        from caelab.models.registry import get_model_def

        info = get_model_def("chgnet")
        assert info is not None
        assert hasattr(info, "name")


class TestInference:
    """推理接口 (V4.4-002)"""

    def test_list_model_status(self):
        from caelab.models.inference import list_model_status

        statuses = list_model_status()
        assert isinstance(statuses, list)

    def test_get_model_status(self):
        from caelab.models.inference import get_model_status

        status = get_model_status("chgnet")
        assert isinstance(status, dict)


class TestTraining:
    """训练管理器 (V4.4-003)"""

    def test_get_default_config(self):
        from caelab.models.training import get_default_config

        config = get_default_config("mace")
        assert isinstance(config, dict)

    def test_training_engine_class(self):
        from caelab.models.training import TrainingEngine, MACETrainer

        assert TrainingEngine is not None
        assert MACETrainer is not None

    def test_load_training_data_missing_path(self):
        from caelab.models.training import load_training_data

        # 不存在的路径应该抛出 FileNotFoundError
        with pytest.raises(FileNotFoundError):
            load_training_data("/nonexistent/path")


class TestExperimentTracker:
    """实验追踪 (V4.4-004)"""

    def test_create_tracker_wandb(self):
        from caelab.models.experiment_tracker import create_tracker, list_trackers

        # create_tracker(backend, project=..., name=...)
        tracker = create_tracker("wandb", project="ci_test", name="ci_test_wandb")
        assert tracker is not None

        trackers = list_trackers()
        assert isinstance(trackers, list)

    def test_create_tracker_mlflow(self):
        from caelab.models.experiment_tracker import create_tracker

        tracker = create_tracker("mlflow", project="ci_test", name="ci_test_mlflow")
        assert tracker is not None

    def test_get_tracker(self):
        from caelab.models.experiment_tracker import get_tracker

        tracker = get_tracker("ci_test_wandb")
        # 可能返回 None（未创建）或 ExperimentTracker 对象
        # 如果返回 tracker，应该有 log_metrics 方法
        assert tracker is None or hasattr(tracker, "log_metrics")


class TestSurrogate:
    """代理模型 (V4.4-005)"""

    def test_surrogate_mlp_class(self):
        from caelab.models.surrogate import SurrogateMLP

        model = SurrogateMLP(input_dim=5)
        assert model is not None
        assert model.input_dim == 5

    def test_segmentation_model_class(self):
        from caelab.models.surrogate import SegmentationModel

        model = SegmentationModel()
        assert model is not None

    def test_default_feature_names(self):
        from caelab.models.surrogate import default_feature_names

        names = default_feature_names()
        assert isinstance(names, list)

    def test_default_property_names(self):
        from caelab.models.surrogate import default_property_names

        names = default_property_names()
        assert isinstance(names, list)


class TestActiveLearning:
    """主动学习 (V4.4-006)"""

    _atoms_data = {
        "positions": [[0, 0, 0], [0.5, 0.5, 0.5]],
        "numbers": [13, 13],
        "cell": [[4.05, 0, 0], [0, 4.05, 0], [0, 0, 4.05]],
        "pbc": [True, True, True],
    }

    def test_compute_mc_dropout_uncertainty(self):
        from caelab.models.active_learning import compute_mc_dropout_uncertainty

        # signature: (model_name, atoms_data, n_samples=10)
        uncertainty = compute_mc_dropout_uncertainty(
            "chgnet", self._atoms_data, n_samples=5
        )
        assert isinstance(uncertainty, dict)
        assert "uncertainty" in uncertainty

    def test_compute_ensemble_uncertainty(self):
        from caelab.models.active_learning import compute_ensemble_uncertainty

        # signature: (model_names, atoms_data)
        uncertainty = compute_ensemble_uncertainty(
            ["chgnet", "m3gnet"], self._atoms_data
        )
        assert isinstance(uncertainty, dict)
        assert "uncertainty" in uncertainty

    def test_select_candidates(self):
        from caelab.models.active_learning import select_candidates

        # signature: (candidates, num_select=5, strategy="max_uncertainty")
        candidates = [
            {"id": f"c{i}", "uncertainty": 0.1 * i,
             "positions": [[float(i), 0, 0]]}
            for i in range(10)
        ]
        result = select_candidates(candidates, num_select=3)
        assert isinstance(result, list)
        assert len(result) <= 3
