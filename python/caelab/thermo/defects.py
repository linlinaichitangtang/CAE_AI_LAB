"""
缺陷热力学 (V4.7-008)

空位形成能/熵、间隙形成能、反位缺陷能。
缺陷浓度: c = exp(Sf/kB) * exp(-Ef/kBT)
"""
from __future__ import annotations
import json, sys, numpy as np

kB = 8.617333262e-5  # eV/K

DEFECT_DATA: dict[str, dict] = {
    "Al": {"vacancy": {"Ef_eV": 0.67, "Sf_kB": 1.5}, "interstitial": {"Ef_eV": 2.93, "Sf_kB": 2.0}},
    "Fe": {"vacancy": {"Ef_eV": 1.79, "Sf_kB": 1.2}, "interstitial": {"Ef_eV": 4.60, "Sf_kB": 3.0}},
    "Ni": {"vacancy": {"Ef_eV": 1.55, "Sf_kB": 1.5}, "interstitial": {"Ef_eV": 4.10, "Sf_kB": 2.5}},
    "Cu": {"vacancy": {"Ef_eV": 1.17, "Sf_kB": 1.3}, "interstitial": {"Ef_eV": 3.60, "Sf_kB": 2.0}},
    "Ti": {"vacancy": {"Ef_eV": 1.95, "Sf_kB": 1.8}, "interstitial": {"Ef_eV": 3.80, "Sf_kB": 2.8}},
    "Si": {"vacancy": {"Ef_eV": 3.60, "Sf_kB": 2.0}, "interstitial": {"Ef_eV": 4.90, "Sf_kB": 3.0}},
    "Mg": {"vacancy": {"Ef_eV": 0.85, "Sf_kB": 1.6}},
}


def vacancy_concentration(element: str, T_range: tuple[float, float], n_points: int = 100):
    """空位浓度 vs 温度"""
    data = DEFECT_DATA.get(element, {}).get("vacancy")
    if data is None:
        return {"error": f"元素 {element} 无空位数据"}

    Ef, Sf = data["Ef_eV"], data.get("Sf_kB", 1.0)
    T = np.linspace(T_range[0], T_range[1], n_points)
    c = np.exp(Sf) * np.exp(-Ef / (kB * T))

    return {"element": element, "defect_type": "vacancy",
            "Ef_eV": Ef, "Sf_kB": Sf,
            "temperature_K": T.tolist(),
            "concentration": [round(float(v), 10) for v in c.tolist()],
            "concentration_at_1000K": round(float(np.exp(Sf) * np.exp(-Ef / (kB * 1000))), 10) if T_range[0] <= 1000 <= T_range[1] else None,
            "is_mock": False}


def arrhenius_fit(T: list[float], defect_concentration: list[float]):
    """Arrhenius 拟合: ln(c) = Sf/kB - Ef/kB * 1/T"""
    inv_T = 1.0 / np.array(T)
    ln_c = np.log(np.array(defect_concentration))
    slope, intercept = np.polyfit(inv_T, ln_c, 1)
    Ef = -slope * kB
    Sf = intercept

    return {"Ef_eV": round(float(Ef), 4), "Sf_kB": round(float(Sf), 4),
            "r_squared": round(float(np.corrcoef(inv_T, ln_c)[0, 1]**2), 4),
            "is_mock": False}


def list_defect_elements():
    return sorted(DEFECT_DATA.keys())


def main():
    if len(sys.argv) < 2:
        print("defects: elements | concentration <el> <Tmin> <Tmax> | fit <T_json> <c_json>")
        sys.exit(1)
    cmd = sys.argv[1]
    if cmd == "elements":
        print(json.dumps(list_defect_elements()))
    elif cmd == "concentration":
        el = sys.argv[2]
        Tmin, Tmax = float(sys.argv[3]), float(sys.argv[4])
        result = vacancy_concentration(el, (Tmin, Tmax))
        print(json.dumps(result))
    elif cmd == "fit":
        T = json.loads(sys.argv[2])
        c = json.loads(sys.argv[3])
        result = arrhenius_fit(T, c)
        print(json.dumps(result))


if __name__ == "__main__":
    main()
