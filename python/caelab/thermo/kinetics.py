"""
JMAK 相变动力学 (V4.7-007)

等温相变: f(t) = 1 - exp(-k * t^n)
非等温: Kissinger 方法 + Avrami 指数拟合
"""
from __future__ import annotations
import json, sys, numpy as np

def jmak_isothermal(k: float = 0.001, n: float = 2.5, t_max: float = 10000, n_points: int = 200):
    """等温 JMAK: f = 1 - exp(-k * t^n)"""
    t = np.linspace(0, t_max, n_points)
    f = 1 - np.exp(-k * t**n)
    df_dt = k * n * t**(n - 1) * np.exp(-k * t**n)

    # TTT 起点: f = 0.01 和 f = 0.99
    t_start = ( -np.log(0.99) / k ) ** (1/n) if k > 0 else 0
    t_end = ( -np.log(0.01) / k ) ** (1/n) if k > 0 else t_max

    return {"type": "isothermal", "k": k, "n": n,
            "t_start_s": round(float(t_start), 1), "t_end_s": round(float(t_end), 1),
            "time_s": t.tolist(), "fraction": [round(float(v), 6) for v in f.tolist()],
            "rate": [round(float(v), 8) for v in df_dt.tolist()],
            "is_mock": False}


def jmak_kissinger(heating_rates: list[float], peak_temperatures: list[float], E: float | None = None):
    """Kissinger 方法: ln(β/Tp²) = -E/RT + const"""
    if len(heating_rates) != len(peak_temperatures):
        return {"error": "heating_rates 和 peak_temperatures 长度必须相同"}

    x = 1.0 / (np.array(peak_temperatures) * 8.314 / 1000)  # 1/(RT) 单位 kJ
    y = np.log(np.array(heating_rates) / np.array(peak_temperatures)**2)

    # 线性拟合
    slope, intercept = np.polyfit(x, y, 1)
    E_fit = -slope * 1000  # kJ → J

    return {"type": "kissinger", "heating_rates": heating_rates,
            "peak_temperatures": peak_temperatures,
            "activation_energy_kJ_per_mol": round(float(E_fit / 1000), 2),
            "r_squared": round(float(np.corrcoef(x, y)[0, 1]**2), 4),
            "is_mock": False}


def fit_avrami(time: list[float], fraction: list[float]):
    """Avrami 指数拟合: ln(-ln(1-f)) = ln(k) + n*ln(t)"""
    t = np.array(time)
    f = np.array(fraction)
    mask = (f > 0.01) & (f < 0.99)

    if mask.sum() < 5:
        return {"error": "数据点不足（需 0.01 < f < 0.99 范围内 >= 5 点）"}

    y = np.log(-np.log(1 - f[mask]))
    x = np.log(t[mask])
    slope, intercept = np.polyfit(x, y, 1)

    return {"n_avrami": round(float(slope), 4),
            "ln_k": round(float(intercept), 4),
            "k": round(float(np.exp(intercept)), 8),
            "r_squared": round(float(np.corrcoef(x, y)[0, 1]**2), 4),
            "points_used": int(mask.sum()),
            "is_mock": False}


def generate_ttt_curve(k_params: list[dict], T_range: list[float]):
    """根据温度相关的 k,n 参数生成 TTT 曲线"""
    curves = []
    for params in k_params:
        T = params["T"]
        k = params["k"]
        n = params.get("n", 2.0)
        if k > 0:
            t_01 = (-np.log(0.99) / k) ** (1/n)
            t_99 = (-np.log(0.01) / k) ** (1/n)
        else:
            t_01, t_99 = 1e6, 1e6
        curves.append({"temperature": T, "t_start_s": round(float(t_01), 1),
                       "t_end_s": round(float(t_99), 1)})
    return {"type": "ttt", "curves": curves, "is_mock": False}


def main():
    if len(sys.argv) < 2:
        print("kinetics: isothermal <k> <n> | kissinger <rates_json> <temps_json> | avrami <t_json> <f_json>")
        sys.exit(1)
    cmd = sys.argv[1]
    if cmd == "isothermal":
        k = float(sys.argv[2]) if len(sys.argv) > 2 else 0.001
        n = float(sys.argv[3]) if len(sys.argv) > 3 else 2.5
        print(json.dumps(jmak_isothermal(k, n)))
    elif cmd == "kissinger":
        rates = json.loads(sys.argv[2])
        temps = json.loads(sys.argv[3])
        print(json.dumps(jmak_kissinger(rates, temps)))
    elif cmd == "avrami":
        t = json.loads(sys.argv[2])
        f = json.loads(sys.argv[3])
        print(json.dumps(fit_avrami(t, f)))


if __name__ == "__main__":
    main()
