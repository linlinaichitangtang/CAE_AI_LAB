"""
不确定性量化模块 (V4.5-005)

- 势函数参数的贝叶斯推断
- DFT k-point 收敛性自动判定（能量变化 < 1 meV/atom）
- 势函数 UQ 给出预测置信区间
"""

from __future__ import annotations

import json
import sys
from typing import Any

import numpy as np


# ============================================================================
# k-point 收敛性检测
# ============================================================================

def check_kpoint_convergence(
    energies: list[float],
    kpoint_grids: list[tuple[int, int, int]],
    threshold_mev_per_atom: float = 1.0,
    n_atoms: int = 1,
) -> dict[str, Any]:
    """判断 k-point 网格是否收敛

    Parameters
    ----------
    energies : list[float]
        不同 k-point 网格对应的总能量
    kpoint_grids : list[tuple[int,int,int]]
        对应的 k-point 网格
    threshold_mev_per_atom : float
        收敛阈值 (meV/atom)
    n_atoms : int
        体系原子数
    """
    if len(energies) < 2:
        return {"converged": False, "error": "至少需要 2 个能量值"}

    diffs = []
    for i in range(1, len(energies)):
        de_per_atom = abs(energies[i] - energies[i - 1]) / n_atoms * 1000.0  # eV → meV
        diffs.append({
            "from_kpoints": kpoint_grids[i - 1],
            "to_kpoints": kpoint_grids[i],
            "delta_energy_meV_per_atom": round(de_per_atom, 6),
        })

    # 自动检测收敛
    last_diff = diffs[-1]["delta_energy_meV_per_atom"] if diffs else float("inf")
    converged = last_diff < threshold_mev_per_atom
    recommended_grid = kpoint_grids[-1] if converged else (
        tuple(max(k[i] + 2, 10) for i in range(3))
    )

    return {
        "converged": converged,
        "threshold_meV_per_atom": threshold_mev_per_atom,
        "n_atoms": n_atoms,
        "energy_differences": diffs,
        "last_delta_meV_per_atom": round(last_diff, 6),
        "converged_kpoints": kpoint_grids[-1] if converged else None,
        "recommended_kpoints": list(recommended_grid),
        "is_mock": False,
    }


# ============================================================================
# 贝叶斯不确定性
# ============================================================================

def bayesian_uq(
    predictions: list[float],
    reference: float | None = None,
    n_samples: int = 10000,
) -> dict[str, Any]:
    """贝叶斯推断不确定量化

    给定模型预测的多个采样值，计算后验分布和置信区间。

    Parameters
    ----------
    predictions : list[float]
        模型多次预测值（例如 MC dropout 的多轮采样）
    reference : float | None
        DFT 参考值
    n_samples : int
        MCMC 采样数
    """
    preds = np.array(predictions)
    mean = float(np.mean(preds))
    std = float(np.std(preds))
    n = len(preds)

    # 后验参数（共轭先验 NIG）
    posterior_samples = np.random.normal(mean, std / np.sqrt(n), n_samples)

    ci_95 = np.percentile(posterior_samples, [2.5, 97.5])
    ci_68 = np.percentile(posterior_samples, [16, 84])

    result = {
        "mean": round(mean, 6),
        "std": round(std, 6),
        "confidence_95": [round(float(ci_95[0]), 6), round(float(ci_95[1]), 6)],
        "confidence_68": [round(float(ci_68[0]), 6), round(float(ci_68[1]), 6)],
        "ci_width_95": round(float(ci_95[1] - ci_95[0]), 6),
        "n_predictions": n,
        "n_mcmc_samples": n_samples,
    }

    if reference is not None:
        error = abs(mean - reference)
        within_ci = ci_95[0] <= reference <= ci_95[1]
        result["reference"] = round(reference, 6)
        result["absolute_error"] = round(error, 6)
        result["relative_error_percent"] = round(error / abs(reference) * 100, 4) if reference != 0 else 0.0
        result["reference_in_95ci"] = bool(within_ci)

    result["is_mock"] = False
    return result


# ============================================================================
# 势函数参数灵敏度
# ============================================================================

def parameter_sensitivity(
    param_ranges: dict[str, list[float]],
    evaluation_fn_result: list[float],
) -> dict[str, Any]:
    """势函数参数灵敏度分析

    计算各参数对目标量的 Sobol 一阶灵敏度指数（近似）。
    """
    n_params = len(param_ranges)
    if len(evaluation_fn_result) < n_params * 2:
        return {"error": "评估结果数量不足"}

    # 简化：方差分解
    total_var = float(np.var(evaluation_fn_result)) + 1e-12
    sensitivities = {}

    for i, (param, ranges) in enumerate(param_ranges.items()):
        # 近似：固定参数时方差的减少量
        subset_indices = list(range(i, len(evaluation_fn_result), n_params))
        subset_vals = [evaluation_fn_result[j] for j in subset_indices if j < len(evaluation_fn_result)]
        reduced_var = float(np.var(subset_vals)) if len(subset_vals) > 1 else total_var
        sensitivity = max(0.0, 1.0 - reduced_var / total_var)
        sensitivities[param] = round(sensitivity, 4)

    return {
        "sensitivities": sensitivities,
        "most_sensitive": max(sensitivities, key=sensitivities.get),
        "total_variance": round(total_var, 6),
        "is_mock": False,
    }


# ============================================================================
# CLI
# ============================================================================

def main():
    if len(sys.argv) < 2:
        print("用法: python -m caelab.dft.uq <command>")
        print("命令: kpoint <energies_json> <grids_json> | bayesian <predictions_json> | sensitivity <params_json> <values_json>")
        sys.exit(1)

    cmd = sys.argv[1]

    if cmd == "kpoint":
        energies = json.loads(sys.argv[2])
        grids = [tuple(g) for g in json.loads(sys.argv[3])]
        n_atoms = int(sys.argv[4]) if len(sys.argv) > 4 else 1
        result = check_kpoint_convergence(energies, grids, n_atoms=n_atoms)
        print(json.dumps(result, default=str))

    elif cmd == "bayesian":
        predictions = json.loads(sys.argv[2])
        reference = float(sys.argv[3]) if len(sys.argv) > 3 else None
        result = bayesian_uq(predictions, reference)
        print(json.dumps(result, default=str))

    elif cmd == "sensitivity":
        params = json.loads(sys.argv[2])
        values = json.loads(sys.argv[3])
        result = parameter_sensitivity(params, values)
        print(json.dumps(result, default=str))

    else:
        print(f"未知命令: {cmd}")
        sys.exit(1)


if __name__ == "__main__":
    main()
