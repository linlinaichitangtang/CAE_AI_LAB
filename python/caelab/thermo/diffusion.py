"""
一维扩散偶模拟 (V4.7-004)

模拟浓度分布（菲克定律: ∂c/∂t = D ∂²c/∂x²）
"""
from __future__ import annotations
import json, sys, numpy as np

def diffusion_couple(length: float = 1e-3, n_points: int = 100, time_total: float = 3600,
                     n_steps: int = 500, D: float = 1e-12, c_left: float = 0.05, c_right: float = 0.0):
    """一维扩散偶模拟"""

    dx = length / (n_points - 1)
    dt = time_total / n_steps

    # 稳定性检查
    if D * dt / dx**2 > 0.5:
        dt = 0.4 * dx**2 / D
        n_steps = max(int(time_total / dt), 100)

    # 初始：阶梯分布
    c = np.ones(n_points) * c_right
    c[:n_points // 2] = c_left

    x = np.linspace(0, length, n_points)
    profiles = [{"time": 0, "x": x.tolist(), "concentration": c.tolist()}]

    for step in range(n_steps):
        c_new = c.copy()
        for i in range(1, n_points - 1):
            c_new[i] = c[i] + D * dt / dx**2 * (c[i+1] - 2*c[i] + c[i-1])

        # 边界条件（Dirichlet）
        c_new[0] = c_left
        c_new[-1] = c_right
        c = c_new

        if step % (n_steps // 10) == 0 or step == n_steps - 1:
            t = (step + 1) * dt
            profiles.append({"time": round(t, 1), "x": x.tolist(),
                             "concentration": [round(float(v), 8) for v in c.tolist()]})

    # 扩散距离 (x_D ≈ 2√(Dt))
    diffusion_distance = 2 * np.sqrt(D * time_total)

    return {"length_m": length, "time_total_s": time_total, "diffusivity_m2_s": D,
            "diffusion_distance_m": round(float(diffusion_distance), 8),
            "profiles": profiles, "is_mock": False}


def main():
    if len(sys.argv) < 2:
        print("diffusion: [L] [n] [t] [D] [c_left] [c_right]")
        sys.exit(1)
    args = [float(a) for a in sys.argv[1:]]
    result = diffusion_couple(*args) if len(args) >= 6 else diffusion_couple()
    print(json.dumps(result))


if __name__ == "__main__":
    main()
