"""
热物性参数库 (V4.7-006)

内置常用工程合金的热容、热膨胀、热导率、密度随温度变化数据。
"""
from __future__ import annotations
import json, sys, numpy as np

# 材料热物性数据库: {name: {property: [T_min, T_max, [coeffs]]}}
# 多项式: prop(T) = c0 + c1*T + c2*T² + c3*T³
MATERIALS: dict[str, dict] = {
    "Al_1100": {"density_g_cm3": [2.71], "thermal_expansion_1e6_K": [20, 300, [18, 0.015, 0, 0]],
                "thermal_conductivity_W_mK": [100, 600, [230, -0.05, 0, 0]],
                "specific_heat_J_kgK": [100, 800, [880, 0.001, 0, 0]],
                "melting_point_K": 933, "elastic_modulus_GPa": [0, 500, [69, -0.02, 0, 0]]},
    "Al_6061": {"density_g_cm3": [2.70], "thermal_expansion_1e6_K": [20, 300, [23.6, 0, 0, 0]],
                "thermal_conductivity_W_mK": [100, 600, [167, -0.03, 0, 0]],
                "specific_heat_J_kgK": [100, 700, [896, 0.0005, 0, 0]],
                "melting_point_K": 855, "elastic_modulus_GPa": [0, 500, [69, -0.015, 0, 0]],
                "yield_strength_MPa": 276},
    "Ti_6Al4V": {"density_g_cm3": [4.43], "thermal_expansion_1e6_K": [20, 800, [8.6, 0.005, 0, 0]],
                 "thermal_conductivity_W_mK": [100, 800, [6.7, 0.007, 0, 0]],
                 "specific_heat_J_kgK": [100, 800, [526, 0.0008, 0, 0]],
                 "melting_point_K": 1923, "elastic_modulus_GPa": [0, 600, [114, -0.04, 0, 0]],
                 "yield_strength_MPa": 880},
    "Steel_A36": {"density_g_cm3": [7.85], "thermal_expansion_1e6_K": [20, 600, [11.7, 0.005, 0, 0]],
                  "thermal_conductivity_W_mK": [100, 800, [50, -0.02, 0, 0]],
                  "specific_heat_J_kgK": [100, 800, [470, 0.0006, 0, 0]],
                  "melting_point_K": 1800, "elastic_modulus_GPa": [0, 600, [210, -0.05, 0, 0]],
                  "yield_strength_MPa": 250},
    "SS304": {"density_g_cm3": [8.00], "thermal_expansion_1e6_K": [20, 800, [17.3, 0.004, 0, 0]],
              "thermal_conductivity_W_mK": [100, 800, [16.2, 0.01, 0, 0]],
              "specific_heat_J_kgK": [100, 800, [500, 0.0004, 0, 0]],
              "melting_point_K": 1700, "elastic_modulus_GPa": [0, 600, [193, -0.04, 0, 0]],
              "yield_strength_MPa": 215},
    "Cu_pure": {"density_g_cm3": [8.96], "thermal_expansion_1e6_K": [20, 500, [16.7, 0.003, 0, 0]],
                "thermal_conductivity_W_mK": [100, 600, [401, -0.06, 0, 0]],
                "specific_heat_J_kgK": [100, 800, [385, 0.0003, 0, 0]],
                "melting_point_K": 1358, "elastic_modulus_GPa": [0, 500, [117, -0.03, 0, 0]],
                "yield_strength_MPa": 70},
    "Ni_superalloy": {"density_g_cm3": [8.19], "thermal_expansion_1e6_K": [20, 800, [13.0, 0.004, 0, 0]],
                      "thermal_conductivity_W_mK": [100, 800, [11, 0.012, 0, 0]],
                      "specific_heat_J_kgK": [100, 800, [435, 0.0005, 0, 0]],
                      "melting_point_K": 1623, "elastic_modulus_GPa": [0, 600, [200, -0.03, 0, 0]],
                      "yield_strength_MPa": 800},
    "Mg_AZ31": {"density_g_cm3": [1.77], "thermal_expansion_1e6_K": [20, 400, [26, 0.006, 0, 0]],
                "thermal_conductivity_W_mK": [100, 500, [96, -0.02, 0, 0]],
                "specific_heat_J_kgK": [100, 600, [1024, 0.0006, 0, 0]],
                "melting_point_K": 903, "elastic_modulus_GPa": [0, 300, [45, -0.01, 0, 0]],
                "yield_strength_MPa": 130},
    "SiC": {"density_g_cm3": [3.21], "thermal_expansion_1e6_K": [20, 800, [4.0, 0.002, 0, 0]],
            "thermal_conductivity_W_mK": [100, 800, [300, -0.3, 0.0003, 0]],
            "specific_heat_J_kgK": [100, 800, [670, 0.0007, 0, 0]],
            "melting_point_K": 3000, "elastic_modulus_GPa": [0, 700, [420, -0.02, 0, 0]]},
    "Al2O3": {"density_g_cm3": [3.98], "thermal_expansion_1e6_K": [20, 600, [8.1, 0.002, 0, 0]],
              "thermal_conductivity_W_mK": [100, 800, [35, -0.04, 0, 0]],
              "specific_heat_J_kgK": [100, 800, [880, 0.0006, 0, 0]],
              "melting_point_K": 2327, "elastic_modulus_GPa": [0, 500, [390, -0.03, 0, 0]]},
}


def get_material(material_name: str) -> dict | None:
    return MATERIALS.get(material_name)


def eval_property(material_name: str, property_name: str, T: float) -> float | None:
    mat = MATERIALS.get(material_name)
    if mat is None:
        return None
    prop = mat.get(property_name)
    if prop is None:
        return None
    if isinstance(prop, list) and len(prop) >= 2:
        params = prop
        if len(params) < 3:
            return params[0]
        T_min, T_max, coeffs = params[0], params[1], params[2]
        if T < T_min or T > T_max:
            return None
        val = coeffs[0] + coeffs[1]*T + coeffs[2]*T**2 + coeffs[3]*T**3
        return round(float(val), 4)
    return prop


def list_materials():
    return sorted(MATERIALS.keys())


def get_all_properties(material_name: str, T: float) -> dict:
    mat = MATERIALS.get(material_name)
    if mat is None:
        return {"error": f"材料不存在: {material_name}"}
    result = {"name": material_name, "temperature_K": T}
    for prop_name in mat:
        val = eval_property(material_name, prop_name, T)
        if val is not None:
            result[prop_name] = val
    return result


def main():
    if len(sys.argv) < 2:
        print("thermophysical: list | get <name> <T> | property <name> <prop> <T>")
        sys.exit(1)
    cmd = sys.argv[1]
    if cmd == "list":
        print(json.dumps(list_materials()))
    elif cmd == "get":
        name, T = sys.argv[2], float(sys.argv[3])
        print(json.dumps(get_all_properties(name, T), ensure_ascii=False))
    elif cmd == "property":
        name, prop, T = sys.argv[2], sys.argv[3], float(sys.argv[4])
        val = eval_property(name, prop, T)
        print(json.dumps({"material": name, "property": prop, "T": T, "value": val}))


if __name__ == "__main__":
    main()
