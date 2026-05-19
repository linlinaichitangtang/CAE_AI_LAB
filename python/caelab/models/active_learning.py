"""
主动学习闭环引擎 (V4.4-006)

真实不确定性计算：
- MC Dropout: 多次前向传播，统计预测方差
- Ensemble: 多个模型的预测差异
- 基于不确定性选择候选结构提交 DFT 验证
"""

from __future__ import annotations

import json
import os
import sys
import time
from typing import Any

import numpy as np


# ============================================================================
# 不确定性计算
# ============================================================================

def compute_mc_dropout_uncertainty(
    model_name: str,
    atoms_data: dict[str, Any],
    n_samples: int = 10,
) -> dict[str, Any]:
    """MC Dropout 不确定性估计

    多次前向传播（开启 dropout），统计能量/力的方差。
    无真实模型时使用基于物理先验的 mock。
    """
    try:
        from .inference import predict_structure
        predictions = []
        for _ in range(n_samples):
            result = predict_structure(model_name, atoms_data, ["energy", "forces"])
            predictions.append(result)

        energies = [p["energy"] for p in predictions]
        forces_list = [np.array(p["forces"]) for p in predictions]

        energy_mean = float(np.mean(energies))
        energy_std = float(np.std(energies))

        forces_mean = np.mean(forces_list, axis=0)
        forces_std = np.std(forces_list, axis=0)

        # 综合不确定性分数
        uncertainty = energy_std * 50.0 + float(np.mean(forces_std)) * 5.0

        return {
            "uncertainty": round(uncertainty, 6),
            "energy_mean": round(energy_mean, 6),
            "energy_std": round(energy_std, 6),
            "force_std_mean": round(float(np.mean(forces_std)), 6),
            "method": "mc_dropout",
            "n_samples": n_samples,
            "is_mock": False,
        }
    except Exception:
        return _mock_uncertainty(atoms_data, "mc_dropout", n_samples)


def compute_ensemble_uncertainty(
    model_names: list[str],
    atoms_data: dict[str, Any],
) -> dict[str, Any]:
    """集合模型不确定性估计

    使用多个不同模型（M3GNet/CHGNet/MACE）的预测差异作为不确定性。
    """
    try:
        from .inference import predict_structure
        predictions = []
        for name in model_names:
            result = predict_structure(name, atoms_data, ["energy", "forces"])
            predictions.append({"model": name, **result})

        energies = [p["energy"] for p in predictions]
        forces_list = [np.array(p["forces"]) for p in predictions]

        energy_mean = float(np.mean(energies))
        energy_std = float(np.std(energies))

        forces_mean = np.mean(forces_list, axis=0)
        forces_std = np.std(forces_list, axis=0)

        uncertainty = energy_std * 50.0 + float(np.mean(forces_std)) * 5.0

        return {
            "uncertainty": round(uncertainty, 6),
            "energy_mean": round(energy_mean, 6),
            "energy_std": round(energy_std, 6),
            "force_std_mean": round(float(np.mean(forces_std)), 6),
            "method": "ensemble",
            "models_used": model_names,
            "per_model": [
                {"model": p["model"], "energy": p["energy"]}
                for p in predictions
            ],
            "is_mock": False,
        }
    except Exception:
        return _mock_uncertainty(atoms_data, "ensemble", len(model_names))


def _mock_uncertainty(atoms_data: dict, method: str, n: int) -> dict[str, Any]:
    """Mock 不确定性（无模型时的 fallback）"""
    num_atoms = len(atoms_data.get("atomic_numbers", atoms_data.get("positions", [])))
    base_energy = num_atoms * (-5.0)

    energies = [base_energy + np.random.normal(0, 0.1) for _ in range(n)]
    energy_mean = float(np.mean(energies))
    energy_std = float(np.std(energies))

    force_std_mean = 0.08 * np.random.random()

    uncertainty = energy_std * 50.0 + force_std_mean * 5.0

    return {
        "uncertainty": round(uncertainty, 6),
        "energy_mean": round(energy_mean, 6),
        "energy_std": round(energy_std, 6),
        "force_std_mean": round(force_std_mean, 6),
        "method": method,
        "n_samples": n,
        "is_mock": True,
    }


# ============================================================================
# 候选结构选择
# ============================================================================

def select_candidates(
    candidates: list[dict[str, Any]],
    num_select: int = 5,
    strategy: str = "max_uncertainty",
) -> list[dict[str, Any]]:
    """根据策略选择候选结构

    Parameters
    ----------
    candidates : list[dict]
        每个候选包含 uncertainty, positions, atomic_numbers 等字段
    num_select : int
        选择数量
    strategy : str
        "max_uncertainty" | "diverse" | "random"
    """
    if not candidates:
        return []

    if strategy == "random":
        import random
        indices = random.sample(range(len(candidates)), min(num_select, len(candidates)))
        return [candidates[i] for i in sorted(indices)]

    # 按不确定性排序
    sorted_cands = sorted(candidates, key=lambda c: c.get("uncertainty", 0), reverse=True)

    if strategy == "max_uncertainty":
        return sorted_cands[:num_select]

    if strategy == "diverse":
        # 贪心多样性选择
        selected = [sorted_cands[0]]
        for cand in sorted_cands[1:]:
            if len(selected) >= num_select:
                break
            # 检查与已选结构的距离
            positions = cand.get("positions", [[0, 0, 0]])
            is_diverse = all(
                _euclidean_distance(positions[0], s.get("positions", [[0, 0, 0]])[0]) > 2.0
                for s in selected
            )
            if is_diverse:
                selected.append(cand)
        return selected

    return sorted_cands[:num_select]


def _euclidean_distance(a: list[float], b: list[float]) -> float:
    return float(np.sqrt(sum((x - y) ** 2 for x, y in zip(a, b))))


# ============================================================================
# CLI 入口
# ============================================================================

def main():
    """命令行: python -m caelab.models.active_learning <command> [args]"""
    if len(sys.argv) < 2:
        print("用法: python -m caelab.models.active_learning <command>")
        print("命令: mc_dropout <model> <atoms_json> [n_samples] | ensemble <atoms_json> | select <candidates_json> [n]")
        sys.exit(1)

    cmd = sys.argv[1]

    if cmd == "mc_dropout":
        model = sys.argv[2]
        atoms = json.loads(sys.argv[3])
        n = int(sys.argv[4]) if len(sys.argv) > 4 else 10
        result = compute_mc_dropout_uncertainty(model, atoms, n)
        print(json.dumps(result, default=str))

    elif cmd == "ensemble":
        atoms = json.loads(sys.argv[2])
        models = ["chgnet", "m3gnet", "mace"]
        result = compute_ensemble_uncertainty(models, atoms)
        print(json.dumps(result, default=str))

    elif cmd == "select":
        candidates = json.loads(sys.argv[2])
        n = int(sys.argv[3]) if len(sys.argv) > 3 else 5
        result = select_candidates(candidates, n)
        print(json.dumps(result, default=str))

    else:
        print(f"未知命令: {cmd}")
        sys.exit(1)


if __name__ == "__main__":
    main()
