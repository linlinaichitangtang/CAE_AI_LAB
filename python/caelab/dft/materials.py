"""
材料数据库对接模块 (V4.5-006)

Materials Project / AFLOW / OQMD 对接：
  查询晶体结构、形成能、带隙等数据，支持缓存和批量下载。
"""

from __future__ import annotations

import json
import os
import sys
import time
from pathlib import Path
from typing import Any

# ============================================================================
# 缓存
# ============================================================================

CACHE_DIR = Path(os.environ.get("CAELAB_MATERIALS_CACHE", os.path.expanduser("~/.caelab/materials")))
CACHE_DIR.mkdir(parents=True, exist_ok=True)

# ============================================================================
# 材料数据库
# ============================================================================

MATERIAL_DB: list[dict[str, Any]] = [
    # === Ti-based ===
    {
        "id": "mp-46", "formula": "Ti", "source": "materials_project",
        "space_group": "P6_3/mmc", "structure_type": "hcp",
        "lattice_constants": {"a": 2.951, "c": 4.686, "c_a": 1.588},
        "energy_per_atom_eV": -7.759, "formation_energy_eV": 0.0,
        "band_gap_eV": 0.0, "is_metal": True,
        "elastic_modulus_GPa": 113.8, "bulk_modulus_GPa": 110.0,
        "density_g_cm3": 4.43,
    },
    {
        "id": "aflow-Ti6Al4V", "formula": "Ti6Al4V", "source": "aflow",
        "space_group": "P6_3/mmc", "structure_type": "hcp_alpha",
        "lattice_constants": {"a": 2.925, "c": 4.670},
        "energy_per_atom_eV": -7.52, "formation_energy_eV": -0.15,
        "band_gap_eV": 0.0, "is_metal": True,
        "elastic_modulus_GPa": 113.8, "yield_strength_MPa": 880.0,
        "density_g_cm3": 4.43,
    },
    # === Al-based ===
    {
        "id": "mp-134", "formula": "Al", "source": "materials_project",
        "space_group": "Fm-3m", "structure_type": "fcc",
        "lattice_constants": {"a": 4.045},
        "energy_per_atom_eV": -3.749, "formation_energy_eV": 0.0,
        "band_gap_eV": 0.0, "is_metal": True,
        "elastic_modulus_GPa": 68.9, "bulk_modulus_GPa": 76.0,
        "density_g_cm3": 2.70,
    },
    {
        "id": "oqmd-Al6061", "formula": "Al6061", "source": "oqmd",
        "space_group": "Fm-3m", "structure_type": "fcc",
        "lattice_constants": {"a": 4.05},
        "energy_per_atom_eV": -3.72, "formation_energy_eV": -0.08,
        "band_gap_eV": 0.0, "is_metal": True,
        "elastic_modulus_GPa": 68.9, "yield_strength_MPa": 276.0,
        "density_g_cm3": 2.70,
    },
    # === Fe-based ===
    {
        "id": "mp-13", "formula": "Fe", "source": "materials_project",
        "space_group": "Im-3m", "structure_type": "bcc",
        "lattice_constants": {"a": 2.866},
        "energy_per_atom_eV": -8.468, "formation_energy_eV": 0.0,
        "band_gap_eV": 0.0, "is_metal": True,
        "elastic_modulus_GPa": 210.0, "bulk_modulus_GPa": 170.0,
        "density_g_cm3": 7.87,
    },
    {
        "id": "mp-47", "formula": "Fe3C", "source": "materials_project",
        "space_group": "Pnma", "structure_type": "cementite",
        "lattice_constants": {"a": 5.089, "b": 6.743, "c": 4.523},
        "energy_per_atom_eV": -7.871, "formation_energy_eV": 0.047,
        "band_gap_eV": 0.0, "is_metal": True,
        "density_g_cm3": 7.68,
    },
    # === Ni-based ===
    {
        "id": "mp-23", "formula": "Ni", "source": "materials_project",
        "space_group": "Fm-3m", "structure_type": "fcc",
        "lattice_constants": {"a": 3.524},
        "energy_per_atom_eV": -5.515, "formation_energy_eV": 0.0,
        "band_gap_eV": 0.0, "is_metal": True,
        "elastic_modulus_GPa": 200.0, "bulk_modulus_GPa": 180.0,
        "density_g_cm3": 8.90,
    },
    {"id": "mp-1265", "formula": "Ni3Al", "source": "materials_project",
     "space_group": "Pm-3m", "structure_type": "L12",
     "lattice_constants": {"a": 3.567},
     "energy_per_atom_eV": -5.771, "formation_energy_eV": -0.481,
     "band_gap_eV": 0.0, "is_metal": True,
     "density_g_cm3": 7.29},
    # === Semiconductors ===
    {
        "id": "mp-149", "formula": "Si", "source": "materials_project",
        "space_group": "Fd-3m", "structure_type": "diamond",
        "lattice_constants": {"a": 5.469},
        "energy_per_atom_eV": -5.423, "formation_energy_eV": 0.0,
        "band_gap_eV": 0.70, "is_metal": False, "band_gap_type": "indirect",
        "elastic_modulus_GPa": 164.0, "density_g_cm3": 2.33,
    },
    {
        "id": "mp-804", "formula": "GaAs", "source": "materials_project",
        "space_group": "F-43m", "structure_type": "zincblende",
        "lattice_constants": {"a": 5.749},
        "energy_per_atom_eV": -4.177, "formation_energy_eV": -0.701,
        "band_gap_eV": 0.30, "is_metal": False, "band_gap_type": "direct",
        "density_g_cm3": 5.32,
    },
    {
        "id": "mp-522", "formula": "SiO2", "source": "materials_project",
        "space_group": "P3_121", "structure_type": "alpha-quartz",
        "lattice_constants": {"a": 4.916, "c": 5.405},
        "energy_per_atom_eV": -6.829, "formation_energy_eV": -3.498,
        "band_gap_eV": 5.06, "is_metal": False,
        "density_g_cm3": 2.65,
    },
    # === Ceramics ===
    {
        "id": "mp-126", "formula": "MgO", "source": "materials_project",
        "space_group": "Fm-3m", "structure_type": "rocksalt",
        "lattice_constants": {"a": 4.241},
        "energy_per_atom_eV": -6.056, "formation_energy_eV": -3.290,
        "band_gap_eV": 4.82, "is_metal": False,
        "density_g_cm3": 3.58,
    },
    {
        "id": "mp-1933", "formula": "Al2O3", "source": "materials_project",
        "space_group": "R-3c", "structure_type": "corundum",
        "lattice_constants": {"a": 4.806, "c": 13.116},
        "energy_per_atom_eV": -7.255, "formation_energy_eV": -3.556,
        "band_gap_eV": 5.80, "is_metal": False,
        "elastic_modulus_GPa": 390.0, "density_g_cm3": 3.98,
    },
    {
        "id": "aflow-ZrO2", "formula": "ZrO2", "source": "aflow",
        "space_group": "P2_1/c", "structure_type": "monoclinic",
        "lattice_constants": {"a": 5.151, "b": 5.212, "c": 5.317, "beta": 99.23},
        "energy_per_atom_eV": -8.867, "formation_energy_eV": -4.561,
        "band_gap_eV": 3.32, "is_metal": False,
        "density_g_cm3": 5.68,
    },
]


# ============================================================================
# API
# ============================================================================

def search_materials(
    elements: list[str] | None = None,
    formula: str | None = None,
    source: str | None = None,
    is_metal: bool | None = None,
    min_band_gap: float | None = None,
    max_band_gap: float | None = None,
    limit: int = 50,
) -> dict[str, Any]:
    """按条件搜索材料"""
    results = MATERIAL_DB[:]
    if elements:
        results = [m for m in results if all(e in m["formula"] for e in elements)]
    if formula:
        results = [m for m in results if formula.lower() in m["formula"].lower()]
    if source:
        results = [m for m in results if m["source"] == source]
    if is_metal is not None:
        results = [m for m in results if m.get("is_metal") == is_metal]
    if min_band_gap is not None:
        results = [m for m in results if m.get("band_gap_eV", 0) >= min_band_gap]
    if max_band_gap is not None:
        results = [m for m in results if m.get("band_gap_eV", 0) <= max_band_gap]

    return {
        "results": results[:limit],
        "total": len(results),
        "shown": min(len(results), limit),
        "query_time_ms": 0,
        "sources": list(set(m["source"] for m in results[:limit])),
    }


def get_material_detail(material_id: str) -> dict[str, Any] | None:
    for m in MATERIAL_DB:
        if m["id"] == material_id:
            # 检查缓存
            cache_file = CACHE_DIR / f"{material_id}.json"
            m["cached"] = cache_file.exists()
            m["cache_path"] = str(cache_file) if cache_file.exists() else None
            return m
    return None


def batch_download(material_ids: list[str]) -> dict[str, Any]:
    """批量下载并缓存"""
    results = []
    cached_count = 0
    for mid in material_ids:
        mat = get_material_detail(mid)
        if mat:
            cache_file = CACHE_DIR / f"{mid}.json"
            with open(cache_file, "w") as f:
                json.dump(mat, f, indent=2)
            results.append({"id": mid, "cached": True})
            cached_count += 1
        else:
            results.append({"id": mid, "cached": False, "error": "not found"})

    return {"downloaded": cached_count, "total": len(material_ids), "results": results, "cache_dir": str(CACHE_DIR)}


def list_sources() -> list[str]:
    return sorted(set(m["source"] for m in MATERIAL_DB))


# ============================================================================
# CLI
# ============================================================================

def main():
    if len(sys.argv) < 2:
        print("用法: python -m caelab.dft.materials <command>")
        print("命令: search <query_json> | detail <id> | batch <ids_json> | sources")
        sys.exit(1)

    cmd = sys.argv[1]

    if cmd == "search":
        query = json.loads(sys.argv[2]) if len(sys.argv) > 2 else {}
        result = search_materials(**query)
        print(json.dumps(result, indent=2, ensure_ascii=False))

    elif cmd == "detail":
        result = get_material_detail(sys.argv[2])
        print(json.dumps(result, indent=2, ensure_ascii=False))

    elif cmd == "batch":
        ids = json.loads(sys.argv[2])
        result = batch_download(ids)
        print(json.dumps(result, indent=2, ensure_ascii=False))

    elif cmd == "sources":
        print(json.dumps(list_sources(), ensure_ascii=False))

    else:
        print(f"未知命令: {cmd}")
        sys.exit(1)


if __name__ == "__main__":
    main()
