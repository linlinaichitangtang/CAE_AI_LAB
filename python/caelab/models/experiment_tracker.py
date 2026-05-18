"""
实验跟踪模块 (V4.4-004)

统一接口封装 W&B (Weights & Biases) 和 MLflow，
训练过程中自动记录超参数、指标和模型 artifact。
"""

from __future__ import annotations

import json
import os
import sys
import time
from pathlib import Path
from typing import Any


# ============================================================================
# Tracker 基类
# ============================================================================

class ExperimentTracker:
    """实验跟踪器基类"""

    def __init__(self, project: str, name: str | None = None, config: dict | None = None):
        self.project = project
        self.name = name or f"run_{int(time.time())}"
        self.config = config or {}
        self._active = False

    def start(self):
        self._active = True

    def log_params(self, params: dict[str, Any]):
        raise NotImplementedError

    def log_metrics(self, metrics: dict[str, Any], step: int | None = None):
        raise NotImplementedError

    def log_artifact(self, name: str, path: str, artifact_type: str = "model"):
        raise NotImplementedError

    def finish(self):
        self._active = False

    def status(self) -> dict[str, Any]:
        return {
            "tracker": self.__class__.__name__,
            "project": self.project,
            "name": self.name,
            "active": self._active,
        }


# ============================================================================
# W&B Tracker
# ============================================================================

class WandbTracker(ExperimentTracker):
    """Weights & Biases 实验跟踪器"""

    def __init__(self, project: str, name: str | None = None, config: dict | None = None,
                 entity: str | None = None, api_key: str | None = None):
        super().__init__(project, name, config)
        self.entity = entity
        self.api_key = api_key or os.environ.get("WANDB_API_KEY")
        self._run = None

    def start(self):
        try:
            import wandb
            if self.api_key:
                os.environ["WANDB_API_KEY"] = self.api_key
            self._run = wandb.init(
                project=self.project,
                name=self.name,
                config=self.config,
                entity=self.entity,
                reinit=True,
            )
            self._active = True
        except ImportError:
            print(json.dumps({"type": "warning", "message": "wandb 未安装，跳过 W&B 记录"}))
        except Exception as e:
            print(json.dumps({"type": "warning", "message": f"W&B 初始化失败: {e}"}))

    def log_params(self, params: dict[str, Any]):
        if self._run:
            try:
                import wandb
                wandb.config.update(params, allow_val_change=True)
            except Exception:
                pass

    def log_metrics(self, metrics: dict[str, Any], step: int | None = None):
        if self._run:
            try:
                import wandb
                wandb.log(metrics, step=step)
            except Exception:
                pass

    def log_artifact(self, name: str, path: str, artifact_type: str = "model"):
        if self._run:
            try:
                import wandb
                artifact = wandb.Artifact(name=name, type=artifact_type)
                artifact.add_file(path)
                self._run.log_artifact(artifact)
            except Exception:
                pass

    def finish(self):
        if self._run:
            try:
                self._run.finish()
            except Exception:
                pass
        self._run = None
        self._active = False

    def status(self) -> dict[str, Any]:
        s = super().status()
        s["backend"] = "wandb"
        s["api_key_set"] = self.api_key is not None
        return s


# ============================================================================
# MLflow Tracker
# ============================================================================

class MLflowTracker(ExperimentTracker):
    """MLflow 实验跟踪器"""

    def __init__(self, project: str, name: str | None = None, config: dict | None = None,
                 tracking_uri: str | None = None):
        super().__init__(project, name, config)
        self.tracking_uri = tracking_uri or os.environ.get(
            "MLFLOW_TRACKING_URI", "file:///tmp/mlruns"
        )
        self._run_id: str | None = None

    def start(self):
        try:
            import mlflow
            mlflow.set_tracking_uri(self.tracking_uri)
            mlflow.set_experiment(self.project)
            run = mlflow.start_run(run_name=self.name)
            self._run_id = run.info.run_id
            self._active = True
            if self.config:
                mlflow.log_params(self.config)
        except ImportError:
            print(json.dumps({"type": "warning", "message": "mlflow 未安装，跳过 MLflow 记录"}))
        except Exception as e:
            print(json.dumps({"type": "warning", "message": f"MLflow 初始化失败: {e}"}))

    def log_params(self, params: dict[str, Any]):
        if self._run_id:
            try:
                import mlflow
                mlflow.log_params(params)
            except Exception:
                pass

    def log_metrics(self, metrics: dict[str, Any], step: int | None = None):
        if self._run_id:
            try:
                import mlflow
                mlflow.log_metrics(metrics, step=step)
            except Exception:
                pass

    def log_artifact(self, name: str, path: str, artifact_type: str = "model"):
        if self._run_id:
            try:
                import mlflow
                mlflow.log_artifact(path)
            except Exception:
                pass

    def finish(self):
        if self._run_id:
            try:
                import mlflow
                mlflow.end_run()
            except Exception:
                pass
        self._run_id = None
        self._active = False

    def status(self) -> dict[str, Any]:
        s = super().status()
        s["backend"] = "mlflow"
        s["tracking_uri"] = self.tracking_uri
        s["run_id"] = self._run_id
        return s


# ============================================================================
# 工厂函数
# ============================================================================

_TRACKERS: dict[str, ExperimentTracker] = {}


def create_tracker(
    backend: str,
    project: str = "caelab-training",
    name: str | None = None,
    config: dict | None = None,
    **kwargs,
) -> ExperimentTracker:
    """创建实验跟踪器"""
    if backend == "wandb":
        tracker = WandbTracker(project=project, name=name, config=config, **kwargs)
    elif backend == "mlflow":
        tracker = MLflowTracker(project=project, name=name, config=config, **kwargs)
    else:
        raise ValueError(f"不支持的跟踪后端: {backend}，支持 wandb / mlflow")

    tracker.start()
    _TRACKERS[tracker.name] = tracker
    return tracker


def get_tracker(name: str) -> ExperimentTracker | None:
    return _TRACKERS.get(name)


def list_trackers() -> list[dict[str, Any]]:
    return [t.status() for t in _TRACKERS.values()]


# ============================================================================
# 训练集成：自动记录训练过程
# ============================================================================

def log_training_loss(tracker_name: str, epoch: int, metrics: dict[str, Any]):
    """记录单个 epoch 的 loss 指标"""
    tracker = get_tracker(tracker_name)
    if tracker:
        tracker.log_metrics(metrics, step=epoch)


def log_training_result(tracker_name: str, result: dict[str, Any]):
    """记录训练最终结果"""
    tracker = get_tracker(tracker_name)
    if tracker:
        tracker.log_metrics({
            "final_energy_rmse": result.get("final_energy_rmse", 0),
            "final_force_rmse": result.get("final_force_rmse", 0),
            "best_epoch": result.get("best_epoch", 0),
            "training_time_sec": result.get("training_time_sec", 0),
        })
        model_path = result.get("model_path")
        if model_path and Path(model_path).exists():
            tracker.log_artifact(
                name=f"{result.get('potential_type', 'model')}_final",
                path=model_path,
            )
        tracker.finish()


# ============================================================================
# CLI 入口
# ============================================================================

def main():
    """命令行: python -m caelab.models.experiment_tracker <command> [args]"""
    if len(sys.argv) < 2:
        print("用法: python -m caelab.models.experiment_tracker <command>")
        print("命令: create <backend> <project> | list | status <name>")
        sys.exit(1)

    cmd = sys.argv[1]

    if cmd == "create" and len(sys.argv) >= 4:
        backend = sys.argv[2]
        project = sys.argv[3]
        name = sys.argv[4] if len(sys.argv) > 4 else None
        tracker = create_tracker(backend, project, name)
        print(json.dumps(tracker.status(), ensure_ascii=False))

    elif cmd == "list":
        print(json.dumps(list_trackers(), ensure_ascii=False, indent=2))

    elif cmd == "status" and len(sys.argv) >= 3:
        tracker = get_tracker(sys.argv[2])
        if tracker:
            print(json.dumps(tracker.status(), ensure_ascii=False, indent=2))
        else:
            print(json.dumps({"error": f"tracker '{sys.argv[2]}' 不存在"}))
            sys.exit(1)

    else:
        print(f"未知命令: {cmd}")
        sys.exit(1)


if __name__ == "__main__":
    main()
