"""
Scheil-Gulliver 非平衡凝固模拟 (V4.7-003)

预测凝固路径、偏析轮廓、凝固区间。
"""
from __future__ import annotations
import json, sys, numpy as np
from .calphad import gibbs_total, SGTE_PURE_ELEMENTS, BINARY_INTERACTIONS

def scheil_solidification(components: list[str], composition: list[float],
                          phases: list[str], T_liquidus: float, T_solidus: float,
                          k_default: float = 0.1, n_steps: int = 200):
    """Scheil-Gulliver 凝固模拟"""
    if len(components) != 2:
        return {"error": "仅支持二元系"}

    x0 = composition[0]
    dT = (T_liquidus - T_solidus) / n_steps
    fs = 0.0
    T_current = T_liquidus

    f_solid_curve = []
    t_curve = []
    liquid_comp_curve = []
    solid_comp_curve = []

    x_L = x0

    for step in range(n_steps):
        if fs >= 0.999:
            break

        # Scheil 方程: Cs = k * C0 * (1 - fs)^(k-1)
        # 简化：假设两相区，k 取默认值
        k_eff = k_default
        x_S = k_eff * x_L

        # 质量守恒: x0 = fs * x_S + (1 - fs) * x_L
        # → fs = (x_L - x0) / (x_L - x_S)
        if abs(x_L - x_S) > 1e-10:
            fs = (x_L - x0) / (x_L - x_S)
            fs = max(0, min(fs, 0.999))

        f_solid_curve.append(float(fs))
        t_curve.append(float(T_current))
        liquid_comp_curve.append(round(float(x_L), 6))
        solid_comp_curve.append(round(float(x_S), 6))

        # 更新液相成分
        x_L = x_L * (1 - fs * (1 - k_eff))
        x_L = max(0.001, min(0.999, x_L))
        T_current -= dT

    return {
        "components": components, "initial_composition": x0,
        "T_liquidus": T_liquidus, "T_solidus": T_solidus,
        "k_partition": k_default,
        "fs_curve": f_solid_curve, "temperature_curve": t_curve,
        "liquid_composition": liquid_comp_curve, "solid_composition": solid_comp_curve,
        "final_fs": f_solid_curve[-1] if f_solid_curve else 0,
        "is_mock": False,
    }


def main():
    if len(sys.argv) < 2:
        print("scheil: <c1> <c2> <x0> <T_liq> <T_sol> [k]")
        sys.exit(1)
    c1, c2 = sys.argv[1], sys.argv[2]
    x0 = float(sys.argv[3])
    T_liq = float(sys.argv[4])
    T_sol = float(sys.argv[5])
    k = float(sys.argv[6]) if len(sys.argv) > 6 else 0.15
    result = scheil_solidification([c1, c2], [x0, 1-x0], ["FCC_A1", "LIQUID"], T_liq, T_sol, k)
    print(json.dumps(result))


if __name__ == "__main__":
    main()
