"""
TC4 Failure Analysis Module — V3.8
TC4 (Ti-6Al-4V) 钛合金多模态失效分析

三路输入：
- EBSD 晶格数据 → 晶格特征向量 (80d)
- SEM 断口图像 → 图像特征向量 (768d via ViT)
- Load 载荷历史 → 载荷特征向量 (30d)

输出：
- 失效模式分类 (HCF/LCF/TMF/CREEP/OVERLOAD/FOD)
- 疲劳寿命预测 (log10(cycles))

复用 V3.6 SEM 分析和 V3.7 SurrogateModel
"""

__version__ = "3.8.0"

from .synthesis import Tc4SyntheticDataGenerator
from .lattice_features import LatticeFeatureExtractor
from .load_features import LoadFeatureExtractor

__all__ = [
    "Tc4SyntheticDataGenerator",
    "LatticeFeatureExtractor",
    "LoadFeatureExtractor",
]
