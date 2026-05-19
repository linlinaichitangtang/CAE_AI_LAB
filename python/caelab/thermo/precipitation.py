"""
KWN 析出动力学 (V4.7-005)

Kampmann-Wagner Numerical 模型：
形核速率 + 长大速率 + Ostwald 熟化
"""
from __future__ import annotations
import json, sys, numpy as np

def kwn_precipitation(T: float = 723.0, c0: float = 0.04, ceq: float = 0.005,
                      gamma: float = 0.2, n_max: float = 1e24, D0: float = 1e-4,
                      Q: float = 130e3, Vm: float = 1e-5, t_total: float = 36000,
                      n_steps: int = 200):
    """KWN 析出动力学模拟"""
    kB = 1.380649e-23
    R = 8.314

    D = D0 * np.exp(-Q / (R * T))
    dt = t_total / n_steps

    # 临界形核半径
    r_crit = 2 * gamma * Vm / (R * T * np.log(c0 / ceq)) if c0 > ceq else 1e-9

    particles: list[dict] = []
    c_matrix = c0
    history = []

    for step in range(n_steps):
        t = (step + 1) * dt
        supersat = max(0, c_matrix - ceq)

        # 形核速率 (classical nucleation theory)
        if supersat > 0:
            dG_crit = 16 * np.pi * gamma**3 / (3 * (R * T * np.log(c0 / ceq))**2) if c0 > ceq else 1e30
            J_nuc = n_max * D / (r_crit**2) * np.exp(-dG_crit / (kB * T))
            n_new = J_nuc * dt
            for _ in range(int(min(n_new, 100))):
                particles.append({"r": r_crit, "n": 1.0})

        # 长大速率
        for p in particles:
            if supersat > 0:
                p["r"] += D * supersat / (p["r"]) * dt

        # Ostwald 熟化（小粒子溶解）
        if particles:
            r_mean = np.mean([p["r"] for p in particles])
            particles = [p for p in particles if p["r"] > 0.3 * r_mean]

        # 质量守恒更新基体浓度
        if particles:
            total_vol = sum(p["n"] * 4/3*np.pi*p["r"]**3 for p in particles)
            c_precip = total_vol / Vm
            c_matrix = max(ceq, c0 - c_precip)

        if step % (n_steps // 10) == 0 or step == n_steps - 1:
            r_list = [p["r"] for p in particles]
            history.append({
                "time_s": round(t, 1),
                "n_particles": len(particles),
                "mean_radius_nm": round(float(np.mean(r_list) * 1e9), 3) if r_list else 0,
                "max_radius_nm": round(float(np.max(r_list) * 1e9), 3) if r_list else 0,
                "std_radius_nm": round(float(np.std(r_list) * 1e9), 3) if r_list else 0,
                "matrix_composition": round(float(c_matrix), 6),
            })

    r_list = [p["r"] for p in particles]
    return {"temperature_K": T, "initial_composition": c0,
            "equilibrium_composition": ceq, "r_critical_nm": round(r_crit * 1e9, 3),
            "diffusivity_m2_s": D, "history": history,
            "final_n_particles": len(particles),
            "final_mean_radius_nm": round(float(np.mean(r_list) * 1e9), 3) if r_list else 0,
            "is_mock": False}


def main():
    if len(sys.argv) < 2:
        print("kwn: <T> [c0] [ceq] [t_total]")
        sys.exit(1)
    T = float(sys.argv[1])
    c0 = float(sys.argv[2]) if len(sys.argv) > 2 else 0.04
    ceq = float(sys.argv[3]) if len(sys.argv) > 3 else 0.005
    t_total = float(sys.argv[4]) if len(sys.argv) > 4 else 36000
    result = kwn_precipitation(T=T, c0=c0, ceq=ceq, t_total=t_total)
    print(json.dumps(result))


if __name__ == "__main__":
    main()
