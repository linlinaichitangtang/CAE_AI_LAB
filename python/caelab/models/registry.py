"""
模型注册表 — 定义所有预训练模型的元数据、下载源和推理接口。
"""

from __future__ import annotations

import os
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any


@dataclass
class ModelMeta:
    """单个预训练模型的元数据"""

    name: str
    display_name: str
    description: str
    model_type: str  # "universal_potential" | "property_predictor" | "force_field"
    framework: str  # "mace" | "chgnet" | "m3gnet" | "nequip" | "sevennet"
    pip_package: str  # pip 包名
    import_module: str  # Python 导入路径
    supported_elements: list[str] = field(default_factory=list)
    supported_properties: list[str] = field(default_factory=list)
    model_size_mb: float = 0.0
    download_url: str | None = None
    version: str = "latest"

    @property
    def is_potential(self) -> bool:
        return self.model_type == "universal_potential"


# ============================================================================
# 模型定义表
# ============================================================================

MODEL_DEFS: dict[str, ModelMeta] = {
    "chgnet": ModelMeta(
        name="chgnet",
        display_name="CHGNet",
        description="电荷图神经网络势函数，通用材料性质预测，支持结构优化和分子动力学",
        model_type="universal_potential",
        framework="chgnet",
        pip_package="chgnet",
        import_module="chgnet",
        supported_elements=["H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne",
                            "Na", "Mg", "Al", "Si", "P", "S", "Cl", "Ar",
                            "K", "Ca", "Sc", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn",
                            "Ga", "Ge", "As", "Se", "Br", "Kr",
                            "Rb", "Sr", "Y", "Zr", "Nb", "Mo", "Tc", "Ru", "Rh", "Pd", "Ag", "Cd",
                            "In", "Sn", "Sb", "Te", "I", "Xe",
                            "Cs", "Ba", "La", "Ce", "Pr", "Nd", "Pm", "Sm", "Eu", "Gd", "Tb", "Dy",
                            "Ho", "Er", "Tm", "Yb", "Lu", "Hf", "Ta", "W", "Re", "Os", "Ir", "Pt",
                            "Au", "Hg", "Tl", "Pb", "Bi"],
        supported_properties=["energy", "forces", "stress", "magmoms"],
        model_size_mb=85.0,
        version="0.3.0",
    ),
    "m3gnet": ModelMeta(
        name="m3gnet",
        display_name="M3GNet",
        description="材料图神经网络势函数，用于通用材料性质和分子动力学",
        model_type="universal_potential",
        framework="m3gnet",
        pip_package="matgl",
        import_module="matgl",
        supported_elements=["Li", "Be", "B", "C", "N", "O", "F", "Na", "Mg", "Al", "Si", "P", "S",
                            "Cl", "K", "Ca", "Sc", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu",
                            "Zn", "Ga", "Ge", "As", "Se", "Rb", "Sr", "Y", "Zr", "Nb", "Mo", "Ru",
                            "Rh", "Pd", "Ag", "Cd", "In", "Sn", "Sb", "Te", "Cs", "Ba", "La", "Ce",
                            "Hf", "Ta", "W", "Re", "Os", "Ir", "Pt", "Au", "Pb", "Bi"],
        supported_properties=["energy", "forces", "stress"],
        model_size_mb=120.0,
        version="1.0.0",
    ),
    "mace": ModelMeta(
        name="mace",
        display_name="MACE",
        description="高阶等变消息传递势函数，精度接近 DFT，速度快 1000x",
        model_type="universal_potential",
        framework="mace",
        pip_package="mace-torch",
        import_module="mace",
        supported_elements=["H", "Li", "B", "C", "N", "O", "F", "Na", "Mg", "Al", "Si", "P", "S",
                            "Cl", "K", "Ca", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn",
                            "Ga", "Ge", "Se", "Br", "Sr", "Zr", "Nb", "Mo", "Pd", "Ag", "Sn", "Sb",
                            "Te", "I", "Ba", "W", "Pt", "Au", "Pb", "Bi"],
        supported_properties=["energy", "forces", "stress"],
        model_size_mb=45.0,
        version="0.3.6",
    ),
    "nequip": ModelMeta(
        name="nequip",
        display_name="NequIP",
        description="等变神经网络势函数，高精度分子动力学",
        model_type="universal_potential",
        framework="nequip",
        pip_package="nequip",
        import_module="nequip",
        supported_elements=["H", "C", "N", "O", "F", "Si", "S", "Cl", "Cu", "Ge", "As", "Se", "Br",
                            "Mo", "Te", "I", "W", "Pt", "Au", "Pb"],
        supported_properties=["energy", "forces", "stress"],
        model_size_mb=30.0,
        version="0.6.0",
    ),
    "sevennet": ModelMeta(
        name="sevennet",
        display_name="SevenNet",
        description="七阶等变神经网络势函数，高精度通用势",
        model_type="universal_potential",
        framework="sevennet",
        pip_package="sevenn",
        import_module="sevenn",
        supported_elements=["H", "Li", "B", "C", "N", "O", "F", "Na", "Mg", "Al", "Si", "P", "S",
                            "Cl", "K", "Ca", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn",
                            "Ga", "Ge", "As", "Se", "Br", "Rb", "Sr", "Zr", "Nb", "Mo", "Pd", "Ag",
                            "Cd", "In", "Sn", "Sb", "Te", "I", "Cs", "Ba", "La", "Hf", "Ta", "W",
                            "Re", "Os", "Ir", "Pt", "Au", "Hg", "Tl", "Pb", "Bi"],
        supported_properties=["energy", "forces", "stress"],
        model_size_mb=95.0,
        version="1.0.0",
    ),
}


def get_model_def(name: str) -> ModelMeta | None:
    return MODEL_DEFS.get(name)


def list_all_models() -> list[ModelMeta]:
    return list(MODEL_DEFS.values())
