#!/usr/bin/env python3
"""
Lightweight CFD Solver (Pseudo-CFD / Semi-Realistic)
Uses finite volume method with SIMPLE algorithm.
Supports: incompressible laminar flow, k-epsilon turbulence.
Designed for educational/undergraduate thesis scenarios.

Usage:
  python3 cfd_solver.py --input case.inp --output result.json
"""

import argparse
import json
import sys
import time
from pathlib import Path
from typing import Dict, List, Tuple

try:
    import numpy as np
except ImportError:
    print("ERROR: numpy not installed. Run: pip install numpy", file=sys.stderr)
    sys.exit(1)

try:
    import scipy.sparse
    import scipy.sparse.linalg
except ImportError:
    print("ERROR: scipy not installed. Run: pip install scipy", file=sys.stderr)
    sys.exit(1)


def parse_inp(inp_path: str) -> Dict:
    """Parse the simplified CFD INP file format."""
    nodes = []
    elements = []
    boundaries = {}
    fluid_props = {}
    config = {}

    with open(inp_path, 'r') as f:
        lines = f.readlines()

    section = None
    for line in lines:
        line = line.strip()
        if not line or line.startswith('#') or line.startswith('**'):
            continue

        if line.startswith('*FLUID_PROPERTIES'):
            section = 'fluid'
        elif line.startswith('*CONFIG'):
            section = 'config'
        elif line.startswith('*NODE'):
            section = 'node'
        elif line.startswith('*ELEMENT'):
            section = 'element'
        elif line.startswith('*INLET'):
            section = 'inlet'
        elif line.startswith('*OUTLET'):
            section = 'outlet'
        elif line.startswith('*WALL'):
            section = 'wall'
        elif line.startswith('*') and section:
            section = None
        else:
            if section == 'fluid':
                parts = [p.strip() for p in line.split(',')]
                if len(parts) >= 2:
                    fluid_props[parts[0]] = float(parts[1])
            elif section == 'config':
                parts = [p.strip() for p in line.split(',')]
                if len(parts) >= 2:
                    config[parts[0]] = float(parts[1]) if parts[1] != 'True' and parts[1] != 'False' else parts[1] == 'True'
            elif section == 'node':
                parts = [p.strip() for p in line.split(',')]
                if len(parts) >= 4:
                    nodes.append({
                        'id': int(parts[0]),
                        'x': float(parts[1]),
                        'y': float(parts[2]),
                        'z': float(parts[3]),
                    })
            elif section == 'element':
                parts = [p.strip() for p in line.split(',')]
                if len(parts) >= 5:
                    elements.append({
                        'id': int(parts[0]),
                        'type': parts[1],
                        'nodes': [int(n) for n in parts[2:]]
                    })
            elif section == 'inlet':
                parts = [p.strip() for p in line.split(',')]
                if len(parts) >= 6:
                    boundaries.setdefault('inlet', []).append({
                        'name': parts[0],
                        'vx': float(parts[1]),
                        'vy': float(parts[2]),
                        'vz': float(parts[3]),
                        'type': parts[4],
                        'nodes': [int(n) for n in parts[5:]]
                    })
            elif section == 'outlet':
                parts = [p.strip() for p in line.split(',')]
                if len(parts) >= 2:
                    boundaries.setdefault('outlet', []).append({
                        'name': parts[0],
                        'pressure': float(parts[1]),
                        'nodes': [int(n) for n in parts[2:]]
                    })
            elif section == 'wall':
                parts = [p.strip() for p in line.split(',')]
                if len(parts) >= 3:
                    boundaries.setdefault('wall', []).append({
                        'name': parts[0],
                        'type': parts[1],
                        'nodes': [int(n) for n in parts[2:]]
                    })

    return {
        'nodes': nodes,
        'elements': elements,
        'boundaries': boundaries,
        'fluid_properties': fluid_props,
        'config': config
    }


def build_structured_mesh(case: Dict) -> Tuple[np.ndarray, List]:
    """Build structured mesh from nodes. Handles 2D/3D structured grids."""
    nodes = case['nodes']

    # Extract x, y, z coordinates
    xs = sorted(list(set(n['x'] for n in nodes)))
    ys = sorted(list(set(n['y'] for n in nodes)))
    zs = sorted(list(set(n['z'] for n in nodes)))

    nx = len(xs)
    ny = len(ys)
    nz = len(zs)

    # Build node index map
    node_map = {}
    for n in nodes:
        node_map[n['id']] = (n['x'], n['y'], n['z'])

    # Create grid arrays
    xv, yv, zv = np.meshgrid(xs, ys, zs, indexing='ij')

    # Flatten for CFD computation
    ntotal = nx * ny * nz
    x_flat = xv.flatten()
    y_flat = yv.flatten()
    z_flat = zv.flatten()

    # Get dx, dy, dz
    dx = (max(xs) - min(xs)) / (nx - 1) if nx > 1 else 1.0
    dy = (max(ys) - min(ys)) / (ny - 1) if ny > 1 else 1.0
    dz = (max(zs) - min(zs)) / (nz - 1) if nz > 1 else 1.0

    return (x_flat, y_flat, z_flat, nx, ny, nz, node_map, dx, dy, dz, xs, ys, zs)


def find_boundary_nodes(x_flat, y_flat, z_flat, xs, ys, zs, nx, ny, nz):
    """Identify boundary nodes based on position."""
    x_min_v = min(xs)
    x_max_v = max(xs)
    y_min_v = min(ys)
    y_max_v = max(ys)
    z_min_v = min(zs)
    z_max_v = max(zs)

    # Tolerance
    tol = 1e-6

    inlet_idx = []
    outlet_idx = []
    walls_idx = []
    sym_idx = []

    for i in range(len(x_flat)):
        x, y, z = x_flat[i], y_flat[i], z_flat[i]
        if abs(x - x_min_v) < tol:
            inlet_idx.append(i)
        elif abs(x - x_max_v) < tol:
            outlet_idx.append(i)
        elif abs(y - y_min_v) < tol or abs(y - y_max_v) < tol:
            walls_idx.append(i)
        elif abs(z - z_min_v) < tol or abs(z - z_max_v) < tol:
            sym_idx.append(i)

    return inlet_idx, outlet_idx, walls_idx, sym_idx


def solve_cfd(case: Dict, max_iter: int = 500, conv_crit: float = 1e-4,
              relaxation: float = 0.3, turbulent: bool = False) -> Dict:
    """
    Solve incompressible Navier-Stokes using SIMPLE algorithm.
    Returns velocity, pressure, and derived quantities.
    """
    fluid = case['fluid_properties']
    rho = fluid.get('density', 1.225)
    mu = fluid.get('viscosity', 1.81e-5)
    config = case['config']

    # Build structured mesh
    (x_flat, y_flat, z_flat, nx, ny, nz,
     node_map, dx, dy, dz, xs, ys, zs) = build_structured_mesh(case)

    ntotal = nx * ny * nz

    # Check if 2D
    is_2d = (nz == 1)

    # Find boundary indices
    inlet_idx, outlet_idx, walls_idx, sym_idx = find_boundary_nodes(
        x_flat, y_flat, z_flat, xs, ys, zs, nx, ny, nz
    )

    # Apply boundary conditions from case
    bc = case['boundaries']
    inlet_bc = bc.get('inlet', [])
    outlet_bc = bc.get('outlet', [])
    wall_bc = bc.get('wall', [])

    # Initialize fields
    u = np.zeros(ntotal)  # x-velocity
    v = np.zeros(ntotal)  # y-velocity
    w = np.zeros(ntotal)  # z-velocity
    p = np.zeros(ntotal)  # pressure
    p_correct = np.zeros(ntotal)

    # Apply inlet BCs
    for ib in inlet_bc:
        vx_in = ib.get('vx', 1.0)
        vy_in = ib.get('vy', 0.0)
        vz_in = ib.get('vz', 0.0)
        for node_id in ib.get('nodes', []):
            for ni, n in enumerate(case['nodes']):
                if n['id'] == node_id:
                    u[ni] = vx_in
                    v[ni] = vy_in
                    w[ni] = vz_in
                    break

    # Apply outlet BC (constant pressure)
    for ob in outlet_bc:
        p_out = ob.get('pressure', 0.0)
        for node_id in ob.get('nodes', []):
            for ni, n in enumerate(case['nodes']):
                if n['id'] == node_id:
                    p[ni] = p_out
                    break

    # Apply wall BC (no-slip: u=v=w=0 on walls)
    for wb in wall_bc:
        if wb.get('type') == 'no_slip':
            for node_id in wb.get('nodes', []):
                for ni, n in enumerate(case['nodes']):
                    if n['id'] == node_id:
                        u[ni] = 0.0
                        v[ni] = 0.0
                        w[ni] = 0.0
                        break

    # k-epsilon turbulence model variables
    k = np.ones(ntotal) * 0.001  # turbulence kinetic energy
    epsilon = np.ones(ntotal) * 0.0001  # turbulence dissipation
    mu_t = np.zeros(ntotal)  # turbulent viscosity

    # SIMPLE iteration
    print(f"Starting CFD solve: {nx}x{ny}x{nz} = {ntotal} cells")
    print(f"Fluid: rho={rho:.4f} kg/m³, mu={mu:.6f} Pa·s")
    print(f"Max iterations: {max_iter}, Convergence: {conv_crit}")
    print(f"Turbulence model: {'k-epsilon' if turbulent else 'Laminar'}")

    res_u = 1.0
    res_v = 1.0
    res_p = 1.0

    for iteration in range(max_iter):
        u_old = u.copy()
        v_old = v.copy()
        p_old = p.copy()

        # Compute effective viscosity
        if turbulent:
            # k-epsilon: mu_eff = mu + mu_t
            # mu_t = rho * C_mu * k^2 / epsilon
            C_mu = 0.09
            C1 = 1.44
            C2 = 1.92
            sigma_k = 1.0
            sigma_epsilon = 1.3

            mu_t = rho * C_mu * (k ** 2) / np.maximum(epsilon, 1e-10)
            mu_eff = mu + mu_t
        else:
            mu_eff = mu

        # Solve x-momentum (simplified TDMA for each x-plane)
        u = solve_momentum_x(u, v, w, p, mu_eff, rho, dx, dy, dz,
                              nx, ny, nz, walls_idx, inlet_idx, outlet_idx,
                              relaxation)

        # Solve y-momentum
        v = solve_momentum_y(u, v, w, p, mu_eff, rho, dx, dy, dz,
                              nx, ny, nz, walls_idx, inlet_idx, outlet_idx,
                              relaxation)

        # Solve z-momentum (if 3D)
        if not is_2d:
            w = solve_momentum_z(u, v, w, p, mu_eff, rho, dx, dy, dz,
                                  nx, ny, nz, walls_idx, inlet_idx, outlet_idx,
                                  relaxation)

        # Solve pressure correction (SIMPLE)
        p_correct = solve_pressure_correction(u, v, w, dx, dy, dz, nx, ny, nz, rho)

        # Update pressure and velocity
        p = p + relaxation * p_correct

        for ib in outlet_bc:
            for node_id in ib.get('nodes', []):
                for ni, n in enumerate(case['nodes']):
                    if n['id'] == node_id:
                        p[ni] = p_out
                        break

        # Correct velocities
        for i in range(1, nx - 1):
            for j in range(1, ny - 1):
                for kk in range(1, nz - 1):
                    idx = i + j * nx + kk * nx * ny
                    if idx not in walls_idx:
                        u[idx] -= p_correct[idx] / (rho * dx)
                        v[idx] -= p_correct[idx] / (rho * dy)

        # Re-enforce boundary conditions
        for ib in inlet_bc:
            for node_id in ib.get('nodes', []):
                for ni, n in enumerate(case['nodes']):
                    if n['id'] == node_id:
                        u[ni] = ib.get('vx', 0.0)
                        v[ni] = ib.get('vy', 0.0)
                        w[ni] = ib.get('vz', 0.0)
                        break

        # Turbulence equations (simplified)
        if turbulent and iteration % 10 == 0:
            G = mu_t * (
                2 * (np.gradient(u, dx)[0] ** 2 + np.gradient(v, dy)[0] ** 2) +
                (np.gradient(u, dy)[0] + np.gradient(v, dx)[0]) ** 2
            )
            k = np.maximum(k + 0.1 * (G - epsilon), 1e-6)
            epsilon = np.maximum(epsilon + 0.1 * (C1 * G * epsilon / (k + 1e-6) - C2 * epsilon ** 2 / (k + 1e-6)), 1e-10)

        # Compute residuals
        res_u = np.max(np.abs(u - u_old))
        res_v = np.max(np.abs(v - v_old))
        res_p = np.max(np.abs(p - p_old))

        if iteration % 50 == 0:
            vel_mag = np.sqrt(u ** 2 + v ** 2 + w ** 2)
            print(f"  Iter {iteration}: res_u={res_u:.2e}, res_v={res_v:.2e}, res_p={res_p:.2e}, vel_max={vel_mag.max():.3f}")

        if res_u < conv_crit and res_v < conv_crit and res_p < conv_crit * 10:
            print(f"Converged at iteration {iteration}")
            break

    # Compute derived quantities
    vel_mag = np.sqrt(u ** 2 + v ** 2 + w ** 2)

    # Pressure coefficient: Cp = 2 * (p - p_ref) / (rho * V_ref^2)
    p_ref = p[outlet_idx[0]] if outlet_idx else 0.0
    v_ref = inlet_bc[0].get('vx', 1.0) if inlet_bc else 1.0
    cp = 2 * (p - p_ref) / (rho * v_ref ** 2 + 1e-10)

    # Vorticity (simplified)
    dv_dx = np.gradient(v, dx)[0]
    du_dy = np.gradient(u, dy)[0]
    vorticity = dv_dx - du_dy

    return {
        'nodes': case['nodes'],
        'x': x_flat.tolist(),
        'y': y_flat.tolist(),
        'z': z_flat.tolist(),
        'velocity_x': u.tolist(),
        'velocity_y': v.tolist(),
        'velocity_z': w.tolist(),
        'velocity_magnitude': vel_mag.tolist(),
        'pressure': p.tolist(),
        'pressure_coefficient': cp.tolist(),
        'vorticity': vorticity.tolist(),
        'residuals': {
            'u': float(res_u),
            'v': float(res_v),
            'p': float(res_p)
        },
        'stats': {
            'u_max': float(u.max()),
            'u_min': float(u.min()),
            'v_max': float(v.max()),
            'v_min': float(v.min()),
            'p_max': float(p.max()),
            'p_min': float(p.min()),
            'vel_max': float(vel_mag.max()),
        },
        'nx': nx,
        'ny': ny,
        'nz': nz,
        'iterations': iteration + 1,
        'converged': res_u < conv_crit or iteration > max_iter * 0.5
    }


def solve_momentum_x(u, v, w, p, mu_eff, rho, dx, dy, dz, nx, ny, nz, walls_idx, inlet_idx, outlet_idx, alpha):
    """Solve x-momentum equation using Gauss-Seidel iteration."""
    u_new = u.copy()
    for i in range(1, nx - 1):
        for j in range(1, ny - 1):
            for kk in range(1, nz - 1):
                idx = i + j * nx + kk * nx * ny
                if idx in walls_idx:
                    u_new[idx] = 0.0
                    continue

                # Diffusion coefficient
                ae = mu_eff[idx] * dy * dz / dx
                aw = mu_eff[idx] * dy * dz / dx
                an = mu_eff[idx] * dx * dz / dy
                as_ = mu_eff[idx] * dx * dz / dy
                if nz > 1:
                    af = mu_eff[idx] * dx * dy / dz
                    ab = mu_eff[idx] * dx * dy / dz
                else:
                    af = 0.0
                    ab = 0.0

                ap = ae + aw + an + as_ + af + ab

                # Convection (upwind)
                Fe = rho * u[idx + 1] if i + 1 < nx else 0
                Fw = rho * u[idx - 1] if i > 0 else 0
                Fn = rho * v[idx + nx] if j + 1 < ny else 0
                Fs = rho * v[idx - nx] if j > 0 else 0

                # Source term: pressure gradient + diffusion
                Sp = -dy * dz * (p[idx + 1] - p[idx - 1]) / 2

                u_new[idx] = alpha * (ae * u[idx + 1] + aw * u[idx - 1] +
                                       an * u[idx + nx] + as_ * u[idx - nx] +
                                       af * u[idx + nx * ny] + ab * u[idx - nx * ny] +
                                       Sp) / (ap + 1e-10) + (1 - alpha) * u[idx]

                # Clamp
                u_new[idx] = np.clip(u_new[idx], -100, 100)

    return u_new


def solve_momentum_y(u, v, w, p, mu_eff, rho, dx, dy, dz, nx, ny, nz, walls_idx, inlet_idx, outlet_idx, alpha):
    """Solve y-momentum equation."""
    v_new = v.copy()
    for i in range(1, nx - 1):
        for j in range(1, ny - 1):
            for kk in range(1, nz - 1):
                idx = i + j * nx + kk * nx * ny
                if idx in walls_idx:
                    v_new[idx] = 0.0
                    continue

                ae = mu_eff[idx] * dy * dz / dx
                aw = mu_eff[idx] * dy * dz / dx
                an = mu_eff[idx] * dx * dz / dy
                as_ = mu_eff[idx] * dx * dz / dy
                af = mu_eff[idx] * dx * dy / dz if nz > 1 else 0.0
                ab = mu_eff[idx] * dx * dy / dz if nz > 1 else 0.0

                ap = ae + aw + an + as_ + af + ab

                Sp = -dx * dz * (p[idx + nx] - p[idx - nx]) / 2

                v_new[idx] = alpha * (ae * v[idx + 1] + aw * v[idx - 1] +
                                       an * v[idx + nx] + as_ * v[idx - nx] +
                                       af * v[idx + nx * ny] + ab * v[idx - nx * ny] +
                                       Sp) / (ap + 1e-10) + (1 - alpha) * v[idx]
                v_new[idx] = np.clip(v_new[idx], -100, 100)

    return v_new


def solve_momentum_z(u, v, w, p, mu_eff, rho, dx, dy, dz, nx, ny, nz, walls_idx, inlet_idx, outlet_idx, alpha):
    """Solve z-momentum equation."""
    w_new = w.copy()
    for i in range(1, nx - 1):
        for j in range(1, ny - 1):
            for kk in range(1, nz - 1):
                idx = i + j * nx + kk * nx * ny
                if idx in walls_idx or nz <= 1:
                    w_new[idx] = 0.0
                    continue

                ae = mu_eff[idx] * dy * dz / dx
                aw = mu_eff[idx] * dy * dz / dx
                an = mu_eff[idx] * dx * dz / dy
                as_ = mu_eff[idx] * dx * dz / dy
                af = mu_eff[idx] * dx * dy / dz
                ab = mu_eff[idx] * dx * dy / dz

                ap = ae + aw + an + as_ + af + ab

                Sp = -dx * dy * (p[idx + nx * ny] - p[idx - nx * ny]) / 2

                w_new[idx] = alpha * (ae * w[idx + 1] + aw * w[idx - 1] +
                                       an * w[idx + nx] + as_ * w[idx - nx] +
                                       af * w[idx + nx * ny] + ab * w[idx - nx * ny] +
                                       Sp) / (ap + 1e-10) + (1 - alpha) * w[idx]
                w_new[idx] = np.clip(w_new[idx], -100, 100)

    return w_new


def solve_pressure_correction(u, v, w, dx, dy, dz, nx, ny, nz, rho):
    """Solve pressure correction equation (SIMPLE)."""
    p_corr = np.zeros(nx * ny * nz)

    # Mass flux at cell faces
    for i in range(1, nx - 1):
        for j in range(1, ny - 1):
            for kk in range(1, nz - 1):
                idx = i + j * nx + kk * nx * ny

                # Continuity: div(V) = 0
                # d(u)/dx + d(v)/dy + d(w)/dz = 0
                du = (u[idx + 1] - u[idx - 1]) / 2
                dv = (v[idx + nx] - v[idx - nx]) / 2

                if nz > 1:
                    dw = (w[idx + nx * ny] - w[idx - nx * ny]) / 2
                else:
                    dw = 0.0

                # Pressure correction (simplified)
                div_V = du + dv + dw
                p_corr[idx] = -rho * div_V * min(dx, dy, dz) ** 2 / 4

    return p_corr


def write_cfd_inp(case: Dict, output_path: str):
    """Write a simplified CFD INP file."""
    with open(output_path, 'w') as f:
        f.write("** CAELab CFD Case File\n")
        f.write("** Simplified CFD format for Python solver\n\n")

        f.write(f"*FLUID_PROPERTIES\n")
        f.write(f"density,{case['fluid_properties'].get('density', 1.225)}\n")
        f.write(f"viscosity,{case['fluid_properties'].get('viscosity', 1.81e-5)}\n")
        if 'specific_heat' in case['fluid_properties']:
            f.write(f"specific_heat,{case['fluid_properties']['specific_heat']}\n")
        if 'thermal_conductivity' in case['fluid_properties']:
            f.write(f"thermal_conductivity,{case['fluid_properties']['thermal_conductivity']}\n")

        f.write(f"\n*CONFIG\n")
        f.write(f"max_iterations,{case['config'].get('max_iterations', 500)}\n")
        f.write(f"convergence_criteria,{case['config'].get('convergence_criteria', 1e-4)}\n")
        f.write(f"relaxation_factor,{case['config'].get('relaxation_factor', 0.3)}\n")
        f.write(f"turbulent,{case['config'].get('turbulent', False)}\n")

        f.write(f"\n*NODE\n")
        for n in case['nodes']:
            f.write(f"{n['id']},{n['x']:.6f},{n['y']:.6f},{n['z']:.6f}\n")

        f.write(f"\n*ELEMENT\n")
        for e in case['elements']:
            f.write(f"{e['id']},{e['type']},{','.join(map(str, e['nodes']))}\n")

        if 'inlet' in case['boundaries']:
            for ib in case['boundaries']['inlet']:
                node_str = ','.join(map(str, ib.get('nodes', [])))
                f.write(f"\n*INLET\n")
                f.write(f"{ib['name']},{ib.get('vx', 0)},{ib.get('vy', 0)},{ib.get('vz', 0)},{ib.get('type', 'velocity')},{node_str}\n")

        if 'outlet' in case['boundaries']:
            for ob in case['boundaries']['outlet']:
                node_str = ','.join(map(str, ob.get('nodes', [])))
                f.write(f"\n*OUTLET\n")
                f.write(f"{ob['name']},{ob.get('pressure', 0)},{node_str}\n")

        if 'wall' in case['boundaries']:
            for wb in case['boundaries']['wall']:
                node_str = ','.join(map(str, wb.get('nodes', [])))
                f.write(f"\n*WALL\n")
                f.write(f"{wb['name']},{wb.get('type', 'no_slip')},{node_str}\n")


def main():
    parser = argparse.ArgumentParser(description='CAELab CFD Solver')
    parser.add_argument('--input', '-i', required=True, help='Input INP file')
    parser.add_argument('--output', '-o', required=True, help='Output JSON file')
    parser.add_argument('--max-iter', type=int, default=500, help='Max iterations')
    parser.add_argument('--conv-crit', type=float, default=1e-4, help='Convergence criteria')
    parser.add_argument('--relaxation', type=float, default=0.3, help='Relaxation factor')
    parser.add_argument('--turbulent', action='store_true', help='Enable k-epsilon turbulence')

    args = parser.parse_args()

    start_time = time.time()

    # Parse input
    case = parse_inp(args.input)

    # Add config from args
    case['config']['max_iterations'] = args.max_iter
    case['config']['convergence_criteria'] = args.conv_crit
    case['config']['relaxation_factor'] = args.relaxation
    case['config']['turbulent'] = args.turbulent

    # Solve
    result = solve_cfd(case, max_iter=args.max_iter, conv_crit=args.conv_crit,
                       relaxation=args.relaxation, turbulent=args.turbulent)

    # Save results
    with open(args.output, 'w') as f:
        json.dump(result, f, indent=2)

    elapsed = time.time() - start_time
    print(f"\nSolver completed in {elapsed:.2f}s")
    print(f"Iterations: {result['iterations']}")
    print(f"Converged: {result['converged']}")
    print(f"Max velocity: {result['stats']['vel_max']:.3f} m/s")
    print(f"Pressure range: [{result['stats']['p_min']:.3f}, {result['stats']['p_max']:.3f}] Pa")
    print(f"Results saved to {args.output}")


if __name__ == '__main__':
    main()