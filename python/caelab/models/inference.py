"""
推理引擎 — 模型加载、推理、ONNX 导出的统一接口。

所有推理通过 Python 子进程或本模块函数调用，
Rust 端通过 python_bridge 调用本模块。
"""

from __future__ import annotations

import json
import os
import sys
import time
from pathlib import Path
from typing import Any

import numpy as np

from .registry import MODEL_DEFS, ModelMeta, get_model_def, list_all_models

# ============================================================================
# 模型缓存
# ============================================================================

_model_cache: dict[str, Any] = {}
_model_dir: Path | None = None


def get_model_dir() -> Path:
    """获取模型缓存目录"""
    global _model_dir
    if _model_dir is None:
        _model_dir = Path(os.environ.get(
            "CAELAB_MODEL_DIR",
            os.path.expanduser("~/.caelab/models"),
        ))
        _model_dir.mkdir(parents=True, exist_ok=True)
    return _model_dir


def _check_package_installed(package: str) -> bool:
    try:
        __import__(package.replace("-", "_"))
        return True
    except ImportError:
        return False


# ============================================================================
# 模型加载
# ============================================================================

def load_model(model_name: str) -> Any:
    """加载预训练模型，带缓存"""
    if model_name in _model_cache:
        return _model_cache[model_name]

    meta = get_model_def(model_name)
    if meta is None:
        raise ValueError(f"未知模型: {model_name}，可用模型: {list(MODEL_DEFS.keys())}")

    if not _check_package_installed(meta.pip_package):
        raise ImportError(
            f"模型 {model_name} 需要安装 {meta.pip_package}，"
            f"请运行: pip install {meta.pip_package}"
        )

    loader = _get_model_loader(meta)
    model = loader()
    _model_cache[model_name] = model
    return model


def _get_model_loader(meta: ModelMeta):
    """根据模型类型返回对应的加载函数"""
    if meta.name == "chgnet":
        return _load_chgnet
    elif meta.name == "m3gnet":
        return _load_m3gnet
    elif meta.name == "mace":
        return _load_mace
    elif meta.name == "nequip":
        return _load_nequip
    elif meta.name == "sevennet":
        return _load_sevennet
    else:
        raise ValueError(f"不支持的模型: {meta.name}")


def _load_chgnet():
    from chgnet.model import CHGNet
    return CHGNet.load()


def _load_m3gnet():
    import matgl
    return matgl.load_model("M3GNet-MP-2021.2.8-PES")


def _load_mace():
    from mace.calculators import mace_mp
    return mace_mp(model="medium", device="cpu", default_dtype="float32")


def _load_nequip():
    import torch
    from nequip.scripts.deploy import load_model
    model_dir = get_model_dir() / "nequip"
    model_path = model_dir / "nequip_model.pth"
    if not model_path.exists():
        raise FileNotFoundError(
            f"NequIP 模型文件不存在: {model_path}，"
            "请先下载模型到 ~/.caelab/models/nequip/"
        )
    return load_model(str(model_path))


def _load_sevennet():
    from sevenn.calculators import SevenNetCalculator
    return SevenNetCalculator("7net-0", device="cpu")


# ============================================================================
# 推理
# ============================================================================

def predict_structure(
    model_name: str,
    atoms_data: dict[str, Any],
    properties: list[str] | None = None,
) -> dict[str, Any]:
    """对单个结构进行推理

    Parameters
    ----------
    model_name : str
        模型名称 (chgnet / m3gnet / mace / nequip / sevennet)
    atoms_data : dict
        结构数据，包含 positions, cell, atomic_numbers
    properties : list[str] | None
        请求的属性，默认 ["energy", "forces", "stress"]

    Returns
    -------
    dict
        推理结果 {energy, forces, stress, inference_time_ms}
    """
    if properties is None:
        properties = ["energy", "forces", "stress"]

    start = time.time()
    model = load_model(model_name)
    meta = get_model_def(model_name)
    if meta is None:
        raise ValueError(f"未知模型: {model_name}")

    result = _run_inference(model, meta, atoms_data, properties)
    result["inference_time_ms"] = int((time.time() - start) * 1000)
    result["model_name"] = model_name
    result["is_mock"] = False
    return result


def _run_inference(
    model: Any,
    meta: ModelMeta,
    atoms_data: dict[str, Any],
    properties: list[str],
) -> dict[str, Any]:
    """根据模型类型执行推理"""
    atoms = _dict_to_atoms(atoms_data)

    if meta.framework == "chgnet":
        return _infer_chgnet(model, atoms, properties)
    elif meta.framework == "m3gnet":
        return _infer_m3gnet(model, atoms, properties)
    elif meta.framework in ("mace", "nequip", "sevennet"):
        return _infer_calculator(model, atoms, properties)
    else:
        raise ValueError(f"不支持的框架: {meta.framework}")


def _infer_chgnet(model, atoms, properties) -> dict[str, Any]:
    result = model.predict_structure(atoms)
    out = {}
    if "energy" in properties:
        out["energy"] = float(result.get("e", 0.0))
    if "forces" in properties:
        out["forces"] = result.get("f", np.zeros((len(atoms), 3))).tolist()
    if "stress" in properties:
        out["stress"] = result.get("s", np.zeros(6)).tolist()
    return out


def _infer_m3gnet(model, atoms, properties) -> dict[str, Any]:
    from ase.calculators.calculator import Calculator
    atoms.calc = model
    out = {}
    if "energy" in properties:
        out["energy"] = float(atoms.get_potential_energy())
    if "forces" in properties:
        out["forces"] = atoms.get_forces().tolist()
    if "stress" in properties:
        out["stress"] = atoms.get_stress().tolist()
    return out


def _infer_calculator(calc, atoms, properties) -> dict[str, Any]:
    """MACE / NequIP / SevenNet 都使用 ASE Calculator 接口"""
    atoms.calc = calc
    out = {}
    if "energy" in properties:
        out["energy"] = float(atoms.get_potential_energy())
    if "forces" in properties:
        out["forces"] = atoms.get_forces().tolist()
    if "stress" in properties:
        out["stress"] = atoms.get_stress().tolist()
    return out


def _dict_to_atoms(data: dict[str, Any]):
    """将字典转为 ASE Atoms 对象"""
    from ase import Atoms

    positions = data["positions"]
    cell = data.get("cell")
    symbols = data.get("symbols")
    atomic_numbers = data.get("atomic_numbers")

    if symbols:
        atoms = Atoms(symbols=symbols, positions=positions, cell=cell, pbc=cell is not None)
    elif atomic_numbers:
        atoms = Atoms(numbers=atomic_numbers, positions=positions, cell=cell, pbc=cell is not None)
    else:
        raise ValueError("atoms_data 必须包含 symbols 或 atomic_numbers")

    return atoms


# ============================================================================
# 状态查询
# ============================================================================

def get_model_status(model_name: str) -> dict[str, Any]:
    """获取模型安装状态"""
    meta = get_model_def(model_name)
    if meta is None:
        return {"name": model_name, "installed": False, "error": "未知模型"}

    installed = _check_package_installed(meta.pip_package)
    cached = model_name in _model_cache

    return {
        "name": meta.name,
        "display_name": meta.display_name,
        "description": meta.description,
        "model_type": meta.model_type,
        "framework": meta.framework,
        "installed": installed,
        "cached": cached,
        "package": meta.pip_package,
        "version": meta.version,
        "model_size_mb": meta.model_size_mb,
        "supported_elements": meta.supported_elements,
        "supported_properties": meta.supported_properties,
    }


def list_model_status() -> list[dict[str, Any]]:
    """获取所有模型状态"""
    return [get_model_status(m.name) for m in list_all_models()]


# ============================================================================
# ONNX 导出
# ============================================================================

def export_model_onnx(model_name: str, output_path: str) -> dict[str, Any]:
    """将模型导出为 ONNX 格式（仅 CHGNet 支持）"""
    meta = get_model_def(model_name)
    if meta is None:
        return {"success": False, "error": f"未知模型: {model_name}"}

    if meta.framework != "chgnet":
        return {
            "success": False,
            "error": f"{model_name} 暂不支持 ONNX 导出，仅 CHGNet 支持",
        }

    try:
        model = load_model(model_name)
        import torch
        dummy = _make_dummy_input()
        torch.onnx.export(
            model,
            dummy,
            output_path,
            input_names=["原子特征"],
            output_names=["能量", "力", "应力"],
            dynamic_axes={"原子特征": {0: "n_atoms"}},
        )
        return {
            "success": True,
            "output_path": output_path,
            "model_name": model_name,
            "file_size_mb": os.path.getsize(output_path) / 1e6,
        }
    except Exception as e:
        return {"success": False, "error": str(e)}


def _make_dummy_input():
    """构造 dummy 输入用于 ONNX 导出"""
    import torch
    return torch.randn(1, 64)


# ============================================================================
# CLI 入口
# ============================================================================

def main():
    """命令行入口：python -m caelab.models.inference <command> [args]"""
    if len(sys.argv) < 2:
        print("用法: python -m caelab.models.inference <command>")
        print("命令: list | status <model> | predict <model> <json> | export_onnx <model> <path>")
        sys.exit(1)

    cmd = sys.argv[1]

    if cmd == "list":
        result = list_model_status()
        print(json.dumps(result, indent=2, ensure_ascii=False))

    elif cmd == "status" and len(sys.argv) >= 3:
        result = get_model_status(sys.argv[2])
        print(json.dumps(result, indent=2, ensure_ascii=False))

    elif cmd == "predict" and len(sys.argv) >= 4:
        model_name = sys.argv[2]
        atoms_data = json.loads(sys.argv[3])
        props = json.loads(sys.argv[4]) if len(sys.argv) > 4 else None
        result = predict_structure(model_name, atoms_data, props)
        print(json.dumps(result, indent=2, ensure_ascii=False))

    elif cmd == "export_onnx" and len(sys.argv) >= 4:
        result = export_model_onnx(sys.argv[2], sys.argv[3])
        print(json.dumps(result, indent=2, ensure_ascii=False))

    else:
        print(f"未知命令: {cmd}")
        sys.exit(1)


if __name__ == "__main__":
    main()
