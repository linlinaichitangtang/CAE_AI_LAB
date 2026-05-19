"""
相图计算与可视化 (V4.7-002)

二元/三元相图计算，等温截面，液相面投影。
"""
from __future__ import annotations
import json, sys, numpy as np
from .calphad import gibbs_total, gibbs_pure, R_GAS, SGTE_PURE_ELEMENTS

def binary_phase_diagram(comp1: str, comp2: str, phases: list[str],
                         t_range: tuple[float, float], n_temp: int = 50, n_comp: int = 100):
    """计算二元相图（温度-成分网格上找最稳定相）"""
    temps = np.linspace(t_range[0], t_range[1], n_temp)
    xs = np.linspace(0.001, 0.999, n_comp)
    grid = np.zeros((n_temp, n_comp), dtype=int)

    for i, T in enumerate(temps):
        for j, x in enumerate(xs):
            gibbs_vals = [gibbs_total([comp1, comp2], ph, [x, 1-x], T) for ph in phases]
            grid[i, j] = int(np.argmin(gibbs_vals))

    # 简化：提取相界
    phase_boundaries = []
    for i in range(n_temp):
        phases_at_t = []
        for j in range(1, n_comp):
            if grid[i, j] != grid[i, j-1]:
                phases_at_t.append({"temperature": round(float(temps[i]), 1),
                                    "composition": round(float(xs[j]), 4),
                                    "from_phase": phases[grid[i, j-1]],
                                    "to_phase": phases[grid[i, j]]})
        phase_boundaries.extend(phases_at_t)

    return {"components": [comp1, comp2], "phases": phases,
            "temperature_range": list(t_range), "phase_boundaries": phase_boundaries,
            "n_boundaries": len(phase_boundaries), "is_mock": False}


def isothermal_section(comp1: str, comp2: str, phases: list[str], T: float):
    """等温截面：给定温度下 G(x) 曲线"""
    xs = np.linspace(0.01, 0.99, 50)
    curves = []
    for ph in phases:
        g = [gibbs_total([comp1, comp2], ph, [xi, 1-xi], T) for xi in xs]
        curves.append({"phase": ph, "x": xs.tolist(), "gibbs": [round(float(v), 2) for v in g]})

    # 稳定相序列
    stable = []
    for j, x in enumerate(xs):
        g_vals = [curves[p]["gibbs"][j] for p in range(len(phases))]
        best = int(np.argmin(g_vals))
        if not stable or stable[-1]["phase"] != phases[best]:
            stable.append({"composition": round(float(x), 4), "phase": phases[best],
                           "gibbs": round(float(g_vals[best]), 2)})

    return {"components": [comp1, comp2], "temperature": T,
            "stable_phases": stable, "curves": curves, "is_mock": False}


def main():
    if len(sys.argv) < 2:
        print("phase_diagram: binary <c1> <c2> <phases_json> <Tmin> <Tmax>\n"
              "              isothermal <c1> <c2> <phases_json> <T>")
        sys.exit(1)
    cmd = sys.argv[1]
    if cmd == "binary":
        c1, c2 = sys.argv[2], sys.argv[3]
        phases = json.loads(sys.argv[4])
        result = binary_phase_diagram(c1, c2, phases, (float(sys.argv[5]), float(sys.argv[6])))
        print(json.dumps(result))
    elif cmd == "isothermal":
        c1, c2 = sys.argv[2], sys.argv[3]
        phases = json.loads(sys.argv[4])
        result = isothermal_section(c1, c2, phases, float(sys.argv[5]))
        print(json.dumps(result))


if __name__ == "__main__":
    main()
